enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    // using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    // Prompt the user to input the first number
    println!("Enter the first number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let num1: f64 = input.trim().parse().unwrap();

    // Prompt the user to input the operation
    let mut input = String::new();
    println!("Enter the operation (Add, Subtract, Multiply, Divide): ");
    std::io::stdin().read_line(&mut input).unwrap();

    let operation = input.trim();

    // Prompt the user to input the second number
    let mut input = String::new();
    println!("Enter the second number: ");
    std::io::stdin().read_line(&mut input).unwrap();

    let num2: f64 = input.trim().parse().unwrap();

    // Create an Operation enum instance
    let op = match operation {
        "Add" => Operation::Add(num1, num2),
        "Subtract" => Operation::Subtract(num1, num2),
        "Multiply" => Operation::Multiply(num1, num2),
        "Divide" => Operation::Divide(num1, num2),
        _ => unimplemented!(), // Placeholder for other operations
    };

    // Call the calculate function
    let result = calculate(op);

    // Print the result to the console
    println!("The result is: {}", result);
}
