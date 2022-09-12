/*
 * File: structs_generics.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::structs_generics::struct_point::{Point, PointGeneric, PointGenericMultiple};

mod struct_point;

pub(crate) fn structs_generics() {
    point_simple();
    point_generic();
    point_generic_multiple();
    point_generic_implementations();
    point_generic_multiple_mix_up();
}

fn point_generic_multiple_mix_up() {
    let p1 = PointGenericMultiple {
        x: '@',
        y: 0.00000000001,
    };
    let _p2 = PointGenericMultiple {
        x: "X Point",
        y: 10.2000003,
    };
    let p2 = PointGenericMultiple {
        x: "X Point",
        y: 10.2000003,
    };
    let p3 = &p1.mix_up(&p2);
    let p4 = &p2.mix_up(&p1);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
}

fn point_generic_implementations() {
    let p1 = PointGeneric { x: 5, y: 10 };
    let p2 = PointGeneric { x: 5.1, y: 10.2 };
    let _p1_x = p1.x();
    // no method named `y` found for struct `structs_generics::struct_point::PointGeneric<{integer}>`
    //let _p1_y = p1.y();
    let _p2_x = p2.x();
    let _p2_y = p2.y();
}

fn point_generic_multiple() {
    let _p1 = PointGenericMultiple { x: 5, y: 10 };
    let _p2 = PointGenericMultiple { x: 5.1, y: 10.2 };
    let _p3 = PointGenericMultiple { x: 5.1, y: 10 };
}

fn point_generic() {
    let _p1 = PointGeneric { x: 5, y: 10 };
    let _p2 = PointGeneric { x: 5.1, y: 10.2 };
}

fn point_simple() {
    let _p1 = Point { x: 5, y: 10 };
    // ERROR: expected `i32`, found floating-point number
    //let _p2 = Point {x: 5.1, y: 10.2};
}
