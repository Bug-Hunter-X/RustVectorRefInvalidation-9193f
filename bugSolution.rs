fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0).cloned(); // Clone the value to avoid reference issues
    println!("First element: {:?}", first);

    // Modify the vector
    vec.push(3);
    
    // Access the copied value.
    println!("First element after modification: {:?}", first);
} 