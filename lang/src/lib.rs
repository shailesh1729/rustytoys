#![doc="Examples demonstrating language features."]


pub mod unsafe_mod;
pub mod module_structure;

fn add_three(x: i32) -> i32 { x + 3 }

fn times_four(x: i32) -> i32 { x * 4 }

pub fn add_three_times_four(x: i32) -> i32 {
    times_four(add_three(x))
}


#[cfg(test)]
mod test {
    use super::add_three;
    use super::times_four;

    #[test]
    fn test_add_three() {
        let result = add_three(5i32);

        assert_eq!(8i32, result);
    }

    #[test]
    fn test_times_four() {
        let result = times_four(5i32);

        assert_eq!(20i32, result);
    }
}
