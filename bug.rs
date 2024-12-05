fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    println!("First element: {:?}", first);
    // Modify the vector
    vec.push(3);
    // Access the first element again
    println!("First element after modification: {:?}", first);
}