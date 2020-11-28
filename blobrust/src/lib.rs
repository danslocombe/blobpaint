/// Dan Slocombe 2020
/// https://danslocom.be

extern crate rand_core;
extern crate rand_xorshift;

mod utils;
mod pointdata;
mod brush;

use std::f32::consts::TAU;
use std::collections::VecDeque;

use wasm_bindgen::prelude::*;
use rand_core::{SeedableRng, RngCore};
use rand_xorshift::{XorShiftRng};


use utils::{rand_unit};
use pointdata::{PointData, PointDataStore, DirtyRect};
use brush::{Brush};

const MAX_UNDOS : usize = 8;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
  X,
  Y,
  Z,
  THRESH,
}

impl Default for Color {
  fn default() -> Self {
    Color::X
  }
}

/// A point to be draw to the html canvas.
/// If we ever want to have a canvas larger than 256 x 256
/// this needs to be increased.
#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DrawPoint {
  pub x : u8, 
  pub y : u8,
  pub col: Color,
}

#[wasm_bindgen]
pub struct BlobCanvas {
  width : u32,
  height : u32,
  thresh_base : f32,
  thresh_t_var : f32,
  thresh_t_mult : f32,
  data : PointDataStore,
  undo_stack : VecDeque<Vec<PointData>>,
  draw_buffer: Vec<DrawPoint>,
  t : u32,
  rng: XorShiftRng,
}

static RAND_SEED : [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

#[wasm_bindgen]
impl BlobCanvas {
  pub fn new(width : u32, height: u32) -> Self {
    let mut rng = XorShiftRng::from_seed(RAND_SEED);
    let canvas_data = empty_canvas_data(width, height, &mut rng);

    let pixels_updated_per_frame : usize = ((width as f32) * (height as f32) * 0.05) as usize;
    let mut draw_buffer = Vec::with_capacity(pixels_updated_per_frame);
    for _i in 0..pixels_updated_per_frame {
      draw_buffer.push(DrawPoint::default());
    }

    BlobCanvas {
      width : width,
      height : height,
      thresh_base: 0.4,
      thresh_t_var: 0.095,
      thresh_t_mult: TAU / 1_000_000.0,
      data : canvas_data,
      undo_stack : VecDeque::with_capacity(MAX_UNDOS+1),
      draw_buffer: draw_buffer,
      t : 0,
      rng: rng,
    }
  }

  pub fn tick(&mut self, microseconds: u32) {
    self.t += microseconds;
    self.data.flush();
  }

  pub fn sample_pixel(&mut self, x : u32, y : u32) -> Color {
    let t = self.t as f32 * self.thresh_t_mult;
    let t_y_var = t + TAU * (y as f32) / self.height as f32;

    let thresh = self.thresh_base + self.thresh_t_var * (t_y_var).sin();
    let i = self.data.get_index(x, y);
    let point_data = self.data.get_imm()[i];
    point_data.sample(&mut self.rng, thresh, 0.05)
  }

  fn fill_draw_buffer_uniform(&mut self, count : usize) {
    for i in 0..count {
      let x = self.rng.next_u32() % self.width;
      let y = self.rng.next_u32() % self.height;
      let col = self.sample_pixel(x, y);
      self.draw_buffer[i] = DrawPoint {
        x : x as u8,
        y : y as u8,
        col : col
      }
    }
  }

  pub fn fill_draw_buffer(&mut self) {
    // Check to see if pointdata was written to in the last frame
    let rect = self.data.get_last_dirty().clone();
    if rect.is_clean() {
      // If not sample uniformly over the entire canvas
      self.fill_draw_buffer_uniform(self.draw_buffer.len());
    }
    else {
      // Otherwise reserve some of the buffer to fill with data from the dirty
      // region.

      // TODO take the ratio using the relative sizes of the dirty rect and
      // the overall canvas.
      let count = ((self.draw_buffer.len() as f32) * 0.95) as usize;
      self.fill_draw_buffer_uniform(count);

      // We know this is safe to do because of is_clean() check failing.
      let x_range = rect.max_x - rect.min_x;
      let y_range = rect.max_y - rect.min_y;

      for i in count..self.draw_buffer.len() {
        let x = rect.min_x + (self.rng.next_u32() % x_range);
        let y = rect.min_y + (self.rng.next_u32() % y_range);
        let col = self.sample_pixel(x, y);
        self.draw_buffer[i] = DrawPoint {
          x : x as u8,
          y : y as u8,
          col : col
        }
      }
    };
  }

  pub fn get_draw_buffer_size(&self) -> usize {
    self.draw_buffer.len()
  }

  pub fn get_draw_buffer(&self) -> *const DrawPoint {
      self.draw_buffer.as_ptr()
  }

  pub fn push_undo(&mut self) {
    // TODO
    // Setup ring buffer so we don't have to allocate.
    self.undo_stack.push_back(self.data.get_clone());

    while self.undo_stack.len() > MAX_UNDOS {
      let _ = self.undo_stack.pop_front();
    }
  }

  pub fn try_pop_undo(&mut self) -> bool {
    match self.undo_stack.pop_back() {
      Some(data) => {
        self.data = PointDataStore::new(data, self.width, self.height);
        true
      },
      _ => false,
    }
  }

  pub fn clear(&mut self) {
    self.push_undo();
    self.data = empty_canvas_data(self.width, self.height, &mut self.rng)
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
    self.apply_brush_internal(x_norm, y_norm, brush, false);
  }

  pub fn remove_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.apply_brush_internal(x_norm, y_norm, brush, true);
  }
}

impl BlobCanvas {
  pub fn apply_brush_internal(& mut self, x_norm : f32, y_norm : f32, brush : &Brush, remove : bool) {

    let rad = (brush.size / 2.0) as i32;

    let px = (x_norm * (self.width as f32)).floor() as i32;
    let py = (y_norm * (self.height as f32)).floor() as i32;

    let x_min = (px - rad).max(0);
    let x_max = (px + rad).min(self.width as i32);
    let y_min = (py - rad).max(0);
    let y_max = (py + rad).min(self.height as i32);

    self.data.set_dirty(DirtyRect {
      min_x: x_min as u32,
      min_y: y_min as u32,
      max_x: x_max as u32,
      max_y: y_max as u32,
    });

    for y in y_min..y_max {
      for x in x_min..x_max {
        let api = CanvasApi {
          x : x as u32,
          y : y as u32,
          canvas: self,
        };

        let (dx, dy) = ((px - x) as f32, (py - y) as f32);
        brush.sample_canvas(dx, dy, api, remove);
      }
    }
  }
}

fn empty_canvas_data(width: u32, height: u32, rng: &mut dyn RngCore) -> PointDataStore {
    let size = width * height;
    let mut data = Vec::with_capacity(size as usize);
    for _i in 0..size {
      let col = rand_unit(rng);
      data.push(PointData::new(0.0, col));
    }
    PointDataStore::new(data, width, height)
}

pub struct CanvasApi<'t> {
  x : u32,
  y : u32,
  canvas : &'t mut BlobCanvas,
}

impl<'t> CanvasApi<'t>{
  pub fn get_mut(&mut self) -> &mut PointData {
    let i = self.canvas.data.get_index(self.x, self.y);
    &mut self.canvas.data.get_mut()[i]
  }

  pub fn try_get_point(&self, xoff : i32, yoff: i32) -> Option<PointData> {
    let x = self.x as i32 + xoff;
    let y = self.y as i32 + yoff;
    self.canvas.data.try_get_index(x, y)
      .map(|i| self.canvas.data.get_imm()[i])
  }
}