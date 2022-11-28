#![allow(unused)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, Read};
use std::ops::Add;
use std::rc::Rc;
use std::{thread, rc};
use std::time::Duration;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn part_a() {
    let age = 80;

    let can_vote = if (age >= 1) && (age <= 18) {
        true
    } else {
        false
    };


    let x = 10;


    match age.cmp(&x) {
        Ordering::Greater => println!("age is greater than x"),
        _ => println!("age is less than or equal to x")
    };

    match age {
        1..=10 => println!("age is between 1 and 10"),
        80 | 30 => println!("age is either 80 or 30"),
        _ => println!("age is unknown")
    }

    println!("can_vote: {}", can_vote);

    println!("max f32: {}", f32::MAX);
    println!("Whats your name ?");

    let mut name = String::new();
    let greeting = "Nice to meet you";

    io::stdin().read_line(&mut name)
        .expect("yoooo");


    println!("Hello, {} ! {}", name.trim_end(), greeting);
}


fn part_b() {
    let arr = [5,6,7,8];

    println!("arr[0]:{}", arr[0]);

    println!("arr_len:{}", arr.len());

    let mut idx = 0;


    loop {

        if idx >= arr.len() {
            break;
        }

        if idx % 2 == 0 {
            println!("idx is even, moving on");
            idx += 1;
            continue;
        }

        println!("arr[{}]:{}", idx, arr[idx]);
        idx += 1;
    }

    let mut num = [0; 3];

    idx = 0;

    while idx < num.len() {

        let mut line = String::new();

        io::stdin().read_line(& mut line);

        num[idx] = line.trim().parse().expect("we expect a number");

        idx += 1;
    }


    for val in num.iter() {
        println!("{}", val);
    }
}


fn part_c() {
    let tup: (u8, String, f64) = (47, "vineeth".to_string(), 45_000_00.00);
    println!("name:{}", tup.1);

    let mut str = String::new();

    str.push('A');
    str.push_str(" world of lies");

    for word in str.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str.replace("A", "Spread");

    println!("{}", str2);

    let str3 = String::from("abc def  ghi");

    let mut vs: Vec<char> = str3.chars().collect();

    vs.sort();

    vs.dedup();

    for ch in vs.iter() {
        println!("{}", ch);
    }


    let str4: &str = "Random string";

    let mut str5 = str4.to_string();

    let byte_arr = str5.as_bytes();

    for u in byte_arr.iter() {
        println!("{}", u);
    }

    let str6 = &str5[0..6];


    str5.clear();


    let a = String::from("yo");
    let b = String::from(" dude");
    let c = a + &b;

    println!("{}", c);
}


fn part_d() {
    let x: u8 = 5;
    let y: u8 = 10;
    let z: u32 = x as u32 + y as u32;

    enum Level {
        Sde1,
        Sde2,
        Sde3
    }

    impl Level {
        fn is_lead(&self) -> bool {
            match self {
                Level::Sde3 => true,
                _ => false
            }
        }
    }

    let yu: Level = Level::Sde2;

    println!("{}", yu.is_lead());



    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![5, 6, 7, 8];

    vec2.push(10);

    let mut idx = 0;

    loop {
        match vec2.get(idx) {
            Some(val) => {
                println!("{}:{}", idx, val);
            },

            None => break
        }
        idx += 1;
    }


    for i in &mut vec2 {
        *i *= 2; // mutable reference
    }

    for i in &vec2 { // immutable reference
        println!("{}", i);
    }

    println!("{}", sum_a_b(10, 15));

    println!("{}", sum_vec(&vec2));


}


fn sum_a_b(a: i32, b: i32) -> i64 {
    return  (a + b) as i64;
}

fn sum_vec(arr: &[i32]) -> i64 {
    let mut res: i64 = 0;

    // for val in arr {
    //     res += *val as i64;
    // }

    for &val in arr.iter() {
        res += val as i64;
    }

    return res;
}

fn get_sum_gen<T:Add<Output = T>> (x: T, y: T) -> T {
    return x + y;
}

fn part_e() {
    println!("{}", get_sum_gen(20.0, 35.5));

    // Each value has a variable that's called it's owner.
    // There is only one owner at a time.
    // When the owner goes out of scope, the value disappears.
    //

    let str1 = String::from("hello");
    let str2 = str1;
    // println!("{}", str1);
    let str3 = str2.clone();
    println!("{}", str2);

    let mut mp = HashMap::new();


    mp.insert("a", 10);
    mp.insert("b", 20);

    for (key, val) in mp.iter() {
        println!("{}:{}", key, val);
    }

    match mp.get("a") {
        Some(val) => {
            println!("{}", val);
        },
        None => println!("Not found"),
    }
}



fn part_f() {
    struct Customer {
        name: String,
        balance: f32
    };

    let mut bob: Customer = Customer {
        name: String::from("bob"),
        balance: 123.45
    };



    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }


    // struct Rectangle<T,U> {
    //     length: T,
    //     height: U
    // }

    struct Rectangle {
        length: f32, width: f32
    };


    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Self {
            return Rectangle{length, width};
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    const PI: f32 = 3.141592;

    struct Circle {
        radius: f32
    };

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Self {
            return Circle { radius: length };
        }

        fn area(&self) -> f32 {
            return PI * self.radius * self.radius;
        }
    }

    let rect = Rectangle::new(10.1, 11.2);

    let circle = Circle::new(10.0, 10.0);

    println!("{}, {}", rect.area(), circle.area());
}


fn perform_ops<T>(a:i32, b:i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
    return func(a, b);
}

fn part_g() {
    let sum = |a: i32, b: i32| -> i32 {
        return a + b;
    };

    let prod = |a: i32, b: i32| -> i32 {
        return a * b;
    };

    println!("add:{}", perform_ops(5, 10, sum));
    println!("prod:{}", perform_ops(5, 10, prod));


    // & -> to borrow a value rather than taking and cleaning it from memory.
}

fn part_h() {
    // BOX -> smart pointer (stores data on heap instead of stack).


    let box1 = Box::new(10);


    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }


    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            return TreeNode { left: None, right: None, key }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            return self;
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            return self;
        }
    }


    let node_a = TreeNode::new(50)
        .left(TreeNode::new(30))
        .right(TreeNode::new(70));
}


use crate::restaurant::order_food;

// TODO CRATES
fn part_i() {
    // Crates: Modules that produce a library / executable.
    // Modules: Organize & handle privacy.
    // Packages: Build, test & share crates.
    // Paths: A way of naming an item such as struct, function, etc.

    order_food();

    //Result has 2 variants Ok and Err
    // enum Result<T, E> {Ok(T), Err(E)}
    // T represents data type of the value returns and E is the error type.

    let file_path = "lines.txt";
    let mut output = File::create(file_path).unwrap();

    // let mut output = match output {
    //         Ok(file) => file,
    //         Err(error) => {
    //             panic!("Problem creating file: {:?}", error);
    //         }
    // };

    write!(output, "some random words\nhahaha\nabc");

    let input = File::open(file_path).unwrap();
    let buffered  = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }


}

fn part_j() {
    // 1. Threads are accessing data in the wrong order.
    // 2. Threads are blocked from executing because of confusion.
    // over requirements to proceed with execution.
    //


    let th = thread::spawn(|| {
        for i in 1..20 {
            println!("custom thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..10 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    th.join().unwrap();

    // Box, Arc, Mutex, Rc, RefCell, Cow
}