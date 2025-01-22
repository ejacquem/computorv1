
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const EPSILON: f64 = 0.0000001;
const MAX_FLOAT_NO_LOSS: f64 = 9_007_199_254_740_991.0;
const MAX_FRACTION_NB: i64 = 10_000;

pub fn get_irreducible_fraction(mut num: f64, mut den: f64) -> String {
    // println!("num: {}", num);
    // println!("den: {}", den);

    if den == 0.0 {
        return "inf".to_string();
    }
    if num == 0.0 {
        return "0".to_string();
    }

    num *= 1000000.0;
    den *= 1000000.0;

    if (num - num.round()).abs() > EPSILON || (den - den.round()).abs() > EPSILON {
        let solution = num / den;
        // println!("too precise: num: {}", (num - num.round()).abs());
        // println!("too precise: den: {}", (den - den.round()).abs());
        return solution.to_string();
    }
    if num.abs() >= MAX_FLOAT_NO_LOSS || den.abs() >= MAX_FLOAT_NO_LOSS {
        let solution = num / den;
        // println!("too big");
        return solution.to_string();
    }

    let mut numerator: i64 = num.round() as i64;
    let mut denominator: i64 = den.round() as i64;

    let gcd_val = gcd(numerator.abs(), denominator.abs());

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

    if denominator == 1{
        return format!("{}", numerator);
    }

    return format!("{}/{}", numerator, denominator);
}