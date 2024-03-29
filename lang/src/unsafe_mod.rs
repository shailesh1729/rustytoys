#![doc="
Examples showing how to use unsafe territory of Rust.
"]


#[doc="Shows how to take a pointer to the beginning of an array
and then working with the pointer.
"]
#[test]
pub fn ex_array_to_pointer(){
    let a = [1i32,2,3,4];
    let p = &a[0] as *const i32;
    for i in 0..4 {
        let v = unsafe{*p.offset(i)};
        println!("{}", v);
        assert_eq!(v, (i + 1) as i32);
    }
}

