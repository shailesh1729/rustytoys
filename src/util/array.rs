use std::fmt::Display;

pub fn print_array<T: Display>(data: &[T]) {
    for v in data.iter(){
        print!("{} ", v);
    }
    println!("");
}
