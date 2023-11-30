fn main(){
    let mut x : [i32; 5] = [5,4,3,2,1];
    util::array::print_array(&x);
    sort::insertion::insertion_sort(&mut x);
    util::array::print_array(&x);
}
