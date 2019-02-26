/*
###THE PROBLEM###
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.*/

fn calculate_the_sum(maximum_number: i32) -> i32 {
    let divisors = [3, 5];

    let sum = divisors
        .iter()
        .map(|x| [maximum_number / x, x])
        .map(|x| (1 .. x[0]).map(|y| y * x[1]).fold(0, |acc, y| acc + y))
        .fold(0, |acc, x| acc + x);
    println!("{}", sum);

    4
}

fn main() {
    calculate_the_sum(9);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(calculate_the_sum(9), 23)
    }
}
