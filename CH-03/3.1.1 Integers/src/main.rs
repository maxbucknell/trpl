
const SIZE: usize = 8;
type Int = [bool; SIZE];
type NativeInt = i8;

pub fn to(input: NativeInt) -> Int {
    let mut result: Int = [false; 8];
    let mut counter = input;

    for index in (0..SIZE).rev() {
        let value: NativeInt = 1 << index;

        if index == SIZE - 1 {
            // We are in sign bit
            if counter < 0 {
                counter -= value;
                result[index] = true
            }
        } else {
            if counter >= value {
                counter -= value;
                result[index] = true;
            }
        }
    }

    if counter > 0 {
        panic!("Failed to convert {input}, had {counter} left over.")
    }

    result
}

pub fn from(input: Int) -> NativeInt {
    let mut result: NativeInt = 0;

    for index in 0..SIZE {
        if input[index] {
            result += 1 << index;
        }
    }

    result
}

pub fn eq(a: Int, b: Int) -> bool {
    for index in 0..SIZE {
        if a[index] != b[index] {
            return false;
        }
    }

    true
}



pub fn add(a: Int, b: Int) -> Int {
    // Declare the result to be 0 at first
    let mut result: Int = [false; 8];

    for index in 0..SIZE {
        let a = a[index];
        let b = b[index];
        let c = result[index];

        let mut d: usize = 0;

        // There are three possibilities when calculating the sum of
        // bits a + b:
        //  - 0: Both bits 0,
        //  - 1: One bit 1, one 0,
        //  - 2: Both bits 1.
        //
        // If both bits are 1, then the binary representation is 0b10,
        // meaning that we have to carry a bit over to the next loop.
        // The carried bit will be stored at the current position in
        // the result. This is safe because we haven't set that value
        // yet, that's what this loop iteration is for!
        //
        // This gives us a fourth possibility:
        //  - 3: Both bits 1, and something carried from the previous
        //    bit position.
        //
        // We can work out which of these scenarios we are in by just
        // counting how many of a, b, and c (our carried bit) are true.
        for x in [a, b, c] {
            if x {
                d += 1;
            }
        }

        // If the value is greater than 1, we need to carry the 1 over
        // to the next bit. We do a basic check here to see that we do
        // not overflow. In the case that an addition would take us out
        // of bounds, we simply ignore it.
        if d > 1 && (index + 1) < SIZE {
            result[index + 1] = true;
        }

        // Odd is 1, even is 0
        result[index] = d % 2 == 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to() {
        assert_eq!(to(0), [false; 8]);
        assert_eq!(to(1), [true, false, false, false, false, false, false, false]);
        assert_eq!(to(127), [true, true, true, true, true, true, true, false]);
        assert_eq!(to(75), [true, true, false, true, false, false, true, false]);
    }

    #[test]
    fn test_from() {
        assert_eq!(0, from([false; 8]));
        assert_eq!(1, from([true, false, false, false, false, false, false, false]));
        assert_eq!(127, from([true, true, true, true, true, true, true, false]));
        assert_eq!(75, from([true, true, false, true, false, false, true, false]));
    }

    #[test]
    fn test_eq() {
        assert!(eq(to(11), to(11)));
        assert!(!eq(to(124), to(17)));
    }

    #[test]
    fn test_add() {
        assert_eq!(from(add(to(0), to(0))), 0);
        assert_eq!(from(add(to(1), to(1))), 2);
        assert_eq!(from(add(to(1), to(2))), 3);
        assert_eq!(from(add(to(31), to(32))), 63);
    }

    #[test]
    fn test_to_signed() {
        assert_eq!(to(-1), [true; SIZE]);
        assert_eq!(to(-128), [false, false, false, false, false, false, false, true]);
        assert_eq!(to(-19), [true, false, true, true, false, true, true, true]);
    }

    #[test]
    fn test_from_signed() {
        assert_eq!(-1, from([true; SIZE]));
        assert_eq!(-128, from([false, false, false, false, false, false, false, true]));
        assert_eq!(-19, from([true, false, true, true, false, true, true, true]));
    }

    #[test]
    fn test_add_signed() {
        assert_eq!(from(add(to(127), to(-127))), 0);
        assert_eq!(from(add(to(75), to(-19))), 56);
    }
}
