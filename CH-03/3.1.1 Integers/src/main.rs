
const SIZE: usize = 8;

type NativeInt = u8;
type Int = [bool; SIZE];

pub fn to(input: NativeInt) -> Int {
    let mut result: Int = [false; 8];
    let mut counter = input;

    for index in (0..SIZE).rev() {
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

    for index in 0..SIZE {
        if input[index] {
            result += 1 << index;
        }
    }

    result
}

enum BitAddResult {
    A0,
    A1,
    A2,
    A3
}

 impl BitAddResult {
    fn from(a: bool, b: bool, carry: bool) -> Self {
        let mut counter = 0;

        if a {
            counter += 1;
        }

        if b {
            counter += 1;
        }

        if carry {
            counter += 1;
        }

        match counter {
            0 => Self::A0,
            1 => Self::A1,
            2 => Self::A2,
            3 => Self::A3,
            _ => panic!("Unexpected bit value with: {a}, {b}, {carry}.")
        }
    }
}

pub fn add(a: Int, b: Int) -> Int {
    let mut result: Int = [false; 8];

    for index in 0..SIZE {
        let a = a[index];
        let b = b[index];
        let c = result[index];

        let bit = BitAddResult::from(a, b, c);

        result[index] = match bit {
            BitAddResult::A0 => false,
            BitAddResult::A1 => true,
            BitAddResult::A2 => {
                if index < SIZE {
                    result[index + 1] = true;
                }

                false
            },
            BitAddResult::A3 => {
                if index < SIZE {
                    result[index + 1] = true;
                }

                true
            }
        };
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
    fn test_add() {
        assert_eq!(from(add(to(0), to(0))), 0);
        assert_eq!(from(add(to(1), to(1))), 2);
        assert_eq!(from(add(to(1), to(2))), 3);
        assert_eq!(from(add(to(31), to(32))), 63);
    }
}
