/*
 * File: enum_option_result.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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
