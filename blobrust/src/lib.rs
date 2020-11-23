extern crate rand;

mod utils;
mod brush;

use wasm_bindgen::prelude::*;
use rand::prelude::*;

use std::f32::consts::TAU;

use utils::{sqr};
use brush::{Brush, BrushType};

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
  data : Vec<PointData>,
  undo_stack : Vec<Vec<PointData>>,
  t : u32,
}

impl BlobCanvas {
  fn get_index(&self, x: u32, y: u32) -> usize {
    (y * self.width + x) as usize
  }
}

impl BlobCanvas {
  pub fn map_over_pointdata(&mut self, x_norm : f32, y_norm : f32, rad : i32, f : &dyn Fn(f32, f32, &mut PointData)) {
    let px = (x_norm * (self.width as f32)).floor() as i32;
    let py = (y_norm * (self.height as f32)).floor() as i32;

    let x_min = (px - rad).max(0);
    let x_max = (px + rad).min(self.width as i32);
    let y_min = (py - rad).max(0);
    let y_max = (py + rad).min(self.height as i32);

    for y in y_min..y_max {
      for x in x_min..x_max {
        let i = self.get_index(x as u32, y as u32);
        f((px - x) as f32, (py - y) as f32, &mut self.data[i]);
      }
    }
  }
  
  pub fn mutate_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush, remove : bool) {
    match brush.brush_type {
      BrushType::Inv => {
        let paintbrush = brush.paintbrush.as_ref().unwrap();
        let brush_apply = |dx, dy, point_data: &mut PointData| {
          paintbrush.apply_point_mut(dx, dy, point_data, remove)
        };
        self.map_over_pointdata(x_norm, y_norm, paintbrush.size as i32 / 2, &brush_apply);
      },
      BrushType::Outliner => {
        let brush_apply = |dx, dy, point_data: &mut PointData| {
          let dist = (sqr(dx) + sqr(dy)).sqrt();
          if dist < 5.0  {
            point_data.thresh_band = 0.40;
          }
        };

        self.map_over_pointdata(x_norm, y_norm, 4, &brush_apply);
      }
      _ => {},
    }
  }
}

#[wasm_bindgen]
impl BlobCanvas {
  pub fn new(width : u32, height: u32) -> Self {
    let size = width*height;
    let mut data = Vec::with_capacity(size as usize);
    for _i in 0..size {
      data.push(PointData::new(0.0, 0.0));
    }

    BlobCanvas {
      width : width,
      height : height,
      data : data,
      undo_stack : Vec::new(),
      t : 0,
    }
  }

  pub fn incr_t(&mut self) {
    self.t += 1;
  }

  pub fn sample_pixel(&self, x : u32, y : u32) -> Color {
    let i = self.get_index(x, y);

    const THRESH_BASE : f32 = 0.4;
    const THRESH_T_VAR : f32 = 0.035;
    const T_MULT : f32 = TAU / 80.0;
    let t = self.t as f32 * T_MULT;
    let t_y_var = t + TAU * (y as f32) / self.height as f32;

    let thresh = THRESH_BASE + THRESH_T_VAR * (t_y_var).sin();
    self.data[i].sample(thresh, 0.05)
  }

  pub fn push_undo(&mut self) {
    const MAX_UNDOS : usize = 8;

    self.undo_stack.push(self.data.clone());

    while self.undo_stack.len() > MAX_UNDOS {
      let _ = self.undo_stack.pop();
    }
  }

  pub fn try_pop_undo(&mut self) -> bool {
    match self.undo_stack.pop() {
      Some(data) => {
        self.data = data;
        true
      },
      _ => false,
    }
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
