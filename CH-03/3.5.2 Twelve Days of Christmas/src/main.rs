const ORDINALS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const PRESENTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five gold rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids-a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,"
];

fn main() {
    for day in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me,", ORDINALS[day]);

        for inner_day in (0..(day + 1)).rev() {
            println!("{}", PRESENTS[inner_day])
        }

        println!("");
    }
}
