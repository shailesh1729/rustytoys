

/******************************************************
 *
 *  Examples in tests
 *
 *******************************************************/


pub mod test {


#[doc="Shows how to take a pointer to the beginning of an array
and then working with the pointer.
"]
    #[test]
    pub fn test_array_to_pointer(){
        let a = [1i,2,3,4];
        let p = &a[0] as *const int;
        for i in range(0i, 4) {
            let v = unsafe{*p.offset(i)};
            println!("{}", v);
            assert_eq!(v, i + 1);
        }
    }


}

