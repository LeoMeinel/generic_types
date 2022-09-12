/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::enums_generics::enums_generics;
use crate::reduce_code_duplication::reduce_code_duplication;
use crate::structs_generics::structs_generics;

mod enums_generics;
mod reduce_code_duplication;
mod structs_generics;

fn main() {
    reduce_code_duplication();
    structs_generics();
    enums_generics();
}
