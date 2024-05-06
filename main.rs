use std::io;
enum Calculator{
    Add,
    Subtract,
    Multiply,
    Division,

}
fn main() {
    println!("Enter first number ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: i32 = num1.trim().parse().expect("Please enter a number!");

    println!("Enter second munber");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:i32 = num2.trim().parse().expect("Plesse enter numver");

    println!("Choose Operations");
    println!("1.  Add");
    println!("2.  Subtract");
    println!("3.  Multiplication");
    println!("4.  Division");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation:u32 = operation.trim().parse().expect("Enter the number");

    let result = match operation {
        1 => calculate(num1, num2, Calculator::Add),
        2 => calculate(num1, num2, Calculator::Subtract),
        3 => calculate(num1, num2, Calculator::Multiply),
        4 => calculate(num1, num2, Calculator::Division),
        _ => {
            println!("Invalid Option");
            return;
        }

    };
    
    print!("The result is {}", result);

   
}

fn calculate(num1:i32 ,num2: i32, operation: Calculator) ->f32{
    match operation{
    Calculator :: Add => (num1 + num2 ) as f32,
    Calculator :: Subtract => (num1 - num2 ) as f32,
    Calculator :: Multiply => (num1 * num2 ) as f32,
    Calculator :: Division => (num1 / num2 ) as f32,
    }


}
