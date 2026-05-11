fn main() {
    if_function();
    loop_function();
    while_function();
    for_function();
}

fn if_function() {    
    // Create an immutable variable named number with the type i8.
    let number:i8 = 75;

    // Check if number is smaller than 50.
    if number < 50 {
      // This runs if the condition is true.
        println!("condition was true");
    } else {
        // This runs if the condition is false.
        println!("condition was false");
    }


  // Check the score from highest condition to lowest condition.
    if score >= 90 {
        // This runs if score is 90 or higher.
        println!("Excellent");

    } else if score >= 70 {
        // This runs if score is 70 to 89.
        println!("Good");

    } else if score >= 55 {
        // This runs if score is 55 to 69.
        println!("Passed");

    } else {
        // This runs if score is lower than 55.
        println!("Failed");
    }

    // Create a boolean variable.
    let condition: bool = true;

    // In Rust, if can be used as an expression that returns a value.
    // If condition is true, number gets 5. Otherwise, number gets 6.
    // This also shadows the previous number variable.
    let number: i8 = if condition { 5 } else { 6 };

    // Print the final value of number.
    println!("The value of number is: {number}");
}

fn loop_function() {
    // Create a mutable counter variable and start it at 0. 
    let mut counter: i8 = 0;

   // A loop can return a value when we use break with a value.
    let result = loop {
        // Increase counter by 1 on each loop iteration.
        counter += 1;

        // Check if counter reached 10.
        if counter == 10 {
            // Stop the loop and return counter * 2 as the loop result.
            break counter * 2;
        }
    };

    // Print the value returned from the loop.
    println!("The result is {result}");
}

fn while_function() {
    // Create a mutable variable named number and start it at 3.
    let mut number: i8 = 3;

    // Keep running the loop as long as number is not 0.
    while number != 0 {
        // Print the current value of number.
        println!("{number}!");

        // Decrease number by 1 after each loop iteration.
        number -= 1;
    }

    // This runs after the while loop ends.
    println!("LIFTOFF!!!");
}

fn for_function() {
    // Create an array with 5 numbers.
    let a: [i8; 5] = [10, 20, 30, 40, 50];

    // Go over each element in the array.
    for element in a {
        // Print the current element.
        println!("the value is: {element}");
    }

    // Create a range from 1 to 3.
    // 1..4 means: 1, 2, 3. The number 4 is not included.
    for number in 1..4 {
        // Print the current number.
        println!("{number}!");
    }

    // This runs after the countdown loop ends.
    println!("LIFTOFF!!!");
}