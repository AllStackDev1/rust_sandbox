pub fn run() {
    //Print to console
    println!("hello!");

    // Basic Formatting
    println!("{} is from {}", "Chinedu", "Achina");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Chinedu", "Achina", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Chinedu",
        activity = "Baseball"
    );

    // Placeholder Traits
    println!("Binary: {:b}  Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug Traits
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
