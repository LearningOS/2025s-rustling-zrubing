/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn merge<T:Ord+Clone>(left:Vec<T>,right:Vec<T>)->Vec<T>{
    let mut result = Vec::with_capacity(left.len()+right.len());
    let mut left_index = 0;
    let mut right_index=0;

    while left_index < left.len() &&right_index<right.len(){
        if left[left_index] <= right[right_index]{
            result.push(left[left_index].clone());
            left_index+=1;
        }else {
            result.push(right[right_index].clone());
            right_index+=1;
        };
    }

    result.extend_from_slice(&left[left_index..]);
    result.extend_from_slice(&right[right_index..]);
    result
}

fn merge_sort_arr<T: Ord + Clone>(arr: &mut [T]) {
    let vec = arr.to_vec();

    let res = merge_sort(vec);
    arr.clone_from_slice(&res);
}

fn merge_sort<T: Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    match arr.len() {
        0 | 1 => arr,
        _ => {
            let mid = arr.len()/2;
            let left = merge_sort(arr[..mid].to_vec());
            let right = merge_sort(arr[mid..].to_vec());
            merge(left,right)
        }
    }
}

fn sort<T>(array: &mut [T])
where
    T: Ord+Clone,
{
   //merge sort
    merge_sort_arr(array);

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
