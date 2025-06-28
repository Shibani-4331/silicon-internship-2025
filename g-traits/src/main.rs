fn main() {
    // trait_example();
    dyn_dispatch_example();
}

// Traits in Rust define shared behaviour between types.
// Templates/Boilerplate

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;

    fn what_is_this() {
        println!("This is a Shape");
    }
}

trait CopyOfHasSides {
    fn has_sides() -> bool;
}

trait HasSides {
    fn has_sides() -> bool;
}

struct Circle {
    radius: f64
}

struct Square {
    side: f64
}

struct Rectangle {
    width: f64,
    height: f64
}

// impl TRAIT for TYPE
impl Drawable for Circle {
    fn draw(&self) {
        println!("A circle with Radius: {} is drawn.", self.radius);
    }

    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("A rectangle with width: {} and height: {} is drawn.", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("A square with side: {} is drawn.", self.side);
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }

    // override default function
    fn what_is_this() {
        println!("This is a Square");
    }
}

impl HasSides for Circle {
    fn has_sides() -> bool {
        false
    }
}

impl CopyOfHasSides for Circle {
    fn has_sides() -> bool {
        false
    }
}

impl HasSides for Rectangle {
    fn has_sides() -> bool {
        true
    }
}

fn trait_example() {
    let circle = Circle {
        radius: 5.0
    };

    let rectangle = Rectangle {
        width: 2.0,
        height: 1.0
    };

    let square = Square {
        side: 3.0
    };

    // circle.draw();
    // let c_area = circle.area();
    // println!("Area of Circle with radius: {} = {}\n", circle.radius, c_area);

    // square.draw();
    // let s_area = square.area();
    // println!("Area of Square with side: {} = {}\n", square.side, s_area);

    // rectangle.draw();
    // let r_area = rectangle.area();
    // println!("Area of Rectangle with width: {} and height: {} = {}\n", rectangle.width, rectangle.height, r_area);

    // let res = <Circle as CopyOfHasSides>::has_sides(); // Homework
    // println!("Circle has CopyOfHasSides?: {}", res);
    // let res = <Circle as HasSides>::has_sides(); // Homework
    // println!("Circle has HasSides?: {}", res);

    // let res = Rectangle::has_sides();
    // println!("Rectangle has Sides?: {}", res);

    // let res = Square::has_sides();

    // println!("Square has Sides?: {}", res);

    // print_area(&circle);

    // compare(5, 3);
}

// Trait Bounds
// fn print_area<T: Drawable>(shape: &T) {
//     println!("{}", shape.area());
// }

// fn compare<T: PartialOrd + std::fmt::Display>(a: T, b: T) {
//     let max = if a > b {
//         a
//     } else {
//         b
//     };
//     println!("Max: {}", max);
// }

//  fn compare<T, U, V>(a: T, b: T)
//  where 
    //  T: PartialOrd + std::fmt::Display
    //  U: PartialOrd + std::fmt::Display + PartialEq
    //  V: PartialOrd + std::fmt::Display + Clone
//  {
    //  let max = if a > b {
        //  a
    //  } else {
        //  b
    //  };
    //  println!("Max: {}", max);
//  }

// U - Type T
// -> PartialOrd -> Display

// Static Dispatch(Compile Time) and Dynamic Dispatch(Runtime)
// Monomorphization - Rust creates specialized version of generic entities for each concrete type.
// at compile time.

// 
fn compare<T: PartialOrd + std::fmt::Display>(a: T, b: T) {
    let max = if a > b {
        a
    } else {
        b
    };
    println!("Max: {}", max);
}

// compare(5, 3);
// compare(5.9, 3.3);
// // compare('5', '3');

// compare_integer()
// compare_float()
// compare_char()

// struct Point<T> {
//     x: T,
//     y: T
// }

// point1 = Point::<i32> {
//     x: 20,
//     y: 20
// }

// point2 = Point::<f32> {
//     x: 20.789,
//     y: 20.32
// }

// struct IntPoint {
//     x: i32,
//     y: i32
// }

// struct FloatPoint {
//     x: f32,
//     y: f32
// }

trait Draw {
   fn draw_new(&self, item: i32);
   fn name(&self) -> String;
}

impl Draw for Circle {
    fn draw_new(&self, item: i32) {
        println!("Drawing Circle with radius: {} and item {}", self.radius, item);
    }

    fn name(&self) -> String {
        "Circle".to_string()
    }
}

impl Draw for Rectangle {
    fn draw_new(&self, item: i32) {
        println!("Drawing Rectangle with width: {} and height: {}", self.width, self.height);
    }

    fn name(&self) -> String {
        "Rectangle".to_string()
    }
}

impl Draw for Square {
    fn draw_new(&self, item: i32) {
        println!("Drawing Square with side: {}", self.side);
    }

    fn name(&self) -> String {
        "Square".to_string()
    }
}


// Dynamic Dispatch - using trait objects
fn dyn_dispatch_example() {
    let circle = Circle {
        radius: 5.0
    };

    let rectangle = Rectangle {
        width: 2.0,
        height: 1.0
    };

    let square = Square {
        side: 3.0
    };

    let circle_2 = Circle {
        radius: 10.0
    };

    let rectangle_2 = Rectangle {
        width: 4.0,
        height: 2.0
    };

    // Vec<String>
    // Vec<i32>
    // Vec<bool>
    // Vec<&str>
    // Vec<&dyn Drawable>

    let shapes: Vec<&dyn Draw> = vec![&circle, &rectangle, &square, &circle_2, &rectangle_2];
    for shape in shapes {
        shape.draw_new(23);
        println!("Shape Name: {}", shape.name());
    }
}

// Rules to allow dynamic dispatch:
// 1. All functions in the trait must have a 'self' parameter(all functions in traits should be a method).
// 2. The trait must not have any generic parameters.

// trait Draw {
   // fn draw(&self);
   // fn name(&self) -> String;
//}