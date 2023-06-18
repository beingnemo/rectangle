mod rectangles;
mod using_tuple;
mod using_structs;
mod using_method;
mod using_method_mp;

fn main() {
    rectangles::rectangles();
    using_tuple::tuple_rectangle();
    using_structs::rectangle_struct();
    using_method::rectangle_method();
    using_method_mp::rectangle_method_mp()
}
