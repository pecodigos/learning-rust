fn main() {
    /*  You can't do: let guess: u32 = "42".parse().expect("Not a number!");
        If you try to, for example, sign to an u8 a value of 256, it will give a integer overflow. It's possible to --release, but it's considered an error/bad practice.
        Sometimes it's good to be explicit with what actually type you want (for example, u8 or i8, u16 or i16) although it's not necessary


    // Integer types

    let int: i8 = 127; // can be negative, but can store almost half o 'u' positive range
    let int: u8 = 255; // from 0 to 255, but can only store positive
    // and the list goes on

    // Floats

    let x = 2.0; // f64 by default if you don't be explicit

    let y: f32 = 3.0;

    // Numeric Operations same as every language (i think)

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; 

    // Boolean too, same old stuff
    let t = true;

    let f: bool = false; // with explicit type annotation

  Compound types (can group multiple values into one type)
        Rust has two primitive coumpound types: Tuples and Arrays
    */
    // The Tuple Type (can be a group of different types)

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{five_hundred}, {six_point_four} and {one}");

    /* The Array Type (can only be of one type)
    You can declare              -> let months = ["January", "February", "March"];
    But also you can be explicit -> let a: ["i32; 5"] = [1, 2, 3, 4, 5];
    Also, you can declare the same value for a number of times, like  -> let fives [3; 5]; (this is like [3, 3, 3, 3, 3])
    To access, it's pretty simple and __default__
    */ 

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let third = a[2]; // and so on

    println!("First number in the array is {first} and third is {third}!");







}
