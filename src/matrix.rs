use crate::utils::EPSILON;
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
                println!("{:#?}", self.get_cell(r_i, c_i));
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

    pub fn transpose(&self) -> Matrix {
        let mut new_matrix = Matrix::new(self.col_count, self.row_count);
        for col_index in 0..self.col_count {
            for row_index in 0..self.row_count {
                new_matrix.write_cell(col_index, row_index, self.get_cell(row_index, col_index))
            }
        }
        new_matrix
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
        if self.col_count != other.row_count || self.row_count != other.col_count {
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
}
