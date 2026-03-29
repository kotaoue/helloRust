fn main() {
    println!("Hello, world!");
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd_with_odd_number() {
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(is_odd(-5));
    }

    #[test]
    fn test_is_odd_with_even_number() {
        assert!(!is_odd(0));
        assert!(!is_odd(2));
        assert!(!is_odd(-4));
    }

    #[test]
    fn test_is_even_with_even_number() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(-4));
    }

    #[test]
    fn test_is_even_with_odd_number() {
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(-5));
    }
}
