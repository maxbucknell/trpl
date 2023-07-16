
const SIZE: usize = 8;

type NativeInt = u8;
type Int = [bool; SIZE];

pub fn to(input: NativeInt) -> Int {
    let mut result: Int = [false; 8];
    let mut counter = input;

    for index in (0..8).rev() {
        let value: NativeInt = 1 << index;

        if counter >= value {
            counter -= value;
            result[index] = true;
        }
    }

    if counter > 0 {
        panic!("Failed to convert {input}, had {counter} left over.")
    }

    result
}

pub fn from(input: Int) -> NativeInt {
    let mut result: NativeInt = 0;

    for index in 0..8 {
        if input[index] {
            result += 1 << index;
        }
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
}
