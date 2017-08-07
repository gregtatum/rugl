#![allow(dead_code)]

type Vec3 = [f32; 3];
type Mat4 = [f32; 16];

pub fn identity() -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ]
}

pub fn copy(m: &Mat4) -> Mat4 {
    [
        m[0], m[1], m[2], m[3],
        m[4], m[5], m[6], m[7],
        m[8], m[9], m[10], m[11],
        m[12], m[13], m[14], m[15]
    ]
}

pub fn multiply(a: &Mat4, b: &Mat4) -> Mat4 {
    [
        b[0] * a[0] + b[1] * a[4] + b[2] * a[8] + b[3] * a[12],
        b[0] * a[1] + b[1] * a[5] + b[2] * a[9] + b[3] * a[13],
        b[0] * a[2] + b[1] * a[6] + b[2] * a[10] + b[3] * a[14],
        b[0] * a[3] + b[1] * a[7] + b[2] * a[11] + b[3] * a[15],

        b[4] * a[0] + b[5] * a[4] + b[6] * a[8] + b[7] * a[12],
        b[4] * a[1] + b[5] * a[5] + b[6] * a[9] + b[7] * a[13],
        b[4] * a[2] + b[5] * a[6] + b[6] * a[10] + b[7] * a[14],
        b[4] * a[3] + b[5] * a[7] + b[6] * a[11] + b[7] * a[15],

        b[8] * a[0] + b[9] * a[4] + b[10] * a[8] + b[11] * a[12],
        b[8] * a[1] + b[9] * a[5] + b[10] * a[9] + b[11] * a[13],
        b[8] * a[2] + b[9] * a[6] + b[10] * a[10] + b[11] * a[14],
        b[8] * a[3] + b[9] * a[7] + b[10] * a[11] + b[11] * a[15],

        b[12] * a[0] + b[13] * a[4] + b[14] * a[8] + b[15] * a[12],
        b[12] * a[1] + b[13] * a[5] + b[14] * a[9] + b[15] * a[13],
        b[12] * a[2] + b[13] * a[6] + b[14] * a[10] + b[15] * a[14],
        b[12] * a[3] + b[13] * a[7] + b[14] * a[11] + b[15] * a[15]
     ]
}

pub fn translate(m: &Mat4, v: &Vec3) -> Mat4 {
    let mut out = copy(m);

    out[12] = m[0] * v[0] + m[4] * v[1] + m[8] * v[2] + m[12];
    out[13] = m[1] * v[0] + m[5] * v[1] + m[9] * v[2] + m[13];
    out[14] = m[2] * v[0] + m[6] * v[1] + m[10] * v[2] + m[14];
    out[15] = m[3] * v[0] + m[7] * v[1] + m[11] * v[2] + m[15];

    out
}

pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fovy / 2.0).tan();
    let nf = 1.0 / (near - far);
    [
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (far + near) * nf,
        -1.0,
        0.0,
        0.0,
        (2.0 * far * near) * nf,
        0.0,
    ]
}

static EPSILON: f32 = 0.00001;

pub fn look_at(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4 {
    if (eye[0] - center[0]).abs() < EPSILON &&
       (eye[1] - center[1]).abs() < EPSILON &&
       (eye[2] - center[2]).abs() < EPSILON {
        return identity();
    }

    let mut z0 = eye[0] - center[0];
    let mut z1 = eye[1] - center[1];
    let mut z2 = eye[2] - center[2];

    let len_z = 1.0 / (z0 * z0 + z1 * z1 + z2 * z2).sqrt();
    z0 = z0 * len_z;
    z1 = z1 * len_z;
    z2 = z2 * len_z;

    let mut x0 = up[1] * z2 - up[2] * z1;
    let mut x1 = up[2] * z0 - up[0] * z2;
    let mut x2 = up[0] * z1 - up[1] * z0;
    let len_x = (x0 * x0 + x1 * x1 + x2 * x2).sqrt();

    if len_x == 0.0 {
        x0 = 0.0;
        x1 = 0.0;
        x2 = 0.0;
    } else {
        x0 = x0 / len_x;
        x1 = x0 / len_x;
        x2 = x0 / len_x;
    }

    let mut y0 = z1 * x2 - z2 * x1;
    let mut y1 = z2 * x0 - z0 * x2;
    let mut y2 = z0 * x1 - z1 * x0;

    let len_y = (y0 * y0 + y1 * y1 + y2 * y2).sqrt();
    if len_y == 0.0 {
        y0 = 0.0;
        y1 = 0.0;
        y2 = 0.0;
    } else {
        y0 = y0 / len_y;
        y1 = y1 / len_y;
        y2 = y2 / len_y;
    }

    [
        x0,
        y0,
        z0,
        0.0,
        x1,
        y1,
        z1,
        0.0,
        x2,
        y2,
        z2,
        0.0,
        -(x0 * eye[0] + x1 * eye[1] + x2 * eye[2]),
        -(y0 * eye[0] + y1 * eye[1] + y2 * eye[2]),
        -(z0 * eye[0] + z1 * eye[1] + z2 * eye[2]),
        1.0,
    ]
}

pub fn rotate_y(a: &Mat4, rad: f32) -> Mat4 {
    let s = rad.sin();
    let c = rad.cos();
    [
        a[0] * c - a[8] * s,
        a[1] * c - a[9] * s,
        a[2] * c - a[10] * s,
        a[3] * c - a[11] * s,
        a[4],
        a[5],
        a[6],
        a[7],
        a[0] * s + a[8] * c,
        a[1] * s + a[9] * c,
        a[2] * s + a[10] * c,
        a[3] * s + a[11] * c,
        a[12],
        a[13],
        a[14],
        a[15]
    ]
}

#[cfg(test)]
mod mat4_tests {
    use super::*;

    #[test]
    fn mat4_translate() {
        assert_eq!(
            translate(&identity(), &[0.0, 0.0, 0.0]),
            identity()
        );

        assert_eq!(
            translate(&identity(), &[1.0, 2.0, 3.0]),
            [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                1.0, 2.0, 3.0, 1.0
            ]
        );
    }

    #[test]
    fn mat4_multiply() {
        assert_eq!(
            multiply(
                &[
                    1.0, 2.0, 3.0, 5.0,
                    7.0, 9.0, 11.0, 13.0,
                    17.0, 19.0, 23.0, 29.0,
                    31.0, 37.0, 41.0, 43.0
                ],
                &[
                    43.0, 47.0, 53.0, 59.0,
                    61.0, 67.0, 71.0, 73.0,
                    79.0, 83.0, 89.0, 97.0,
                    101.0, 103.0, 107.0, 109.0
                ]
            ),
            [
                3102.0, 3699.0, 4284.0, 4900.0,
                4000.0, 4775.0, 5546.0, 6374.0,
                5180.0, 6185.0, 7174.0, 8226.0,
                6020.0, 7195.0, 8366.0, 9634.0
            ]
        );
    }

    #[test]
    fn mat4_look_at() {
        assert_eq!(
            look_at(),
            [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, -20.0, 1.0
            ]
        );
    }
}
