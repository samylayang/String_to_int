fn main() {
    let s: String = "42".to_string();
    let number: i32 = s.parse().expect("Not a valid number");
    println!("{}", number); // Outputs: 42

    let s = "42a".to_string();
    match s.parse::<i32>() {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    // Outputs: Error: invalid digit found in string
}
