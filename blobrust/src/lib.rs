extern crate rand;

use wasm_bindgen::prelude::*;
use rand::prelude::*;

const TAU : f32 = 2.0 * 3.141;

#[wasm_bindgen]
extern {
  pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn rand_unit() -> f32 {
  //let mut rng = rand::thread_rng();
  //rng.gen_range(0.0, 1.0)
  0.5
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

fn sqr(x : f32) -> f32 {
  x*x
}

impl PointData {
  fn new(thresh_band : f32, color_band : f32) -> Self {
    PointData {
      thresh_band: thresh_band,
      color_band: color_band,
    }
  }
  
  fn new_rand() -> Self {
    let mut rng = rand::thread_rng();
    Self::new(0.0, rng.gen_range(0.0, 1.0))
  }

  fn sample(&self, thresh : f32, thresh_band : f32) -> Color {
    if ((self.thresh_band - thresh).abs() < thresh_band) {
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
  t : u32,
}

impl BlobCanvas {
  fn get_index(&self, x: u32, y: u32) -> usize {
    (y * self.width + x) as usize
  }
}

#[wasm_bindgen]
impl BlobCanvas {
  pub fn new(width : u32, height: u32) -> Self {
    let size = width*height;
    let mut data = Vec::with_capacity(size as usize);
    for i in 0..size {
      data.push(PointData::new(0.0, 0.0));
    }

    BlobCanvas {
      width : width,
      height : height,
      data : data,
      t : 0,
    }
  }

  pub fn incr_t(&mut self) {
    self.t += 1;
  }

  pub fn sample_pixel(&self, x : u32, y : u32) -> Color {
    let i = self.get_index(x, y);

    const thresh_base : f32 = 0.4;
    const thresh_t_var : f32 = 0.035;
    const t_mult : f32 = TAU / 80.0;
    let t = self.t as f32 * t_mult;
    let t_y_var = t + TAU * (y as f32) / self.height as f32;

    let thresh = thresh_base + thresh_t_var * (t_y_var).sin();
    self.data[i].sample(thresh, 0.05)
  }

  pub fn push_undo(&mut self) {
    // TODO
  }

  pub fn try_pop_undo(&mut self) -> bool {
    // TODO
    false
  }

  pub fn mutate_paintbrush(&mut self, x_norm : f32, y_norm : f32, brush : &Paintbrush, remove : bool) {
    let px = (x_norm * (self.width as f32)).floor() as i32;
    let py = (y_norm * (self.height as f32)).floor() as i32;

    let rad = (brush.size / 2) as i32;
    let x_min = (px - rad).max(0);
    let x_max = (px + rad).min(self.width as i32);
    let y_min = (py - rad).max(0);
    let y_max = (py + rad).min(self.height as i32);

    for y in y_min..y_max {
      for x in x_min..x_max {
        let i = self.get_index(x as u32, y as u32);
        brush.apply_point_mut((px - x) as f32, (py - y) as f32, &mut self.data[i], remove);
      }
    }
  }
  
  pub fn mutate_outliner(&mut self, x_norm : f32, y_norm : f32, remove : bool) {
    let size = 8;
    let px = (x_norm * (self.width as f32)).floor() as i32;
    let py = (y_norm * (self.height as f32)).floor() as i32;

    let rad = (size / 2) as i32;
    let x_min = (px - rad).max(0);
    let x_max = (px + rad).min(self.width as i32);
    let y_min = (py - rad).max(0);
    let y_max = (py + rad).min(self.height as i32);

    for y in y_min..y_max {
      for x in x_min..x_max {
        let i = self.get_index(x as u32, y as u32);
        let dist = (sqr(px as f32 - x as f32) + sqr(py as f32 - y as f32)).sqrt();
        if (dist < 5.0)  {
          let existing = self.data[i];
          self.data[i].thresh_band = 0.40;
        }
      }
    }
  }
  
  pub fn mutate_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush, remove : bool) {
    match brush.brush_type {
      BrushType::Inv => {
        self.mutate_paintbrush(x_norm, y_norm, brush.paintbrush.as_ref().unwrap(), remove)
      },
      BrushType::Outliner => {
        self.mutate_outliner(x_norm, y_norm, remove)
      }
      _ => {},
    }
  }

  pub fn apply_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.mutate_brush(x_norm, y_norm, brush, false);
  }

  pub fn remove_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.mutate_brush(x_norm, y_norm, brush, true);
  }
}

fn clamp(x : f32, min : f32, max : f32) -> f32 {
  if x < min {
    min
  }
  else if x > max {
    max
  }
  else {
    x
  }
}

fn clamp_unit(x : f32) -> f32 {
  clamp(x, 0.0, 1.0)
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum BrushType {
  Inv,
  Sqrt,
  Outliner,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Paintbrush {
  size: u32,
  mult: f32,
  curve: f32,
  color: f32,
}

impl Paintbrush {
  fn get_size(&self) -> u32 {
    self.size
  }

  fn apply_point_mut(&self, dx : f32, dy : f32, p : &mut PointData, remove : bool) {
    let dist = (sqr(dx) + sqr(dy)).sqrt();
    let k = self.sample(dist);
    p.thresh_band = lerp_brush_thresh(p.thresh_band, k, remove);
    p.color_band = lerp_brush_color(p.color_band, 4.0*k, self.color, remove);
  }

  fn sample(&self, dist : f32) -> f32 {
    let inv_dist = 1.0 / (1.0 + self.curve * dist);
    inv_dist / self.mult
  }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Outliner {
  size: u32,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Brush {
  brush_type: BrushType,
  paintbrush : Option<Paintbrush>,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
