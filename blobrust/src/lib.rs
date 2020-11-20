extern crate rand;

use wasm_bindgen::prelude::*;
use rand::prelude::*;

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
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PointData {
  x : f32,
  y : f32,
  z : f32,
}

const PI : f32 = 3.141;

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
    PointData::from_spherical(rng.gen_range(0.0, PI * 2.0), rng.gen_range(0.0, PI * 2.0))
  }

  fn sample(&self) -> Color {
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
  }
}

#[wasm_bindgen]
pub struct BlobCanvas {
  width : u32,
  height : u32,
  data : Vec<PointData>,
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
      data.push(PointData::from_spherical(i as f32 / 1000.0, i as f32 * 0.001235))
    }

    BlobCanvas {
      width : width,
      height : height,
      data : data,
    }
  }

  pub fn sample_pixel(&self, x : u32, y : u32) -> Color {
    let i = self.get_index(x, y);
    self.data[i].sample()
  }

  pub fn apply_brush(&mut self, x_norm : f32, y_norm : f32, brush : &Brush) {
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
        self.data[i] = PointData::from_spherical(0.0, 0.0);
      }
    }
  }
}

#[wasm_bindgen]
pub struct Brush {
  size: u32
}

#[wasm_bindgen]
impl Brush {
  pub fn new() -> Self {
    Brush { size : 4 }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
