// pub mod sorting_lib {
//     use std::cmp::Ordering;

//     pub fn quick_sort<T: PartialOrd + std::clone::Clone>(arr: &mut [T]) {
//         if arr.len() > 1 {
//             let pivot_index = partition(arr);
//             quick_sort(&mut arr[..pivot_index]);
//             quick_sort(&mut arr[pivot_index + 1..]);
//         }
//     }

//     fn partition<T: PartialOrd + std::clone::Clone>(arr: &mut [T]) -> usize {
//         let pivot_index = arr.len() / 2;
//         arr.swap(pivot_index, arr.len() - 1);
//         let pivot = arr[arr.len() - 1].clone();
//         let mut i = 0;
//         for j in 0..arr.len() - 1 {
//             if arr[j] < pivot {
//                 arr.swap(i, j);
//                 i += 1;
//             }
//         }
//         arr.swap(i, arr.len() - 1);
//         i
//     }
    
    
//     pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
//         let n = arr.len();
//         for i in 0..n {
//             let mut min_index = i;
//             for j in i + 1..n {
//                 if arr[j] < arr[min_index] {
//                     min_index = j;
//                 }
//             }
//             arr.swap(i, min_index);
//         }
//     }

//     pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
//         for i in 1..arr.len() {
//             let mut j = i;
//             while j > 0 && arr[j - 1] > arr[j] {
//                 arr.swap(j, j - 1);
//                 j -= 1;
//             }
//         }
//     }

//     pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
//         if arr.len() > 1 {
//             let mid = arr.len() / 2;
//             let (left, right) = arr.split_at(mid);
//             let mut left_copy = Vec::from(left);
//             let mut right_copy = Vec::from(right);
//             merge_sort(&mut left_copy);
//             merge_sort(&mut right_copy);
//             merge(arr, &left_copy, &right_copy);
//         }
//     }
    
//     fn merge<T: PartialOrd + Copy>(arr: &mut [T], left: &[T], right: &[T]) {
//         let mut i = 0;
//         let mut j = 0;
//         let mut k = 0;
    
//         while i < left.len() && j < right.len() {
//             if left[i] <= right[j] {
//                 arr[k] = left[i];
//                 i += 1;
//             } else {
//                 arr[k] = right[j];
//                 j += 1;
//             }
//             k += 1;
//         }
    
//         while i < left.len() {
//             arr[k] = left[i];
//             i += 1;
//             k += 1;
//         }
    
//         while j < right.len() {
//             arr[k] = right[j];
//             j += 1;
//             k += 1;
//         }
//     }
    
// }

// pub use sorting_lib::*;








use std::cmp::Ordering;

pub fn quick_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }
    let mut stack = vec![(0, arr.len() - 1)];
    while let Some((lo, hi)) = stack.pop() {
        if lo >= hi {
            continue;
        }
        let pivot_index = partition(&mut arr[lo..=hi], &compare);
        if pivot_index > 0 {
            stack.push((lo, lo + pivot_index - 1));
        }
        stack.push((lo + pivot_index + 1, hi));
    }
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if compare(&arr[j], &arr[len - 1]) == Ordering::Less {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

pub fn selection_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in (i + 1)..arr.len() {
            if compare(&arr[j], &arr[min_index]) == Ordering::Less {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

pub fn insertion_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j - 1], &arr[j]) == Ordering::Greater {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: Clone, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut temp = arr.to_vec();

    merge(&mut arr[..], &mut temp[..], &compare);
}

fn merge<T, F>(arr: &mut [T], temp: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge(&mut arr[..mid], &mut temp[..mid], compare);
    merge(&mut arr[mid..], &mut temp[mid..], compare);

    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < len {
        if compare(&arr[i], &arr[j]) == Ordering::Less {
            temp[k] = arr[i].clone();
            i += 1;
        } else {
            temp[k] = arr[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        temp[k] = arr[i].clone();
        i += 1;
        k += 1;
    }

    while j < len {
        temp[k] = arr[j].clone();
        j += 1;
        k += 1;
    }

    for i in 0..len {
        arr[i] = temp[i].clone();
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quick_sort() {
        let mut arr = vec![4, 2, 5, 1, 3];
        quick_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        
        let mut arr = vec!['d', 'b', 'e', 'a', 'c'];
        quick_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);
    }
    
    #[test]
    fn test_selection_sort() {
        let mut arr = vec![4, 2, 5, 1, 3];
        selection_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        
        let mut arr = vec!['d', 'b', 'e', 'a', 'c'];
        selection_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);
    }
    
    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![4, 2, 5, 1, 3];
        insertion_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        
        let mut arr = vec!['d', 'b', 'e', 'a', 'c'];
        insertion_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);
    }
    
    #[test]
    fn test_merge_sort() {
        let mut arr = vec![4, 2, 5, 1, 3];
        merge_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        
        let mut arr = vec!['d', 'b', 'e', 'a', 'c'];
        merge_sort(&mut arr, |a, b| if a < b { std::cmp::Ordering::Less } else if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
        assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);
    }
}




















//=============================================================================================================second option====================


// pub mod sorting {
//     use std::cmp::Ordering;

//     pub fn quick_sort<T, F>(arr: &mut [T], compare: F)
//     where
//         F: Fn(&T, &T) -> Ordering,
//     {
//         if arr.len() <= 1 {
//             return;
//         }
//         let pivot_index = partition(arr, &compare);
//         quick_sort(&mut arr[0..pivot_index], &compare);
//         quick_sort(&mut arr[pivot_index + 1..], &compare);
//     }

//     fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
//     where
//         F: Fn(&T, &T) -> Ordering,
//     {
//         let pivot_index = arr.len() - 1;
//         let mut i = 0;
//         for j in 0..pivot_index {
//             if compare(&arr[j], &arr[pivot_index]) == Ordering::Less {
//                 arr.swap(i, j);
//                 i += 1;
//             }
//         }
//         arr.swap(i, pivot_index);
//         i
//     }

//     pub fn select_sort<T, F>(arr: &mut [T], compare: F)
//     where
//         F: Fn(&T, &T) -> Ordering,
//     {
//         for i in 0..arr.len() {
//             let mut min_index = i;
//             for j in (i + 1)..arr.len() {
//                 if compare(&arr[j], &arr[min_index]) == Ordering::Less {
//                     min_index = j;
//                 }
//             }
//             if i != min_index {
//                 arr.swap(i, min_index);
//             }
//         }
//     }

//     pub fn insert_sort<T, F>(arr: &mut [T], compare: F)
//     where
//         F: Fn(&T, &T) -> Ordering,
//     {
//         for i in 1..arr.len() {
//             let mut j = i;
//             while j > 0 && compare(&arr[j - 1], &arr[j]) == Ordering::Greater {
//                 arr.swap(j, j - 1);
//                 j -= 1;
//             }
//         }
//     }

//     pub fn merge_sort<T: Clone, F>(arr: &mut [T], compare: F)
// where
//     F: Fn(&T, &T) -> Ordering + Copy,
// {
//     let len = arr.len();
//     if len <= 1 {
//         return;
//     }
//     let mid = len / 2;
//     merge_sort(&mut arr[..mid], compare);
//     merge_sort(&mut arr[mid..], compare);
//     let mut merged = Vec::with_capacity(len);
//     let (mut i, mut j) = (0, mid);
//     while i < mid && j < len {
//         if compare(&arr[i], &arr[j]) == Ordering::Less {
//             merged.push(arr[i].clone());
//             i += 1;
//         } else {
//             merged.push(arr[j].clone());
//             j += 1;
//         }
//     }
//     merged.extend_from_slice(&arr[i..mid]);
//     merged.extend_from_slice(&arr[j..]);
//     arr.copy_from_slice(&merged);
// }

// }

// #[cfg(test)]
// mod tests {
//     use super::sorting::*;
//     use std::cmp::Ordering;

//     #[test]
//     fn test_quick_sort() {
//         let mut arr = vec![5, 3, 8, 4, 2];
//         quick_sort(&mut arr, |a, b| a.cmp(b));
//         assert_eq!(arr, vec![2, 3, 4, 5, 8]);
//     }

//     #[test]
//     fn test_select_sort() {
//         let mut arr = vec![5, 3, 8, 4, 2];
//         select_sort(&mut arr, |a, b| a.cmp(b));
//         assert_eq!(arr, vec![2, 3, 4, 5, 8]);
//     }

//     #[test]
//     fn test_insert_sort() {
//         let mut arr = vec![5, 3, 8, 4, 2];
//         insert_sort(&mut arr, |a, b| a.cmp(b));
//         assert_eq!(arr, vec![2, 3, 4, 5, 8]);
//     }

//     #[test]
//     fn test_merge_sort() {
//         let mut arr = vec![5, 3, 8, 4, 2];
//         merge_sort(&mut arr, |a, b| a.cmp(b));
//         assert_eq!(arr, vec![2, 3, 4, 5, 8]);
//     }

//     #[test]
//     fn test_custom_objects_sort() {
//         #[derive(Debug, PartialEq)]
//         struct Custom {
//             id: i32,
//             name: String,
//         }

//         let mut arr = vec![
//             Custom {
//                 id: 2,
//                 name: "Bob".to_string(),
//             },
//             Custom {
//                 id: 1,
//                 name: "Alice".to_string(),
//             },
//             Custom {
//                 id: 3,
//                 name: "Charlie".to_string(),
//             },
//         ];

//         quick_sort(&mut arr, |a, b| a.id.cmp(&b.id));
//         assert_eq!(
//             arr,
//             vec![
//                 Custom {
//                     id: 1,
//                     name: "Alice".to_string()
//                 },
//                 Custom {
//                     id: 2,
//                     name: "Bob".to_string()
//                 },
//                 Custom {
//                     id: 3,
//                     name: "Charlie".to_string()
//                 }
//             ]
//         );
//     }
// }
