#![allow(unused)]

use std::{fmt::Display, rc::Rc, collections::HashMap};

mod derek_banas;
mod linked_list;
mod restaurant;
mod scoping;

mod store;

mod concurrency;

mod parallel_letter_freq;

fn rust_by_example() {
    scoping::notes::raii();
    scoping::notes::ownership_move();
    scoping::notes::mutability();
    scoping::notes::partial_moves();
    scoping::notes::borrowing();
    scoping::notes::borrowing_mutability();
    scoping::notes::borrowing_aliasing();
    scoping::notes::borrowing_ref_pattern();
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let n = nums.len();

    let mut freq = 0;
    let mut j = 0;

    for i in 0..n {
        if nums[i] == val {
            freq += 1;
        } else {
            nums[j] = nums[i];
            j += 1;
        }
    }

    return (n - freq) as i32;
}

fn my_sqrt(x: i32) -> i32 {
    let mut lo: i64 = 0;
    let mut hi: i64 = i32::MAX as i64;
    let mut mid: i64 = 0;
    let x64 = x as i64;

    while lo <= hi {
        mid = lo + (hi - lo) / 2;

        if (mid * mid == x64) {
            return mid as i32;
        }

        if (mid * mid < x64) {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    if ((x64 - mid * mid) > ((mid + 1) * (mid + 1) - x64)) {
        mid += 1;
    }

    return mid as i32;
}

fn demo() {
    linked_list::dsll::boxed::demo();
    linked_list::dsll::rced::demo();

    let mut arr = vec![2, 5, 2, 3, 10, 2, 1];

    let val = 2;

    println!("{}", remove_element(&mut arr, val));

    for i in 0..arr.len() {
        println!("{}:{}", i, arr[i]);
    }
}

fn test_parallel_letter_freq() {
    let a = String::from("hello world");
    let b = "prometheus";

    let arr = vec![b];

    // parallel_letter_freq::solution::multi_threaded_a(&arr, 1);

    print!("{}", a);
}

fn printx(x: i32) {
    println!("{}", x);
}

fn main() {

    // concurrency::threads::arc_mutex_threads();
    // concurrency::message_passing::channels();

    // scoping::notes::lifetimes();


    // parallel_letter_freq::solution::test_multi_threaded_map();

    let x = 10;
    let y = x;

    println!("{}", y);

    // println!(x);
    




    


    

}
