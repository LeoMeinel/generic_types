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

/*
 * PERFORMANCE:
 * There is no performance hit because the compiler will automatically split OptionO into 2(or more)
 * enums as displayed here
 */

#[allow(dead_code)]
pub(crate) enum OptionO<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
pub(crate) enum OptionOI32 {
    Some(i32),
    None,
}

#[allow(dead_code)]
pub(crate) enum OptionOF64 {
    Some(f64),
    None,
}

#[allow(dead_code)]
pub(crate) enum ResultO<T, E> {
    Ok(T),
    Err(E),
}
