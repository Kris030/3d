use super::matrix::Matrix;

pub type Veci<const N: usize> = Vector<N, i32>;
pub type Vec2i = Veci<2>;
pub type Vec3i = Veci<3>;
pub type Vec4i = Veci<4>;

pub type Vecf<const N: usize> = Vector<N, f64>;

pub type Vec2f = Vecf<2>;
pub type Vec3f = Vecf<3>;
pub type Vec4f = Vecf<4>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize, T>(pub(crate) [T; N]);

impl<const N: usize, T> From<[T; N]> for Vector<N, T> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}
impl<const N: usize, T: Copy> From<&[T; N]> for Vector<N, T> {
    fn from(value: &[T; N]) -> Self {
        Self(*value)
    }
}

impl<const N: usize> From<Matrix<1, N>> for Vecf<N> {
    fn from(value: Matrix<1, N>) -> Self {
        Self(*value[0])
    }
}
impl<const N: usize> From<Vecf<N>> for Matrix<1, N> {
    fn from(value: Vecf<N>) -> Self {
        Matrix([value])
    }
}

impl<const N: usize, T: std::fmt::Display> std::fmt::Display for Vector<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let Some(first) = self.first() else {
            return write!(f, "]");
        };
        write!(f, "{first:.2}")?;
        for i in 1..N {
            write!(f, ", {:.2}", self[i])?;
        }

        write!(f, "]")
    }
}

impl<const N: usize, T> std::ops::Deref for Vector<N, T> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize, T> std::ops::DerefMut for Vector<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize, T> Vector<N, T> {
    pub const fn new(values: [T; N]) -> Self {
        Self(values)
    }
}

impl<const N: usize> Vecf<N> {
    pub const fn zero() -> Self {
        Self([0.; N])
    }
    pub const fn one() -> Self {
        Self([1.; N])
    }

    pub const fn one_one(at: usize) -> Self {
        let mut v = Self::zero();
        v.0[at] = 1.;
        v
    }

    pub fn len_sqr(&self) -> f64 {
        self.iter().map(|a| a * a).sum()
    }
    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn normalized(self) -> Self {
        let mut v = self;
        let len = v.len();

        for a in v.iter_mut() {
            *a /= len;
        }

        v
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.iter().zip(rhs.iter()).map(|(s, r)| s * r).sum()
    }
}

impl<const N: usize> Veci<N> {
    pub const fn zero() -> Self {
        Self([0; N])
    }
    pub const fn one() -> Self {
        Self([1; N])
    }
}

impl<const N: usize, T: Copy + std::ops::Neg<Output = T>> std::ops::Neg for Vector<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.map(|a| -a))
    }
}

impl<const N: usize, T: Copy + std::ops::Add<Output = T>> std::ops::Add for Vector<N, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl<const N: usize, T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Vector<N, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<const N: usize, T: Copy + std::ops::Mul<T, Output = T>> std::ops::Mul<T> for Vector<N, T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(std::array::from_fn(|i| self[i] * rhs))
    }
}

impl<const N: usize, T: Copy + std::ops::Div<T, Output = T>> std::ops::Div<T> for Vector<N, T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(std::array::from_fn(|i| self[i] / rhs))
    }
}

impl<const N: usize, T: Copy + std::ops::AddAssign> std::ops::AddAssign for Vector<N, T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }
}

impl<const N: usize, T: Copy + std::ops::SubAssign> std::ops::SubAssign for Vector<N, T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }
}

impl<const N: usize, T: Copy + std::ops::MulAssign<T>> std::ops::MulAssign<T> for Vector<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self[i] *= rhs;
        }
    }
}

impl<const N: usize, T: Copy + std::ops::DivAssign<T>> std::ops::DivAssign<T> for Vector<N, T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            self[i] /= rhs;
        }
    }
}

impl<const N: usize, T: Copy> Vector<N, T> {
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn z(&self) -> T {
        self[2]
    }
    pub fn w(&self) -> T {
        self[3]
    }

    pub fn width(&self) -> T {
        self[0]
    }
    pub fn height(&self) -> T {
        self[1]
    }

    pub fn u(&self) -> T {
        self[0]
    }
    pub fn v(&self) -> T {
        self[1]
    }
}

impl Vec2f {
    pub fn offset(&self, (ox, oy): (i16, i16)) -> (i16, i16) {
        (self.x().round() as i16 + ox, -self.y().round() as i16 + oy)
    }
}

impl Vec3f {
    pub fn cross(&self, v: Self) -> Self {
        Self([
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        ])
    }
}

impl Vec4f {
    pub fn div_w(&self) -> Vec3f {
        Vec3f::new([
            self.x() / self.w(),
            self.y() / self.w(),
            self.z() / self.w(),
        ])
    }
}
