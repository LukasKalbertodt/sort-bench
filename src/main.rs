
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand;

#[cfg(test)]
#[macro_use]
mod test;

mod bench;
mod gen;
mod impls;


use std::time::Duration;


fn main() {
    let meas = bench::run(
        // impls::quick::quick_sort_hoare_center,
        |arr| arr.sort(),
        || gen::random::<u32>(10_000),
        Duration::from_millis(3_000),
        50_000,
    );

    println!("{:#?}", meas.unwrap().analyse());
}
