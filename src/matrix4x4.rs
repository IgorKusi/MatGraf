pub struct Matrix4x4 {
    entries: [f64; 16]
}

impl Matrix4x4 {
    pub fn new(entries: [f64; 16]) -> Matrix4x4 {
        Matrix4x4 { entries }
    }

    pub fn from_values(
        e00: f64, e01: f64, e02: f64, e03: f64,
        e10: f64, e11: f64, e12: f64, e13: f64,
        e20: f64, e21: f64, e22: f64, e23: f64,
        e30: f64, e31: f64, e32: f64, e33: f64,
    ) -> Matrix4x4 {
        let entries = [
            e00, e01, e02, e03,
            e10, e11, e12, e13,
            e20, e21, e22, e23,
            e30, e31, e32, e33,
        ];
        Matrix4x4 { entries }
    }

    pub fn add(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut arr = [0.0; 16];
        for i in 0..16 {
            arr[i] = self.entries[i] + other.entries[i];
        }
        Matrix4x4 { entries: arr }
    }

    pub fn sub(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut arr = [0.0; 16];
        for i in 0..16 {
            arr[i] = self.entries[i] - other.entries[i];
        }
        Matrix4x4 { entries: arr }
    }

    pub fn mul_scalar(&self, scalar: f64) -> Matrix4x4 {
        let mut arr = [0.0; 16];
        for i in 0..16 {
            arr[i] = self.entries[i] * scalar;
        }
        Matrix4x4 { entries: arr }
    }

    pub fn mul(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut result = [0.0; 16];

        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.entries[i * 4 + k] * other.entries[k * 4 + j];
                }
                result[i * 4 + j] = sum;
            }
        }

        Matrix4x4 { entries: result }
    }

    pub fn identity() -> Matrix4x4 {
        let mut result = [0.0; 16];
        for i in 0..4 {
            result[i * 4 + i] = 1.0;
        }
        Matrix4x4 { entries: result }
    }

    pub fn inverse(&self) -> Option<Matrix4x4> {
        let a = &self.entries;

        let det = a[0] * (a[5] * (a[10] * a[15] - a[11] * a[14]) - a[6] * (a[9] * a[15] - a[11] * a[13]) + a[7] * (a[9] * a[14] - a[10] * a[13]))
            - a[1] * (a[4] * (a[10] * a[15] - a[11] * a[14]) - a[6] * (a[8] * a[15] - a[11] * a[12]) + a[7] * (a[8] * a[14] - a[10] * a[12]))
            + a[2] * (a[4] * (a[9] * a[15] - a[11] * a[13]) - a[5] * (a[8] * a[15] - a[11] * a[12]) + a[7] * (a[8] * a[13] - a[9] * a[12]))
            - a[3] * (a[4] * (a[9] * a[14] - a[10] * a[13]) - a[5] * (a[8] * a[14] - a[10] * a[12]) + a[6] * (a[8] * a[13] - a[9] * a[12]));

        if det == 0.0 {
            return None; // Matrix is not invertible
        }

        let inv_det = 1.0 / det;
        let mut result = [0.0; 16];

        result[0] = (a[5] * (a[10] * a[15] - a[11] * a[14]) - a[6] * (a[9] * a[15] - a[11] * a[13]) + a[7] * (a[9] * a[14] - a[10] * a[13])) * inv_det;
        result[1] = (-a[1] * (a[10] * a[15] - a[11] * a[14]) + a[2] * (a[9] * a[15] - a[11] * a[13]) - a[3] * (a[9] * a[14] - a[10] * a[13])) * inv_det;
        result[2] = (a[1] * (a[6] * a[15] - a[7] * a[14]) - a[2] * (a[5] * a[15] - a[7] * a[13]) + a[3] * (a[5] * a[14] - a[6] * a[13])) * inv_det;
        result[3] = (-a[1] * (a[6] * a[11] - a[7] * a[10]) + a[2] * (a[5] * a[11] - a[7] * a[9]) - a[3] * (a[5] * a[10] - a[6] * a[9])) * inv_det;

        result[4] = (-a[4] * (a[10] * a[15] - a[11] * a[14]) + a[6] * (a[8] * a[15] - a[11] * a[12]) - a[7] * (a[8] * a[14] - a[10] * a[12])) * inv_det;
        result[5] = (a[0] * (a[10] * a[15] - a[11] * a[14]) - a[2] * (a[8] * a[15] - a[11] * a[12]) + a[3] * (a[8] * a[14] - a[10] * a[12])) * inv_det;
        result[6] = (-a[0] * (a[6] * a[15] - a[7] * a[14]) + a[2] * (a[4] * a[15] - a[7] * a[12]) - a[3] * (a[4] * a[14] - a[6] * a[12])) * inv_det;
        result[7] = (a[0] * (a[6] * a[11] - a[7] * a[10]) - a[2] * (a[4] * a[11] - a[7] * a[8]) + a[3] * (a[4] * a[10] - a[6] * a[8])) * inv_det;

        result[8] = (a[4] * (a[9] * a[15] - a[11] * a[13]) - a[5] * (a[8] * a[15] - a[11] * a[12]) + a[7] * (a[8] * a[13] - a[9] * a[12])) * inv_det;
        result[9] = (-a[0] * (a[9] * a[15] - a[11] * a[13]) + a[1] * (a[8] * a[15] - a[11] * a[12]) - a[3] * (a[8] * a[13] - a[9] * a[12])) * inv_det;
        result[10] = (a[0] * (a[5] * a[15] - a[7] * a[13]) - a[1] * (a[4] * a[15] - a[7] * a[12]) + a[3] * (a[4] * a[13] - a[5] * a[12])) * inv_det;
        result[11] = (-a[0] * (a[5] * a[11] - a[7] * a[9]) + a[1] * (a[4] * a[11] - a[7] * a[8]) - a[3] * (a[4] * a[9] - a[5] * a[8])) * inv_det;

        result[12] = (-a[4] * (a[9] * a[14] - a[10] * a[13]) + a[5] * (a[8] * a[14] - a[10] * a[12]) - a[6] * (a[8] * a[13] - a[9] * a[12])) * inv_det;
        result[13] = (a[0] * (a[9] * a[14] - a[10] * a[13]) - a[1] * (a[8] * a[14] - a[10] * a[12]) + a[2] * (a[8] * a[13] - a[9] * a[12])) * inv_det;
        result[14] = (-a[0] * (a[5] * a[14] - a[6] * a[13]) + a[1] * (a[4] * a[14] - a[6] * a[12]) - a[2] * (a[4] * a[13] - a[5] * a[12])) * inv_det;
        result[15] = (a[0] * (a[5] * a[10] - a[6] * a[9]) - a[1] * (a[4] * a[10] - a[6] * a[8]) + a[2] * (a[4] * a[9] - a[5] * a[8])) * inv_det;

        Some(Matrix4x4 { entries: result })
    }

}
