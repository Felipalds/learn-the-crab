fn main() {
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{s}"); this would not work, because S is not from main anymore!

    let x = 5;
    only_copies(5);

    println!("{x}"); //this works because it is Copy mode (i32, u32, bool and simmilars!)
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn only_copies(some_integer: i32) {
    println!("{some_integer}");
}
