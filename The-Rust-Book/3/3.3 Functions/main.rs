fn main() {
    // main is the first function that runs.
    println!("Hello, world!");

    
    // Calls function_one.
    function_one();

    // Calls function_two and sends the value 5.
    function_two(5);

    // Calls function_three, but ignores the returned value.
    function_three();

    // Calls function_four.
    function_four();
}

fn function_one() {    
    // Prints a message to the console.
    println!("Another function.");
}

fn function_two(x: i8) {
    // x is a parameter of type i8.
    println!("The value of x is: {}", x);
} 

fn function_three() -> i8 {
    // -> i8 means this function returns an i8 value.

    // Returns the number 5 as i8.
    5
}

fn function_four() {
     // Stores the returned value from function_three in x.
    let x = function_three();

    // Prints the value of x.
    println!("The value of x is: {}", x);
}