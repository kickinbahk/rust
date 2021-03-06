// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Passing structs via FFI should work regardless of whether
// they get passed in multiple registers, byval pointers or the stack

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct Rect {
    a: i32,
    b: i32,
    c: i32,
    d: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct BiggerRect {
    s: Rect,
    a: i32,
    b: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct FloatRect {
    a: i32,
    b: i32,
    c: f64
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct Huge {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32
}

#[link(name = "test", kind = "static")]
extern {
    fn byval_rect(a: i32, b: i32, c: i32, d: i32, e: i32, s: Rect);

    fn byval_rect_floats(a: f32, b: f32, c: f64, d: f32, e: f32,
                         f: f32, g: f64, s: Rect, t: FloatRect);

    fn byval_rect_with_float(a: i32, b: i32, c: f32, d: i32, e: i32, f: i32, s: Rect);

    fn split_rect(a: i32, b: i32, s: Rect);

    fn split_rect_floats(a: f32, b: f32, s: FloatRect);

    fn split_rect_with_floats(a: i32, b: i32, c: f32, d: i32, e: f32, f: i32, s: Rect);

    fn split_and_byval_rect(a: i32, b: i32, c: i32, s: Rect, t: Rect);

    fn split_ret_byval_struct(a: i32, b: i32, s: Rect) -> Rect;

    fn sret_byval_struct(a: i32, b: i32, c: i32, d: i32, s: Rect) -> BiggerRect;

    fn sret_split_struct(a: i32, b: i32, s: Rect) -> BiggerRect;

    fn huge_struct(s: Huge) -> Huge;
}

fn main() {
    let s = Rect { a: 553, b: 554, c: 555, d: 556 };
    let t = BiggerRect { s: s, a: 27834, b: 7657 };
    let u = FloatRect { a: 3489, b: 3490, c: 8. };
    let v = Huge { a: 5647, b: 5648, c: 5649, d: 5650, e: 5651 };

    unsafe {
        byval_rect(1, 2, 3, 4, 5, s);
        byval_rect_floats(1., 2., 3., 4., 5., 6., 7., s, u);
        byval_rect_with_float(1, 2, 3.0, 4, 5, 6, s);
        split_rect(1, 2, s);
        split_rect_floats(1., 2., u);
        split_rect_with_floats(1, 2, 3.0, 4, 5.0, 6, s);
        split_and_byval_rect(1, 2, 3, s, s);
        split_rect(1, 2, s);
        assert_eq!(huge_struct(v), v);
        assert_eq!(split_ret_byval_struct(1, 2, s), s);
        assert_eq!(sret_byval_struct(1, 2, 3, 4, s), t);
        assert_eq!(sret_split_struct(1, 2, s), t);
    }
}
