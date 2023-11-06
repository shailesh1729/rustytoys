extern crate rtoys;

use std::ptr;


fn succ(x: &i32) -> i32 { *x + 1 }




#[test]
fn pointer0(){
    let x: i32 = 5;
    let y = &x;
    assert_eq!(5, *y);
    println!("{:p}", y);


}


#[test]
fn pointer1(){
    let mut x : i32 = 5;
    let y = &mut x;
    succ(y);
    *y  = *y + 1;
    assert_eq!(6, *y);
}



#[test]
fn raw_pointer0(){
    let my_num: i32 = 10;
    // Creating an unsafe pointer to a constant value
    let my_num_ptr: *const i32 = &my_num;
    // A mutable value
    let mut my_speed: i32 = 88;
    // Creating an unsafe pointer to a mutable value
    let my_speed_ptr: *mut i32 = &mut my_speed;
    assert_eq!(10, my_num);
    // Dereferencing from an unsafe pointer to constant value
    assert_eq!(10, unsafe{*my_num_ptr});
    // Dereferencing from an unsafe pointer to mutable value
    assert_eq!(88, unsafe{*my_speed_ptr});
    // Changing the value through unsafe pointer
    unsafe{ 
        *my_speed_ptr = 20;
    }
    assert_eq!(20, unsafe{*my_speed_ptr});
    assert_eq!(20, my_speed);

    let p: *const i32 = ptr::null();
    assert!(p.is_null());
    let x = p as usize;
    assert!(x == 0);
    assert!(my_speed_ptr as usize != 0);
}

