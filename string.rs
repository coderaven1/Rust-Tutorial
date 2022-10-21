/**
 * String Literal
 * Also known as String Slices
 * (&str)
 * String literals are used when the value of a string is known at compile time. 
 * String literals are hardcoded into a variable.
 */
fn str_literal() {

    let name: &str = "John Doe";
    let mail: &str = "jhondoe@jd.jo";

    
    // String literals are static by default. 
    // This means that string literals are guaranteed to be valid for the duration of the entire program. 
    // We can also explicitly specify the variable as static
   //  println!("Name : {} Mail : {}", name, mail);
    println!("Name : {}", name);
    println!("Mail : {}", mail);

    
    // We can also explicitly specify the variable as static by using &'static str
    // (&'static str)
     let address:&'static str = "New York";

     println!("Address : {}", address);
}

/**
 * String Object
 * The String object type is provided in Standard Library.
 * Unlike string literal, the string object type is not a part of the core language. 
 * String is a growable collection. It is mutable and UTF-8 encoded type.
 * String object is allocated in the heap.
 */
fn str_object() {

    // To create a String object, we can use any of the following 
    // String::new()
    // String::from()

    // String::new() create an empty string.
    let blood_type = String::new();

    // String::from() creates a string with some default value passed as parameter.
    let company = String::from("DarK MatteR");

    // printing the value of both the company and blood_type.
    println!("{}", blood_type.len());
    println!("{}", company);

    // We also create a mutable String object and psuh the value later.
    let mut designation = String::new();

    // push_str push value to a String object
    designation.push_str("Senior Engineer");

    // Print the value of designation.
    println!("{}", designation);

}

/**
 * String Function.
 * 
 */
fn str_func() {
 
    // new()
    // Create an empty string object.
    let new = String::new();
    println!("Value is Empyt : {}", new); // Print the value of new.

    // to_string()
    // Convert a string literal to object type.
    let to_str = "Hallo World!";
    println!("{}", to_str.to_string()); 

    // replace()
    // Replace word in string.
    // The replace() function takes two parameters.
    // the first parameter is a string pattern to search for and the second parameter is the new value to be replaced.
    let replace = "Hallo World!";
    let replace = replace.replace("Hallo", "Oh, Hi");
    println!("{}", replace); 

    // as_str()
    // Extracts a string slice containing the entire string.
    let as_str = String::from("Example String");
    println!("{}", as_str.as_str());

    // push()
    // Appends the given char to the end of this String.
    // Only apply to string object.
    let mut push = "Rust Tutorial".to_string();
    push.push('s');
    println!("{}", push);

    // let mut test = String::new();
    // test.push('X');
    // println!("{}", test);

    // push_str()
    // Appends a given string slice onto the end of a String.
    // Only apply to string object.
    let mut push_str = "Rust Tutorial".to_string();
    push_str.push_str(", Python Tutorial");
    println!("{}", push_str);


    // len()
    // Returns the total number of characters in a string (including spaces).
    let len = "I'm learning Rust";
    println!("{}", len.len());

    // trim()
    // Removes leading and trailing spaces in a string. 
    // NOTE : that this function will not remove the inline spaces.
    let trim = " Let's test trim function \r\n";
    println!("Before trim : {}", trim);
    println!("length is : {}",trim.len());
    println!("After trim : {}", trim.trim());
    println!("length is after trim : {}",trim.trim().len());


    // split_whitespace()
    // Splits the input string into different strings.
    let split_whitespace = "Rust is preatty awesome!".to_string();

    // Loop through ther string.
    let mut i = 1;

    // for loop.
    for token in split_whitespace.split_whitespace() {
        println!("Token : {} {}", i, token);
        i += 1;
    }
    
    // split()
    // Returns an iterator over substrings of a string slice, separated by characters matched by a pattern.
    let split = "PHP, Python, C++, Javascript, Rust, Ruby";

    // for loop.
    for name in split.split(",") {
        println!("Skill : {}", name);
    }

    // chars()
    // Individual characters in a string can be accessed using the chars method. 
    let chars = "HELLO RUST!".to_string();

    // For loop.
    for char in chars.chars() {
        println!("{}", char);
    }

}


/**
 * Main function
 * Mamin function run all ther other function. 
 */
fn main() {

    // Example of String Literal.
    str_literal();

    // Example of String Object.
    str_object();

    // Example of String Function.
    str_func();
}
