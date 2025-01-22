fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const MAX_FLOAT_NO_LOSS: f64 = 9_007_199_254_740_991.0;
const MAX_FRACTION_NB: i64 = 10_000;

fn float_to_irreducible_fraction(mut num: f64, mut den: f64) -> String {
    println!("num: {}", num);
    println!("den: {}", den);
    num *= 1000000.0;
    den *= 1000000.0;

    if (num - num.round()).abs() > 0.0 || (den - den.round()).abs() > 0.0 {
        let solution = num / den;
        println!("too precise: num: {}", (num - num.round()).abs());
        println!("too precise: den: {}", (den - den.round()).abs());
        return solution.to_string();
    }
    if num.abs() >= MAX_FLOAT_NO_LOSS || den.abs() >= MAX_FLOAT_NO_LOSS {
        let solution = num / den;
        println!("too big");
        return solution.to_string();
    }

    // Convert to integers
    let mut numerator: i64 = num.round() as i64;
    let mut denominator: i64 = den.round() as i64;

    // Find the greatest common divisor (GCD)
    let gcd_val = gcd(numerator.abs(), denominator.abs());

    // Reduce the fraction
    numerator /= gcd_val;
    denominator /= gcd_val;

    if numerator.abs() >= MAX_FRACTION_NB || denominator.abs() >= MAX_FRACTION_NB {
        let solution = num / den;
        return solution.to_string();
    }

    if denominator < 0 {
        numerator *= -1;
        denominator = denominator.abs();
    }

    return format!("{}/{}", numerator, denominator)
}

fn main() {
    // let num = 1.0/7.0;
    let str = float_to_irreducible_fraction(1.1, -7.0); // Should print -3/4
    println!("{str}");
}


/*
    split the given string with delimiter + and -, and keeps them at the beginning of the string
*/
// fn split_with_signs(input: &str) -> Vec<String> {
//     let mut result = Vec::new();
//     let mut current = String::new();
    
//     for ch in input.chars() {
//         if ch == '+' || ch == '-' {
//             result.push(current);
//             current = ch.to_string();
//         } else {
//             current.push(ch);
//         }
//     }
//     result.push(current);
    
//     return result;
// }
// fn main() {
//     let input = "4+5x+8x^2-1++";
//     let result = split_with_signs(input);
//     for part in result {
//         println!("{}", part);
//     }
// }