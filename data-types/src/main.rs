fn main() {
    /* integers ================================================================== */

    // signed
    let x: i8 = -10;
    // unsigned
    let y: u16 = 24;

    /* decimals and hexes */
    // all of these print out `255`
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;
    println!("{}, {}, {}, {}", decimal, hex, octal, binary);

    /* byte */
    let byte = b'A'; // ASCII value of letter A
    println!("{}", byte);

    /* floating points ================================================================== */
    // `f64` default
    let x = 2.0;
    let y: f32 = 1.5;

    /* boolean ================================================================== */
    let t = true;
    let f: bool = false;

    /* char ================================================================== */
    let c: char = 'c';

    /* char ================================================================== */
    // arithmetic operations: +. -, /, *, %

    // raise to power
    println!("{}", (10_i32).pow(2))
}