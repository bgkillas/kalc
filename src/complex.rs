use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    math::inverse,
};
use rug::{ops::Pow, Complex};
#[derive(Clone)]
pub enum NumStr
{
    Num(Complex),
    Str(String),
    Vector(Vec<Complex>),
    Matrix(Vec<Vec<Complex>>),
}
impl NumStr
{
    pub fn mul(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(a * b.clone()),
            (Num(b), Vector(a)) | (Vector(a), Num(b)) =>
            {
                Vector(a.iter().map(|a| a * b.clone()).collect())
            }
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| a * b.clone()).collect())
            }
            (Num(b), Matrix(a)) | (Matrix(a), Num(b)) => Matrix(
                a.iter()
                    .map(|a| a.iter().map(|a| a * b.clone()).collect())
                    .collect(),
            ),
            (Vector(b), Matrix(a)) | (Matrix(a), Vector(b)) if a[0].len() == b.len() => Vector(
                a.iter()
                    .map(|j| {
                        (0..a[0].len())
                            .map(|i| b[i].clone() * j[i].clone())
                            .fold(Complex::new(b[0].prec()), |sum, val| sum + val)
                    })
                    .collect::<Vec<Complex>>(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() => Matrix(
                a.iter()
                    .map(|row| {
                        (0..b[0].len())
                            .map(|j| {
                                (0..b.len())
                                    .map(|i| row[i].clone() * b[i][j].clone())
                                    .fold(Complex::new(a[0][0].prec()), |sum, val| sum + val)
                            })
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("string err"),
        })
    }
    pub fn div(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(a / b.clone()),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| a / b.clone()).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| a / b.clone()).collect()),
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| a / b.clone()).collect())
            }
            (Num(a), Matrix(b)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| a / b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Num(b)) => Matrix(
                a.iter()
                    .map(|a| a.iter().map(|a| a / b.clone()).collect())
                    .collect(),
            ),
            (Vector(a), Matrix(b)) if b.len() == a.len() => Matrix(
                (0..b.len())
                    .map(|j| b[j].iter().map(|b| a[j].clone() / b).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| a[j].iter().map(|a| a / b[j].clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| {
                        (0..a[0].len())
                            .map(|k| a[j][k].clone() / b[j][k].clone())
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("string err"),
        })
    }
    pub fn add(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(a + b.clone()),
            (Num(a), Vector(b)) | (Vector(b), Num(a)) =>
            {
                Vector(b.iter().map(|b| a + b.clone()).collect())
            }
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| a + b.clone()).collect())
            }
            (Num(a), Matrix(b)) | (Matrix(b), Num(a)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| a + b.clone()).collect())
                    .collect(),
            ),
            (Vector(a), Matrix(b)) | (Matrix(b), Vector(a)) if b.len() == a.len() => Matrix(
                (0..b.len())
                    .map(|j| b[j].iter().map(|b| a[j].clone() + b).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| {
                        (0..a[0].len())
                            .map(|k| a[j][k].clone() + b[j][k].clone())
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("string err"),
        })
    }
    pub fn sub(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(a - b.clone()),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| a - b.clone()).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| a - b.clone()).collect()),
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| a - b.clone()).collect())
            }
            (Num(a), Matrix(b)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| a - b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Num(b)) => Matrix(
                a.iter()
                    .map(|a| a.iter().map(|a| a - b.clone()).collect())
                    .collect(),
            ),
            (Vector(a), Matrix(b)) if b.len() == a.len() => Matrix(
                (0..b.len())
                    .map(|j| b[j].iter().map(|b| a[j].clone() - b).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| a[j].iter().map(|a| a - b[j].clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| {
                        (0..a[0].len())
                            .map(|k| a[j][k].clone() - b[j][k].clone())
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("string err"),
        })
    }
    pub fn pow(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(a.pow(b.clone())),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| a.pow(b.clone())).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| a.pow(b.clone())).collect()),
            (Vector(a), Vector(b)) if a.len() == b.len() => Vector(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.pow(b.clone()))
                    .collect(),
            ),
            (Num(a), Matrix(b)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| a.pow(b.clone())).collect())
                    .collect(),
            ),
            (Matrix(a), Num(b)) =>
            {
                if b.imag() == &0.0 && b.real().clone().fract() == 0.0
                {
                    let mut mat = Matrix(a.clone());
                    let c = b.real().to_f64().abs() as usize;
                    for _ in 1..c
                    {
                        mat = mat.mul(&Matrix(a.clone()))?;
                    }
                    if b.real() > &0.0
                    {
                        mat
                    }
                    else
                    {
                        Matrix(inverse(mat.mat()?)?)
                    }
                }
                else
                {
                    return Err("no imag/fractional support for powers");
                }
            }
            (Vector(a), Matrix(b)) if b.len() == a.len() => Matrix(
                (0..b.len())
                    .map(|j| b[j].iter().map(|b| a[j].clone().pow(b)).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| a[j].iter().map(|a| a.pow(b[j].clone())).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                (0..a.len())
                    .map(|j| {
                        (0..a[0].len())
                            .map(|k| a[j][k].clone().pow(b[j][k].clone()))
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("string err"),
        })
    }
    pub fn str_is(&self, s: &str) -> bool
    {
        match self
        {
            Str(s2) => s == s2,
            _ => false,
        }
    }
    pub fn num(&self) -> Result<Complex, &'static str>
    {
        match self
        {
            Num(n) => Ok(n.clone()),
            _ => Err("failed to get number"),
        }
    }
    pub fn vec(&self) -> Result<Vec<Complex>, &'static str>
    {
        match self
        {
            Vector(v) => Ok(v.clone()),
            _ => Err("failed to get vector"),
        }
    }
    pub fn mat(&self) -> Result<Vec<Vec<Complex>>, &'static str>
    {
        match self
        {
            Matrix(m) => Ok(m.clone()),
            _ => Err("failed to get matrix"),
        }
    }
}

pub trait Float
{
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;
}
impl Float for (f64, f64)
{
    fn add(self, other: Self) -> Self
    {
        (self.0 + other.0, self.1 + other.1)
    }
    fn sub(self, other: Self) -> Self
    {
        (self.0 - other.0, self.1 - other.1)
    }
    fn mul(self, other: Self) -> Self
    {
        match (self.0 == 0.0, self.1 == 0.0, other.0 == 0.0, other.1 == 0.0)
        {
            (true, true, _, _) | (_, _, true, true) => (0.0, 0.0), // (0)(c+di)=0 | (a+bi)(0)=0
            (true, false, true, false) => (-self.1 * other.1, 0.0), // (bi)(di)=-bd
            (false, true, false, true) => (self.0 * other.0, 0.0), // (a)(c)=ac
            (false, false, false, true) => (self.0 * other.0, self.1 * other.0), // (a+bi)(c)=ac+bci
            (false, true, false, false) => (self.0 * other.0, self.0 * other.1), // (a)(c+di)=ac+adi
            (false, false, true, false) => (-self.1 * other.1, self.0 * other.1), // (a+bi)(di)=-bd+adi
            (true, false, false, false) => (-self.1 * other.1, self.1 * other.0), // (bi)(c+di)=-bd+bci
            (false, false, false, false) => (
                self.0 * other.0 - self.1 * other.1,
                self.0 * other.1 + self.1 * other.0,
            ), // (a+bi)(c+di)=(ac-bd)+(ad+bc)i
            _ => unreachable!(),
        }
    }
    fn div(self, other: Self) -> Self
    {
        match (self.0 == 0.0, self.1 == 0.0, other.0 == 0.0, other.1 == 0.0)
        {
            (true, true, _, _) => (0.0, 0.0), // (0)/(c+di)=0
            (_, _, true, true) => (f64::INFINITY, f64::INFINITY), // (a+bi)/(0)=inf
            (true, false, true, false) => (self.1 / other.1, 0.0), // (bi)/(di)=b/d
            (false, true, false, true) => (self.0 / other.0, 0.0), // (a)/(c)=a/c
            (false, false, false, true) => (self.0 / other.0, self.1 / other.0), // (a+bi)/(c)=a/c+(b/c)i
            (false, false, true, false) => (self.1 / other.1, self.0 / other.1), // (a+bi)/(di)=b/d+(a/d)i
            (false, true, false, false) =>
            // (a)/(c+di)=(ac)/(c^2+d^2)-((ad)/(c^2+d^2))i
            {
                let d = other.0 * other.0 + other.1 * other.1;
                (self.0 * other.0 / d, -self.0 * other.1 / d)
            }
            (true, false, false, false) =>
            // (bi)/(c+di)=
            {
                let d = other.0 * other.0 + other.1 * other.1;
                (self.1 * other.1 / d, self.1 * other.0 / d)
            }
            (false, false, false, false) =>
            // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2)+((bc-ad)/(c^2+d^2))i
            {
                let d = other.0 * other.0 + other.1 * other.1;
                (
                    (self.0 * other.0 + self.1 * other.1) / d,
                    (self.1 * other.0 - self.0 * other.1) / d,
                )
            }
            _ => unreachable!(),
        }
    }
    fn cos(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.cos(), 0.0)
        }
        else if self.0 == 0.0
        {
            (self.1.cosh(), 0.0)
        }
        else
        {
            (self.0.cos() * self.1.cosh(), -self.0.sin() * self.1.sinh())
        }
    }
    fn sin(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.sin(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.sinh())
        }
        else
        {
            (self.0.sin() * self.1.cosh(), self.0.cos() * self.1.sinh())
        }
    }
    fn tan(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.tan(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.tanh())
        }
        else
        {
            let c = (2.0 * self.0).cos() + (2.0 * self.1).cosh();
            ((2.0 * self.0).sin() / c, (2.0 * self.1).sinh() / c)
        }
    }
}
impl Float for (f32, f32)
{
    fn add(self, other: Self) -> Self
    {
        (self.0 + other.0, self.1 + other.1)
    }
    fn sub(self, other: Self) -> Self
    {
        (self.0 - other.0, self.1 - other.1)
    }
    fn mul(self, other: Self) -> Self
    {
        match (self.0 == 0.0, self.1 == 0.0, other.0 == 0.0, other.1 == 0.0)
        {
            (true, true, _, _) | (_, _, true, true) => (0.0, 0.0), // (0)(c+di)=0 | (a+bi)(0)=0
            (true, false, true, false) => (-self.1 * other.1, 0.0), // (bi)(di)=-bd
            (false, true, false, true) => (self.0 * other.0, 0.0), // (a)(c)=ac
            (false, false, false, true) => (self.0 * other.0, self.1 * other.0), // (a+bi)(c)=ac+bci
            (false, true, false, false) => (self.0 * other.0, self.0 * other.1), // (a)(c+di)=ac+adi
            (false, false, true, false) => (-self.1 * other.1, self.0 * other.1), // (a+bi)(di)=-bd+adi
            (true, false, false, false) => (-self.1 * other.1, self.1 * other.0), // (bi)(c+di)=-bd+bci
            (false, false, false, false) => (
                self.0 * other.0 - self.1 * other.1,
                self.0 * other.1 + self.1 * other.0,
            ), // (a+bi)(c+di)=(ac-bd)+(ad+bc)i
            _ => unreachable!(),
        }
    }
    fn div(self, other: Self) -> Self
    {
        match (self.0 == 0.0, self.1 == 0.0, other.0 == 0.0, other.1 == 0.0)
        {
            (true, true, _, _) => (0.0, 0.0), // (0)/(c+di)=0
            (_, _, true, true) => (f32::INFINITY, f32::INFINITY), // (a+bi)/(0)=inf
            (true, false, true, false) => (self.1 / other.1, 0.0), // (bi)/(di)=b/d
            (false, true, false, true) => (self.0 / other.0, 0.0), // (a)/(c)=a/c
            (false, false, false, true) => (self.0 / other.0, self.1 / other.0), // (a+bi)/(c)=a/c+(b/c)i
            (false, false, true, false) => (self.1 / other.1, self.0 / other.1), // (a+bi)/(di)=b/d+(a/d)i
            (false, true, false, false) =>
            // (a)/(c+di)=(ac)/(c^2+d^2)-((ad)/(c^2+d^2))i
            {
                let d = other.0 * other.0 + other.1 * other.1;
                ((self.0 * other.0) / d, -(self.0 * other.1) / d)
            }
            (true, false, false, false) =>
            // (bi)/(c+di)=
            {
                let d = other.0 * other.0 + other.1 * other.1;
                ((self.1 * other.1) / d, (self.1 * other.0) / d)
            }
            (false, false, false, false) =>
            // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2)+((bc-ad)/(c^2+d^2))i
            {
                let d = other.0 * other.0 + other.1 * other.1;
                (
                    (self.0 * other.0 + self.1 * other.1) / d,
                    (self.1 * other.0 - self.0 * other.1) / d,
                )
            }
            _ => unreachable!(),
        }
    }
    fn cos(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.cos(), 0.0)
        }
        else if self.0 == 0.0
        {
            (self.1.cosh(), 0.0)
        }
        else
        {
            (self.0.cos() * self.1.cosh(), -self.0.sin() * self.1.sinh())
        }
    }
    fn sin(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.sin(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.sinh())
        }
        else
        {
            (self.0.sin() * self.1.cosh(), self.0.cos() * self.1.sinh())
        }
    }
    fn tan(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.tan(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.tanh())
        }
        else
        {
            let c = (2.0 * self.0).cos() + (2.0 * self.1).cosh();
            ((2.0 * self.0).sin() / c, (2.0 * self.1).sinh() / c)
        }
    }
}
impl Float for Complex
{
    fn add(self, other: Self) -> Self
    {
        self + other
    }
    fn sub(self, other: Self) -> Self
    {
        self - other
    }
    fn mul(self, other: Self) -> Self
    {
        self * other
    }
    fn div(self, other: Self) -> Self
    {
        self / other
    }
    fn cos(self) -> Self
    {
        self.cos()
    }
    fn sin(self) -> Self
    {
        self.sin()
    }
    fn tan(self) -> Self
    {
        self.tan()
    }
}