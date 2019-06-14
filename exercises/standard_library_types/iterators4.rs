// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1,|f,i|{f*i})
}

// Oops! I thought it said fibonacci...
// I thought every programming exercise book HAD to include the fibonacci sequence!
pub fn fibonacci(num: u64) -> u64 {
    (0..num).fold((0,1),|(a,b),_|{(b,a+b)}).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn fibonacci_up_10() {
        assert_eq!(
            "[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]",
            format!("{:?}", (0..10).map(|n| fibonacci(n)).collect::<Vec<u64>>()));
    }
}

























// In an imperative language you might write a for loop to iterate through
// multiply the values into a mutable variable. Or you might write code more
// functionally with recursion and a match clause. But you can also use ranges
// and iterators to solve this in rust.
