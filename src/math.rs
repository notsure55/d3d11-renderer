use std::ops::{Div, Mul};

#[derive(Debug, Default)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

struct Scalar(f32);

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Matrix4x4 {
            m: [
                [
                    self.m[0][0] * other.m[0][0],
                    self.m[0][1] * other.m[0][1],
                    self.m[0][2] * other.m[0][2],
                    self.m[0][3] * other.m[0][3],
                ],
                [
                    self.m[1][0] * other.m[1][0],
                    self.m[1][1] * other.m[1][1],
                    self.m[1][2] * other.m[1][2],
                    self.m[1][3] * other.m[1][3],
                ],
                [
                    self.m[2][0] * other.m[2][0],
                    self.m[2][1] * other.m[2][1],
                    self.m[2][2] * other.m[2][2],
                    self.m[2][3] * other.m[2][3],
                ],
                [
                    self.m[3][0] * other.m[3][0],
                    self.m[3][1] * other.m[3][1],
                    self.m[3][2] * other.m[3][2],
                    self.m[3][3] * other.m[3][3],
                ],
            ],
        }
    }
}

impl Div<Scalar> for Matrix4x4 {
    type Output = Self;

    fn div(self, determinant: Scalar) -> Self::Output {
        Matrix4x4 {
            m: [
                [
                    self.m[0][0] / determinant.0,
                    self.m[0][1] / determinant.0,
                    self.m[0][2] / determinant.0,
                    self.m[0][3] / determinant.0,
                ],
                [
                    self.m[1][0] / determinant.0,
                    self.m[1][1] / determinant.0,
                    self.m[1][2] / determinant.0,
                    self.m[1][3] / determinant.0,
                ],
                [
                    self.m[2][0] / determinant.0,
                    self.m[2][1] / determinant.0,
                    self.m[2][2] / determinant.0,
                    self.m[2][3] / determinant.0,
                ],
                [
                    self.m[3][0] / determinant.0,
                    self.m[3][1] / determinant.0,
                    self.m[3][2] / determinant.0,
                    self.m[3][3] / determinant.0,
                ],
            ],
        }
    }
}

#[derive(Debug)]
pub struct Matrix3x3 {
    m: [[f32; 3]; 3],
}

// f32 f32 f32
// f32 f32 f32
// f32 f32 f32

impl Matrix3x3 {
    // we always use first row first column
    pub fn calculate_determinant(&self) -> f32 {
        let det1 = self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]);
        let det2 = self.m[0][1] * (self.m[1][2] * self.m[2][0] - self.m[1][0] * self.m[2][2]);
        let det3 = self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0]);

        det1 + det2 + det3
    }
    pub fn calculate_determinant_diagonal(&self) -> f32 {
        let pos1 = self.m[0][0] * self.m[1][1] * self.m[2][2];
        let pos2 = self.m[0][1] * self.m[1][2] * self.m[2][0];
        let pos3 = self.m[0][2] * self.m[1][0] * self.m[2][1];

        let neg1 = self.m[0][0] * self.m[1][2] * self.m[2][1];
        let neg2 = self.m[0][1] * self.m[1][0] * self.m[2][2];
        let neg3 = self.m[0][2] * self.m[1][1] * self.m[2][0];

        pos1 + pos2 + pos3 - neg1 - neg2 - neg3
    }
    pub fn from_matrix4x4(mat: &Matrix4x4) -> Vec<(f32, Matrix3x3)> {
        let mut matrix_vec = Vec::new();

        let cofactor_1 = mat.m[0][0];
        let matrix_1 = Matrix3x3 {
            m: [
                [mat.m[1][1], mat.m[1][2], mat.m[1][3]],
                [mat.m[2][1], mat.m[2][2], mat.m[2][3]],
                [mat.m[3][1], mat.m[3][2], mat.m[3][3]],
            ],
        };
        matrix_vec.push((cofactor_1, matrix_1));

        let cofactor_2 = mat.m[0][1];
        let matrix_2 = Matrix3x3 {
            m: [
                [mat.m[1][0], mat.m[1][2], mat.m[1][3]],
                [mat.m[2][0], mat.m[2][2], mat.m[2][3]],
                [mat.m[3][0], mat.m[3][2], mat.m[3][3]],
            ],
        };
        matrix_vec.push((cofactor_2, matrix_2));

        let cofactor_3 = mat.m[0][2];
        let matrix_3 = Matrix3x3 {
            m: [
                [mat.m[1][0], mat.m[1][1], mat.m[1][3]],
                [mat.m[2][0], mat.m[2][1], mat.m[2][3]],
                [mat.m[3][0], mat.m[3][1], mat.m[3][3]],
            ],
        };
        matrix_vec.push((cofactor_3, matrix_3));

        let cofactor_4 = mat.m[0][3];
        let matrix_4 = Matrix3x3 {
            m: [
                [mat.m[1][0], mat.m[1][1], mat.m[1][2]],
                [mat.m[2][0], mat.m[2][1], mat.m[2][2]],
                [mat.m[3][0], mat.m[3][1], mat.m[3][2]],
            ],
        };
        matrix_vec.push((cofactor_4, matrix_4));

        matrix_vec
    }
}

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Matrix4x4 {
    pub fn mult_vector(&self, vector: Vec3) -> Option<Vec3> {
        let x = self.m[0][0] * vector.x
            + self.m[0][1] * vector.y
            + self.m[0][2] * vector.z
            + self.m[0][3] * 0.0;
        let y = self.m[1][0] * vector.x
            + self.m[1][1] * vector.y
            + self.m[1][2] * vector.z
            + self.m[1][3] * 0.0;
        let z = self.m[2][0] * vector.x
            + self.m[2][1] * vector.y
            + self.m[2][2] * vector.z
            + self.m[2][3] * 0.0;
        let w = self.m[3][0] * vector.x
            + self.m[3][1] * vector.y
            + self.m[3][2] * vector.z
            + self.m[3][3] * 0.0;

        if w < 0.01 {
            None
        } else {
            Some(Vec3 { x, y, z })
        }
    }
    pub fn calculate_determinant(&self) -> f32 {
        let three_x_threes = Matrix3x3::from_matrix4x4(self);
        let mut final_det: f32 = 0.0;

        for (i, (cofactor, mat)) in three_x_threes.iter().enumerate() {
            let determinant = mat.calculate_determinant();

            if (i + 1) % 2 != 0 {
                final_det += cofactor * determinant;
            } else {
                final_det -= cofactor * determinant;
            }
        }

        final_det
    }
    pub fn calculate_determinant_c4_minus_c1(&self) -> f32 {
        let c1 = self.col(0);
        let c4 = self.col(3);
        let c4 = [c4[0] - c1[0], c4[1] - c1[1], c4[2] - c1[2], c4[3] - c1[3]];

        let cofactor_matrix = Matrix3x3 {
            m: [
                [self.m[1][1], self.m[1][2], c4[1]],
                [self.m[2][1], self.m[2][2], c4[2]],
                [self.m[3][1], self.m[3][2], c4[3]],
            ],
        };

        cofactor_matrix.calculate_determinant_diagonal()
    }
    // ro = 0; col = 0
    pub fn minor(&self, ro: usize, col: usize) -> Matrix3x3 {
        let mut positions = vec![];

        for row in 0..self.m.len() {
            for column in 0..self.m[row].len() {
                if row == ro {
                    break;
                }
                if column == col {
                    continue;
                }

                positions.push(self.m[row][column]);
            }
        }

        assert!(positions.len() == 9);

        Matrix3x3 {
            m: [
                [positions[0], positions[1], positions[2]],
                [positions[3], positions[4], positions[5]],
                [positions[6], positions[7], positions[8]],
            ],
        }
    }
    pub fn calculate_adjugate(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::default();

        let mut plus = true;

        for row in 0..4 {
            for col in 0..4 {
                let minor = self.minor(row, col);
                let determinant = minor.calculate_determinant();
                matrix.m[row][col] = if plus { determinant } else { -determinant };

                if col != 3 {
                    plus = !plus;
                }
            }
        }

        matrix
    }
    pub fn inverse(&self) -> Matrix4x4 {
        let determ = Scalar(self.calculate_determinant());

        let matrix = self.calculate_adjugate();

        let transposed = matrix.transpose();

        transposed / determ
    }
    pub fn transpose(&self) -> Matrix4x4 {
        let row1 = self.row(0);
        let row2 = self.row(1);
        let row3 = self.row(2);
        let row4 = self.row(3);

        Matrix4x4 {
            m: [
                [row1[0], row2[0], row3[0], row4[0]],
                [row1[1], row2[1], row3[1], row4[1]],
                [row1[2], row2[2], row3[2], row4[2]],
                [row1[3], row2[3], row3[3], row4[3]],
            ],
        }
    }
    pub fn row(&self, index: usize) -> [f32; 4] {
        [
            self.m[index][0],
            self.m[index][1],
            self.m[index][2],
            self.m[index][3],
        ]
    }
    pub fn col(&self, index: usize) -> [f32; 4] {
        [
            self.m[0][index],
            self.m[1][index],
            self.m[2][index],
            self.m[3][index],
        ]
    }
}
