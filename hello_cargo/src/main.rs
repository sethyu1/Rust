// fn main() {
//     println!("Hello, world!");
// }


// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }


// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// fn five () -> i32 {
//     5
// }

// fn main() {
//     let x: i32 = five();
    
//     println!("The value of x is: {x}");
// }



// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let mut s = String::from("hello");
//     s = String::from("ahoy");

//     println!("{s}, world!");

// }


// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // Because i32 implements the Copy trait,
//                                     // x does NOT move into the function,
//                                     // so it's okay to use x afterward.

//     // println!("The value of x is: {x}"); // This works because x is still valid      
//     println!("The value of s is: {s}"); // This will cause a compile-time error

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//   // nothing special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let reference_to_nothing = no_dangle();
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// fn main() {
//     first_word(&String::from("hello world"));
//     println!("The first word is: {}", first_word(&String::from("hello world")));
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error!

//     println!("the first word is: {word}");
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }

// fn main() {
//     //     let v = vec![100, 32, 57];
//     // for i in &v {
//     //     println!("{i}");
//     // }

//     // let mut v = vec![100, 32, 57];
//     // for i in &mut v {
//     //     *i += 50;

//     //     print!("{i} "); 
//     // }
//     //     for c in "中国地图".bytes() {
//     //     println!("{c}");
//     // }

//     // use std::collections::HashMap;

//     // let mut scores = HashMap::new();

//     // scores.insert(String::from("Blue"), 10);
//     // scores.insert(String::from("Yellow"), 50);

//     // for (key, value) in &scores {
//     //     println!("{key}: {value}");
//     // }

// // use std::collections::HashMap;

// // let field_name = String::from("Favorite color");
// // let field_value = String::from("Blue");

// // let mut map = HashMap::new();
// // map.insert(field_name, field_value);

// // println!("field_name is {}", field_name); // ❌ ERROR


//     // use std::collections::HashMap;

//     // let mut scores = HashMap::new();
//     // scores.insert(String::from("Blue"), 10);

//     // scores.entry(String::from("Yellow")).or_insert(50);
//     // scores.entry(String::from("Blue")).or_insert(50);

//     // println!("{scores:?}");

//         use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{map:?}");



// }


// fn main() {
//     panic!("crash and burn");
// }


// #[derive(Debug)]
// enum Option<T> {
//     Some(T),
//     None,
// }

// fn main() {
//     let integer = Option::Some(5);   // T = i32
//     let float = Option::Some(5.0);   // T = f64
//     println!("integer: {:?}, float: {:?}", integer, float);
// }


// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap(); 

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }

    
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {v:?}");
//     });

//     handle.join().unwrap();
// }

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         println!("Sending: {val}");
//         tx.send(val).unwrap();
        
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {received}");
// }

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}