use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(params: &str) -> String {
  let nums: (f32, f32, f32) = serde_json::from_str(&params).unwrap();
  let mut solution: (f32, f32, f32) = (0., 0., 0.);

  let mut cos_a: f32 = 0.;
  let mut cos_b: f32 = 0.;

  // Check if the triangle is valid

  if (!check_valid(nums.0, nums.1, nums.2)) {
    return String::from("not a real triangle");
  }

  // cos A = (b^2 + c^2 - a^2) / (2bc)
  cos_a = (nums.1.powf(2.) + nums.2.powf(2.) - nums.0.powf(2.)) / (2. * nums.1 * nums.2);

  // Spent so long figuring out why this was wrong, turns out .acos returns the angle in RADIANS!!
  solution.0 = (cos_a.acos() * 180.) / std::f32::consts::PI;

  // cos B = (a^2 + c^2 - b^2) / (2ac)
  cos_b = (nums.0.powf(2.) + nums.2.powf(2.) - nums.1.powf(2.)) / (2. * nums.0 * nums.2);
  solution.1 = (cos_b.acos() * 180.) / std::f32::consts::PI;

  // Law of triangles: 180 degrees - Angle A - Angle B = Angle C
  solution.2 = 180. - solution.0 - solution.1;

  return serde_json::to_string(&solution).unwrap();
}

fn check_valid(a: f32, b: f32, c: f32) -> bool {
  if (a + b <= c || a + c <= b || b + c <= a) {
    return false;
  } else {
    return true;
  }
}
