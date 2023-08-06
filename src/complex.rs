use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    math::inverse,
};
use rug::{ops::Pow, Complex};
#[derive(Clone)]
pub enum NumStr {
    Num(Complex),
    Str(String),
    Vector(Vec<Complex>),
    Matrix(Vec<Vec<Complex>>),
}
impl NumStr {
    pub fn mul(&self, b: &Self) -> Result<Self, &'static str> {
        Ok(match (self, b) {
            (Num(a), Num(b)) => Num(a * b.clone()),
            (Num(b), Vector(a)) | (Vector(a), Num(b)) => {
                Vector(a.iter().map(|a| a * b.clone()).collect())
            }
            (Vector(a), Vector(b)) if a.len() == b.len() => {
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
            _ => return Err("mul err"),
        })
    }
    pub fn div(&self, b: &Self) -> Result<Self, &'static str> {
        Ok(match (self, b) {
            (Num(a), Num(b)) => Num(a / b.clone()),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| a / b.clone()).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| a / b.clone()).collect()),
            (Vector(a), Vector(b)) if a.len() == b.len() => {
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
            _ => return Err("div err"),
        })
    }
    pub fn pm(&self, b: &Self) -> Result<Self, &'static str> {
        Ok(match (self, b) {
            (Num(a), Num(b)) => Vector(vec![a + b.clone(), a - b.clone()]),
            (Num(a), Vector(b)) | (Vector(b), Num(a)) => Matrix(vec![
                b.iter().map(|b| a + b.clone()).collect(),
                b.iter().map(|b| a - b.clone()).collect(),
            ]),
            (Vector(a), Vector(b)) if a.len() == b.len() => Matrix(vec![
                a.iter().zip(b.iter()).map(|(a, b)| a + b.clone()).collect(),
                a.iter().zip(b.iter()).map(|(a, b)| a - b.clone()).collect(),
            ]),
            _ => return Err("plus-minus unsupported"),
        })
    }
    pub fn add(&self, b: &Self) -> Result<Self, &'static str> {
        Ok(match (self, b) {
            (Num(a), Num(b)) => Num(a + b.clone()),
            (Num(a), Vector(b)) | (Vector(b), Num(a)) => {
                Vector(b.iter().map(|b| a + b.clone()).collect())
            }
            (Vector(a), Vector(b)) if a.len() == b.len() => {
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
            _ => return Err("add err"),
        })
    }
    pub fn sub(&self, b: &Self) -> Result<Self, &'static str> {
        Ok(match (self, b) {
            (Num(a), Num(b)) => Num(a - b.clone()),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| a - b.clone()).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| a - b.clone()).collect()),
            (Vector(a), Vector(b)) if a.len() == b.len() => {
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
            _ => return Err("sub err"),
        })
    }
    pub fn pow(&self, b: &Self) -> Result<Self, &'static str> {
        Ok(match (self, b) {
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
            (Matrix(a), Num(b)) if a.len() == a[0].len() => {
                if b.imag() == &0.0 && b.real().clone().fract() == 0.0 {
                    if b.real() == &0.0 {
                        let mut mat = Vec::new();
                        for i in 0..a.len() {
                            let mut vec = Vec::new();
                            for j in 0..a.len() {
                                vec.push(if i == j {
                                    Complex::with_val(a[0][0].prec(), 1)
                                } else {
                                    Complex::new(a[0][0].prec())
                                })
                            }
                            mat.push(vec);
                        }
                        Matrix(mat)
                    } else {
                        let mut mat = Matrix(a.clone());
                        let c = b.real().to_f64().abs() as usize;
                        for _ in 1..c {
                            mat = mat.mul(&Matrix(a.clone()))?;
                        }
                        if b.real() > &0.0 {
                            mat
                        } else {
                            Matrix(inverse(mat.mat()?)?)
                        }
                    }
                } else {
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
            _ => return Err("pow err"),
        })
    }
    pub fn str_is(&self, s: &str) -> bool {
        match self {
            Str(s2) => s == s2,
            _ => false,
        }
    }
    pub fn num(&self) -> Result<Complex, &'static str> {
        match self {
            Num(n) => Ok(n.clone()),
            _ => Err("failed to get number"),
        }
    }
    pub fn vec(&self) -> Result<Vec<Complex>, &'static str> {
        match self {
            Vector(v) => Ok(v.clone()),
            _ => Err("failed to get vector"),
        }
    }
    pub fn mat(&self) -> Result<Vec<Vec<Complex>>, &'static str> {
        match self {
            Matrix(m) => Ok(m.clone()),
            _ => Err("failed to get matrix"),
        }
    }
}
