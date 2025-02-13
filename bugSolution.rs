fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Several solutions to avoid UB:
    // 1. Clone the value
    let x = vec[0].clone();
    vec.push(4);
    println!("x (cloned) = {}", x);

    // 2. Iterate and create a new vector
    let x = vec.iter().next().copied().unwrap();
    vec.push(4);
    println!("x (iterated) = {}", x);

    // 3. If you only need to read the value once without modifying the vector, create a local variable to hold the value.
    let x = vec[0];
    println!("x = {}", x);
    vec.push(4);
    
    // Note:  In this last solution, vec[0] may refer to stale memory after vec.push(4) but this is not UB since the value is not used anymore.
} 