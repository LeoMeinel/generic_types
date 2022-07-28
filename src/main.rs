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
