// fn gcd(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         a
//     } else {
//         gcd(b, a % b)
//     }
// }

// const PRECISION: f64 = 1000000.0;

// fn is_integer(num: f64) -> bool {
//     let scaled = num * PRECISION;
//     scaled != scaled.round()
// }

// fn float_to_irreducible_fraction(num: f64) {
//     let numerator = num * 1000000.0; // Shift decimal 6 places (for precision)
//     let denominator = 1000000;
    
//     // Convert to integers
//     let mut numerator: i32 = numerator.round() as i32;
//     let denominator: i32 = denominator;

//     // Find the greatest common divisor (GCD)
//     let gcd_val = gcd(numerator.abs(), denominator.abs());

//     // Reduce the fraction
//     numerator /= gcd_val;
//     let reduced_denominator = denominator / gcd_val;

//     // Handle the sign
//     let sign = if (numerator < 0) != (reduced_denominator < 0) { -1 } else { 1 };
//     let result_numerator = numerator.abs() * sign;

//     println!("The irreducible fraction is: {}/{}", result_numerator, reduced_denominator);
// }

// fn main() {
//     let num = 1.0/7.0;
//     float_to_irreducible_fraction(num); // Should print -3/4
// }


/*
    split the given string with delimiter + and -, and keeps them at the beginning of the string
*/
fn split_with_signs(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    
    for ch in input.chars() {
        if ch == '+' || ch == '-' {
            result.push(current);
            current = ch.to_string();
        } else {
            current.push(ch);
        }
    }
    result.push(current);
    
    return result;
}
fn main() {
    let input = "4+5x+8x^2-1++";
    let result = split_with_signs(input);
    for part in result {
        println!("{}", part);
    }
}