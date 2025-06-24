fn main() {
    // println!("Hello, world!");
    // compare_i8(5, 10);
    // compare_f64(5.0, 3.79);
    // compare(5, 10);
    // compare_hashmaps();
    // print_anything();
    test_point();
}

// Generics allow you to write code that can work with multiple types.
// You don't need to duplicate the code for a different type.

// fn compare_i8(a: i8, b: i8) {
//     let max = if a > b {
//         a
//     } else {
//         b
//     };
//     println!("Max: {}", max);
// }

// fn compare_f64(a: f64, b: f64) {
//     let max = if a > b {
//         a
//     } else {
//         b
//     };
//     println!("Max: {}", max);
// }

// struct Student {
//     age: i8,
//     name: String
// }

// s1 = Student {....}
// s2 = Student {....}

// compare(s1, s2);

// We are allowing comparison over all possible types in Rust(system and user defined).
// fn compare<T>(a: T, b: T) {
//     let max = if a > b {
//         a
//     } else {
//         b
//     };
//     println!("Max: {}", max);
// }

// use std::collections::HashMap;
// fn compare_hashmaps() {
//     let hm_one: HashMap<String, i32> = HashMap::new(); 
//     let hm_two: HashMap<String, i32> = HashMap::new(); 

//     println!("{}", hm_one > hm_two);
// }

// We restrict using trait bounds
// fn print_anything<T>(item_to_be_printed: T) {
//     println!("{:?}", item_to_be_printed);
// }

// Generic Structs
// struct IntPoint {
//     x: i32,
//     y: i32
// }

// struct FloatPoint {
//     x: f64,
//     y: f64
// }

// struct FancyPoint {
//     x: i32,
//     y: f64
// }

// struct ReverseFancyPoint {
//     x: f64,
//     y: i32
// }

// struct Point<T> {
//     x: T,
//     y: T
// }
use std::marker::PhantomData;
struct Point<T, U, Y> {
    x: T,
    y: U,
    _phantom: PhantomData<Y>
}

// enum Shape<R, S, W, H> {
//     Circle(R), // radius
//     Square(S), // side length
//     Rectangle(W, H), // width, height
// }

fn test_point() {
    let point = Point::<i32, f64, String> {
        x: 20,
        y: 56.34,
        _phantom: PhantomData
    };

    println!("({}, {})", point.x, point.y);
}