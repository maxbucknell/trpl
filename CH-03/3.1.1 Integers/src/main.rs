
const SIZE: usize = 8;

type NativeInt = u8;
type Int = [bool; SIZE];

pub fn to(input: NativeInt) -> Int {
    let mut result: Int = [false; 8];
    let mut counter = input;

    for index in 0..8 {
        let power = 7 - index;
        let value: NativeInt = 1 << power;

        if counter >= value {
            counter -= value;
            result[index] = true;
        }
    }

    assert_eq!(counter, 0);

    result
}

pub fn from(input: Int) -> NativeInt {
    let mut result: NativeInt = 0;

    for index in 0..8 {
        let power = 7 - index;
        if input[index] {
            result += 1 << power;
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
        assert_eq!(to(1), [false, false, false, false, false, false, false, true]);
        assert_eq!(to(127), [false, true, true, true, true, true, true, true]);
        assert_eq!(to(75), [false, true, false, false, true, false, true, true]);
    }


    #[test]
    fn test_from() {
        assert_eq!(0, from([false; 8]));
        assert_eq!(1, from([false, false, false, false, false, false, false, true]));
        assert_eq!(127, from([false, true, true, true, true, true, true, true]));
        assert_eq!(75, from([false, true, false, false, true, false, true, true]));
    }
}
