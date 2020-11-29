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

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Paintbrush {
  mult: f32,
  curve: f32,
  color: f32,
}

impl Paintbrush {
  fn sample(&self, dist : f32) -> f32 {
    self.mult / (1.0 + self.curve * dist)
  }

  pub fn apply_point_mut(&self, dx : f32, dy : f32, p : &mut PointData, remove : bool) {
    let dist = (sqr(dx) + sqr(dy)).sqrt();
    let k = self.sample(dist);
    p.thresh_band = lerp_brush_thresh(p.thresh_band, k, remove);
    p.color_band = lerp_brush_color(p.color_band, 4.0*k, self.color, remove);
  }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Outliner {
  pub height: f32,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Smudger {
  smudge_dist_mult : f32,
  mult: f32,
  curve: f32,
  smudge_vec_x: f32,
  smudge_vec_y: f32,
  smudge_vec_x_norm: f32,
  smudge_vec_y_norm: f32,
}

fn lerpk(x0: f32, x1: f32, k: f32) -> f32 {
  (x0 * k + x1) / (k + 1.0)
}

fn normalize(x : f32, y : f32) -> (f32, f32) {
  let mag = (sqr(x) + sqr(y)).sqrt();
  (x / mag, y / mag)
}

impl<'t> Smudger {
  pub fn apply_smudge(&self, offset_x : f32, offset_y : f32, mut api : CanvasApi<'t>) {

    // Want to apply smudge only to points in the direction of the smudge
    // Strength is dot of offset from the brush centre with the smudge vector
    let offset_dist = (sqr(offset_x) + sqr(offset_x)).sqrt();
    let (offset_x_norm, offset_y_norm) = (offset_x / offset_dist, offset_y / offset_dist);
    let strength = (self.smudge_vec_x_norm * offset_x_norm + self.smudge_vec_y * offset_y_norm).max(0.0);

    if strength > 0.0 {
      // Todo interpolate?
      let sample_xo = (self.smudge_vec_x * self.smudge_dist_mult) as i32;
      let sample_yo = (self.smudge_vec_y * self.smudge_dist_mult) as i32;

      match api.try_get_point(sample_xo, sample_yo) {
        Some(source_smudge) => {
          let k = 2.0 * (1.0 + offset_dist);

          let mut cur = api.get_mut();
          cur.thresh_band = lerpk(cur.thresh_band, source_smudge.thresh_band, k);
          cur.color_band = lerpk(cur.color_band, source_smudge.color_band, k);
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
  pub size : f32,
}

#[wasm_bindgen]
impl Brush {
  pub fn new_inv(size : u32, curve : f32, mult : f32) -> Self {
    let mut brush = Brush::default();
    brush.brush_type = BrushType::Inv;
    brush.paintbrush = Some(Paintbrush {
        mult: mult,
        curve: curve,
        color: 0.0,
    });
    brush.size = size as f32;

    brush
  }
  
  pub fn new_outliner(size : f32) -> Self {
    let mut brush = Brush::default();
    brush.brush_type = BrushType::Outliner;
    brush.outliner = Some(Outliner {
      height: 0.5,
    });
    brush.size = size;

    brush
  }
  
  pub fn new_smudger(size : f32) -> Self {
    let mut brush = Brush::default();
    brush.brush_type = BrushType::Smudger;
    brush.smudger = Some(Smudger {
      smudge_dist_mult: 16.0,
      smudge_vec_x : 1.0,
      smudge_vec_y : 0.0,
      smudge_vec_x_norm : 1.0,
      smudge_vec_y_norm : 0.0,
      curve: 0.0,
      mult: 0.0,
    });
    brush.size = size;

    brush
  }

  pub fn set_size(&mut self, size : f32) {
    self.size = size;
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

  pub fn set_outliner_height(&mut self, height: f32) {
    match self.brush_type {
      BrushType::Outliner => {self.outliner.as_mut().unwrap().height = height},
      _ => {},
    }
  }

  pub fn set_smudger_dx(&mut self, dx: f32) {
    match self.brush_type {
      BrushType::Smudger => {
        let smudger = self.smudger.as_mut().unwrap();
        smudger.smudge_vec_x = dx;
        let (nx, ny) = normalize(smudger.smudge_vec_x, smudger.smudge_vec_y);
        smudger.smudge_vec_x_norm = nx;
        smudger.smudge_vec_y_norm = ny;
      },
      _ => {},
    }
  }

  pub fn set_smudger_dy(&mut self, dy: f32) {
    match self.brush_type {
      BrushType::Smudger => {
        let smudger = self.smudger.as_mut().unwrap();
        smudger.smudge_vec_y = dy;
        let (nx, ny) = normalize(smudger.smudge_vec_x, smudger.smudge_vec_y);
        smudger.smudge_vec_x_norm = nx;
        smudger.smudge_vec_y_norm = ny;
      },
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

impl<'t> Brush {
  pub fn sample_canvas(&self, dx: f32, dy: f32, mut api: CanvasApi<'t>, remove: bool)
  {
    match self.brush_type {
      BrushType::Inv => {
        let paintbrush = self.paintbrush.as_ref().unwrap();
        paintbrush.apply_point_mut(dx, dy, api.get_mut(), remove);
      },
      BrushType::Outliner => {
        let outliner_config = self.outliner.as_ref().unwrap();
        let dist = (sqr(dx) + sqr(dy)).sqrt();
        let curve = 1.0;
        let mult = 0.625 *outliner_config.height;
        let rad = self.size / 2.0;
        let k = (1.0 - curve * (dist / rad).sqrt()) * mult;
        if k > 0.0 {
          let point_data = api.get_mut();
          point_data.thresh_band = (point_data.thresh_band + k).min(outliner_config.height);
        }
      },
      BrushType::Smudger => {
        let smudge = self.smudger.as_ref().unwrap();
        smudge.apply_smudge(dx, dy, api);
      }
      _ => {},
    }
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