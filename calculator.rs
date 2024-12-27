use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn parse_input(input: &str, x: f64, y: f64) -> Result<Result<f64, String>, String> {
        match input {
            "Add" => Ok(calculate(Operation::Add(x, y))),
            "Subtract" => Ok(calculate(Operation::Subtract(x, y))),
            "Multiply" => Ok(calculate(Operation::Multiply(x, y))),
            "Divide" => Ok(calculate(Operation::Divide(x, y))),
            _ => Err("Invalid operation!".to_string()),
        }
    }
}

fn calculate(oper: Operation) -> Result<f64, String> {
    match oper {
        Operation::Add(x, y) => Ok(x + y),
        Operation::Subtract(x, y) => Ok(x - y),
        Operation::Multiply(x, y) => Ok(x * y),
        Operation::Divide(x, y) => {
            if y == 0.0 {
                Err("Invalid! (Divide by 0)".to_string())
            } else {
                Ok(x / y)
            }
        }
    }
}

fn main() {
    let mut number_1 = String::new();
    let mut number_2 = String::new();
    let mut operation = String::new();

    println!("Enter first number: ");
    io::stdin().read_line(&mut number_1).unwrap();
    let value_1 = number_1.trim().parse::<f64>().unwrap();

    println!("Enter second number: ");
    io::stdin().read_line(&mut number_2).unwrap();
    let value_2 = number_2.trim().parse::<f64>().unwrap();

    println!("Enter operation (Add, Subtract, Multiply, Divide): ");
    io::stdin().read_line(&mut operation).unwrap();

    // trim() is needed for string input to erase tracing spaces
    let operator = Operation::parse_input(&operation.trim(), val_1, val_2);

    // 'operation' will be either Result<f64,String> or Err
    match operator {
        // 'x' will be either f64 or Error
        Ok(x) => match x {
            Ok(value) => println!("{:?}", value),
            Err(msg) => println!("{}", msg),
        },
        Err(error) => println!("{}", error),
    };
}
