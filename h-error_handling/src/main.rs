fn main() {
    // println!("Hello, world!");
    // simple_option_example();
    // create_option();
    // another_example();
    // option_methods();
    // option_methods_2();
    // result_example();
    test_operator();

}


// Option
// Result

// handles error in rust


// var phone_number = db.get_phone_number(user_id);
// phone_number.length()

// enum Option<T> {
//     Some(T), // I have a value
//     None //  I don't have a value
// }

// When to use option
// array: specific index 
// hashmap: specific key



fn simple_option_example(){

    let alice_phone: Option<String> = get_phone_number("Alice");
    let bob_phone = get_phone_number("Bob");

    println!("Alice's phone number: {:?}", alice_phone);
    println!("Bob's phone number: {:?}", bob_phone);


    // println!("Length of alice phonr number: {}", alice_phone.len());

// some(phone ) is matched with some("1234567890")
    // match alice_phone {
    //     Some(phone) => println!("Alice's phone number is {}", phone),
    //     None => println!("Alice's phone number not found"),
    // }

    // match bob_phone {
    //     Some(phone) => println!("Bob's phone number is {}", phone),
    //     None => println!("Bob's phone number not found"),
    // }
}

// db return the value, we are querying form db
// this is a simple function that represents db query
fn get_phone_number(name: &str) -> Option<String> {
    match name {
        "Alice" => Some("1234567890".to_string()),
        "Charlie" => Some("0987654321".to_string()),
        _ => None,
    }
}

fn create_option() {
    let has_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;


    println!("Has value: {:?}", has_value);
    // println!("No value: {:?}", no_value);

    // match has_value {
    //     Some(Value) => println!("Has value: {}", Value),
    //     None => println!("No value present"),
    // }

    // match no_value {
    //     Some(value) => println!("Has value: {}", value),
    //     None => println!("No value present"),
    // }


    // using if let
    // some(number) is matched with some(42)
    // let (x, y) = (1, 2);
    // if let Some(number) = has_value{
    //     println!("Has some value: {}", number);
    // }

    if let Some(number) = no_value {
        println!("Has some value: {}", number);
    } else {
        println!("No value present");
    }


}


fn another_example() {
    let numbers = vec![10, 20, 30, 40, 50];

    println!("numbers: {:?}", numbers);
    

    // let first_number = numbers[10]; 
    // println!("First number: {}", first_number);


    let first_number_again = numbers.get(2); 
    //get  returns an option type

    println!("First number again: {:?}", first_number_again);

    match first_number_again {
        Some(number) => println!("First number again: {}", number),
        None => println!("No value found at index 10"),
    }


    let text = "hello,world,rust,programming";
    let words: Vec<&str> = text.split(",").collect();
    println!("Words: {:?}", words);
    let first_word = words.get(0);
    let fifth_word = words.get(5);
    println!("First word: {:?}", first_word);
    println!("Fifth word: {:?}", fifth_word);
}

fn  option_methods(){
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;


    // is_some() and is_none()
    println!("Some_number exists?: {}", some_number.is_some());
    // option
    // Some or none
    // is_some returns true of the type is Option::Some()

    // println!("Some_number exists?: {}", some_number.is_none());

    println!("No_number exists?: {}", no_number.is_none());
    // option
    // Some or none
    // is_none return true if the type is Option::None
    // println!("No_number exists?: {}", no_number.is_some());

    //unwrap
    //unwrap_or()
}

fn option_methods_2() {
    let some_val = Some(42);
    let none_value: Option<i32> = None;

    // println!("Some value is: {}", some_val.unwrap());

    // if none_value.is_some(){
    //     println!("None is value: {}", none_value.unwrap());
    // }

    //  never use unwrap(), try to not use it

    // println!("Some value is: {}", some_val.unwrap_or(0));
    // println!("None value is: {}", none_value.unwrap_or(0));

    // some_val must be of type Option::Some(x), if it is Option::None, we can make it panic
    // with custom error message
    println!("Some value is: {}", some_val.expect("NO value or none value"));
    println!("None value is: {}", none_value.expect("NO value or none value"));

    // TODO: For students
    // unwrap_or_else()
    // unwrap_or_default()
}


// Result
// "42".parse::<i32>()
// "abc".parse::<i32>()  ParseIntError
// opening a file NotfoundError, PermissionError, NoMemory
// Api call ServerError, NOtfounderror, timeout, badrequest


// enum Result<T, E> {
//     Ok(T),  // Operation is success and return ok with type T( T can be any type)
//     Err(E)  // Operation is failed and return err with type E( E can be any type NotfoundError, ParseIntError, etc.)
// }


// difference with option

//Option: Either a value exists or none
//Result: Either an operation fails or successful


// "42".parse::<i32>() --- return Ok(42)
// "abc".parse::<i32>() --- return Err(ParseIntError)

fn result_example(){
    // let good_parse = "42".parse::<i32>();
    // let bad_parse = "abc".parse::<i32>();
    // parse function has a return type  Result<i32, ParseIntError> i32 is for type T
    // parseint error is not for type E

    // println!("Good parse result: {:?}", good_parse);
    // println!("Bad parse result: {:?}", bad_parse);
    // this return Result::Err()

    // enum Result<i32, ParseIntError> {
    //     Ok(i32),  // Operation is success and return ok with type T( T can be any type)
    //     Err(ParseIntError)  // Operation is failed and return err with type E( E can be any type NotfoundError, ParseIntError, etc.)
    // }

    // match good_parse {
    //     Ok(value) => println!("Parsed value is {}", value),
    //     Err(e) => println!("Failed to parse: {}", e),
    // }

    // match bad_parse {
    //     Ok(value) => println!("Parsed value is {}", value),
    //     Err(e) => println!("Failed to parse: {}", e),
    // }

    // using if let
    // if let Ok(value) = good_parse {
    //     println!("Parsed value is {}", value);
    // } else {
    //     println!("Failed to parse good_parse");
    // }


    // let parsing_test = vec!["42", "abc", "100", "xyz"];
    // for i in parsing_test{
    //     match i.parse::<i32>() {
    //         Ok(value) => println!("Parsed value is {}", value),
    //         Err(e) => println!("Failed to parse '{}': {}", i, e),
    //     }
    // }


    // let math_test = vec![("10/2", safe_division(10, 2)),
    //                      ("10/0", safe_division(10, 0)),
    //                      ("20/5", safe_division(20, 5)),
    //                      ("20/0", safe_division(20, 0))];

    // // for i in array
    // // i is a tuple ("10/2", safe_division(10, 2))
    // // let (x, y) = i;
    // println!("Math Test Results: {:?}", math_test);
    // for (description, k) in math_test {
    //     // println!("Calculating: {}", k);
    //     match k {
    //         Ok(value) => println!("{} = {}", description, value), //Ok(value)=> Ok(5)
    //         Err(e) => println!("Error in {}: {}", description, e), // Err(e) => Err("Can not")
    //     }
    // }


    let good_result: Result<i32, &str> = Ok(100);
    let bad_result: Result<i32, &str> = Err("Something went wrong");


    // println!("Good result: {:?}", good_result.is_ok());
    // println!("Good result: {:?}", good_result.is_err());
    // println!("Bad result: {:?}", bad_result.is_ok());
    // println!("Bad result: {:?}", bad_result.is_err());

    println!("bad result: {:?}", bad_result.unwrap_or(0));
    println!("good result {:?}", good_result.unwrap_or(0));


    // Todo: homework
    // and_then
    // map

    // into_iter(), collect(), filter(), min , max
}
 
fn safe_division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Can not divide by zero".to_string());//Err(String)
    } else{
        return Ok(a / b) //Ok(5)
    }
  
}

// option : is_some and is_none
// result: is_ok and is_err


//  operator ?


fn parse_number(text: &str) -> Result<i32, String> {
    match text.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Failed to parse number".to_string()),
    }
}
 
fn double_string_number(text: &str) -> Result<i32, String> {
    let number = parse_number(text)?;  // return Err(e)

    Ok(number * 2)
    // Err(format!("{}", number*2))
    // match number {
    //     Ok(value) => Ok(value * 2),
    //     Err(e) => Err(e),
    // }

    // let double_number_str = (number*2).to_string();
    // println!("Double number string: {}", double_number_str);
}

fn test_operator(){

    // match parse_number("42") {
    //     Ok(value) => println!("Parsed value: {}", value),
    //     Err(e) => println!("Error: {}", e),
    // }

    // match parse_number("abc") {
    //     Ok(value) => println!("Parsed value: {}", value),
    //     Err(e) => println!("Error: {}", e),
    // }


    match double_string_number("42") {
        Ok(value) => println!("Doubled value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match double_string_number("abc") {
        Ok(value) => println!("Doubled value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

// TODO
// unwrap_or_else, unwrap_or_default, map, map_err, filter, into_iter, iter
// collect, min, max, and_then, or_else, ok_or, ok_or_else


