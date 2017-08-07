// Based on https://github.com/mikolalysenko/angle-normals
// Original license: The MIT License (MIT)
//
// Copyright (c) 2013 Mikola Lysenko
//
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

fn hypot(x: f32, y: f32, z: f32) -> f32 {
  (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

fn weight(s: f32, r: f32, a: f32) -> f32 {
  r.atan2(s - a)
}

fn mul_add(dest: &mut [f32; 3], s: f32, x: f32, y: f32, z: f32) {
  dest[0] = dest[0] + s * x;
  dest[1] = dest[1] + s * y;
  dest[2] = dest[2] + s * z;
}

pub fn compute(
    cells: &[[u32; 3]],
    positions: &[[f32; 3]]
) -> Vec<[f32; 3]> {
    // Allocate normal array
    let mut normals: Vec<[f32; 3]> = positions
        .into_iter()
        .map(|_| [0.0, 0.0, 0.0])
        .collect();

    // Scan cells
    for cell in cells.iter() {
        // This could panic if the cells aren't correctly indexed.
        let a = positions[cell[0] as usize];
        let b = positions[cell[1] as usize];
        let c = positions[cell[2] as usize];

        let abx = a[0] - b[0];
        let aby = a[1] - b[1];
        let abz = a[2] - b[2];
        let ab = hypot(abx, aby, abz);

        let bcx = b[0] - c[0];
        let bcy = b[1] - c[1];
        let bcz = b[2] - c[2];
        let bc = hypot(bcx, bcy, bcz);

        let cax = c[0] - a[0];
        let cay = c[1] - a[1];
        let caz = c[2] - a[2];
        let ca = hypot(cax, cay, caz);

        if ab.min(bc.min(ca)) < 1e-6 {
            continue;
        }

        let s = 0.5 * (ab + bc + ca);
        let r = ((s - ab) * (s - bc) * (s - ca) / s).sqrt();

        let mut nx = aby * bcz - abz * bcy;
        let mut ny = abz * bcx - abx * bcz;
        let mut nz = abx * bcy - aby * bcx;
        let nl = hypot(nx, ny, nz);
        nx = nx / nl;
        ny = ny / nl;
        nz = nz / nl;

        mul_add(&mut normals[cell[0] as usize], weight(s, r, bc), nx, ny, nz);
        mul_add(&mut normals[cell[1] as usize], weight(s, r, ca), nx, ny, nz);
        mul_add(&mut normals[cell[2] as usize], weight(s, r, ab), nx, ny, nz);
    }

    //Normalize all the normals
    for n in normals.iter_mut() {
        let l = (
            n[0].powi(2) +
            n[1].powi(2) +
            n[2].powi(2)
        ).sqrt();

        if l < 1e-8f32 {
            n[0] = 1.0;
            n[1] = 0.0;
            n[2] = 0.0;
            continue;
        }
        n[0] = n[0] / l;
        n[1] = n[1] / l;
        n[2] = n[2] / l;
    }

    normals
}
