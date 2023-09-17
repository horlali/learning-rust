fn main() {
    println!("Hello, world!");

    // Variables are immutable
    let x = 1;
    println!("The value of x is: {}", x);
    // x = 2;
    // println!("The value of x is: {}", x);

    // Variables are mutable if you add the `mut`
    let mut x = 1;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);

    // Variables can be shadowed (ie. reassigning them) [use let again to shadow]
    let y = true;
    println!("The value of y is: {}", y);
    let y = false;
    println!("The value of y is: {}", y);

    // Variables can be constants use `const`
    const STRING: &str = "hello";
    println!("The value of the string constant is: {}", STRING)
}

// signed integers mean a number can either be positive or negative
// they have a plus or minus sign

// unsigned numbers are always positive
