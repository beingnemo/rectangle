pub fn tuple_rectangle() {
  let rect_one = (30, 50);

  println!(
      "The area of the rectangle is {} square pixels.",
      area(rect_one)
  );
}


fn area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}