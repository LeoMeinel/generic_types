/*
 * File: struct_point.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[allow(dead_code)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[allow(dead_code)]
pub(crate) struct PointGeneric<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

#[allow(dead_code)]
impl<T> PointGeneric<T> {
    pub(crate) fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
impl PointGeneric<f64> {
    pub(crate) fn y(&self) -> f64 {
        self.y
    }
}

#[allow(dead_code)]
pub(crate) struct PointGenericMultiple<T, U> {
    pub(crate) x: T,
    pub(crate) y: U,
}

#[allow(dead_code)]
impl<T: Copy, U: Copy> PointGenericMultiple<T, U> {
    pub(crate) fn mix_up<V: Copy, W: Copy>(
        &self,
        other: &PointGenericMultiple<V, W>,
    ) -> PointGenericMultiple<T, W> {
        PointGenericMultiple {
            x: self.x,
            y: other.y,
        }
    }
}
