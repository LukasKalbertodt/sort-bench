
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

        // Generate unit tests for this algorithm
        #[cfg(test)]
        mod $fn_name {
            use super::$fn_name;
            gen_sort_test_suite!($fn_name);
        }
    }
}

gen_quick_sort!(quick_sort_hoare_center, partition_hoare, pivot_center);
gen_quick_sort!(quick_sort_lomuto_center, partition_lomuto, pivot_center);

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

fn partition_lomuto<T: Ord>(arr: &mut [T], pivot_idx: usize) -> usize {
    // Store the pivot at the very beginning)
    arr.swap(0, pivot_idx);

    // During partitioning we manage four different sections within the slice:
    //
    //     +------------------------------------------------+
    //     | p |  <= pivot  |  > pivot  |  not checked yet  |
    //     +------------------------------------------------+
    //       ^               ^           ^
    //       0          startUpper    startRest
    //
    //
    // The index `start_upper` refers to the first element of the section that
    // only contains elements greater than the pivot. The index `start_rest`
    // refers to the first element in the section of elements we still need
    // to check.
    let mut start_upper = 1;
    for start_rest in 1..arr.len() {
        // We check the first element of the last section now. If it is greater
        // than the pivot (and belongs into the third section), we don't need
        // to do anything: the third section is simply enlarged by one (which
        // is done by the loop counter).
        //
        // If, however, the element is smaller or equal to the pivot, we need
        // to put it into the second section. To do this, we swap it with
        // the first element of the third section and enlarge the second
        // section by one. This is operation runs in O(1).
        if &arr[start_rest] <= &arr[0] {
            arr.swap(start_upper, start_rest);
            start_upper += 1;
        }
    }

    // Now we only need to swap the pivot back into the middle and return its
    // position.
    let new_pivot_index = start_upper - 1;
    arr.swap(0, new_pivot_index);

    new_pivot_index
}
