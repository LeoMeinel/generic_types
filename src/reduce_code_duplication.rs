/*
 * File: reduce_code_duplication.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn reduce_code_duplication() {
    find_largest_number_duplication();
    call_find_largest_number_extracted();
    call_find_largest_char_extracted();
    call_find_largest_generics_extracted();
}

fn call_find_largest_generics_extracted() {
    let largest = find_largest_generics_extracted(vec!['a', 'c', 'z', '8', '§']);
    println!("The largest char is: {}", largest);
    let largest = find_largest_generics_extracted(vec!['#', 'ü', 'h', '€', '@']);
    println!("The largest char is: {}", largest);
    let largest = find_largest_number_extracted(vec![34, 50, 25, 100, 65]);
    println!("The largest number is: {}", largest);
    let largest = find_largest_number_extracted(vec![22, 11, 6, 8, -100]);
    println!("The largest number is: {}", largest);
}
// T stands for Type, needs to be restricted with traits to use > -> no duplication!

fn find_largest_generics_extracted<T: PartialOrd + Copy>(char_list: Vec<T>) -> T {
    let mut largest = char_list[0];
    for number in char_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn call_find_largest_char_extracted() {
    let largest = find_largest_char_extracted(vec!['a', 'c', 'z', '8', '§']);
    println!("The largest char is: {}", largest);
    let largest = find_largest_char_extracted(vec!['#', 'ü', 'h', '€', '@']);
    println!("The largest char is: {}", largest);
}
// Would result in code duplication because theres one for i32 and one for char

fn find_largest_char_extracted(char_list: Vec<char>) -> char {
    let mut largest = char_list[0];
    for number in char_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn call_find_largest_number_extracted() {
    let largest = find_largest_number_extracted(vec![34, 50, 25, 100, 65]);
    println!("The largest number is: {}", largest);
    let largest = find_largest_number_extracted(vec![22, 11, 6, 8, -100]);
    println!("The largest number is: {}", largest);
}

fn find_largest_number_extracted(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn find_largest_number_duplication() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is: {}", largest);
    let number_list = vec![22, 11, 6, 8, -100];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is: {}", largest)
}
