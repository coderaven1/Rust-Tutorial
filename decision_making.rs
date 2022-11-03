/**
 * Decision Making.
 * Conditional statement.
 */

 /**
  * Conditional statement
  * @ if, else, if else, nested if and if else statement.
  */
fn conditional_statement() {
    
    // let numb: i32 = -10;
    let numb: i32 = 70;

    // if statement.
    // An if statement consists of a Boolean expression followed by one or more statements.
    // If the Boolean expression evaluates to true, then the block of code inside the if statement will be executed.
    if numb > 0 {
        println!("Number: {} is positive number.", numb);
    }

    // let voting_age: i32 = 21;
    let voting_age: i32 = 16;

    //if...else statement.
    // An if statement can be followed by an optional else statement, which executes when the Boolean expression is false.
    // An if can be followed by an optional else block. The else block will execute if the Boolean expression tested by the if statement evaluates to false.
    if voting_age >= 18 {
        println!("You are allowed to vote.");
    } else {
        println!("Sorry! You can't vote!");
    }

    let user_name: &str = "jhondoe";
    // let password: &str = "abc";
    let password: &str = "123abc";

    // Checking multiple condition with if else statement.
    if user_name == "jhondoe" && password == "123abc" {
        println!("Welcome {}.", user_name);
    } else {
        println!("Oops! Looks like the username or the password you entered was incorrect!");
    }

    // let operating_system = "ChromeOS";
    // let operating_system = "MacOS";
    // let operating_system = "Linux";
    let operating_system: &str = "Windows";

    // else if statement.
    // You can use one if or else if statement inside another if or else if statement(s).
    if operating_system == "Windows" {
        println!("You are using {}.", operating_system);
    } else if operating_system == "Linux" {
        println!("You are using {}.", operating_system);
    } else if operating_system == "MacOS" {
        println!("You are using {}.", operating_system);
    } else {
        println!("You are using operating system is unknown.");
    }


    let university = "Stanford University";
    // let university = "UC Berkeley";
    let major = "CSE";
    // let major = "EEE";

    // Nested ifstatement.
    // It is possible to use one if or else if statement inside another if or else if statement(s).
    if university == "Stanford University" {
        if major == "CSE" {
            println!("You are studying at {} and your major is {}.", university, major);
        } else {
            println!("You are studying at {}.", university);
        }
    } else {
        println!("Sorry! University not found.");
    }

}

 /**
  * Conditional statement
  * @ match statement.
  */
  fn match_statement() {
    
    // let gpa: i8 = 90;
    // let gpa: i8 = 50;
    let gpa: i8 = 85;

    // The match statement checks if a current value is matching from a list of values.

    // double dot (..) is range operatot.
    // underscore (_) is the else part of the match.
    let result = match gpa {
        97..= 100 => "A+",
        93..= 96 => "A",
        90..= 92 => "A-",
        87..= 89 => "B+",
        83..= 86 => "B",
        80..= 82 => "B-",
        77..= 79 => "C+",
        73..= 76 => "C",
        70..= 72 => "C-",
        67..= 69 => "D+",
        65..= 66 => "D",
        _ => "F",
    };

    println!("Letter Grade : {}", result);

  }

fn main() {
    // if, else, if else, nested if and if else statement.
    conditional_statement();
    // match statement.
    match_statement();
}
