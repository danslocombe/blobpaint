use super::rand_core::RngCore;

pub fn sqr(x : f32) -> f32 {
  x*x
}

pub fn clamp(x : f32, min : f32, max : f32) -> f32 {
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

pub fn clamp_unit(x : f32) -> f32 {
  clamp(x, 0.0, 1.0)
}

pub fn rand_unit(rng: &mut dyn RngCore) -> f32{
  (rng.next_u32() as f32) / (std::u32::MAX as f32)
}

pub fn lerpk(x0: f32, x1: f32, k: f32) -> f32 {
  (x0 * k + x1) / (k + 1.0)
}

pub fn normalize(x : f32, y : f32) -> (f32, f32) {
  let mag = (sqr(x) + sqr(y)).sqrt();
  (x / mag, y / mag)
}