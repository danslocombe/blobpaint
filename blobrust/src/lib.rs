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
  x : f32,
  y : f32,
  z : f32,
}

fn sqr(x : f32) -> f32 {
  x*x
}

impl PointData {
  fn from_spherical(theta : f32, phi : f32) -> Self {
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    PointData { x : x, y : y, z : z }
  }

  fn new() -> Self {
    let mut rng = rand::thread_rng();
    PointData::from_spherical(rng.gen_range(0.0, TAU), rng.gen_range(0.0, TAU))
  }

  fn sample(&self, thresh : f32, thresh_band : f32) -> Color {
    /*
    let mut rng = rand::thread_rng();
    let seed = rng.gen_range(0.0, 1.0);

    // Unit vector so we know these sum to 1
    let p_x = sqr(self.x);
    let p_y = sqr(self.y);
    let p_z = sqr(self.z);

    if seed < p_x {
      Color::X
    }
    else if seed < p_x + p_y {
      Color::Y
    }
    else {
      Color::Z
    }
    */

    if ((self.x - thresh).abs() < thresh_band) {
      return Color::THRESH;
    }

    if self.x > thresh {
      Color::X
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
      //data.push(PointData::from_spherical(i as f32 / 1000.0, i as f32 * 0.001235))
      data.push(PointData::from_spherical(0.0, 0.0));
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

  pub fn mutate_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush, remove : bool) {
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
        let existing = self.data[i];
        let new = brush.apply_point((px - x) as f32, (py - y) as f32, &self.data[i], remove);
        self.data[i] = new;
        //self.data[i] = PointData::from_spherical(0.0, 0.0);
      }
    }
  }

  pub fn apply_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.mutate_brush(x_norm, y_norm, brush, false);
  }

  pub fn remove_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
    self.mutate_brush(x_norm, y_norm, brush, true);
  }
}

fn lerp_x(p0 : &PointData, k : f32, remove : bool) -> PointData {
    let xnew = if !remove {
      (p0.x * (1.0 - k) + k).max(p0.x)
    }
    else {
      p0.x * (1.0 - k)
    };

    let ynew = p0.y * (1.0 - k);
    let znew = p0.z * (1.0 - k);

    PointData {
      x : xnew,
      y : ynew,
      z : znew,
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrushType {
  Inv,
  Sqrt,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Brush {
  size: u32,
  mult: f32,
  curve: f32,
  brush_type: BrushType,
}

impl Brush {
  fn get_size(&self) -> u32 {
    self.size
  }

  fn apply_point(&self, dx : f32, dy : f32, p0 : &PointData, remove : bool) -> PointData {
    let dist = (sqr(dx) + sqr(dy)).sqrt();
    let k = match self.brush_type {
      BrushType::Inv => {
        let inv_dist = 1.0 / (1.0 + self.curve * dist);
        inv_dist / self.mult
      },
      BrushType::Sqrt => {
        (1.0 - self.curve * dist.sqrt()) / self.mult
      },
      _ => 0.0,
    };

    lerp_x(p0, k, remove)
  }
}

#[wasm_bindgen]
impl Brush {
  pub fn new_inv(size : u32, mult : f32, curve : f32) -> Self {
    Brush { size : size, mult : mult, curve : curve, brush_type : BrushType::Inv }
  }

  pub fn new_sqrt(size : u32, mult : f32, curve : f32) -> Self {
    Brush { size : size, mult : mult, curve : curve, brush_type : BrushType::Sqrt }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
