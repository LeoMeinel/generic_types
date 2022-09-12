/*
 * File: enums_generics.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::enums_generics::enum_option_result::{OptionO, OptionOF64, OptionOI32, ResultO};

mod enum_option_result;

pub(crate) fn enums_generics() {
    get_enums();
    get_enums_i32_f64();
}

fn get_enums_i32_f64() {
    let _option_i32 = OptionOI32::Some(76);
    let _option_f64 = OptionOF64::Some(4.000002);
}

fn get_enums() {
    let _option = OptionO::Some(1);
    let _result: ResultO<i32, i32> = ResultO::Ok(1);
}
