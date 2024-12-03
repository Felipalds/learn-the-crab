fn main() {
    let mut b = 32;
    let mut a: &mut i32 = &mut b;

    *a += 1;

    println!("{b}");
}
