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
