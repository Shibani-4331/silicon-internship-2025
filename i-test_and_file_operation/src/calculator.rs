// use crate::helper;
// use super::helper;

pub fn simple_add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_with_greeting(a: i32, b: i32, name: &str) -> String {
    let sum = simple_add(a, b);
    let greeting = crate::helper::greet(name);
    format!("{} The sum of {} and {} is: {}", greeting, a, b, sum)
}