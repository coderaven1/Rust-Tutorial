/**
 * An operator defines some function that will be performed on the data. 
 * The data on which operators work are called operands. 
 * 
 * The major operators in Rust can be classified as −
 * @ Arithmetic
 * @ Bitwise
 * @ Comparison
 * @ Logical
 * @ Bitwise
 * @ Conditional
 */

 /**
  * Arithmetic Operators.
  NOTE : The ++ and -- operators are not supported in Rust.
  */
fn arithmetic_operators() {
    let a = 5;
    let b = 10;

    // Addition Operator (+).
    // returns the sum of the operands
    let addition = a + b;

    // Subtraction Operator (-).
    // returns the difference of the values
    let subtraction = a - b;

    // Multiplication Operator (*).
    // returns the product of the values
    let multiplication = a * b;

    // Division Operator (/).
    // performs division operation and returns the quotient
    let division = a / b;

    // Modulus Operator (%).
    // performs division operation and returns the remainder
    let modulus = a % b;

    // Print the values of the operators.
    println!("Valu of Addition Operation : {}", addition);
    println!("Valu of Subtraction Operation : {}", subtraction);
    println!("Valu of Multiplication Operation : {}", multiplication);
    println!("Valu of Division Operation : {}", division);
    println!("Valu of Modulus Operation : {}", modulus);

}


/**
 * Relational Operators.
 * Relational Operators test or define the kind of relationship between two entities. 
 * Relational operators are used to compare two or more values. 
 * Relational operators return a Boolean value − true or false.
 */
fn relational_operators() {
    let a = 10;
    let b = 20;

    // Greater than (>).
    // return true if the operands value is greater otherewise return false.
    let greater_than = a > b;

    // Lesser than (<).
    // return true if the operands value is lesser otherewise return false.
    let lesser_than = a < b;

    // Greater than or equal to (>=)
    // return true if the operands value is greater or equale otherewise return false.
    let greater_than_or_equal_to = a >= b;

    // Lesser than or equal to (<=)
    // return true if the operands value is lesser or equal otherewise return false.
    let lesser_than_or_equal_to = a <= b;

    // Equality (==)
    // return true if the operands value is equal otherewise return false.
    let equal = a == b;

    // Not equal (!=)
    // return true if the operands value is not equal otherewise return false.
    let not_equal = a != b;

    // Print the values of the operators.
    println!("A is = 10 and B is = 20");
    println!("A > B : {}", greater_than);
    println!("A < B : {}", lesser_than);
    println!("A >= B : {}", greater_than_or_equal_to);
    println!("A <= B : {}", lesser_than_or_equal_to);
    println!("A == B : {}", equal);
    println!("A != B : {}", not_equal);
    
}

/**
 * Logical Operators.
 * Logical Operators are used to combine two or more conditions. 
 * Logical operators also, return a Boolean value
 */
fn logical_operators() {
    let a = 10;
    let b = 20;
    let c = 20;
    let d = true;

    // And Operator (&&).
    // The operator returns true only if all the expressions specified return true
    let and = a == b && c == b;

    // OR Operator (||).
    // The operator returns true if at least one of the expressions specified return true
    let or = a == b || c == b;

    // NOT (!).
    // The operator returns the inverse of the expression’s result. 
    let not = !d;

    // Print the values of the operators.
    println!("If A is equal to B and C is equal to B : {}", and);
    println!("If A is equal to B or C is equal to B : {}", or);
    println!("Value of variable D : {}", not);

}

/**
 * Bitwise Operators.
 * bitwise operation operates on a bit string, a bit array or a binary numeral at the level of its individual bits
 */
fn bitwise_perators() {
    let a = 2;
    let b = 3;

    // Bitwise AND (&)
    // It performs a Boolean AND operation on each bit of its integer arguments.
    let bitwise_and = a & b;

    // Bitwise OR (|)
    // It performs a Boolean OR operation on each bit of its integer arguments.
    let bitwise_or = a | b;

    // Bitwise XOR (^)
    // It performs a Boolean exclusive OR operation on each bit of its integer arguments. Exclusive OR means that either operand one is true or operand two is true, but not both.
    let bitwise_xor = a ^ b;

    // Bitwise Not (!)
    // It is a unary operator and operates by reversing all the bits in the operand.
    let bitwise_not = !b;

    // Left Shift (<<).
    // It moves all the bits in its first operand to the left by the number of places specified in the second operand. New bits are filled with zeros. Shifting a value left by one position is equivalent to multiplying it by 2, shifting two positions is equivalent to multiplying by 4, and so on.
    let bitwise_left_shift = a << b;

    // Right Shift (>>).
    // Binary Right Shift Operator. The left operand’s value is moved right by the number of bits specified by the right operand.
    let bitwise_right_shift = a >> b;

    // Right Shift with Zero (>>>).
    // This operator is just like the >> operator, except that the bits shifted to the left are always zero.
    // let bitwise_right_shift_with_zero = a >>> b;

    // Print the values of the operators.
    println!("(a & b) => {} ",bitwise_and);
    println!("(a | b) => {} ",bitwise_or) ;
    println!("(a ^ b) => {} ",bitwise_xor);
    println!("(!b) => {} ",bitwise_not);
    println!("(a << b) => {}",bitwise_left_shift);
    println!("(a >> b) => {}",bitwise_right_shift);

}


// Main Function.
fn main() {
    // Arithmetic Operators.
    arithmetic_operators();

    // Relational Operators.
    relational_operators();

    // Logical Operators.
    logical_operators();

    // Bitwise Operators.
    bitwise_perators();
}
