use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    math::do_math,
    options::AngleType,
};
use rug::{
    float::{Constant::Pi, Special::Nan},
    ops::Pow,
    Complex, Float,
};
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
            (Vector(b), Matrix(a)) if a[0].len() == b.len() => Vector(
                a.iter()
                    .map(|a| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| a * b.clone())
                            .fold(Complex::new(b[0].prec()), |sum, val| sum + val)
                    })
                    .collect::<Vec<Complex>>(),
            ),
            (Matrix(a), Vector(b)) if a[0].len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a * b.clone()).collect::<Vec<Complex>>())
                    .collect::<Vec<Vec<Complex>>>(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() => Matrix(
                a.iter()
                    .map(|a| {
                        transpose(b)
                            .iter()
                            .map(|b| {
                                a.iter()
                                    .zip(b.iter())
                                    .map(|(a, b)| a * b.clone())
                                    .fold(Complex::new(a[0].prec()), |sum, val| sum + val)
                            })
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("mul err"),
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
            (Vector(b), Matrix(a)) if b.len() == a.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a / b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a / b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| a / b.clone())
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("div err"),
        })
    }
    pub fn pm(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Vector(vec![a + b.clone(), a - b.clone()]),
            (Num(a), Vector(b)) => Matrix(vec![
                b.iter().map(|b| a + b.clone()).collect(),
                b.iter().map(|b| a - b.clone()).collect(),
            ]),
            (Vector(b), Num(a)) => Matrix(vec![
                b.iter().map(|b| b + a.clone()).collect(),
                b.iter().map(|b| b - a.clone()).collect(),
            ]),
            (Vector(a), Vector(b)) if a.len() == b.len() => Matrix(vec![
                a.iter().zip(b.iter()).map(|(a, b)| a + b.clone()).collect(),
                a.iter().zip(b.iter()).map(|(a, b)| a - b.clone()).collect(),
            ]),
            (Matrix(a), Num(b)) | (Num(b), Matrix(a)) => Matrix(
                if a.len() == a[0].len()
                {
                    a.iter()
                        .map(|a| {
                            a.iter()
                                .map(|a| a + b.clone())
                                .chain(a.iter().map(|a| a - b.clone()))
                                .collect::<Vec<Complex>>()
                        })
                        .collect::<Vec<Vec<Complex>>>()
                }
                else
                {
                    a.iter()
                        .map(|a| a.iter().map(|a| a + b.clone()).collect::<Vec<Complex>>())
                        .chain(
                            a.iter()
                                .map(|a| a.iter().map(|a| a - b.clone()).collect::<Vec<Complex>>()),
                        )
                        .collect::<Vec<Vec<Complex>>>()
                },
            ),
            _ => return Err("plus-minus unsupported"),
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
            (Vector(b), Matrix(a)) | (Matrix(a), Vector(b)) if b.len() == a.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a + b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| a.clone() + b)
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("add err"),
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
            (Vector(b), Matrix(a)) if b.len() == a.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a - b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a - b.clone()).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| a - b.clone())
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("sub err"),
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
            (Matrix(a), Num(b)) if a.len() == a[0].len() =>
            {
                if b.imag().is_zero() && b.real().clone().fract().is_zero()
                {
                    if b.real().is_zero()
                    {
                        let mut mat = Vec::new();
                        for i in 0..a.len()
                        {
                            let mut vec = Vec::new();
                            for j in 0..a.len()
                            {
                                vec.push(
                                    if i == j
                                    {
                                        Complex::with_val(a[0][0].prec(), 1)
                                    }
                                    else
                                    {
                                        Complex::new(a[0][0].prec())
                                    },
                                )
                            }
                            mat.push(vec);
                        }
                        Matrix(mat)
                    }
                    else
                    {
                        let mut mat = Matrix(a.clone());
                        let c = b.real().to_f64().abs() as usize;
                        for _ in 1..c
                        {
                            mat = mat.mul(&Matrix(a.clone()))?;
                        }
                        if b.real() > &0
                        {
                            mat
                        }
                        else
                        {
                            Matrix(inverse(&mat.mat()?)?)
                        }
                    }
                }
                else
                {
                    return Err("no imag/fractional support for powers");
                }
            }
            (Vector(b), Matrix(a)) if b.len() == a.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| b.pow(a.clone())).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| a.pow(b.clone())).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| a.pow(b.clone()))
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("pow err"),
        })
    }
    pub fn tetration(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(tetration(a, b)),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| tetration(a, b)).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| tetration(a, b)).collect()),
            (Num(a), Matrix(b)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| tetration(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Num(b)) => Matrix(
                a.iter()
                    .map(|a| a.iter().map(|a| tetration(a, b)).collect())
                    .collect(),
            ),
            (Vector(a), Vector(b)) if a.len() == b.len() => Vector(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| tetration(a, b))
                    .collect(),
            ),
            (Vector(b), Matrix(a)) if b.len() == a.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| tetration(b, a)).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| tetration(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| tetration(a, b))
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),

            _ => return Err("tetration err"),
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
pub fn hyperoperation(n: &Float, a: &Complex, b: &Complex) -> Complex
{
    if n == &0
    {
        b.clone() + 1
    }
    else if n == &1 && b == &0
    {
        a.clone()
    }
    else if n == &2 && b == &0
    {
        Complex::new(a.prec())
    }
    else if n >= &3 && b == &0
    {
        Complex::with_val(a.prec(), 1)
    }
    else
    {
        hyperoperation(&(n.clone() - 1), a, &hyperoperation(n, a, &(b.clone() - 1)))
    }
}
fn tetration(a: &Complex, b: &Complex) -> Complex
{
    if b.real().clone().fract().is_zero()
    {
        (0..=b.real().to_f64() as usize)
            .fold(Complex::new(b.prec()), |tetration, _| a.pow(tetration))
    }
    else if b.real() > &0
    {
        a.pow(tetration(a, &(b.clone() - 1)))
    }
    else if b.real() <= &-1
    {
        tetration(a, &(b.clone() + 1)).ln() / a.clone().ln()
    }
    else
    {
        let a = a.clone().ln();
        1 + (2 * b.clone() * a.clone() / (1 + a.clone()))
            - (b.clone().pow(2) * (1 - a.clone()) / (1 + a))
    }
}
pub fn slog(a: &Complex, b: &Complex) -> Complex
{
    if b.real() <= &0
    {
        let z = &a.clone().pow(b);
        if z.real() <= b.real()
        {
            Complex::with_val(a.prec(), Nan)
        }
        else
        {
            slog(a, z) - 1
        }
    }
    else if b.real() > &1
    {
        let z = &(b.clone().ln() / a.clone().ln());
        if z.real() >= b.real()
        {
            Complex::with_val(a.prec(), Nan)
        }
        else
        {
            slog(a, z) + 1
        }
    }
    else
    {
        let a = a.clone().ln();
        (2 * a.clone() * b.clone() / (1 + a.clone()))
            + (b.clone().pow(2) * (1 - a.clone()) / (1 + a))
            - 1
    }
}
pub fn to_polar(a: Vec<Complex>, to_deg: Complex) -> Vec<Complex>
{
    let mut a = a;
    if a.len() == 1
    {
        a.push(Complex::new(a[0].prec()));
    }
    if a.len() != 2 && a.len() != 3
    {
        vec![]
    }
    else if a.len() == 2
    {
        if a[1].is_zero()
        {
            if a[0].is_zero()
            {
                vec![Complex::new(a[0].prec()), Complex::new(a[0].prec())]
            }
            else
            {
                vec![
                    a[0].clone().abs(),
                    if a[0].real().is_sign_positive()
                    {
                        Complex::with_val(a[0].prec(), 0)
                    }
                    else
                    {
                        to_deg * Complex::with_val(a[0].prec(), Pi)
                    },
                ]
            }
        }
        else
        {
            let mut n: Complex = a[0].clone().pow(2) + a[1].clone().pow(2);
            n = n.sqrt();
            vec![
                n.clone(),
                a[1].clone() / a[1].clone().abs() * (&a[0] / n).acos() * to_deg,
            ]
        }
    }
    else if a[1].is_zero()
    {
        if a[0].is_zero()
        {
            if a[2].is_zero()
            {
                vec![
                    Complex::with_val(a[0].prec(), 0),
                    Complex::with_val(a[0].prec(), 0),
                    Complex::with_val(a[0].prec(), 0),
                ]
            }
            else
            {
                vec![
                    a[2].clone().abs(),
                    Complex::with_val(a[0].prec(), 0),
                    Complex::with_val(a[0].prec(), 0),
                ]
            }
        }
        else
        {
            let mut n: Complex = a[0].clone().pow(2) + a[1].clone().pow(2) + a[2].clone().pow(2);
            n = n.sqrt();
            vec![
                n.clone(),
                (&a[2] / n).acos() * to_deg.clone(),
                Complex::with_val(a[0].prec(), 0),
            ]
        }
    }
    else
    {
        let mut n: Complex = a[0].clone().pow(2) + a[1].clone().pow(2) + a[2].clone().pow(2);
        n = n.sqrt();
        let t: Complex = a[0].clone().pow(2) + a[1].clone().pow(2);
        vec![
            n.clone(),
            (&a[2] / n).acos() * to_deg.clone(),
            a[1].clone() / a[1].clone().abs() * (&a[0] / t.sqrt()).acos() * to_deg,
        ]
    }
}
pub fn mvec(
    function: Vec<NumStr>,
    var: &str,
    start: usize,
    end: usize,
    mvec: bool,
    deg: AngleType,
    prec: u32,
) -> Result<NumStr, &'static str>
{
    let mut vec = Vec::new();
    let mut mat = Vec::new();
    let mut func;
    for z in start..end + 1
    {
        func = function.clone();
        for k in func.iter_mut()
        {
            if k.str_is(var)
            {
                *k = Num(Complex::with_val(prec, z));
            }
        }
        let math = do_math(func, deg, prec)?;
        match math
        {
            Num(n) => vec.push(n),
            Vector(v) if mvec => vec.extend(v),
            Vector(v) => mat.push(v),
            Matrix(m) if !mvec => mat.extend(m),
            _ => return Err("cant create 3d matrix"),
        }
    }
    if mat.is_empty()
    {
        Ok(Vector(vec))
    }
    else
    {
        Ok(Matrix(mat))
    }
}
pub fn sum(
    function: Vec<NumStr>,
    var: &str,
    start: usize,
    end: usize,
    product: bool,
    deg: AngleType,
    prec: u32,
) -> Result<NumStr, &'static str>
{
    let mut func = function.clone();
    let mut math;
    for k in func.iter_mut()
    {
        if k.str_is(var)
        {
            *k = Num(Complex::with_val(prec, start));
        }
    }
    let mut value = do_math(func, deg, prec)?;
    for z in start + 1..=end
    {
        func = function.clone();
        for k in func.iter_mut()
        {
            if k.str_is(var)
            {
                *k = Num(Complex::with_val(prec, z));
            }
        }
        math = do_math(func, deg, prec)?;
        if !product
        {
            value = value.add(&math)?;
        }
        else
        {
            value = value.mul(&math)?;
        }
    }
    Ok(value)
}
pub fn submatrix(a: &[Vec<Complex>], row: usize, col: usize) -> Vec<Vec<Complex>>
{
    a.iter()
        .enumerate()
        .filter(|&(i, _)| i != row)
        .map(|(_, r)| {
            r.iter()
                .enumerate()
                .filter(|&(j, _)| j != col)
                .map(|(_, value)| value.clone())
                .collect::<Vec<Complex>>()
        })
        .collect()
}
pub fn determinant(a: &[Vec<Complex>]) -> Complex
{
    if a.len() == 1
    {
        a[0][0].clone()
    }
    else if a.len() == 2
    {
        a[0][0].clone() * a[1][1].clone() - a[1][0].clone() * a[0][1].clone()
    }
    else if a.len() == 3
    {
        a[0][0].clone() * (a[1][1].clone() * a[2][2].clone() - a[1][2].clone() * a[2][1].clone())
            + a[0][1].clone()
                * (a[1][2].clone() * a[2][0].clone() - a[1][0].clone() * a[2][2].clone())
            + a[0][2].clone()
                * (a[1][0].clone() * a[2][1].clone() - a[1][1].clone() * a[2][0].clone())
    }
    else
    {
        let mut det = Complex::new(a[0][0].prec());
        for (i, x) in a[0].iter().enumerate()
        {
            let mut sub_matrix = a[1..].to_vec();
            for row in &mut sub_matrix
            {
                row.remove(i);
            }
            det += x * determinant(&sub_matrix) * if i % 2 == 0 { 1.0 } else { -1.0 };
        }
        det
    }
}
pub fn transpose(a: &[Vec<Complex>]) -> Vec<Vec<Complex>>
{
    let mut b = vec![vec![Complex::new(1); a.len()]; a[0].len()];
    for (i, l) in a.iter().enumerate()
    {
        for (j, n) in l.iter().enumerate()
        {
            b[j][i] = n.clone();
        }
    }
    b
}
pub fn minors(a: &[Vec<Complex>]) -> Vec<Vec<Complex>>
{
    let mut result = vec![vec![Complex::new(1); a[0].len()]; a.len()];
    for (i, k) in result.iter_mut().enumerate()
    {
        for (j, l) in k.iter_mut().enumerate()
        {
            *l = determinant(&submatrix(a, i, j));
        }
    }
    result
}
pub fn cofactor(a: &[Vec<Complex>]) -> Vec<Vec<Complex>>
{
    let mut result = vec![vec![Complex::new(1); a[0].len()]; a.len()];
    for (i, k) in result.iter_mut().enumerate()
    {
        for (j, l) in k.iter_mut().enumerate()
        {
            *l = if (i + j) % 2 == 1
            {
                -determinant(&submatrix(a, i, j))
            }
            else
            {
                determinant(&submatrix(a, i, j))
            };
        }
    }
    result
}
pub fn inverse(a: &[Vec<Complex>]) -> Result<Vec<Vec<Complex>>, &'static str>
{
    if a.len() == a[0].len() && a.len() > 1
    {
        Matrix(transpose(&cofactor(a)))
            .div(&Num(determinant(a)))?
            .mat()
    }
    else
    {
        Err("not square")
    }
}
pub fn is_prime(num: u128) -> bool
{
    if num <= 1
    {
        return false;
    }
    if num <= 3
    {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0
    {
        return false;
    }
    let mut i = 5;
    while i * i <= num
    {
        if num % i == 0 || num % (i + 2) == 0
        {
            return false;
        }
        i += 6;
    }
    true
}
pub fn nth_prime(n: u128) -> u128
{
    let mut count = 0;
    let mut num = 2;
    if n == 0
    {
        num = 0
    }
    while count < n
    {
        if is_prime(num)
        {
            count += 1;
        }
        if count < n
        {
            num += 1;
        }
    }
    num
}