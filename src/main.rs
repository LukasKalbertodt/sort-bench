mod impls;

fn main() {
    let mut arr = [9, 3, 6, 1, 4, 2];
    impls::quick::quick_sort_hoare_center(&mut arr);
    println!("{:?}", arr);
}
