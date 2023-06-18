#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

pub fn rectangle_struct() {
  let rect_one = Rectangle {
      width: 30,
      height: 50,
  };

  println!("rect one is: {:#?}", rect_one);
  println!(
      "The area of the rectangle is {} square pixels.",
      area(&rect_one)
  );
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}