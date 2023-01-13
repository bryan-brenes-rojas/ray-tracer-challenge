use crate::utils::EPSILON;
use std::cmp::PartialEq;
use std::ops::Add;
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
}
