use wasm_bindgen::prelude::*;

use super::{PointData};
use super::utils::{clamp_unit, sqr};

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum BrushType {
  Inv,
  Sqrt,
  Outliner,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Paintbrush {
  pub size: u32,
  mult: f32,
  curve: f32,
  color: f32,
}

impl Paintbrush {
  pub fn apply_point_mut(&self, dx : f32, dy : f32, p : &mut PointData, remove : bool) {
    let dist = (sqr(dx) + sqr(dy)).sqrt();
    let k = self.sample(dist);
    p.thresh_band = lerp_brush_thresh(p.thresh_band, k, remove);
    p.color_band = lerp_brush_color(p.color_band, 4.0*k, self.color, remove);
  }

  pub fn sample(&self, dist : f32) -> f32 {
    let inv_dist = 1.0 / (1.0 + self.curve * dist);
    inv_dist / self.mult
  }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Outliner {
  pub size: u32,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Brush {
  pub brush_type: BrushType,
  pub paintbrush : Option<Paintbrush>,
}

#[wasm_bindgen]
impl Brush {
  pub fn new_inv(size : u32, mult : f32, curve : f32) -> Self {
    Brush {
      brush_type: BrushType::Inv,
      paintbrush: Some(Paintbrush {
        size: size,
        mult: mult,
        curve: curve,
        color: 0.0,
      }),
    }
  }
  
  pub fn new_outliner() -> Self {
    Brush {
      brush_type: BrushType::Outliner,
      paintbrush: None,
    }
  }
  
  pub fn set_curve(&mut self, curve : f32) {
    match self.brush_type {
      BrushType::Inv => {self.paintbrush.as_mut().unwrap().curve = curve},
      _ => {},
    }
  }
  
  pub fn set_mult(&mut self, mult : f32) {
    match self.brush_type {
      BrushType::Inv => {self.paintbrush.as_mut().unwrap().mult = mult},
      _ => {},
    }
  }
  
  pub fn set_color(&mut self, color: f32) {
    match self.brush_type {
      BrushType::Inv => {self.paintbrush.as_mut().unwrap().color = color},
      _ => {},
    }
  }
  
  pub fn sample(&self, dist : f32) -> f32 {
    // Just inv atm
    match self.brush_type {
      BrushType::Inv => {
        self.paintbrush.as_ref().unwrap().sample(dist)
      }
      _ => {0.0}
    }
    /*
    match self.brush_type {
      BrushType::Inv => {
        let inv_dist = 1.0 / (1.0 + self.curve * dist);
        inv_dist / self.mult
      },
      BrushType::Sqrt => {
        (1.0 - self.curve * dist.sqrt()) / self.mult
      },
      _ => 0.0,
    }
    */
  }
}

fn lerp_brush_thresh(thresh0 : f32, k : f32, remove : bool) -> f32 {
  clamp_unit(if !remove {
    (thresh0 * (1.0 - k) + k).max(thresh0)
  }
  else {
    thresh0 * (1.0 - k)
  })
}

fn lerp_brush_color(color0 : f32, k : f32, target_color: f32, remove : bool) -> f32 {
  clamp_unit(if !remove {
    color0 * (1.0 - k) + target_color * k
  }
  else {
    color0
  })
}