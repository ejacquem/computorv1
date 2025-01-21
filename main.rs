#[derive(Debug, Copy, Clone)]
struct Data
{
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();

    if argc != 2 {
        return println!("usage: ./computor [equation]");
    }
    let equation = parse_input(args[1].clone());
    let data = parse_equation(equation);
    let data = match data {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    print_reduced(data);
    print_degree(data);
    print_solution(data);
}


// transform an equation from x + 1 = 4 - 3x into x + 1 - 4 + 3x = 0
fn parse_input(equation: String) -> String
{
    let mut found_equal_sign = false;

    let mut eq: Vec<char> = equation.replace(" ", "").chars().collect();

    for i in 0..eq.len() {
        if found_equal_sign {
            if eq[i] == '+' {
                eq[i] = '-';
            } else if eq[i] == '-' {
                eq[i] = '+';
            }
        } else if eq[i] == '=' {
            found_equal_sign = true;
            // if the char after = is a - or +, replace = by a space and let the loop do the job
            if i + 1 < eq.len() && (eq[i + 1] == '+' || eq[i + 1] == '-') {
                eq[i] = ' '
            } else { // if there is no + or -, the next number is positive and need to be inverted
                eq[i] = '-'
            }
        }
    }

    return eq.iter().collect();
}

// split the given string with delimiter + and -, and keeps them at the beginning of the string
fn split_with_signs(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    
    for ch in input.chars() {
        if ch == '+' || ch == '-' {
            if !current.is_empty() {
                result.push(current);
            }
            current = ch.to_string();
        } else {
            current.push(ch);
        }
    }
    result.push(current);
    
    return result;
}

/*
parse an equation in reduced format like "4+1*X+5*X^2" and return the coefficient of each power of X:
x0: 4
x1: 1
x2: 5
*/
fn parse_equation(equation: String) -> Result<Data, String>
{
    let trim_equation = equation.replace(" ", "");
    let mut data = Data{ x0: 0.0, x1: 0.0, x2: 0.0, x3: 0.0 };
    let parts: Vec<String> = split_with_signs(&trim_equation);
    // println!("{:?}", parts);
    for part in parts {
        let term: Vec<&str> = part.split('*').collect();
        let mut deg: f64 = 1.0;
        let mut coef = String::new();
        if term.len() == 1 && term[0].find("X").is_some(){
            if let Some(first_char) = term[0].chars().next() {
                if first_char == '+' || first_char == '-' {
                    coef = term[0].chars().skip(1).collect(); // Skip first character and collect into String
                } else {
                    coef = term[0].to_string(); // Directly convert &str to String
                }
            }
        } else {
            deg = term[0].parse::<f64>().map_err(|_| "Invalid number".to_string())?;
        }
        // println!("part: {}", term[0]);
        if term.len() > 2 {
            return Err("Invalid format".to_string() + term[1]);
        } else if term.len() == 2{
            coef = term[1].to_string();
        }

        if coef == "X^3" {
            data.x3 += deg;
        } else if coef == "X^2" {
            data.x2 += deg;
        } else if coef == "X^1" || coef == "X" {
            data.x1 += deg;
        } else if coef == "X^0" || coef == ""{
            data.x0 += deg;
        } else if term.len() == 1 {
            
        } else {
            return Err("Invalid format".to_string() + term[1]);
        }
    }
    // println!("{:?}", data);
    return Ok(data);
}

fn get_exponent(exp: i32) -> String {
    match exp {
        0 => "".to_string(),
        1 => "x".to_string(),
        2 => "x²".to_string(),
        3 => "x³".to_string(),
        _ => format!("x^{}", exp),
    }
}

/*
Reduced form: 5 + 4 * X = 0
*/
fn print_reduced(data: Data)
{
    let mut exponent = 0;
    let mut first_print = true;
    
    print!("Reduced form:");
    for number in [data.x0, data.x1, data.x2, data.x3] {
        let mut sign = if number < 0.0 {"- "} else {"+ "};
        if number != 0.0 {
            if sign == "+ " && first_print {
                sign = "";
            }
            // print!(" {}{} * X^{}", sign, number.abs(), exponent);
            print!(" {}{}{}", sign, number.abs(), get_exponent(exponent));
            first_print = false;
        }
        exponent += 1;
    }
    println!(" = 0");
}

fn print_degree(data: Data)
{
    let mut max_degree = 0;
    if data.x1 != 0.0 {
        max_degree = 1;
    }
    if data.x2 != 0.0 {
        max_degree = 2;
    }
    if data.x3 != 0.0 {
        max_degree = 3;
    }
    println!("Polynomial degree: {}", max_degree);
}

fn print_solution(data: Data)
{
    if data.x3 != 0.0 {
        println!("The polynomial degree is strictly greater than 2, I can't solve.");
        return;
    }

    let (a, b, c) = (data.x2, data.x1, data.x0);
    let mut solution;
    if a != 0.0 {
        let mut delta = b * b - 4.0 * a * c;
        println!("Delta: {delta}");
        if delta < 0.0 {
            println!("There is no solution.");
        }
        else {
            delta = delta.sqrt();
            let solution_a = (-b + delta) / (2.0 * a);
            let solution_b = (-b - delta) / (2.0 * a);
            println!("The solutions are:\n{solution_a} and {solution_b}");
        }
    }
    else {
        solution = -c/b;
        if solution == -0.0 {
            solution = 0.0;
        }
        println!("The solution is:\n{solution}");
    }
}