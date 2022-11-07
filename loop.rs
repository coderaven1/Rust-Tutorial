/**
 * Loop
 * A loop statement allows us to execute a statement or group of statements multiple times.
 * Rust provides different types of loops to handle looping requirements −
 * - while
 * - loop
 * - for
 */

fn main() {

    // Whle Loop.

    // Rust Loop.

    println!("-------------------- For Loop --------------------");

    // For Loop.
    // The for loop executes the code block for a specified number of times. 
    // It can be used to iterate over a fixed set of values, such as an array. 
    for i in 1..10 {
        println!("{}", i);
    }

    println!("-------------------- For Loop With Continue --------------------");

    // For loop with continue statement.
    // The continue statement skips the subsequent statements in the current iteration and takes the control back to the beginning of the loop. 
    for x in 1..11 {
        if x == 5 { 
            continue;
        }
        println!("{}", x);
    }

    println!("-------------------- While Loop --------------------");
    
    let mut num = 0;
    while num < 10 {
        println!("{}", num);
        num = num + 1;
    }

    println!("-------------------- While Loop --------------------");

    let mut num_2 = 1;
    while num_2 <= 20 {
        println!("{}", num_2);
        num_2 += 1;
    }
    
    println!("-------------------- Rust Loop --------------------");

    // Rust Loop.
    // The break statement is used to take the control out of a construct. Using break in a loop causes the program to exit the loop.
    let mut count = 0;
    loop {
        println!("{}", count);
        count += 1;

        if count == 15 {
            break;
        }
    }

}