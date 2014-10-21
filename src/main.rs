extern crate rtoys;
#[cfg(not(test))]
fn main(){
    use rtoys::sort;
    use rtoys::util;
    let mut x : [int, ..5] = [5,4,3,2,1];
    util::array::print_array(x);
    sort::insertion::insertion_sort(x);
    util::array::print_array(x);
}
