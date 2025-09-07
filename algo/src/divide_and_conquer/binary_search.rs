fn main() {
    // Example usage
    let arr = [1, 3, 5, 7, 9];
    println!("{:?}", binary_search(&arr, 3)); // Some(1)
    println!("{:?}", binary_search(&arr, -1)); // None
}

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }

    if arr[low] == target {
        Some(low)
    } else {
        None
    }
}