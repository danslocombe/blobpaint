use wasm_bindgen::prelude::*;

use super::{PointData, CanvasApi};
use super::utils::{clamp_unit, sqr};

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum BrushType {
  Inv,
  Sqrt,
  Outliner,
  Smudger,
}

impl Default for BrushType {
  // Default to basic paintbrush.
  fn default() -> Self {
    BrushType::Inv
  }
}

// TODO find a better name for this trait.
// Brush Tool?
// Point mutator?
pub trait BrushT {
  fn apply_point_mut(&self, dx: f32, dy: f32, p: &mut PointData, remove: bool);
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
  fn sample(&self, dist : f32) -> f32 {
    self.mult / (1.0 + self.curve * dist)
  }
}

impl BrushT for Paintbrush {
  fn apply_point_mut(&self, dx : f32, dy : f32, p : &mut PointData, remove : bool) {
    let dist = (sqr(dx) + sqr(dy)).sqrt();
    let k = self.sample(dist);
    p.thresh_band = lerp_brush_thresh(p.thresh_band, k, remove);
    p.color_band = lerp_brush_color(p.color_band, 4.0*k, self.color, remove);
  }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Outliner {
  pub size: u32,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Smudger {
  smudge_dist_mult : f32,
  mult: f32,
  curve: f32,
  smudge_vec_x: f32,
  smudge_vec_y: f32,
  pub size: u32,
}

fn lerpk(x0: f32, x1: f32, k: f32) -> f32 {
  (x0 * k + x1) / (k + 1.0)
}

impl<'t> Smudger {
  pub fn apply_smudge(&self, offset_x : f32, offset_y : f32, mut api : CanvasApi<'t>) {

    // Want to apply smudge only to points in the direction of the smudge
    // Strength is dot of offset from the brush centre with the smudge vector
    let strength = (self.smudge_vec_x * offset_x + self.smudge_vec_y * offset_y).max(0.0);

    if strength > 0.0 {
      // Todo interpolate
      let sample_xo = (self.smudge_vec_x * self.smudge_dist_mult) as i32;
      let sample_yo = (self.smudge_vec_y * self.smudge_dist_mult) as i32;

      match api.try_get_offset(sample_xo, sample_yo) {
        Some(source_smudge) => {
          let k = 8.0;
          let mut cur = api.get_mut();
          cur.thresh_band = lerpk(cur.thresh_band, source_smudge.thresh_band, k);
          cur.color_band = lerpk(cur.color_band, source_smudge.color_band, k);
          //cur.thresh_band = (source_smudge.thresh_band + cur.thresh_band) / 2.0;
          //cur.color_band = (source_smudge.color_band + cur.color_band) / 2.0;
        }
        _ => {},
      }
    }
  }
}

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Brush {
  // We can only keep c-style enums when we generate wasm.
  // Emulate a union with a series of optional structs.
  pub brush_type: BrushType,
  pub paintbrush : Option<Paintbrush>,
  pub outliner : Option<Outliner>,
  pub smudger : Option<Smudger>,
}

#[wasm_bindgen]
impl Brush {
  pub fn new_inv(size : u32, mult : f32, curve : f32) -> Self {
    let mut brush = Brush::default();
    brush.brush_type = BrushType::Inv;
    brush.paintbrush = Some(Paintbrush {
        size: size,
        mult: mult,
        curve: curve,
        color: 0.0,
    });

    brush
  }
  
  pub fn new_outliner() -> Self {
    let mut brush = Brush::default();
    brush.brush_type = BrushType::Outliner;
    brush.outliner = Some(Outliner {
      size: 8,
    });

    brush
  }
  
  pub fn new_smudger() -> Self {
    let mut brush = Brush::default();
    brush.brush_type = BrushType::Smudger;
    brush.smudger = Some(Smudger {
      smudge_dist_mult: 16.0,
      smudge_vec_x : 1.0,
      smudge_vec_y : 0.0,
      curve: 0.0,
      mult: 0.0,
      size: 8,
    });

    brush
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

  pub fn set_smudger_dx(&mut self, dx: f32) {
    match self.brush_type {
      BrushType::Smudger => {self.smudger.as_mut().unwrap().smudge_vec_x = dx},
      _ => {},
    }
  }

  pub fn set_smudger_dy(&mut self, dy: f32) {
    match self.brush_type {
      BrushType::Smudger => {self.smudger.as_mut().unwrap().smudge_vec_y = dy},
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