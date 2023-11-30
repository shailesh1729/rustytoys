pub mod insertion;

/// Returns whether the slice is sorted in ascending order or not.
pub fn is_ascending<T : PartialOrd>(data : & [T]) -> bool {
    let n = data.len();
    for i in 0..(n-1){
        if data[i] > data[i + 1]{
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = [1,2,3, 4];
        let result = is_ascending(&x);
        assert_eq!(result, true);
    }
}
