/*
 * generic_types is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/generic_types/blob/main/LICENSE
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
