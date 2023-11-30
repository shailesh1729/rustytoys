
pub mod xxx {
pub struct X {
    pub x : i32,
}

pub trait A{
    fn a(&self)-> i32;
}

impl A for X {
    fn a(&self) -> i32{
        30
    }
}

pub mod yyy {

pub mod trait_b {
   pub trait B {
        fn b(&self)-> i32;
   }
}

pub mod impl_b{

  //use crate::lang::module_structure::xxx;
  //use crate::lang::module_structure::xxx::A;
  use crate::module_structure::xxx::yyy::trait_b::B;
  use super::super::X;
  
  impl B for X {
    fn b(&self) -> i32{
        40
    }
  }
}

}

}

#[test]
pub fn test_dependencies(){

  use crate::module_structure::xxx;
  use crate::module_structure::xxx::A;
  use crate::module_structure::xxx::yyy::trait_b::B;

    let x1 = xxx::X{x:20};
    println!("{}", x1.x);
    println!("{}", x1.a());
    println!("{}", x1.b());
}

