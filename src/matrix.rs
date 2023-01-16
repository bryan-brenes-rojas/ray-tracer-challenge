use crate::point::Point;
use crate::utils::EPSILON;
use crate::vector::Vector;
use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug)]
pub struct Matrix {
    matrix: Vec<Vec<f32>>,
    pub row_count: usize,
    pub col_count: usize,
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(row_num: usize, col_num: usize) -> Self {
        let mut m = Vec::new();
        for _ in 0..row_num {
            let mut v = Vec::new();
            for _ in 0..col_num {
                v.push(0.0);
            }
            m.push(v);
        }
        Matrix {
            matrix: m,
            row_count: row_num,
            col_count: col_num,
        }
    }

    pub fn identity(size: usize) -> Matrix {
        let mut matrix = Matrix::new(size, size);
        for r in 0..size {
            for c in 0..size {
                if r == c {
                    matrix.write_cell(r, c, 1.0)
                } else {
                    matrix.write_cell(r, c, 0.0)
                }
            }
        }
        matrix
    }

    pub fn translation_3d(x: f32, y: f32, z: f32) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_cell(0, 3, x);
        m.write_cell(1, 3, y);
        m.write_cell(2, 3, z);
        m
    }

    pub fn scaling_3d(x: f32, y: f32, z: f32) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_cell(0, 0, x);
        m.write_cell(1, 1, y);
        m.write_cell(2, 2, z);
        m
    }

    pub fn get_cell(&self, row: usize, col: usize) -> f32 {
        self.matrix[row][col]
    }

    pub fn write_cell(&mut self, row: usize, col: usize, value: f32) {
        self.matrix[row][col] = value;
    }

    pub fn patch(&mut self, new_value: Vec<Vec<f32>>) {
        if new_value.len() != self.row_count || new_value[0].len() != self.col_count {
            panic!("The new value doesn't have the same dimensions");
        }

        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                self.write_cell(row_index, col_index, new_value[row_index][col_index]);
            }
        }
    }

    pub fn get_col(&self, col_index: usize) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        for row_index in 0..self.row_count {
            vec.push(self.get_cell(row_index, col_index));
        }
        return vec;
    }

    pub fn get_row(&self, row_index: usize) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        for col_index in 0..self.col_count {
            vec.push(self.get_cell(row_index, col_index));
        }
        return vec;
    }

    pub fn sub_matrix(&self, row_index: usize, col_index: usize) -> Matrix {
        let mut new_matrix = Matrix::new(self.row_count - 1, self.col_count - 1);
        for r_i in 0..self.row_count {
            for c_i in 0..self.col_count {
                if row_index == r_i || col_index == c_i {
                    continue;
                }

                let cur_row = if r_i > row_index { r_i - 1 } else { r_i };
                let cur_col = if c_i > col_index { c_i - 1 } else { c_i };
                new_matrix.write_cell(cur_row, cur_col, self.get_cell(r_i, c_i));
            }
        }
        new_matrix
    }

    pub fn determinant(&self) -> f32 {
        if !self.is_square() {
            panic!(
                "Cofactor::matrix not squared {}x{}",
                self.row_count, self.col_count
            );
        }
        if self.row_count == 2 {
            return self.get_cell(0, 0) * self.get_cell(1, 1)
                - self.get_cell(0, 1) * self.get_cell(1, 0);
        }

        let mut det = 0.0;
        for c_i in 0..self.col_count {
            det += (-1f32).powf(0f32 + c_i as f32 + 2f32)
                * self.get_cell(0, c_i)
                * self.sub_matrix(0, c_i).determinant();
        }
        det
    }

    pub fn transpose(&self) -> Matrix {
        let mut new_matrix = Matrix::new(self.col_count, self.row_count);
        for col_index in 0..self.col_count {
            for row_index in 0..self.row_count {
                new_matrix.write_cell(col_index, row_index, self.get_cell(row_index, col_index))
            }
        }
        new_matrix
    }

    fn is_square(&self) -> bool {
        self.row_count == self.col_count
    }

    pub fn has_inverse(&self) -> bool {
        self.determinant() != 0.0
    }

    fn co_factors_transpose_matrix(&self) -> Matrix {
        let mut co_matrix = Matrix::new(self.row_count, self.col_count);
        for r_i in 0..self.row_count {
            for c_i in 0..self.col_count {
                let co_factor = (-1f32).powf(r_i as f32 + c_i as f32 + 2.0)
                    * self.sub_matrix(r_i, c_i).determinant();
                co_matrix.write_cell(c_i, r_i, co_factor);
            }
        }
        co_matrix
    }

    pub fn inverse(&self) -> Matrix {
        if !self.has_inverse() {
            panic!("Inverse::matrix doesn't have inverse");
        }
        let co_factors_transpose_matrix = self.co_factors_transpose_matrix();
        &co_factors_transpose_matrix * (1f32 / self.determinant())
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Matrix {
        if self.row_count != other.row_count || self.col_count != other.col_count {
            panic!("Matrices are not the same size");
        }

        let mut new_matrix = Matrix::new(self.row_count, self.col_count);

        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                new_matrix.write_cell(
                    row_index,
                    col_index,
                    self.get_cell(row_index, col_index) + other.get_cell(row_index, col_index),
                );
            }
        }
        new_matrix
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &Matrix) -> Matrix {
        if self.row_count != other.row_count || self.col_count != other.col_count {
            panic!("Matrices are not the same size");
        }

        let mut new_matrix = Matrix::new(self.row_count, self.col_count);

        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                new_matrix.write_cell(
                    row_index,
                    col_index,
                    self.get_cell(row_index, col_index) - other.get_cell(row_index, col_index),
                );
            }
        }
        new_matrix
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        if self.col_count != other.row_count {
            panic!("Matrices of wrong dimensions mxn - nxm");
        }
        let mut new_matrix = Matrix::new(self.row_count, other.col_count);
        for row_index in 0..self.row_count {
            for col_index in 0..other.col_count {
                let row = self.get_row(row_index);
                let col = other.get_col(col_index);

                let mut new_value = 0.0;
                for i in 0..row.len() {
                    new_value += row[i] * col[i];
                }
                new_matrix.write_cell(row_index, col_index, new_value);
            }
        }
        new_matrix
    }
}

impl Mul<f32> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: f32) -> Matrix {
        let mut new_matrix = Matrix::new(self.row_count, self.col_count);
        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                new_matrix.write_cell(
                    row_index,
                    col_index,
                    other * self.get_cell(row_index, col_index),
                );
            }
        }
        new_matrix
    }
}

impl Mul<&Point> for &Matrix {
    type Output = Point;

    fn mul(self, p: &Point) -> Point {
        let point_matrix = p.to_matrix();
        let new_point_matrix = self * &point_matrix;
        Point::new(
            new_point_matrix.get_cell(0, 0),
            new_point_matrix.get_cell(1, 0),
            new_point_matrix.get_cell(2, 0),
        )
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, v: &Vector) -> Vector {
        let vector_matrix = v.to_matrix();
        let new_vector_matrix = self * &vector_matrix;
        Vector::new(
            new_vector_matrix.get_cell(0, 0),
            new_vector_matrix.get_cell(1, 0),
            new_vector_matrix.get_cell(2, 0),
        )
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                let diff =
                    self.get_cell(row_index, col_index) - other.get_cell(row_index, col_index);
                if diff.abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_matrix() {
        let m = Matrix::new(2, 3);
        assert_eq!(m.row_count, 2);
        assert_eq!(m.col_count, 3);
    }

    #[test]
    fn should_be_able_to_write_a_cell() {
        let mut m = Matrix::new(2, 3);
        assert_eq!(m.get_cell(0, 0), 0.0);
        m.write_cell(0, 0, 3.0);
        assert_eq!(m.get_cell(0, 0), 3.0);
    }

    #[test]
    fn should_be_able_to_get_a_cell() {
        let m = Matrix::new(2, 3);
        assert_eq!(m.get_cell(0, 0), 0.0);
    }

    #[test]
    fn should_patch_the_matrix() {
        let new_value = vec![vec![1.0, 2.0, 3.0]];
        let mut m = Matrix::new(1, 3);
        m.patch(new_value);
        assert_eq!(m.get_cell(0, 0), 1.0);
        assert_eq!(m.get_cell(0, 1), 2.0);
        assert_eq!(m.get_cell(0, 2), 3.0);
    }

    #[test]
    fn should_add_matrices() {
        let mut m1 = Matrix::new(1, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0]]);

        let mut m2 = Matrix::new(1, 3);
        m2.patch(vec![vec![2.0, 3.0, 4.0]]);

        let m3 = &m1 + &m2;
        assert_eq!(m3.get_cell(0, 0), 3.0);
        assert_eq!(m3.get_cell(0, 1), 5.0);
        assert_eq!(m3.get_cell(0, 2), 7.0);

        let m4 = &m1.add(&m2);
        assert_eq!(m4.get_cell(0, 0), 3.0);
        assert_eq!(m4.get_cell(0, 1), 5.0);
        assert_eq!(m4.get_cell(0, 2), 7.0);
    }

    #[test]
    fn should_sub_matrices() {
        let mut m1 = Matrix::new(1, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0]]);

        let mut m2 = Matrix::new(1, 3);
        m2.patch(vec![vec![2.0, 3.0, 4.0]]);

        let m3 = &m1 - &m2;
        assert_eq!(m3.get_cell(0, 0), -1.0);
        assert_eq!(m3.get_cell(0, 1), -1.0);
        assert_eq!(m3.get_cell(0, 2), -1.0);

        let m4 = &m1.sub(&m2);
        assert_eq!(m4.get_cell(0, 0), -1.0);
        assert_eq!(m4.get_cell(0, 1), -1.0);
        assert_eq!(m4.get_cell(0, 2), -1.0);
    }

    #[test]
    fn should_compare_eq_matrices() {
        let mut m1 = Matrix::new(1, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0]]);

        let mut m2 = Matrix::new(1, 3);
        m2.patch(vec![vec![2.0, 3.0, 4.0]]);

        let cmp = &m1 == &m2;
        assert_eq!(cmp, false);

        let mut m1 = Matrix::new(1, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0]]);

        let mut m2 = Matrix::new(1, 3);
        m2.patch(vec![vec![1.0, 2.0, 3.0]]);

        let cmp = &m1 == &m2;
        assert_eq!(cmp, true);
    }

    #[test]
    fn should_compare_neq_matrices() {
        let mut m1 = Matrix::new(1, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0]]);

        let mut m2 = Matrix::new(1, 3);
        m2.patch(vec![vec![2.0, 3.0, 4.0]]);

        let cmp = &m1 != &m2;
        assert_eq!(cmp, true);

        let mut m1 = Matrix::new(1, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0]]);

        let mut m2 = Matrix::new(1, 3);
        m2.patch(vec![vec![1.0, 2.0, 3.0]]);

        let cmp = &m1 != &m2;
        assert_eq!(cmp, false);
    }

    #[test]
    fn should_return_row() {
        let mut m = Matrix::new(2, 3);
        m.patch(vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0]]);
        let row = m.get_row(1);
        assert_eq!(row[0], 2.0);
        assert_eq!(row[1], 3.0);
        assert_eq!(row[2], 4.0);
    }

    #[test]
    fn should_return_col() {
        let mut m = Matrix::new(2, 3);
        m.patch(vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0]]);
        let row = m.get_col(1);
        assert_eq!(row[0], 2.0);
        assert_eq!(row[1], 3.0);
    }

    #[test]
    fn should_multiply_matrices() {
        let mut m1 = Matrix::new(2, 3);
        let mut m2 = Matrix::new(3, 2);
        m1.patch(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        m2.patch(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);
        let new_matrix = &m1 * &m2;
        assert_eq!(new_matrix.row_count, 2);
        assert_eq!(new_matrix.col_count, 2);

        assert_eq!(new_matrix.get_cell(0, 0), 58.0);
        assert_eq!(new_matrix.get_cell(0, 1), 64.0);
        assert_eq!(new_matrix.get_cell(1, 0), 139.0);
        assert_eq!(new_matrix.get_cell(1, 1), 154.0);
    }

    #[test]
    fn should_multiply_with_scalars() {
        let mut m1 = Matrix::new(2, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let new_matrix = &m1 * 2.0;
        assert_eq!(new_matrix.row_count, 2);
        assert_eq!(new_matrix.col_count, 3);

        assert_eq!(new_matrix.get_cell(0, 0), 2.0);
        assert_eq!(new_matrix.get_cell(0, 1), 4.0);
        assert_eq!(new_matrix.get_cell(0, 2), 6.0);
        assert_eq!(new_matrix.get_cell(1, 0), 8.0);
        assert_eq!(new_matrix.get_cell(1, 1), 10.0);
        assert_eq!(new_matrix.get_cell(1, 2), 12.0);
    }

    #[test]
    fn should_transpose() {
        let mut m1 = Matrix::new(2, 3);
        m1.patch(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let new_matrix = m1.transpose();
        assert_eq!(new_matrix.row_count, 3);
        assert_eq!(new_matrix.col_count, 2);

        assert_eq!(new_matrix.get_cell(0, 0), 1.0);
        assert_eq!(new_matrix.get_cell(0, 1), 4.0);
        assert_eq!(new_matrix.get_cell(1, 0), 2.0);
        assert_eq!(new_matrix.get_cell(1, 1), 5.0);
        assert_eq!(new_matrix.get_cell(2, 0), 3.0);
        assert_eq!(new_matrix.get_cell(2, 1), 6.0);
    }

    #[test]
    fn get_identity_matrix() {
        let matrix = Matrix::identity(2);
        assert_eq!(matrix.get_cell(0, 0), 1.0);
        assert_eq!(matrix.get_cell(1, 1), 1.0);
        assert_eq!(matrix.get_cell(0, 1), 0.0);
        assert_eq!(matrix.get_cell(1, 0), 0.0);
    }

    #[test]
    fn should_get_sub_matrix() {
        let mut m1 = Matrix::new(3, 3);
        m1.patch(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);
        let new_matrix = m1.sub_matrix(1, 1);
        assert_eq!(new_matrix.row_count, 2);
        assert_eq!(new_matrix.col_count, 2);

        assert_eq!(new_matrix.get_cell(0, 0), 1.0);
        assert_eq!(new_matrix.get_cell(0, 1), 3.0);
        assert_eq!(new_matrix.get_cell(1, 0), 7.0);
        assert_eq!(new_matrix.get_cell(1, 1), 9.0);
    }

    #[test]
    fn should_get_determinant_2x2() {
        let mut m1 = Matrix::new(2, 2);
        m1.patch(vec![vec![3.0, 8.0], vec![4.0, 6.0]]);
        assert_eq!(m1.determinant(), -14.0);
    }

    #[test]
    fn should_get_determinant_3x3() {
        let mut m1 = Matrix::new(3, 3);
        m1.patch(vec![
            vec![6.0, 1.0, 1.0],
            vec![4.0, -2.0, 5.0],
            vec![2.0, 8.0, 7.0],
        ]);
        assert_eq!(m1.determinant(), -306.0);
    }

    #[test]
    fn should_get_determinant_4x4() {
        let mut m1 = Matrix::new(4, 4);
        m1.patch(vec![
            vec![6.0, 1.0, 1.0, 6.0],
            vec![4.0, -2.0, 5.0, 8.0],
            vec![2.0, 8.0, 7.0, 9.0],
            vec![12.0, -1.0, 7.0, 7.0],
        ]);
        assert_eq!(m1.determinant(), 2944.0);
    }

    #[test]
    fn should_return_if_has_inverse() {
        let mut m_with_inverse = Matrix::new(4, 4);
        m_with_inverse.patch(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m_with_inverse.determinant(), -2120.0);
        assert_eq!(m_with_inverse.has_inverse(), true);
    }

    #[test]
    fn should_return_if_has_no_inverse() {
        let mut m_with_inverse = Matrix::new(4, 4);
        m_with_inverse.patch(vec![
            vec![-4.0, 2.0, -2.0, 3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m_with_inverse.determinant(), 0.0);
        assert_eq!(m_with_inverse.has_inverse(), false);
    }

    #[test]
    fn should_get_the_inverse() {
        let mut m1 = Matrix::new(4, 4);
        m1.patch(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let inverse = m1.inverse();
        let mut expected_matrix = Matrix::new(4, 4);
        expected_matrix.patch(vec![
            vec![0.21804512, 0.45112783, 0.24060151, -0.04511278],
            vec![-0.8082707, -1.456767, -0.44360903, 0.52067673],
            vec![-0.07894737, -0.22368422, -0.05263158, 0.19736843],
            vec![-0.5225564, -0.81390977, -0.3007519, 0.30639097],
        ]);
        assert_eq!(inverse, expected_matrix);
    }

    #[test]
    fn should_inverse_a_multiplication_result() {
        let mut a = Matrix::new(4, 4);
        a.patch(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let mut b = Matrix::new(4, 4);
        b.patch(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        let c = &a * &b;
        let c_mul_b_inverse = &c * &b.inverse();
        assert_eq!(c_mul_b_inverse, a);
    }

    #[test]
    fn should_create_translation_matrix_3d() {
        let m = Matrix::translation_3d(1.0, 2.0, 3.0);
        assert_eq!(m.get_cell(0, 3), 1.0);
        assert_eq!(m.get_cell(1, 3), 2.0);
        assert_eq!(m.get_cell(2, 3), 3.0);
    }

    #[test]
    fn should_translate_a_point() {
        let m = Matrix::translation_3d(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let new_point = &m * &p;
        assert_eq!(new_point.x, 2.0);
        assert_eq!(new_point.y, 1.0);
        assert_eq!(new_point.z, 7.0);
        assert_eq!(new_point.w, 1.0);
    }

    #[test]
    fn should_revert_point_translation() {
        let m = Matrix::translation_3d(5.0, -3.0, 2.0);
        let m_inverse = m.inverse();

        let p = Point::new(-3.0, 4.0, 5.0);
        let new_point = &m * &p;

        let restoring_point = &m_inverse * &new_point;
        assert_eq!(restoring_point.x, -3.0);
        assert_eq!(restoring_point.y, 4.0);
        assert_eq!(restoring_point.z, 5.0);
        assert_eq!(restoring_point.w, 1.0);
    }

    #[test]
    fn should_translate_a_vector() {
        let m = Matrix::translation_3d(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        let new_vector = &m * &v;
        assert_eq!(new_vector.x, -3.0);
        assert_eq!(new_vector.y, 4.0);
        assert_eq!(new_vector.z, 5.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn should_scale_a_point() {
        let m = Matrix::scaling_3d(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        let new_point = &m * &p;
        assert_eq!(new_point.x, -8.0);
        assert_eq!(new_point.y, 18.0);
        assert_eq!(new_point.z, 32.0);
        assert_eq!(new_point.w, 1.0);
    }

    #[test]
    fn should_revert_point_scaling() {
        let m = Matrix::scaling_3d(2.0, 3.0, 4.0);
        let m_inverse = m.inverse();

        let p = Point::new(-4.0, 6.0, 8.0);
        let new_point = &m * &p;

        let restoring_point = &m_inverse * &new_point;
        assert_eq!(restoring_point.x, -4.0);
        assert_eq!(restoring_point.y, 6.0);
        assert_eq!(restoring_point.z, 8.0);
        assert_eq!(restoring_point.w, 1.0);
    }

    #[test]
    fn should_scale_a_vector() {
        let m = Matrix::scaling_3d(2.0, 3.0, 4.0);

        let v = Vector::new(-4.0, 6.0, 8.0);
        let new_vector = &m * &v;

        assert_eq!(new_vector.x, -8.0);
        assert_eq!(new_vector.y, 18.0);
        assert_eq!(new_vector.z, 32.0);
        assert_eq!(new_vector.w, 0.0);
    }
}
