use rand::{self, Rng};
use rand::distributions::range::SampleRange;

pub fn random<T: SampleRange + FromUsize + PartialOrd>(count: usize) -> Vec<T> {
    let mut rng = rand::weak_rng();
    (0..count)
        .map(|_| rng.gen_range(T::from_usize(0), T::from_usize(count)))
        .collect()
}

pub fn one_element<T: Clone>(count: usize, elem: T) -> Vec<T> {
    vec![elem; count]
}

pub fn simple_increasing<T: FromUsize>(count: usize) -> Vec<T> {
    (0..count).map(FromUsize::from_usize).collect()
}

pub fn simple_decreasing<T: FromUsize>(count: usize) -> Vec<T> {
    (0..count).rev().map(FromUsize::from_usize).collect()
}



pub trait FromUsize {
    fn from_usize(x: usize) -> Self;
}

macro_rules! impl_from_usize_with_cast {
    ($ty:ident) => {
        impl FromUsize for $ty {
            fn from_usize(x: usize) -> Self {
                x as $ty
            }
        }
    }
}

impl_from_usize_with_cast!(u8);
impl_from_usize_with_cast!(u16);
impl_from_usize_with_cast!(u32);
impl_from_usize_with_cast!(u64);
impl_from_usize_with_cast!(i8);
impl_from_usize_with_cast!(i16);
impl_from_usize_with_cast!(i32);
impl_from_usize_with_cast!(i64);
impl_from_usize_with_cast!(f32);
impl_from_usize_with_cast!(f64);
