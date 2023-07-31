const RATIO: f64 = 9.0 / 5.0;

pub fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / RATIO
}

pub fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * RATIO + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_celsius() {
        assert_eq!(to_celsius(212.0), 100.0);
        assert_eq!(to_celsius(32.0), 0.0);
        assert_eq!(to_celsius(-40.0), -40.0);
    }

    #[test]
    fn test_to_fahrenheit() {
        assert_eq!(to_fahrenheit(100.0), 212.0);
        assert_eq!(to_fahrenheit(30.0), 86.0);
        assert_eq!(to_fahrenheit(-40.0), -40.0);
    }
}
