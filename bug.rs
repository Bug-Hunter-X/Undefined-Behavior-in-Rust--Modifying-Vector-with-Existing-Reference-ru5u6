fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let x = &vec[0];

    // This is UB. Modifying a vector while holding a reference to one of its elements.
    vec.push(4);

    println!("x = {}", x);
}