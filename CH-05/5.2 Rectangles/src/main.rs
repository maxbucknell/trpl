type Dimension = u64;

#[derive(Debug)]
pub struct Rectangle {
    width: Dimension,
    height: Dimension
}

impl Rectangle {
    pub fn new(width: Dimension, height: Dimension) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn square(width: Dimension) -> Self {
        Self {
            width,
            height: width
        }
    }

    pub fn area(&self) -> Dimension {
        self.width * self.height
    }

    pub fn contains(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let r = Rectangle::new(1, 1);

        assert_eq!(r.area(), 1);

        let r = Rectangle::new(2, 3);

        assert_eq!(r.area(), 6);

        let r = Rectangle::new(27, 12);

        assert_eq!(r.area(), 324);
    }

    #[test]
    fn test_formatting() {
        let r = Rectangle::new(3, 5);

        let result = format!("{:?}", r);

        assert_eq!(result, "Rectangle { width: 3, height: 5 }");
    }


    #[test]
    fn test_contains() {
        let r1 = Rectangle::square(12);
        let r2 = Rectangle::new(7, 9);
        let r3 = Rectangle::new(6, 19);

        assert!(r1.contains(&r1));
        assert!(r1.contains(&r2));
        assert!(!r1.contains(&r3));

        assert!(!r2.contains(&r1));
        assert!(!r2.contains(&r3));

        assert!(!r3.contains(&r1));
        assert!(!r3.contains(&r2));
    }
}
