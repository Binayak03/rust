use std::io;
enum Operation{
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

    println!("Choose number");
    println!("1.  Add");
    println!("2.  Subtract");
    println!("3.  Multiplication");
    println!("4.  Division");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number:u32 = number.trim().parse().expect("Enter the number");

    let result = match number {
        1 => calculate(num1, num2, Operation::Add),
        2 => calculate(num1, num2, Operation::Subtract),
        3 => calculate(num1, num2, Operation::Multiply),
        4 => calculate(num1, num2, Operation::Division),
        _ => {
            println!("Invalid Option");
            return;
        }

    };
    
    print!("The result is {}", result);

   
}

fn calculate(num1:i32 ,num2: i32, number: Operation) ->f32{
    match number{
    Operation :: Add => (num1 + num2 ) as f32,
    Operation :: Subtract => (num1 - num2 ) as f32,
    Operation :: Multiply => (num1 * num2 ) as f32,
    Operation :: Division => (num1 / num2 ) as f32,
    }


}
