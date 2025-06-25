mod calculator;
mod helper;
mod tests;

fn main() {
    println!("Hello, world!");
    let result = calculator::simple_add(5, 10);
    println!("The result of the addition is: {}", result);

    let message = calculator::add_with_greeting(5, 10, "Abhilash");
    println!("{}", message);
}


// Testing 
// Rust h does not use any framework(outside framework)
// Test stays alongside your code
// #[test] - this converts a normal function into a test function


// types of tests
// unit tests - test individual funtions
// integration test - test modules working together
// doc test - test code in documentation comments

// unit tests

// macros
// assert!(true, "This will always pass");
// assert_eq!(a, b)
// assert_ne!(a, b)
// panic!("This will always panic")

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}


// #[cfg(test)]
// mod tests {
//     use super::*;


//     #[test]
//     fn test_add() {
//         let result = add(2,3);
//         // result is original value
//         // 5 is expected value
//         // 3rd is fallback message
//         assert_eq!(result, 5);
//         // assert_eq!(result, 5, "Result must be 5");
//     }

//     #[test]
//     fn test_even() {
//         let result = is_even(4);
//         // assert!(result);
//         // assert!(result, "4 should be even");
//         // assert_eq!(result, true);
//         assert_ne!(result, false);
//     }
// }


// cargo test
// cargo test test_name
// cargo test --lib


// unit test, doc test, integration