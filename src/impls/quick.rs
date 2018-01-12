
macro_rules! gen_quick_sort {
    ($fn_name:ident, $partition_algo:ident, $pivot_algo:ident) => {
        pub fn $fn_name<T: Ord>(arr: &mut [T]) {
            if arr.len() <= 1 {
                return;
            }

            let pivot_idx = $pivot_algo(arr);
            let center = $partition_algo(arr, pivot_idx);

            $fn_name(&mut arr[..center]);
            $fn_name(&mut arr[center + 1 ..]);
        }
    }
}

gen_quick_sort!(quick_sort_hoare_center, partition_hoare, pivot_center);

// ===========================================================================
// ===== Pivot choosing algorithms
// ===========================================================================
fn pivot_center<T>(arr: &[T]) -> usize {
    arr.len() / 2
}


// ===========================================================================
// ===== Partitioning Algorithms
// ===========================================================================

/// The array needs to be at least 1 long
fn partition_hoare<T: Ord>(arr: &mut [T], pivot_idx: usize) -> usize {
    // First, swap the pivot to the very beginning.
    arr.swap(0, pivot_idx);

    // Left and right indices. Everything left from `l` will be smaller than or
    // equal to the pivot element; everything right from `r` will be larger
    // than the pivot. Both indices will get closer together. The algorithm is
    // finished once they touch or cross each other.
    let mut l = 1;
    let mut r = arr.len() - 1;

    loop {
        // Find an element which is larger than the pivot (or stop when at last
        // element).
        while l < arr.len() - 1 && &arr[l] <= &arr[0] {
            l += 1;
        }

        // We don't need the `r > 0` check here, because we know that the pivot
        // itself is stored at index 0. So this loop will stop at r = 0 anyway.
        while &arr[r] > &arr[0] {
            r -= 1;
        }

        if l < r {
            arr.swap(l, r);
        } else {
            break;
        }
    }

    // The `r` index will stop being decreased once it points to an element
    // smaller than or equal to the pivot. That means that in the end, it
    // points to the last element of the left section. *Except* if there is not
    // a single element smaller than or equal to the pivot (the left section is
    // empty, except for the pivot itself, of course). If that's the case, we
    // don't need to do anything. Otherwise, we swap the pivot element to the
    // end of the first section.
    if r != 0 {
        arr.swap(0, r);
    }

    // In any case, `r` is the new position of the pivot element.
    r
}

// pub fn partition_lomuto<T>(arr: &mut [T], pivot: &T)
// where
//     T: Ord,
// {
//     let mut end_lower = 0;

//     for end_upper in 0..arr.len() {
//         if &arr[end_upper] <= pivot {
//             arr.swap(end_lower, end_upper);
//             end_lower += 1;
//         }
//     }
// }

#[cfg(test)]
gen_sort_test_suite!(quick_sort_hoare_center);
