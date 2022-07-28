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
 * along with this program. If not, see https://github.com/LeoMeinel/generic_types/blob/main/LICENSE
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
