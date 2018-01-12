extern crate rand;

#[cfg(test)]
#[macro_use]
mod test;
mod gen;
mod impls;

fn main() {
    let mut arr = [9, 3, 6, 1, 4, 2];
    impls::quick::quick_sort_hoare_center(&mut arr);
    println!("{:?}", arr);
    impls::quick::quick_sort_lomuto_center(&mut arr);
}


// TODO Next:
// - add failure
// - finish testing
