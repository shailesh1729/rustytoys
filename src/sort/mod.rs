pub mod insertion;

/// Returns whether the slice is sorted in ascending order or not.
pub fn is_ascending<T : PartialOrd>(data : & [T]) -> bool {
    let n = data.len();
    for i in range(0, n-1){
        if data[i] > data[i + 1]{
            return false;
        }
    }
    true
}