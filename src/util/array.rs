use std::fmt::Show;

pub fn print_array<T : Show>(data: &[T]) {
    for v in data.iter(){
        print!("{} ", v);
    }
    println!("");
}
