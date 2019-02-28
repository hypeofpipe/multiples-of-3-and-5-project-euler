/*
###THE PROBLEM###
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

Example: divisors [3, 5] maximum nuber below 10 (9)

1. Divide max number by divisors => 9 / 3 = 3; 9 / 5 = 1.8 (omit the rest, left the whole number) = 1
2. For each that sum, create an array from 1 .. maxnum/divisor [[1, 2, 3], [1]] and for each number in that array
 multiply it by the divisor [[1 * 3, 2 * 3, 3 * 3], [1 * 5]]
3. Add the result [[3 + 6 + 9] + [5]] => [18 + 5] => 23
*/

fn calculate_the_sum(maximum_number: i32) -> i32 {
    let divisors: Vec<i32> = vec![3, 5];

    let result: i32 = divisors
        .into_iter()
        .map(|x| {
            (0i32..(maximum_number / x))
                .map(|y| y + 1)
                .map(|y| y * x)
                .fold(0, |acc, y| acc + y)
        })
        .fold(0, |acc, x| acc + x);

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(calculate_the_sum(9), 23)
    }
}

fn main() {
    let sum = calculate_the_sum(9);
    print!("{}", sum)
}
