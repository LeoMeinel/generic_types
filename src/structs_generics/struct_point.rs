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
