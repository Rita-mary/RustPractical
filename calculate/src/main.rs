use std::io;
fn main() {
    println!("This is my calculator attempt in Rust!!");
    let mut r#continue_a : bool = true;
    let mut r#continue_b : bool = true;
    while r#continue_b{
        let answer = temperature()  ;
        println!("The result of your calculation is: {}", answer);
        let mut confirm = String::new();
        println!("Enter 'continue' ,to keep calculating or any other input to quit: ");
        io::stdin().read_line(&mut confirm).expect("Unable to read input");
        let confirm = confirm.trim().to_lowercase();
        if confirm.trim() ==  "continue" {
            continue
        }else {
            r#continue_b = false;
        }
    }
    while r#continue_a{
        let result = calculate();
        println!("The result of your calculation is: {}", result);
        let mut check = String::new();
        println!("Enter 'continue' ,to keep calculating or any other input to quit: ");
        io::stdin().read_line(&mut check).expect("Unable to read input");
        let check = check.trim().to_lowercase();
        if check.trim() ==  "continue" {
            continue
        }else {
            r#continue_a = false;
        }
}
}
fn calculate()-> f32 {
    let mut num1s =   String::new();
    let mut num2s =   String::new();
    let mut ops =   String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1s).expect("Unable to read input");
    println!("Enter the second number: ");
    let first_num :f32 = num1s.trim().parse().expect("unable to convert to number");
    io::stdin().read_line(&mut num2s).expect("Unable to read input");
    let second_num : f32= num2s.trim().parse().expect("unable to convert to number");
    println!("Enter the operation to be performed(+, - , / , * )");
    io::stdin().read_line(&mut ops).expect("Unable to read input");
    let operation: char   = ops.trim().parse().expect("Operation could not be converted to String");
    match operation {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' => first_num * second_num,
        '/' => if second_num == 0.0{
            panic!("Division by zero is not possible");
        }else{
            first_num / second_num
        } 
        _ => panic!("Invalid operation, use only (+, - , / or *)"),
    }
}
fn temperature() ->String{
    println!("Enter 'F' to convert from Fahrenheit to Celsius or 'C' to convert from Celsius to Fahrenheit");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Error reading the input ");
    let unit = unit.trim().to_lowercase();
    println!("Enter the number you wish to convert");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error reading the input ");
    let num : f32 = num.trim().parse().expect("Unable to convert to number");
    if unit.trim() == "f"{
        let value:f32 = (num - 32.0) * 5.0/9.0;
        format!("{}C" , value)
    }else if unit.trim() =="c" {
        let value:f32 = (num * 9.0/5.0) + 32.0;
        format!("{}F" , value)
    } else {
        panic!("Invlaid input for unit")
    }
}