fn main() {
    /// ARRAYS
    // init_explicit();
    // init_repeated();
    // init_mutation();
    // accessing_elements();
    // get_length();
    // iterate_simple();
    // iterate_indices();
    // iterate_mutably();
    // array_slicing();
    // find_max();

    /// TUPLES
    // init_tuples();
    // access_elements();
    // destructure_tuple();
    // let ret = return_tuple();
    // println!("Returned Tuple: {:?}", ret);
    // println!("First element of returned tuple: {}", ret.0);
    // println!("Second element of returned tuple: {}", ret.1);
    // match_tuple();
    // tuple_example();

    // VECTORS
    // create_vector();
    // push_elements();
    // insert_elements();
    // delete_last_element();
    // delete_at_position();
    // length_of_vector();
    // iterating_over_vector();

    // HASHMAPS
    // create_empty_hashmap();
    // insert_into_hashmap();
    // get_from_hashmap();
    // contains_key_in_hashmap();
    // remove_from_hashmap();
    // entry_into_hashmap();
    // iterating_over_keys();
    // iterating_over_values();
    // iterating_over_key_value();
    fancy_map();
}

// ARRAY EXAMPLES
fn init_explicit() {
    let arr = [1, 2, 3, 4, 5];
    println!("ARRAY: {:?}", arr);
}

fn init_repeated() {
    let arr = [24; 20];
    println!("ARRAY: {:?}", arr);
}

fn init_mutation() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("ARRAY BEFORE: {:?}", arr);

    arr[2] = 10;

    println!("ARRAY AFTER: {:?}", arr);
}

fn accessing_elements() {
    let arr = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);
    println!("Second element: {}", arr[1]);
    println!("Third element: {}", arr[2]);
    println!("Fourth element: {}", arr[3]);
    println!("Fifth element: {}", arr[4]);
    // println!("Error! {}", arr[200]); // This will cause an error at compile time.
}

fn get_length() {
    let arr = ['1', '2', '3', 'ñ', '😎'];
    let length = arr.len();
    println!("Length of the array: {}", length);
}

fn iterate_simple() {
    // let arr = ['1', '2', '3', 'ñ', '😎'];
    // for element in arr {
    //     println!("Element: {}", element);
    // }

    // println!("Array[2] = {}", arr[2]);

    // for element in &arr {
    //     println!("Element (by reference): {}", element);
    // }

    let string_array: [String; 3] = [String::from("Hello"), String::from("World"), String::from("Rust")];
    for element in string_array {
        println!("String Element: {}", element);
    }

    // println!("String Array[2] = {}", string_array[2]); // will throw error

    // for element in &string_array {
    //     println!("String Element: {}", element);
    // }
}

fn iterate_indices() {
    let arr = ['1', '2', '3', 'ñ', '😎'];
    for i in 0..arr.len() {
        println!("Element at index {}: {}", i, arr[i]);
    }
}

fn iterate_mutably() {
    let mut arr = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        arr[i] = arr[i] * arr[i];
    }

    println!("Mutated Array: {:?}", arr);
}

fn array_slicing() {
    let arr = [1, 2, 3, 4, 5];
    let slc = &arr[1..4]; // Type -> [i32] -> &[i32]
    println!("Sliced Array: {:?}", slc);
}

fn find_max() {
    let arr = [1, 2, 3, 4, 5];
    let mut max = 0;
    for num in arr {
        if num > max {
            max = num;
        }
    }
    println!("Max value in the array: {}", max);
}

// TUPLE EXAMPLES
fn init_tuples() {
    let tup = (1, 2, 3, 4, 5); // -> (i32, i32, i32, i32, i32)
    println!("TUPLE: {:?}", tup);

    let tup = (1, 2.6, true, 'f'); // -> (i32, f64, bool, char)
    println!("TUPLE: {:?}", tup);

    let tup = ();
    println!("TUPLE: {:?}", tup);

    let tup = ((1, 't'), ([1, 1, 1], false, 3.92), [1]);
    println!("TUPLE: {:?}", tup);
}

fn access_elements() {
    let tup = (1, 20, 13);
    println!("First element: {}", tup.0);
    println!("Second element: {}", tup.1);
    // println!("Third element: {}", tup.12); // Homework
}

fn destructure_tuple() {
    let tup = (1, 20, 13);
    let (.., a) = (12, 34, 80, 12, 123, 14, 35, 6, 57, 68, 79);
    println!("Destructured elements: a = {}", a);
}

fn return_tuple() -> (i32, bool) {
    // Computation GO!
    (30, false)
}

fn match_tuple() {
    let point = (-2, 10);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On X-axis at {}", x),
        (0, y) => println!("On Y-axis at {}", y),
        (-2, 10) => println!("Special point at (-2, 10)"),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}

// fn tuple_example() {
    // let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    // let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Tuple: {:?}", tup);
// }

// Tuple - Type
// (1) -> Rules(Traits) Implemented [Debug - Print, Display - Print, Clone - Copying]
// (1, 2) -> Rules(Traits) ReImplemented

// [i32; N] - Rules(Traits) Implemented [Debug - Print, Display - Print, Clone - Copying]

// [1, 2]
// [3]
// [1, 2, 3]

// (T1) -> Implemented - (i32) - (String) - (char)
// (T1, T2) - 
// (T1, T2, T3)

// ('a', 'b', 'c')
// ('a', true)

// (A, B, C) - (i32, f64, bool) - (f64, char, String)
// (U, T) - (i32, String) - (String, f64)
// (H, L) - (f64, char) - (String, f64)

// VECTOR EXAMPLES
fn return_vec() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

fn create_vector() {
    let empty_vector: Vec<i32> = Vec::new();
    let empty_vector = Vec::<i32>::new(); // Turbo-fish operator
    let empty_vector: Vec<f64> = vec![];

    let vector_with_elements = vec![1, 2, 3, 4, 5];

    let vector_with_same_elements = vec![19; 10];

    println!("Empty Vector: {:?}", empty_vector);
    println!("Vector with elements: {:?}", vector_with_elements);
    println!("Vector with same elements: {:?}", vector_with_same_elements);
}

fn push_elements() {
    let mut empty_vector: Vec<i32> = vec![];

    println!("Empty Vector before pushing: {:?}", empty_vector);
    // 1 2 3 4 5
    empty_vector.push(1);
    empty_vector.push(2);
    empty_vector.push(3);
    empty_vector.push(4);
    empty_vector.push(5);
    empty_vector.push(100);

    println!("Empty Vector after pushing: {:?}", empty_vector);
}

fn insert_elements() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    let new_element = 28;
    let index = 4;

    vec.insert(index, new_element);
    // vec.insert(100, new_element); // Will panic at runtime
    println!("Vector after inserting {} at index {}: {:?}", new_element, index, vec);
}

fn delete_last_element() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Vector before popping: {:?}", vec);

    let popped_element = vec.pop();

    println!("Popped element: {:?}", popped_element);
    println!("Vector after popping: {:?}", vec);

    let mut vec: Vec<i32> = vec![];
    let popped_element = vec.pop();

    println!("Popped element from empty vector: {:?}", popped_element);
}

fn delete_at_position() {
    let mut vec = return_vec();
    println!("Vector before removing: {:?}", vec);

    let index_to_remove_at = 2;

    let removed_element = vec.remove(index_to_remove_at);
    println!("Vector after removing element at index {}: {:?}", index_to_remove_at, vec);

    println!("Removed element: {}", removed_element);
}

fn length_of_vector() {
    let vec = return_vec();
    println!("Vector: {:?}", vec);

    let length = vec.len();
    println!("Length of the vector: {}", length);
}

fn iterating_over_vector() {
    let vec = return_vec();
    println!("Vector: {:?}", vec);

    for element in &vec {
        println!("Element: {}", element);
    }

    let third = vec[2];
    println!("Third element: {}", third);
}

// HASHMAPS EXAMPLES
use std::collections::HashMap;

fn create_empty_hashmap() {
    // Name - Favorite IceCream Flavor
    let ice_cream_map: HashMap<String, String> = HashMap::new();
    println!("Empty HashMap: {:?}", ice_cream_map);
}

fn insert_into_hashmap() {
    // Name - Favorite IceCream Flavor
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    println!("Empty HashMap: {:?}", ice_cream_map);

    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    println!("HashMap: {:?}", ice_cream_map);
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    println!("HashMap: {:?}", ice_cream_map);
    ice_cream_map.insert("Sourav".to_owned(), String::from("Mint Chocolate"));
    println!("HashMap: {:?}", ice_cream_map);

    ice_cream_map.insert("Sourav".to_owned(), String::from("Mint Chocolate"));
    println!("HashMap: {:?}", ice_cream_map);
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Vanilla"));
    println!("HashMap: {:?}", ice_cream_map);
    ice_cream_map.insert("Ayush".to_string(), String::from("Butter Scotch"));
    println!("HashMap: {:?}", ice_cream_map);
}

fn get_from_hashmap() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_owned(), String::from("Mint Chocolate"));

    let res = ice_cream_map.get("Sourav");
    println!("Get Sourav's favorite ice cream flavor: {:?}", res);
}

fn contains_key_in_hashmap() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_owned(), String::from("Mint Chocolate"));
    
    let key_one = "Mahavir";
    let key_two = "Yuvraj";

    println!("Map contains key_one - {}: {}", key_one, ice_cream_map.contains_key(key_one));
    println!("Map contains key_two - {}: {}", key_two, ice_cream_map.contains_key(key_two));
}

fn remove_from_hashmap() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_owned(), String::from("Mint Chocolate"));
    
    let key_to_remove = "Yuvraj";
    println!("HashMap before removing {}: {:?}", key_to_remove, ice_cream_map);

    let removed_flavour = ice_cream_map.remove(key_to_remove);

    println!("HashMap after removing {}: {:?}", key_to_remove, ice_cream_map);
    println!("Removed flavour for {}: {:?}", key_to_remove, removed_flavour);
}

fn entry_into_hashmap() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_string(), String::from("Mint Chocolate"));

    let entry = ice_cream_map.entry("Mahavir".to_string()).or_insert(String::from("Vanilla"));
    // println!("HashMap after entry for Mahavir: {:?}", ice_cream_map);
    println!("Entry for Mahavir: {:?}", entry);
}

fn iterating_over_keys() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_string(), String::from("Mint Chocolate"));

    let _keys = ice_cream_map.keys();

    for key in ice_cream_map.keys() {
        println!("Key: {}", key);
    }
}

fn iterating_over_values() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_string(), String::from("Mint Chocolate"));

    let _values = ice_cream_map.values();

    for value in ice_cream_map.values() {
        println!("Value: {}", value);
    }
}

fn iterating_over_key_value() {
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Yuvraj".to_string(), String::from("Chocolate"));
    ice_cream_map.insert("Abhilash".to_string(), String::from("Belgian Chocolate"));
    ice_cream_map.insert("Sourav".to_string(), String::from("Mint Chocolate"));

    for (key, value) in &ice_cream_map {
        println!("Key: {}, Value: {}", key, value);
    }
}

fn fancy_map() {
    let mut fancy_map: HashMap<Vec<i32>, Vec<bool>> = HashMap::new();
    println!("Fancy Map: {:?}", fancy_map);

    let key_one = vec![1, 2, 3];
    let value_one = vec![true];

    fancy_map.insert(key_one, value_one);
    println!("Fancy Map: {:?}", fancy_map);
}
