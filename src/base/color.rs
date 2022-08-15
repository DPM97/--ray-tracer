use super::vec3::Vec3;

trait Color {
    fn r(&self) -> &i32;
    fn g(&self) -> &i32;
    fn b(&self) -> &i32;
}

impl Color for Vec3 {
  fn r(&self) -> &i32 {
    return self.zero
  }
  fn g(&self) -> &i32 {
    return self.one
  }
  fn b(&self) -> &i32 {
    return self.two
  }
}
