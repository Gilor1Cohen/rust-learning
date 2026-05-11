fn main() {
    let x: u8  = 5;
    // x is immutable by default, so we cannot change its value. 

    let mut y: u8 = 10;
    // y is mutable, so we can change its value.
    
    x = 6; // This will cause a compile-time error because x is immutable.
    y = 15; // This is allowed because y is mutable.


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // const is a completely fixed value that Rust must know in advance while the program is being built, and it cannot change at all.
    // let is a regular variable: it is also immutable by default, but it can be made into a variable using mut, and it does not have to be constant “in advance” like const.


    // Shadowing:
    // Shadowing in Rust is a situation where you create a new variable with the same name as a previous variable, and it “hides” it from that point on.
    // Shadowing is needed even though mut exists, because mut only changes the value of the same variable and of the same type, but shadowing creates a new variable with the same name, which can have both a new value and a new type.

    //Example of shadowing:
    // Create a variable named x with the value 5
    let x: u8 = 5;
    // Shadowing: create a NEW variable named x
    // The new x uses the old x value and adds 1
    // So now x is 6
    let x: u8 = x + 1;

        {
        // Inner scope starts here

        // Shadowing again, but only inside this block
        // The inner x uses the outer x value, which is 6
        // 6 * 2 = 12
        let x: u8 = x * 2;

        // This prints the inner x, which is 12
        println!("The value of x in the inner scope is: {x}");

        // Inner scope ends here
        // The inner x disappears after this block
    }

    // This prints the outer x, which is still 6
    println!("The value of x is: {x}");
}