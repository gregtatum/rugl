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
