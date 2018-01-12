use std::panic::{self, UnwindSafe};
use std::fmt::Debug;

/// For internal use. Used by `gen_sort_test_suite`. Don't use directly!
macro_rules! gen_sort_test {
    ($sorter:ident, $name:ident, $gen:expr) => {
        #[test]
        fn $name() {
            let lens = [1, 2, 3, 4, 10, 23, 77, 100];
            for &len in &lens {
                let v = $gen(len);
                if let Err(e) = $crate::test::test_vec($sorter, v) {
                    e.print_and_panic();
                }
            }
        }
    }
}

/// Generates several unit tests for the given sorting algorithm.
///
/// You only need to pass the name of the function you want to test. This
/// sorting function needs the following signature:
///
/// ```ignore
/// fn name<T: Ord>(arr: &mut [T])
/// ```
///
/// The following tests are generated:
/// - `test_empty`: empty array
/// - `test_simple_incr`: just counting up values (e.g. `[0, 1, 2, 3]`)
/// - `test_simple_decr`: just counting down values (e.g. `[3, 2, 1, 0]`)
/// - `test_zeroes`: an array full of zeroes
/// - `test_random`: an array filled with random values
///
/// All those tests (except `test_empty`) tests arrays of different lengths.
///
///
/// # Example
///
/// ```
/// #[cfg(test)]
/// gen_sort_test_suite!(quick_sort_hoare_center);
/// ```
macro_rules! gen_sort_test_suite {
    ($sorter:ident) => {
        #[test]
        fn test_empty() {
            if let Err(e) = $crate::test::test_vec::<u32, _>($sorter, vec![]) {
                e.print_and_panic();
            }
        }

        gen_sort_test!($sorter, test_random, $crate::gen::random::<u32>);
        gen_sort_test!($sorter, test_simple_incr, $crate::gen::simple_increasing::<u32>);
        gen_sort_test!($sorter, test_simple_decr, $crate::gen::simple_decreasing::<u32>);
        gen_sort_test!($sorter, test_zeroes, |size| $crate::gen::one_element::<u32>(size, 0));
    }
}


/// Everything a sorting algorithm can fuck up.
pub enum TestError<T> {
    /// The algorithm panicked
    Panic,

    /// The result is not correctly sorted.
    Incorrect {
        original: Vec<T>,
        result: Vec<T>,
    },
}

impl<T: Debug> TestError<T> {
    pub fn print_and_panic(self) -> ! {
        match self {
            TestError::Panic => {
                panic!("Sorting algorithm panicked!");
            }
            TestError::Incorrect { original, result } => {
                println!("Incorrect sorting result! Original:");
                println!("{:?}", original);
                println!("Incorrect result:");
                println!("{:?}", result);

                panic!("Incorrect sorting result. See stdout for more details!");
            }
        }
    }
}


pub fn test_vec<T, S>(sorter: S, v: Vec<T>) -> Result<(), TestError<T>>
where
    T: Ord + Clone + UnwindSafe,
    S: FnOnce(&mut [T]) + UnwindSafe,
{
    let orig = v.clone();
    let mut result = v.clone();
    let mut correct = v;

    // Use the given sorting algorithm. We want to catch any panics emerging
    // from the algorithm to show a nice error.
    let result = panic::catch_unwind(move || {
        sorter(&mut result);
        result
    }).map_err(|_| TestError::Panic)?;

    // Use the algorithm from the standard library. We assume it's correct.
    correct.sort();

    if correct != result {
        return Err(TestError::Incorrect {
            original: orig,
            result: result,
        });
    }

    Ok(())
}
