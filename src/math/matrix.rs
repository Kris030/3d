use super::vec::Vecf;
use std::array::from_fn as arr;

pub type Matx<const N: usize> = Matrix<N, N>;
pub type Mat2x2 = Matrix<2, 2>;
pub type Mat3x3 = Matrix<3, 3>;
pub type Mat4x4 = Matrix<4, 4>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const M: usize, const N: usize>(pub(crate) [Vecf<N>; M]);

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub const fn from_rows(rows: [Vecf<N>; M]) -> Self {
        Self(rows)
    }
    // pub const fn from_columns(columns: [Vecf<M>; N]) -> Self {
    //     Self(columns)
    // }

    pub const fn zero() -> Self {
        Self([Vecf::zero(); M])
    }
    pub const fn one() -> Self {
        Self([Vecf::one(); M])
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        Matrix(arr(|y| Vecf::new(arr(|x| self[x][y]))))
    }
}

impl<const N: usize> Matrix<N, N> {
    // pub const fn identity() -> Self {
    //     Self(arr(asd))
    // }
}

impl<const M: usize, const N: usize> std::fmt::Display for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let Some(first) = self.0.first() else {
            return write!(f, "]");
        };
        write!(f, "{first}")?;
        for i in 1..N {
            write!(f, ", {}", self[i])?;
        }

        write!(f, "]")
    }
}

impl<const M: usize, const N: usize> std::ops::Index<(usize, usize)> for Matrix<M, N> {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.0[i][j]
    }
}

impl<const M: usize, const N: usize> std::ops::IndexMut<(usize, usize)> for Matrix<M, N> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i][j]
    }
}

impl<const M: usize, const N: usize> std::ops::Index<usize> for Matrix<M, N> {
    type Output = Vecf<N>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<const M: usize, const N: usize> std::ops::IndexMut<usize> for Matrix<M, N> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<const M: usize, const N: usize> std::ops::Neg for Matrix<M, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.map(|v| -v))
    }
}
impl<const M: usize, const N: usize> std::ops::Add for Matrix<M, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(arr(|i| self.0[i] + rhs[i]))
    }
}

impl<const M: usize, const N: usize> std::ops::Sub for Matrix<M, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(arr(|i| self.0[i] - rhs[i]))
    }
}

impl<const M: usize, const N: usize> std::ops::Mul<f64> for Matrix<M, N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(arr(|i| self.0[i] * rhs))
    }
}

impl<const M: usize, const N: usize> std::ops::Div<f64> for Matrix<M, N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(arr(|i| self.0[i] / rhs))
    }
}
