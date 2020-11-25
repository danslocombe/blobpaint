extern crate rand;

mod utils;
mod data_doublebuffer;
mod brush;

use wasm_bindgen::prelude::*;
use rand::prelude::*;

use std::f32::consts::TAU;
use std::collections::VecDeque;

use utils::{sqr};
use data_doublebuffer::DataDoubleBuffer;
use brush::{Brush, BrushT, BrushType};

const MAX_UNDOS : usize = 8;

#[wasm_bindgen]
extern {
  pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
  X,
  Y,
  Z,
  THRESH,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PointData {
  thresh_band : f32,
  color_band : f32,
}

impl PointData {
  fn new(thresh_band : f32, color_band : f32) -> Self {
    PointData {
      thresh_band: thresh_band,
      color_band: color_band,
    }
  }
  
  /*
  fn new_rand() -> Self {
    let mut rng = rand::thread_rng();
    Self::new(0.0, rng.gen_range(0.0, 1.0))
  }
  */

  fn sample(&self, thresh : f32, thresh_band : f32) -> Color {
    if (self.thresh_band - thresh).abs() < thresh_band {
      return Color::THRESH;
    }

    if self.thresh_band > thresh {
      // Decide on y or x
      let mut rng = rand::thread_rng();
      let seed = rng.gen_range(0.0, 1.0);
      if self.color_band < seed {
        Color::X
      }
      else {
        Color::Y
      }
    }
    else {
      Color::Z
    }
  }
}

#[wasm_bindgen]
pub struct BlobCanvas {
  width : u32,
  height : u32,
  thresh_base : f32,
  thresh_t_var : f32,
  thresh_t_mult : f32,
  data : DataDoubleBuffer<PointData>,
  undo_stack : VecDeque<Vec<PointData>>,
  t : u32,
}

pub struct CanvasApi<'t> {
  x : u32,
  y : u32,
  canvas : &'t mut BlobCanvas,
}

impl<'t> CanvasApi<'t>{
  pub fn get_mut(&mut self) -> &mut PointData {
    let i = self.canvas.get_index(self.x, self.y);
    &mut self.canvas.data.get_mut()[i]
  }

  pub fn try_get_offset(&self, xoff : i32, yoff: i32) -> Option<PointData> {
    let x = self.x as i32 + xoff;
    let y = self.y as i32 + yoff;
    self.canvas.try_get_index(x, y)
      .map(|i| self.canvas.data.get_imm()[i])
  }
}

impl BlobCanvas {
  fn get_index(&self, x: u32, y: u32) -> usize {
    (y * self.width + x) as usize
  }

  fn try_get_index(&self, x: i32, y: i32) -> Option<usize> {
    if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
      None
    }
    else {
      Some((y as u32 * self.width + x as u32) as usize)
    }
  }
}

impl BlobCanvas {
  pub fn mutate_brush(& mut self, x_norm : f32, y_norm : f32, brush : &Brush, remove : bool) {

    let rad = (brush.size / 2.0) as i32;

    let px = (x_norm * (self.width as f32)).floor() as i32;
    let py = (y_norm * (self.height as f32)).floor() as i32;

    let x_min = (px - rad).max(0);
    let x_max = (px + rad).min(self.width as i32);
    let y_min = (py - rad).max(0);
    let y_max = (py + rad).min(self.height as i32);

    for y in y_min..y_max {
      for x in x_min..x_max {
        let mut api = CanvasApi {
          x : x as u32,
          y : y as u32,
          canvas: self,
        };
        let (dx, dy) = ((px - x) as f32, (py - y) as f32);

        match brush.brush_type {
          BrushType::Inv => {
            let paintbrush = brush.paintbrush.as_ref().unwrap();
            paintbrush.apply_point_mut(dx, dy, api.get_mut(), remove);
          },
          BrushType::Outliner => {
            let dist = (sqr(dx) + sqr(dy)).sqrt();
            let outliner_config = brush.outliner.as_ref().unwrap();
            if dist < outliner_config.size {
              let point_data = api.get_mut();
              point_data.thresh_band = outliner_config.height;
            }
          },
          BrushType::Smudger => {
            let smudge = brush.smudger.as_ref().unwrap();
            smudge.apply_smudge(dx, dy, api);
          }
          _ => {},
        }
      }
    }
  }
}

fn empty_canvas_data(size: usize) -> DataDoubleBuffer<PointData> {
    let mut data = Vec::with_capacity(size as usize);
    for _i in 0..size {
      data.push(PointData::new(0.0, 0.0));
    }
    DataDoubleBuffer::new(data)
}

#[wasm_bindgen]
impl BlobCanvas {
  pub fn new(width : u32, height: u32) -> Self {
    let size = width * height;

    BlobCanvas {
      width : width,
      height : height,
      thresh_base: 0.4,
      thresh_t_var: 0.095,
      thresh_t_mult: TAU / 1_000_000.0,
      data : empty_canvas_data(size as usize),
      undo_stack : VecDeque::with_capacity(MAX_UNDOS+1),
      t : 0,
    }
  }

  pub fn tick(&mut self, microseconds: u32) {
    self.t += microseconds;
    self.data.incr();
  }

  pub fn sample_pixel(&self, x : u32, y : u32) -> Color {
    let t = self.t as f32 * self.thresh_t_mult;
    let t_y_var = t + TAU * (y as f32) / self.height as f32;

    let thresh = self.thresh_base + self.thresh_t_var * (t_y_var).sin();
    let i = self.get_index(x, y);
    let point_data = self.data.get_imm()[i];
    point_data.sample(thresh, 0.05)
  }

  pub fn push_undo(&mut self) {

    self.undo_stack.push_back(self.data.get_clone());

    while self.undo_stack.len() > MAX_UNDOS {
      let _ = self.undo_stack.pop_front();
    }
  }

  pub fn try_pop_undo(&mut self) -> bool {
    match self.undo_stack.pop_back() {
      Some(data) => {
        self.data = DataDoubleBuffer::new(data);
        true
      },
      _ => false,
    }
  }

  pub fn clear(&mut self) {
    self.push_undo();
    self.data = empty_canvas_data(self.data.len())
  }

  pub fn set_thresh_base(&mut self, val : f32) {
    self.thresh_base = val;
  }

  pub fn set_thresh_t_var(&mut self, val : f32) {
    self.thresh_t_var = val;
  }

  pub fn set_thresh_t_mult(&mut self, val : f32) {
    self.thresh_t_mult = val;
  }

  pub fn get_thresh_t_mult(&self) -> f32 {
    self.thresh_t_mult
  }

  pub fn apply_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.mutate_brush(x_norm, y_norm, brush, false);
  }

  pub fn remove_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.mutate_brush(x_norm, y_norm, brush, true);
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
