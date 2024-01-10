use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    math::do_math,
    Options,
};
use rug::{
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    ops::Pow,
    Complex, Float,
};
use std::ops::{Shl, Shr};
#[derive(Clone, PartialEq)]
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
        fn m(a: &Complex, b: &Complex) -> Complex
        {
            if a.real().is_infinite() || b.real().is_infinite()
            {
                if (a.real().is_infinite() && b.is_zero())
                    || (b.real().is_infinite() && a.is_zero())
                {
                    Complex::with_val(a.prec(), Nan)
                }
                else
                {
                    match (a.real().is_sign_positive(), b.real().is_sign_positive())
                    {
                        (true, true) | (false, false) => Complex::with_val(a.prec(), Infinity),
                        (false, true) | (true, false) => -Complex::with_val(a.prec(), Infinity),
                    }
                }
            }
            else
            {
                a * b.clone()
            }
        }
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(m(a, b)),
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
            (Matrix(a), Matrix(b))
                if a.len() == b[0].len() && (0..b.len()).all(|j| b.len() == b[j].len()) =>
            {
                Matrix(
                    a.iter()
                        .map(|a| {
                            transpose(b)
                                .unwrap()
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
                )
            }
            _ => return Err("mul err"),
        })
    }
    pub fn pm(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Vector(vec![a + b.clone(), a - b.clone()]),
            (Num(a), Vector(b)) => Vector(
                b.iter()
                    .map(|b| a + b.clone())
                    .chain(b.iter().map(|b| a - b.clone()))
                    .collect(),
            ),
            (Vector(b), Num(a)) => Vector(
                b.iter()
                    .map(|b| b + a.clone())
                    .chain(b.iter().map(|b| b - a.clone()))
                    .collect(),
            ),
            (Vector(a), Vector(b)) if a.len() == b.len() => Vector(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a + b.clone())
                    .chain(a.iter().zip(b.iter()).map(|(a, b)| a - b.clone()))
                    .collect(),
            ),
            (Matrix(a), Num(b)) | (Num(b), Matrix(a)) => Vector(
                a.iter()
                    .flat_map(|a| {
                        a.iter()
                            .map(|a| a + b.clone())
                            .chain(a.iter().map(|a| a - b.clone()))
                            .collect::<Vec<Complex>>()
                    })
                    .collect::<Vec<Complex>>(),
            ),
            _ => return Err("plus-minus unsupported"),
        })
    }
    pub fn pow(&self, b: &Self) -> Result<Self, &'static str>
    {
        fn p(a: &Complex, b: &Complex) -> Complex
        {
            if a.real().is_infinite()
            {
                if b.is_zero()
                {
                    Complex::with_val(a.prec(), Nan)
                }
                else if b.real().is_sign_negative()
                {
                    Complex::with_val(a.prec(), Infinity)
                }
                else
                {
                    Complex::new(a.prec())
                }
            }
            else if b.real().is_infinite()
            {
                if a.clone().abs() == 1
                {
                    Complex::with_val(a.prec(), Nan)
                }
                else if b.real().is_sign_positive() == a.real().clone().trunc().is_zero()
                {
                    Complex::new(a.prec())
                }
                else
                {
                    Complex::with_val(a.prec(), Infinity)
                }
            }
            else if a.is_zero() && b.real().is_zero()
            {
                Complex::with_val(a.prec(), Nan)
            }
            else
            {
                a.pow(b.clone())
            }
        }
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(p(a, b)),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| p(a, b)).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| p(a, b)).collect()),
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| p(a, b)).collect())
            }
            (Num(a), Matrix(b)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| p(a, b)).collect())
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
                        if b.real().is_sign_positive()
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
                    .map(|(a, b)| a.iter().map(|a| p(b, a)).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| p(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| p(a, b))
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),
            _ => return Err("pow err"),
        })
    }
    pub fn func<F>(&self, b: &Self, func: F) -> Result<Self, &'static str>
    where
        F: Fn(&Complex, &Complex) -> Complex,
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(func(a, b)),
            (Num(a), Vector(b)) => Vector(b.iter().map(|b| func(a, b)).collect()),
            (Vector(a), Num(b)) => Vector(a.iter().map(|a| func(a, b)).collect()),
            (Num(a), Matrix(b)) => Matrix(
                b.iter()
                    .map(|b| b.iter().map(|b| func(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Num(b)) => Matrix(
                a.iter()
                    .map(|a| a.iter().map(|a| func(a, b)).collect())
                    .collect(),
            ),
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| func(a, b)).collect())
            }
            (Vector(b), Matrix(a)) if b.len() == a.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| rem(b, a)).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.iter().map(|a| func(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a.len() == b[0].len() && a.len() == b.len() => Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(a, b)| func(a, b))
                            .collect::<Vec<Complex>>()
                    })
                    .collect(),
            ),

            _ => return Err("operation err"),
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
pub fn and(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(
        a.prec(),
        (a.imag().is_zero() && b.imag().is_zero() && a.real() == &1 && b.real() == &1) as u8,
    )
}
pub fn or(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(
        a.prec(),
        (a.imag().is_zero() && b.imag().is_zero() && (a.real() == &1 || b.real() == &1)) as u8,
    )
}
pub fn sub(a: &Complex, b: &Complex) -> Complex
{
    a - b.clone()
}
pub fn div(a: &Complex, b: &Complex) -> Complex
{
    if b.is_zero() || a.real().is_infinite()
    {
        if a.is_zero() || b.real().is_infinite()
        {
            Complex::with_val(a.prec(), Nan)
        }
        else if a.real().is_sign_positive()
        {
            Complex::with_val(a.prec(), Infinity)
        }
        else
        {
            -Complex::with_val(a.prec(), Infinity)
        }
    }
    else
    {
        a / b.clone()
    }
}
pub fn root(a: &Complex, b: &Complex) -> Complex
{
    let c: Float = b.real().clone() / 2;
    match b.imag().is_zero()
        && !c.fract().is_zero()
        && b.real().clone().fract().is_zero()
        && a.imag().is_zero()
    {
        true => Complex::with_val(
            a.prec(),
            a.real() / a.real().clone().abs()
                * a.real().clone().abs().pow(b.real().clone().recip()),
        ),
        false => a.pow(b.clone().recip()),
    }
}
pub fn add(a: &Complex, b: &Complex) -> Complex
{
    a + b.clone()
}
pub fn shl(a: &Complex, b: &Complex) -> Complex
{
    a.clone().shl(b.real().to_u32_saturating().unwrap_or(0))
}
pub fn shr(a: &Complex, b: &Complex) -> Complex
{
    a.clone().shr(b.real().to_u32_saturating().unwrap_or(0))
}
pub fn ne(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(a.prec(), (a != b) as u8)
}
pub fn eq(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(a.prec(), (a == b) as u8)
}
pub fn ge(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(a.prec(), (a.real() >= b.real()) as u8)
}
pub fn gt(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(a.prec(), (a.real() > b.real()) as u8)
}
pub fn le(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(a.prec(), (a.real() <= b.real()) as u8)
}
pub fn lt(a: &Complex, b: &Complex) -> Complex
{
    Complex::with_val(a.prec(), (a.real() < b.real()) as u8)
}
pub fn rem(a: &Complex, b: &Complex) -> Complex
{
    let c = a / b.clone();
    let c = Complex::with_val(
        a.prec(),
        (c.real().clone().floor(), c.imag().clone().floor()),
    );
    a - b * c
}
pub fn gamma(a: &Float) -> Float
{
    if a.is_sign_negative() && a.clone().fract().is_zero()
    {
        Float::with_val(a.prec(), Infinity)
    }
    else
    {
        a.clone().gamma()
    }
}
pub fn tetration(a: &Complex, b: &Complex) -> Complex
{
    if b.real().clone().fract().is_zero() && b.real().is_sign_positive()
    {
        (0..=b.real().to_f64() as usize)
            .fold(Complex::new(b.prec()), |tetration, _| a.pow(tetration))
    }
    else if b.real().is_sign_positive()
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
pub fn to_polar(mut a: Vec<Complex>, to_deg: Complex) -> Vec<Complex>
{
    if a.len() == 1
    {
        a.push(Complex::new(a[0].prec()));
    }
    if a.len() != 2 && a.len() != 3
    {
        Vec::new()
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
pub fn to(a: &NumStr, b: &NumStr) -> Result<NumStr, &'static str>
{
    Ok(match (a, b)
    {
        (Num(a), Num(b)) =>
        {
            let prec = a.prec();
            let a = a.real().to_f64() as isize;
            let b = b.real().to_f64() as isize;
            let vec: Vec<Complex> = if a < b
            {
                (a..=b).map(|a| Complex::with_val(prec, a)).collect()
            }
            else
            {
                (b..=a).rev().map(|a| Complex::with_val(prec, a)).collect()
            };
            if vec.is_empty()
            {
                return Err("start range greater then end range");
            }
            Vector(vec)
        }
        (Vector(a), Num(b)) =>
        {
            let prec = b.prec();
            let b = b.real().to_f64() as isize;
            let mat: Vec<Vec<Complex>> = a
                .iter()
                .map(|a| {
                    let a = a.real().to_f64() as isize;
                    if a < b
                    {
                        (a..=b).map(|a| Complex::with_val(prec, a)).collect()
                    }
                    else
                    {
                        (b..=a).rev().map(|a| Complex::with_val(prec, a)).collect()
                    }
                })
                .collect();
            if mat.is_empty() || mat.iter().any(|vec| vec.is_empty())
            {
                return Err("start range greater then end range");
            }
            Matrix(mat)
        }
        (Num(a), Vector(b)) =>
        {
            let prec = a.prec();
            let a = a.real().to_f64() as isize;
            let mat: Vec<Vec<Complex>> = b
                .iter()
                .map(|b| {
                    let b = b.real().to_f64() as isize;
                    if a < b
                    {
                        (a..=b).map(|a| Complex::with_val(prec, a)).collect()
                    }
                    else
                    {
                        (b..=a).rev().map(|a| Complex::with_val(prec, a)).collect()
                    }
                })
                .collect();
            if mat.is_empty() || mat.iter().any(|vec| vec.is_empty())
            {
                return Err("start range greater then end range");
            }
            Matrix(mat)
        }
        _ => return Err(".. err"),
    })
}
pub fn mvec(
    function: Vec<NumStr>,
    var: &str,
    start: isize,
    end: isize,
    mvec: bool,
    options: Options,
) -> Result<NumStr, &'static str>
{
    let mut vec = Vec::new();
    let mut mat = Vec::new();
    if start < end
    {
        for z in start..=end
        {
            let mut func = function.clone();
            for k in func.iter_mut()
            {
                if k.str_is(var)
                {
                    *k = Num(Complex::with_val(options.prec, z));
                }
            }
            let math = do_math(func, options)?;
            match math
            {
                Num(n) => vec.push(n),
                Vector(v) if mvec => vec.extend(v),
                Vector(v) => mat.push(v),
                Matrix(m) if !mvec => mat.extend(m),
                _ => return Err("cant create 3d matrix"),
            }
        }
    }
    else
    {
        for z in (end..=start).rev()
        {
            let mut func = function.clone();
            for k in func.iter_mut()
            {
                if k.str_is(var)
                {
                    *k = Num(Complex::with_val(options.prec, z));
                }
            }
            let math = do_math(func, options)?;
            match math
            {
                Num(n) => vec.push(n),
                Vector(v) if mvec => vec.extend(v),
                Vector(v) => mat.push(v),
                Matrix(m) if !mvec => mat.extend(m),
                _ => return Err("cant create 3d matrix"),
            }
        }
    }
    if mat.is_empty()
    {
        if vec.is_empty()
        {
            Err("start>end")
        }
        else
        {
            Ok(Vector(vec))
        }
    }
    else
    {
        Ok(Matrix(mat))
    }
}
pub fn sum(
    function: Vec<NumStr>,
    var: &str,
    start: isize,
    end: isize,
    product: bool,
    options: Options,
) -> Result<NumStr, &'static str>
{
    let mut value = Num(
        if product
        {
            Complex::with_val(options.prec, 1)
        }
        else
        {
            Complex::new(options.prec)
        },
    );
    for z in if start < end
    {
        start..=end
    }
    else
    {
        end..=start
    }
    {
        let mut func = function.clone();
        for k in func.iter_mut()
        {
            if k.str_is(var)
            {
                *k = Num(Complex::with_val(options.prec, z));
            }
        }
        let math = do_math(func, options)?;
        if product
        {
            value = value.mul(&math)?;
        }
        else
        {
            value = value.func(&math, add)?;
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
pub fn trace(a: &[Vec<Complex>]) -> Complex
{
    let mut n = Complex::new(a[0][0].prec());
    for (i, j) in a.iter().enumerate()
    {
        if j.len() == i
        {
            break;
        }
        n += j[i].clone();
    }
    n
}
pub fn identity(a: usize, prec: (u32, u32)) -> Vec<Vec<Complex>>
{
    let mut mat = Vec::with_capacity(a);
    for i in 0..a
    {
        let mut vec = Vec::with_capacity(a);
        for j in 0..a
        {
            if i == j
            {
                vec.push(Complex::with_val(prec, 1));
            }
            else
            {
                vec.push(Complex::new(prec));
            }
        }
        mat.push(vec);
    }
    mat
}
pub fn determinant(a: &[Vec<Complex>]) -> Result<Complex, &'static str>
{
    if !a.is_empty() && (0..a.len()).all(|j| a.len() == a[j].len())
    {
        Ok(match a.len()
        {
            1 => a[0][0].clone(),
            2 => a[0][0].clone() * a[1][1].clone() - a[1][0].clone() * a[0][1].clone(),
            3 =>
            {
                a[0][0].clone()
                    * (a[1][1].clone() * a[2][2].clone() - a[1][2].clone() * a[2][1].clone())
                    + a[0][1].clone()
                        * (a[1][2].clone() * a[2][0].clone() - a[1][0].clone() * a[2][2].clone())
                    + a[0][2].clone()
                        * (a[1][0].clone() * a[2][1].clone() - a[1][1].clone() * a[2][0].clone())
            }
            _ =>
            {
                let mut det = Complex::new(a[0][0].prec());
                for (i, x) in a[0].iter().enumerate()
                {
                    let mut sub_matrix = a[1..].to_vec();
                    for row in &mut sub_matrix
                    {
                        row.remove(i);
                    }
                    det += x * determinant(&sub_matrix)? * if i % 2 == 0 { 1.0 } else { -1.0 };
                }
                det
            }
        })
    }
    else
    {
        Err("not square")
    }
}
pub fn transpose(a: &[Vec<Complex>]) -> Result<Vec<Vec<Complex>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        let mut b = vec![vec![Complex::new(1); a.len()]; a[0].len()];
        for (i, l) in a.iter().enumerate()
        {
            for (j, n) in l.iter().enumerate()
            {
                b[j][i] = n.clone();
            }
        }
        Ok(b)
    }
    else
    {
        Err("not square")
    }
}
pub fn minors(a: &[Vec<Complex>]) -> Result<Vec<Vec<Complex>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        let mut result = vec![vec![Complex::new(1); a[0].len()]; a.len()];
        for (i, k) in result.iter_mut().enumerate()
        {
            for (j, l) in k.iter_mut().enumerate()
            {
                *l = determinant(&submatrix(a, i, j))?
            }
        }
        Ok(result)
    }
    else
    {
        Err("not square")
    }
}
pub fn cofactor(a: &[Vec<Complex>]) -> Result<Vec<Vec<Complex>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        let mut result = vec![vec![Complex::new(1); a[0].len()]; a.len()];
        for (i, k) in result.iter_mut().enumerate()
        {
            for (j, l) in k.iter_mut().enumerate()
            {
                *l = if (i + j) % 2 == 1
                {
                    -determinant(&submatrix(a, i, j))?
                }
                else
                {
                    determinant(&submatrix(a, i, j))?
                };
            }
        }
        Ok(result)
    }
    else
    {
        Err("not square")
    }
}
pub fn inverse(a: &[Vec<Complex>]) -> Result<Vec<Vec<Complex>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        Matrix(transpose(&cofactor(a)?)?)
            .func(&Num(determinant(a)?), div)?
            .mat()
    }
    else
    {
        Err("not square")
    }
}
pub fn is_prime(num: usize) -> bool
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
pub fn nth_prime(n: usize) -> usize
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
pub fn sort(mut a: Vec<Complex>) -> Vec<Complex>
{
    let mut i = 0;
    let mut dirty = false;
    loop
    {
        if i + 1 == a.len()
        {
            if dirty
            {
                i = 0;
                dirty = false;
            }
            else
            {
                break;
            }
        }
        if a[i].real() > a[i + 1].real()
        {
            dirty = true;
            a.swap(i, i + 1)
        }
        else
        {
            i += 1;
        }
    }
    a
}
pub fn eigenvalues(a: &[Vec<Complex>]) -> Result<Vec<Complex>, &'static str>
{
    if !a.is_empty() && (0..a.len()).all(|j| a.len() == a[j].len())
    {
        match a.len()
        {
            1 => Ok(a[0].clone()),
            2 => Ok(quadratic(
                Complex::with_val(a[0][0].prec(), 1),
                -a[0][0].clone() - a[1][1].clone(),
                a[0][0].clone() * a[1][1].clone() - a[0][1].clone() * a[1][0].clone(),
            )),
            3 => Ok(cubic(
                Complex::with_val(a[0][0].prec(), -1),
                a[2][2].clone() + a[1][1].clone() + a[0][0].clone(),
                -a[0][0].clone() * a[1][1].clone() - a[0][0].clone() * a[2][2].clone()
                    + a[0][1].clone() * a[1][0].clone()
                    + a[0][2].clone() * a[2][0].clone()
                    - a[1][1].clone() * a[2][2].clone()
                    + a[1][2].clone() * a[2][1].clone(),
                a[0][0].clone() * a[1][1].clone() * a[2][2].clone()
                    - a[0][0].clone() * a[1][2].clone() * a[2][1].clone()
                    - a[0][1].clone() * a[1][0].clone() * a[2][2].clone()
                    + a[0][1].clone() * a[1][2].clone() * a[2][0].clone()
                    + a[0][2].clone() * a[1][0].clone() * a[2][1].clone()
                    - a[0][2].clone() * a[1][1].clone() * a[2][0].clone(),
            )),
            _ => Err("unsupported"),
        }
    }
    else
    {
        Err("not square")
    }
}
pub fn quadratic(a: Complex, b: Complex, c: Complex) -> Vec<Complex>
{
    if a.is_zero()
    {
        return vec![-c / b];
    }
    let p: Complex = b.clone().pow(2);
    let p: Complex = p - (4 * c * a.clone());
    let p = p.sqrt();
    let a: Complex = 2 * a;
    vec![(p.clone() - b.clone()) / a.clone(), (-p - b) / a]
}
pub fn cubic(a: Complex, b: Complex, c: Complex, d: Complex) -> Vec<Complex>
{
    if a.is_zero()
    {
        return quadratic(b, c, d);
    }
    let prec = a.prec();
    let threerecip = Float::with_val(prec.0, 3).recip();
    if b.is_zero() && c.is_zero()
    {
        return if d.is_zero()
        {
            vec![Complex::new(prec), Complex::new(prec), Complex::new(prec)]
        }
        else
        {
            let reuse = (d / a).pow(threerecip.clone());
            vec![
                -reuse.clone(),
                reuse.clone() * Complex::with_val(prec, -1).pow(threerecip.clone()),
                -reuse * Complex::with_val(prec, -1).pow(2 * threerecip),
            ]
        };
    }
    let b = b / a.clone();
    let c = c / a.clone();
    let d = d / a.clone();
    let threesqrt = Float::with_val(prec.0, 3).sqrt();
    let cbrtwo = Float::with_val(prec.0, 2).pow(threerecip.clone());
    let mut reuse: Complex = (4 * b.clone().pow(3) * d.clone())
        - (b.clone().pow(2) * c.clone().pow(2))
        - (18 * b.clone() * c.clone() * d.clone())
        + (4 * c.clone().pow(3))
        + (27 * d.clone().pow(2));
    reuse = (-2 * b.clone().pow(3))
        + (3 * threesqrt.clone() * reuse.clone().sqrt())
        + (9 * b.clone() * c.clone())
        - (27 * d.clone());
    reuse = reuse.pow(threerecip.clone());
    let left: Complex = reuse.clone() / cbrtwo.clone();
    let right: Complex = cbrtwo * (3 * c.clone() - b.clone().pow(2)) / reuse.clone();
    //(-2 b^3 + 3 sqrt(3) sqrt(4 b^3 d - b^2 c^2 - 18 b c d + 4 c^3 + 27 d^2) + 9 b c - 27 d)^(1/3)/(3 2^(1/3)) - (2^(1/3) (3 c - b^2))/(3 (-2 b^3 + 3 sqrt(3) sqrt(4 b^3 d - b^2 c^2 - 18 b c d + 4 c^3 + 27 d^2) + 9 b c - 27 d)^(1/3)) - b/3
    //-((1 - i sqrt(3)) (-2 b^3 + 3 sqrt(3) sqrt(4 b^3 d - b^2 c^2 - 18 b c d + 4 c^3 + 27 d^2) + 9 b c - 27 d)^(1/3))/(6 2^(1/3)) + ((1 + i sqrt(3)) (3 c - b^2))/(3 2^(2/3) (-2 b^3 + 3 sqrt(3) sqrt(4 b^3 d - b^2 c^2 - 18 b c d + 4 c^3 + 27 d^2) + 9 b c - 27 d)^(1/3)) - b/3
    //-((1 + i sqrt(3)) (-2 b^3 + 3 sqrt(3) sqrt(4 b^3 d - b^2 c^2 - 18 b c d + 4 c^3 + 27 d^2) + 9 b c - 27 d)^(1/3))/(6 2^(1/3)) + ((1 - i sqrt(3)) (3 c - b^2))/(3 2^(2/3) (-2 b^3 + 3 sqrt(3) sqrt(4 b^3 d - b^2 c^2 - 18 b c d + 4 c^3 + 27 d^2) + 9 b c - 27 d)^(1/3)) - b/3
    let omega: Complex = (1 + (Complex::with_val(prec, (0, 1)) * threesqrt.clone())) / 2;
    vec![
        (left.clone() - right.clone() - b.clone()) / 3,
        ((-omega.clone() * left.clone()) + (omega.clone().conj() * right.clone()) - b.clone()) / 3,
        ((-omega.clone().conj() * left) + (omega * right) - b.clone()) / 3,
    ]
}
pub fn variance(a: &[Complex], prec: (u32, u32)) -> Complex
{
    let mean = a.iter().fold(Complex::new(prec), |sum, val| sum + val) / a.len();
    let mut variance = Complex::new(prec);
    for a in a
    {
        variance += (a - mean.clone()).pow(2)
    }
    variance / (a.len() - 1)
}