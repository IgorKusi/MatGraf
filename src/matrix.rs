use std::convert::identity;
use std::ops::*;
use nalgebra::ArrayStorage;
use crate::vector3::Vector3;

use crate::vector4::Vector4;

#[derive(Debug)]
pub struct Matrix4 {
    pub entries: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        let mut m: Matrix4 = Matrix4 { entries: [[0f64; 4]; 4] };
        for i in 0..4 {
            m[i][i] = 1f64;
        }
        return m;
    }

    pub fn det3x3(matrix: [[f64; 3]; 3]) -> f64 {
        let [[a, b, c], [d, e, f], [g, h, i]] = matrix;
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    pub fn translate(vec: &Vector3) -> Matrix4 {
        let mut ret = Self::identity();
        for i in 0..3 {
            ret[i][3] = vec[i];
        }
        return ret;
    }

    pub fn scale(vec: &Vector3) -> Matrix4 {
        let mut ret = Self::identity();
        for i in 0..3 {
            ret[i][i] = vec[i];
        }
        return ret;
    }

    pub fn det(&self) -> f64 {
        let mut determinant = 0.0;
        for i in 0..4 {
            let sub_matrix: [[f64; 3]; 3] = [
                [self[1][(i + 1) % 4], self[1][(i + 2) % 4], self[1][(i + 3) % 4]],
                [self[2][(i + 1) % 4], self[2][(i + 2) % 4], self[2][(i + 3) % 4]],
                [self[3][(i + 1) % 4], self[3][(i + 2) % 4], self[3][(i + 3) % 4]],
            ];
            determinant += self[0][i] * Self::det3x3(sub_matrix) * if i % 2 == 0 { 1.0 } else { -1.0 };
        }
        return determinant;
    }


    pub fn col_as_vec4(&self, x: i8) -> Vector4 {
        Vector4::new(self[0][x], self[1][x], self[2][x], self[3][x])
    }

    pub fn row_as_vec4(&self, y: i8) -> Vector4 {
        Vector4::new(self[y][0], self[y][1], self[y][2], self[y][3])
    }

    pub fn add(&self, other: &Matrix4) -> Matrix4 {
        let mut arr = [[0f64; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                arr[y][x] = self[y][x] + other[y][x];
            }
        }

        return Matrix4 { entries: arr };
    }

    pub fn mut_add(&mut self, other: &Matrix4) {
        for y in 0..4 {
            for x in 0..4 {
                self[y][x] += other[y][x];
            }
        }
    }

    pub fn sub(&self, other: &Matrix4) -> Matrix4 {
        let mut arr = [[0f64; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                arr[y][x] = self[y][x] - other[y][x];
            }
        }

        return Matrix4 { entries: arr };
    }

    pub fn mut_sub(&mut self, other: &Matrix4) {
        for y in 0..4 {
            for x in 0..4 {
                self[y][x] -= other[y][x];
            }
        }
    }

    pub fn mul_sclr(&self, scalar: &f64) -> Matrix4 {
        let mut arr = [[0f64; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                arr[y][x] = self[y][x] * scalar;
            }
        }

        return Matrix4 { entries: arr };
    }

    pub fn mut_mul_sclr(&mut self, scalar: &f64) {
        for y in 0..4 {
            for x in 0..4 {
                self[y][x] *= scalar;
            }
        }
    }

    pub fn mul(&self, other: &Matrix4) -> Matrix4 {
        let mut arr = [[0f64; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                arr[y][x] = self.row_as_vec4(y as i8).dot_product(&other.col_as_vec4(x as i8));
            }
        }

        return Matrix4 { entries: arr };
    }

    pub fn mut_mul(&mut self, other: &Matrix4) {
        let mut res = self.mul(&other);

        //??????
        // self = &mut res;
        for y in 0..4 {
            for x in 0..4 {
                self[y][x] = res[y][x];
            }
        }
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut ret = Self::identity();
        for y in 0..4 {
            for x in 0..4 {
                ret[y][x] = self[x][y];
            }
        }

        return ret;
    }

    pub fn mut_transpose(&mut self) {
        //???????
        // self = &mut self.transpose()
        let res = self.transpose();
        for y in 0..4 {
            for x in 0..4 {
                self[y][x] = res[y][x];
            }
        }
    }

    pub fn invert(&self) -> Matrix4 {
        self.transpose().mul_sclr(1 / self.det())
    }

    pub fn mut_invert(&mut self) {
        let mut res = self.invert();
        for y in 0..4 {
            for x in 0..4 {
                self[y][x] = res[y][x];
            }
        }
    }

    pub fn trans(&self, vec: Vector3) -> Matrix4 {
        let mut ret: Matrix4 = Self::identity();
        for y in 0..4 {
            for x in 0..4 {
                ret[y][x] = self[y][x];
            }
        }
        for i in 0..3 {
            ret[i][3] = vec[i];
        }
        return ret;
    }

    pub fn mut_trans(&mut self, vec: Vector3) {
        for i in 0..3 {
            self[i][3] = vec[i];
        }
    }

    pub fn _scale(&self, vec: Vector3) -> Matrix4 {
        let mut ret: Matrix4 = Self::identity();
        for y in 0..4 {
            for x in 0..4 {
                ret[y][x] = self[y][x];
            }
        }
        for i in 0..3 {
            ret[i][i] *= vec[i];
        }
        return ret;
    }

    pub fn _mut_scale(&mut self, vec: Vector3) {
        for i in 0..3 {
            self[i][i] *= vec[i];
        }
    }


}

impl Add<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn add(&self, rhs: &Matrix4) -> Self::Output {
        self.add(&rhs)
    }
}

impl AddAssign<Matrix4> for Matrix4 {
    fn add_assign(&mut self, rhs: &Matrix4) {
        self.mut_add(&rhs)
    }
}

impl Sub<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn sub(&self, rhs: &Matrix4) -> Self::Output {
        self.sub(&rhs)
    }
}

impl SubAssign<Matrix4> for Matrix4 {
    fn sub_assign(&mut self, rhs: &Matrix4) {
        self.mut_sub(&rhs)
    }
}

impl Mul<f64> for Matrix4 {
    type Output = Matrix4;

    fn mul(&self, rhs: &f64) -> Self::Output {
        self.mul_sclr(&rhs)
    }
}

impl MulAssign<f64> for Matrix4 {
    fn mul_assign(&mut self, rhs: &f64) {
        self.mut_mul_sclr(&rhs)
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(&self, rhs: &Matrix4) -> Self::Output {
        self.mul(&rhs)
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: &Matrix4) {
        self.mut_mul(&rhs)
    }
}

impl Neg for Matrix4 {
    type Output = Matrix4;

    fn neg(self) -> Self::Output {
        self.invert()
    }
}