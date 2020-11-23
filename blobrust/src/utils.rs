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