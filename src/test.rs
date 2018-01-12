use std::panic::{self, UnwindSafe};
use std::fmt::Debug;


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



pub enum TestError<T> {
    Panic,
    ChangedLength {
        before: usize,
        after: usize,
    },
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
            TestError::ChangedLength { before, after } => {
                panic!(
                    "Sorting algorithm changed the length of the input from {} to {}",
                    before,
                    after,
                );
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

    // Check for different bugs
    if result.len() != orig.len() {
        return Err(TestError::ChangedLength {
            before: orig.len(),
            after: result.len(),
        });
    }

    if correct != result {
        return Err(TestError::Incorrect {
            original: orig,
            result: result,
        });
    }

    Ok(())
}
