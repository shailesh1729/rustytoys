//use std::mem;

/// Performs insertion sort on an array of i32egers
pub fn insertion_sort_int(data : &mut [i32]){
    let n  = data.len();
    for j in 1..n {
        let key = data[j];
        // we insert data[j] into the sorted sequence 
        //data[0...j-1] 
        let mut i = j -1;
        while data[i] > key{
            data[i + 1]  = data[i];
            if i == 0{
                break;
            }
            i -= 1;
        }
        data[i] = key;
    }
}


/// Performs insertion sort on an array of type T
pub fn insertion_sort_ord_copy<T : PartialOrd + Copy>(data : &mut [T]){
    let n  = data.len();
    for j in 1..n {
        let key = data[j];
        // we insert data[j] into the sorted sequence 
        //data[0...j-1]
        let mut i = j -1;
        while data[i] > key{
            data[i + 1]  = data[i];
            if i == 0{
                break;
            }
            i -= 1;
        }
        data[i] = key;
    }
}

pub fn insertion_sort<T : PartialOrd>(data : &mut [T]){
    let n  = data.len();
    for j in 1..n {
        // we insert data[j] into the sorted sequence 
        //data[0...j-1]
        let mut i = j -1;
        while data[i] > data[i+1]{
            data.swap(i + 1, i);
            if i == 0{
                break;
            }
            i -= 1;
        }
    }
}


#[cfg(test)]
mod test {
    use super::insertion_sort;
    use super::super::is_ascending; 

    #[test]
    fn test0() {
        let mut x : [i32; 5] = [1, 2, 3, 4, 5];       
        insertion_sort(&mut x);
        assert!(is_ascending(&x));
    }

    #[test]
    fn test1() {
        let mut x : [i32; 5] = [5,4,3,2,1];
        insertion_sort(&mut x);
        assert!(is_ascending(&x));
    }


    #[test]
    fn test2() {
        let mut x : [i32; 9] = [5,4,1,2,3, 10, 7, 8, 6];
        insertion_sort(&mut x);
        assert!(is_ascending(&x));
    }

    #[test]
    fn test3() {
        let mut x : [f64; 9] = [5f64,4.,1.,2.,3., 10., 7., 8., 6.];
        insertion_sort(&mut x);
        assert!(is_ascending(&x));
    }
}
