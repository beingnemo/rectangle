#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn width(&self) -> bool {
    self.width > 0
  }
}

pub fn rectangle_method() {
  let rect_one = Rectangle {
    width: 40,
    height: 50,
  };
  println!("The area of the rectangle is {}", rect_one.area());

  if rect_one.width() {
    println!("The rectangle has a nonzero width; it is {}", rect_one.width)
  }
}