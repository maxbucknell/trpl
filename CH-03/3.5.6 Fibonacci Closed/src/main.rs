const PHI: f64 = 1.618033988749894;
const SQRT_5: f64 = 2.2360679775;

pub fn fibonacci(n: u64) -> u64 {
    (PHI.powi((n as i32) + 1) / SQRT_5).round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(1), 1);

        assert_eq!(fibonacci(10), 89);
    }

    #[test]
    fn time_fibonacci() {
        let start = Instant::now();

        fibonacci(49);

        let elapsed = start.elapsed();

        println!("Computing 50th Fibonacci term took {} seconds.", elapsed.as_secs());
    }
}
