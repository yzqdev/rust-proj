/// String utilities
pub mod str_util {
    /// Reverse a string
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// Count words in a string
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    /// Check if string is palindrome
    pub fn is_palindrome(s: &str) -> bool {
        let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        let lower = cleaned.to_lowercase();
        lower == reverse(&lower)
    }
}

/// Math utilities
pub mod math_util {
    /// Calculate factorial
    pub fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }

    /// Check if number is prime
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let limit = (n as f64).sqrt() as u64;
        for i in 2..=limit {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// Calculate fibonacci
    pub fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let c = a + b;
                    a = b;
                    b = c;
                }
                b
            }
        }
    }
}

/// Random utilities
pub mod rand_util {
    use rand::Rng;

    /// Generate a random string of given length
    pub fn random_string(len: usize) -> String {
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            .chars()
            .collect();
        let mut rng = rand::thread_rng();
        (0..len).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
    }

    /// Generate a random number in range [min, max]
    pub fn random_in_range(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }
}
