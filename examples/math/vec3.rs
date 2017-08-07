// Based on https://github.com/toji/gl-matrix/blob/master/src/gl-matrix/vec3.js
// Original license:
// Copyright (c) 2015, Brandon Jones, Colin MacKenzie IV.
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
#![allow(dead_code)]

mod vec3 {
    type Vec3 = [f64; 3];

    pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
        [ a[0] + b[0], a[1] + b[1], a[2] + b[2] ]
    }

    pub fn subtract(a: &Vec3, b: &Vec3) -> Vec3 {
        [ a[0] - b[0], a[1] - b[1], a[2] - b[2] ]
    }

    pub fn multiply(a: &Vec3, b: &Vec3) -> Vec3 {
        [ a[0] * b[0], a[1] * b[1], a[2] * b[2] ]
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn vec3_add() {
        assert_eq!(
            vec3::add(
                &[1.0, 10.0, 100.0],
                &[2.0, 20.0, 200.0]
            ),
            [3.0, 30.0, 300.0]
        );
    }

    #[test]
    fn vec3_subtract() {
        assert_eq!(
            vec3::subtract(
                &[3.0, 30.0, 300.0],
                &[1.0, 10.0, 100.0]
            ),
            [2.0, 20.0, 200.0]
        );
    }

    #[test]
    fn vec3_multiply() {
        assert_eq!(
            vec3::multiply(
                &[3.0, 30.0, 300.0],
                &[2.0, 20.0, 200.0]
            ),
            [6.0, 600.0, 60000.0]
        );
    }

    #[test]
    fn vec3_dot() {
        assert_eq!(
            vec3::dot(
                &[3.0, 30.0, 300.0],
                &[2.0, 20.0, 200.0]
            ),
            60606.0
        );
    }

}
