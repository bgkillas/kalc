use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    math::do_math,
    misc::do_math_with_var,
    Options,
};
use rug::{
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    ops::Pow,
    Complex, Float, Integer,
};
use std::{
    cmp::Ordering,
    ops::{Shl, Shr},
};
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
                else if b.real().is_sign_positive()
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
        true => (a.real() / a.real().clone().abs()
            * a.real().clone().abs().pow(b.real().clone().recip()))
        .into(),
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
    let c: Complex = a - b.clone();
    let int = Integer::from(10).pow(a.prec().0 / 4);
    let re: Float = c.real().clone() * int.clone();
    let re: Float = re.round() / int.clone();
    let im: Float = c.imag().clone() * int.clone();
    let im: Float = im.round() / int;
    Complex::with_val(a.prec(), (!re.is_zero() || !im.is_zero()) as u8)
}
pub fn eq(a: &Complex, b: &Complex) -> Complex
{
    let c: Complex = a - b.clone();
    let int = Integer::from(10).pow(a.prec().0 / 4);
    let re: Float = c.real().clone() * int.clone();
    let re: Float = re.round() / int.clone();
    let im: Float = c.imag().clone() * int.clone();
    let im: Float = im.round() / int;
    Complex::with_val(a.prec(), (re.is_zero() && im.is_zero()) as u8)
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
pub fn gamma(a: Complex) -> Complex
{
    if !a.imag().is_zero()
    {
        let p = a.prec();
        incomplete_gamma(a, Complex::new(p))
    }
    else if a.real().is_sign_negative() && a.real().clone().fract().is_zero()
    {
        Complex::with_val(a.prec(), Infinity)
    }
    else
    {
        a.real().clone().gamma().into()
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
                        Complex::new(a[0].prec())
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
                    Complex::new(a[0].prec()),
                    Complex::new(a[0].prec()),
                    Complex::new(a[0].prec()),
                ]
            }
            else
            {
                vec![
                    a[2].clone().abs(),
                    Complex::new(a[0].prec()),
                    Complex::new(a[0].prec()),
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
                Complex::new(a[0].prec()),
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
    func_vars: Vec<(String, Vec<NumStr>)>,
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
            match do_math_with_var(
                function.clone(),
                options,
                func_vars.clone(),
                var,
                Num(Complex::with_val(options.prec, z)),
            )?
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
            match do_math_with_var(
                function.clone(),
                options,
                func_vars.clone(),
                var,
                Num(Complex::with_val(options.prec, z)),
            )?
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
    func_vars: Vec<(String, Vec<NumStr>)>,
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
        let math = do_math_with_var(
            function.clone(),
            options,
            func_vars.clone(),
            var,
            Num(Complex::with_val(options.prec, z)),
        )?;
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
pub fn sort(mut a: Vec<Complex>) -> Vec<Complex>
{
    a.sort_by(|x, y| x.real().partial_cmp(y.real()).unwrap_or(Ordering::Equal));
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
                false,
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
                false,
            )),
            _ => Err("unsupported"),
        }
    }
    else
    {
        Err("not square")
    }
}
pub fn quadratic(a: Complex, b: Complex, c: Complex, real: bool) -> Vec<Complex>
{
    if a.is_zero()
    {
        return vec![-c / b];
    }
    let p: Complex = b.clone().pow(2);
    let p: Complex = p - (4 * c * a.clone());
    let p = p.sqrt();
    let a: Complex = 2 * a;
    if real
    {
        let z1 = (p.clone() - b.clone()) / a.clone();
        let z2 = (-p - b) / a;
        let mut vec = Vec::new();
        if z1.imag().to_f64().abs() < 0.0000000000000001
        {
            vec.push(z1)
        }
        if z2.imag().to_f64().abs() < 0.0000000000000001
        {
            vec.push(z2)
        }
        vec
    }
    else
    {
        vec![(p.clone() - b.clone()) / a.clone(), (-p - b) / a]
    }
}
pub fn cubic(a: Complex, b: Complex, c: Complex, d: Complex, real: bool) -> Vec<Complex>
{
    if a.is_zero()
    {
        return quadratic(b, c, d, real);
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
    if real
    {
        let z1: Complex = (left.clone() - right.clone() - b.clone()) / 3;
        let z2: Complex =
            ((-omega.clone() * left.clone()) + (omega.clone().conj() * right.clone()) - b.clone())
                / 3;
        let z3: Complex = ((-omega.clone().conj() * left) + (omega * right) - b.clone()) / 3;
        let mut vec = Vec::new();
        if z1.imag().to_f64().abs() < 0.0000000000000001
        {
            vec.push(z1)
        }
        if z2.imag().to_f64().abs() < 0.0000000000000001
        {
            vec.push(z2)
        }
        if z3.imag().to_f64().abs() < 0.0000000000000001
        {
            vec.push(z3)
        }
        vec
    }
    else
    {
        vec![
            (left.clone() - right.clone() - b.clone()) / 3,
            ((-omega.clone() * left.clone()) + (omega.clone().conj() * right.clone()) - b.clone())
                / 3,
            ((-omega.clone().conj() * left) + (omega * right) - b.clone()) / 3,
        ]
    }
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
pub fn recursion(
    mut func_vars: Vec<(String, Vec<NumStr>)>,
    mut func: Vec<NumStr>,
    options: Options,
) -> Result<NumStr, &'static str>
{
    for fv in func_vars.clone()
    {
        if fv.0.ends_with(')')
        {
            if fv.0.contains(',')
            {
                let mut vars = fv.0.split(',').collect::<Vec<&str>>();
                vars[0] = vars[0].split('(').last().unwrap();
                {
                    let vl = vars.len() - 1;
                    vars[vl] = &vars[vl][0..vars[vl].len() - 1];
                }
                let mut x = func.len();
                while x > 0
                {
                    x -= 1;
                    if func[x].str_is(&fv.0)
                    {
                        let mut fv = fv.clone();
                        let mut i = 0;
                        while i < func_vars.len()
                        {
                            if vars.contains(&func_vars[i].0.as_str())
                            {
                                func_vars.remove(i);
                                continue;
                            }
                            i += 1;
                        }
                        let mut bracket = 0;
                        let mut k = 0;
                        let mut processed = Vec::new();
                        let mut last = 0;
                        for (i, n) in func[x + 2..].iter().enumerate()
                        {
                            if let Str(s) = n
                            {
                                if s == "(" || s == "{"
                                {
                                    bracket += 1
                                }
                                else if s == ")" || s == "}"
                                {
                                    if bracket == 0
                                    {
                                        if let Ok(n) = do_math(
                                            func[x + 2 + last..x + 2 + i].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )
                                        {
                                            processed.push(vec![n]);
                                        }
                                        else
                                        {
                                            let iden = format!(
                                                "@{}{}@",
                                                func_vars.len(),
                                                vars[processed.len()]
                                            );
                                            func_vars.push((
                                                iden.clone(),
                                                func[x + 2 + last..x + 2 + i].to_vec(),
                                            ));
                                            processed.push(vec![Str(iden)]);
                                        }
                                        k = i;
                                        break;
                                    }
                                    bracket -= 1;
                                }
                                else if s == "," && bracket == 0
                                {
                                    if let Ok(n) = do_math(
                                        func[x + 2 + last..x + 2 + i].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )
                                    {
                                        processed.push(vec![n]);
                                    }
                                    else
                                    {
                                        let iden = format!(
                                            "@{}{}@",
                                            func_vars.len(),
                                            vars[processed.len()]
                                        );
                                        func_vars.push((
                                            iden.clone(),
                                            func[x + 2 + last..x + 2 + i].to_vec(),
                                        ));
                                        processed.push(vec![Str(iden)]);
                                    }
                                    last = i + 1;
                                }
                            }
                        }
                        let mut i = 0;
                        while i < fv.1.len()
                        {
                            if let Str(s) = &fv.1[i]
                            {
                                for v in processed.iter().zip(vars.clone())
                                {
                                    if *s == v.1
                                    {
                                        fv.1.remove(i);
                                        fv.1.splice(i..i, v.0.clone());
                                        break;
                                    }
                                }
                            }
                            i += 1;
                        }
                        func.drain(x..=k + x + 2);
                        func.splice(x..x, fv.1.clone());
                    }
                }
            }
            else
            {
                let var = fv.0.split('(').last().unwrap();
                let var = &var[0..var.len() - 1];
                let mut x = func.len();
                while x > 0
                {
                    x -= 1;
                    if func[x].str_is(&fv.0)
                    {
                        let mut fv = fv.clone();
                        for (i, j) in func_vars.clone().iter().enumerate()
                        {
                            if j.0 == var
                            {
                                func_vars.remove(i);
                            }
                        }
                        let mut bracket = 0;
                        let mut k = 0;
                        for (i, n) in func[x + 2..].iter().enumerate()
                        {
                            if let Str(s) = n
                            {
                                if s == "(" || s == "{"
                                {
                                    bracket += 1
                                }
                                else if s == ")" || s == "}"
                                {
                                    if bracket == 0
                                    {
                                        k = i;
                                    }
                                    bracket -= 1;
                                }
                            }
                        }
                        let mut i = 0;
                        while i < func_vars.len()
                        {
                            if var == func_vars[i].0
                            {
                                func_vars.remove(i);
                                break;
                            }
                            i += 1;
                        }
                        let iden = format!("@{}{}@", func_vars.len(), var);
                        let mut i = 0;
                        while i < fv.1.len()
                        {
                            if let Str(s) = &fv.1[i]
                            {
                                if *s == var
                                {
                                    fv.1[i] = Str(iden.clone());
                                }
                            }
                            i += 1;
                        }
                        func_vars.push((iden.clone(), func[i + 2..=k + 1].to_vec()));
                        func.drain(x..=k + x + 2);
                        func.splice(x..x, fv.1.clone());
                    }
                }
            }
        }
    }
    do_math(func, options, func_vars)
}
pub fn gcd(mut x: Integer, mut y: Integer) -> Integer
{
    while x != y
    {
        if x > y
        {
            x -= y.clone()
        }
        else
        {
            y -= x.clone()
        }
    }
    x
}
//simpsons rule
// pub fn incomplete_beta(x: Complex, a: Complex, b: Complex) -> Complex
// {
//     let mut last = Complex::new(a.prec());
//     let i = 12u32;
//     let mut area = Complex::new(a.prec());
//     let n: Complex = x.clone() / 2.pow(i);
//     for k in 1..=2.pow(i)
//     {
//         let g: Complex = k * n.clone();
//         let f: Complex = 1 - g.clone();
//         let num: Complex = g.pow(a.clone()) * f.pow(b.clone());
//         let g: Complex = (k * 2 - 1) * n.clone() / 2;
//         let f: Complex = 1 - g.clone();
//         let mid: Complex = g.pow(a.clone()) * f.pow(b.clone());
//         area += (last + 4 * mid + num.clone()) * x.clone() / (3 * 2.pow(i + 1));
//         last = num;
//     }
//     area
// }
pub fn incomplete_beta(x: Complex, a: Complex, b: Complex) -> Complex
{
    if x.real() > &((a.real().clone() + 1) / (a.real() + b.real().clone() + 2))
    {
        (gamma(a.clone()) * gamma(b.clone()) / gamma(a.clone() + b.clone()))
            - incomplete_beta(1 - x, b, a)
    }
    else
    {
        let f: Complex = 1 - x.clone();
        x.clone().pow(a.clone()) * f.pow(b.clone())
            / (a.clone() * (1 + incomplete_beta_recursion(x, a, b, 1, 10)))
    }
}
fn incomplete_beta_recursion(x: Complex, a: Complex, b: Complex, iter: usize, max: usize)
    -> Complex
{
    if iter == max
    {
        Complex::new(x.prec())
    }
    else if iter % 2 == 1
    {
        let m = (iter - 1) / 2;
        (-x.clone() * (a.clone() + m) * (a.clone() + b.clone() + m)
            / ((a.clone() + (2 * m)) * (a.clone() + (2 * m) + 1)))
            / (1 + incomplete_beta_recursion(x, a, b, iter + 1, max))
    }
    else
    {
        let m = iter / 2;
        (x.clone() * m * (b.clone() - m) / ((a.clone() + (2 * m)) * (a.clone() + (2 * m) - 1)))
            / (1 + incomplete_beta_recursion(x, a, b, iter + 1, max))
    }
}
fn gamma0(z: Complex) -> Complex
{
    gamma0_recursion_first(z.clone(), 0, 100) + gamma0_recursion_second(z, 0, 100)
}
fn gamma0_recursion_first(z: Complex, iter: usize, max: usize) -> Complex
{
    if iter == max
    {
        Complex::with_val(z.prec(), 1)
    }
    else if iter == 0
    {
        Float::with_val(z.prec().0, -1).exp() / gamma0_recursion_first(z, 1, max)
    }
    else
    {
        2 * iter - z.clone() + iter * (z.clone() - iter) / gamma0_recursion_first(z, iter + 1, max)
    }
}
fn gamma0_recursion_second(z: Complex, iter: usize, max: usize) -> Complex
{
    if iter == max
    {
        Complex::with_val(z.prec(), 1)
    }
    else if iter == 0
    {
        Float::with_val(z.prec().0, -1).exp() / gamma0_recursion_second(z, 1, max)
    }
    else if iter % 2 == 1
    {
        (iter - 1) + z.clone() - (z.clone() + iter / 2) / gamma0_recursion_second(z, iter + 1, max)
    }
    else
    {
        (iter - 1) + z.clone() + (iter / 2) / gamma0_recursion_second(z, iter + 1, max)
    }
}
pub fn incomplete_gamma(s: Complex, z: Complex) -> Complex
{
    // let prec = Float::with_val(z.prec().0, 0.1).pow(z.prec().0 / 2);
    // let mut last: Complex = incomplete_gamma_recursion(s.clone(), z.clone(), 0, 1);
    // let mut num = incomplete_gamma_recursion(s.clone(), z.clone(), 0, 2);
    // for m in 3..100
    // {
    //     if (num.clone() - last.clone()).abs().real() > &prec
    //     {
    //         last = num.clone();
    //         num = incomplete_gamma_recursion(s.clone(), z.clone(), 0, m);
    //     }
    //     else
    //     {
    //         break;
    //     }
    // }
    if z.is_zero()
    {
        gamma0(s)
    }
    else
    {
        incomplete_gamma_recursion(s, z, 0, 100)
    }
}
fn incomplete_gamma_recursion(s: Complex, z: Complex, iter: usize, max: usize) -> Complex
{
    if iter == max
    {
        Complex::with_val(s.prec(), 1)
    }
    else if iter == 0
    {
        (z.clone().pow(s.clone()) / z.clone().exp()) / incomplete_gamma_recursion(s, z, 1, max)
    }
    else if iter % 2 == 1
    {
        z.clone()
            + ((iter.div_ceil(2) - s.clone()) / incomplete_gamma_recursion(s, z, iter + 1, max))
    }
    else
    {
        1 + (iter.div_ceil(2) / incomplete_gamma_recursion(s, z, iter + 1, max))
    }
}
pub fn subfactorial(z: Complex) -> Complex
{
    //let prec = Float::with_val(z.prec().0, 0.1).pow(z.prec().0 / 2);
    // let mut last: Complex = subfactorial_recursion(z.clone(), 0, 1);
    // let mut num = subfactorial_recursion(z.clone(), 0, 2);
    // for m in 3..100
    // {
    //     if (num.clone() - last.clone()).abs().real() > &prec
    //     {
    //         last = num.clone();
    //         num = subfactorial_recursion(z.clone(), 0, m);
    //     }
    //     else
    //     {
    //         break;
    //     }
    // }
    subfactorial_recursion(z.clone(), 0, 100)
        + gamma(z.clone() + 1) / Complex::with_val(z.prec(), 1).exp()
}
fn subfactorial_recursion(z: Complex, iter: usize, max: usize) -> Complex
{
    if iter == max
    {
        Complex::with_val(z.prec(), 1)
    }
    else if iter == 0
    {
        Complex::with_val(z.prec(), -1).pow(z.clone()) / subfactorial_recursion(z, 1, max)
    }
    else
    {
        (z.clone() + iter + 1) - iter / subfactorial_recursion(z, iter + 1, max)
    }
}
pub fn length(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    mut start: Complex,
    mut end: Complex,
    points: usize,
) -> Result<Complex, &'static str>
{
    if start.real() > end.real()
    {
        (start, end) = (end, start)
    }
    let delta: Complex = (end.clone() - start.clone()) / points;
    let mut x0: Complex = match slope(
        func.clone(),
        func_vars.clone(),
        options,
        var.clone(),
        start.clone(),
        false,
    )?
    {
        Num(nx0) =>
        {
            if !nx0.real().is_finite()
            {
                Complex::new(options.prec)
            }
            else
            {
                1 + nx0.pow(2)
            }
        }
        Vector(nx0) => nx0
            .iter()
            .map(|n| n.clone().pow(2))
            .fold(Complex::new(options.prec), |total, x| total + x),
        _ => return Err("not supported arc length data"),
    }
    .sqrt();
    let mut length = Complex::new(options.prec);
    let h: Complex = delta.clone() / 4;
    for i in 0..points
    {
        if i + 1 == points
        {
            start = end.clone()
        }
        else
        {
            start += delta.clone();
        }
        let x1 = slope(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            start.clone() - 3 * h.clone(),
            false,
        )?;
        let x2 = slope(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            start.clone() - 2 * h.clone(),
            false,
        )?;
        let x3 = slope(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            start.clone() - h.clone(),
            false,
        )?;
        let x4 = slope(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            start.clone(),
            false,
        )?;
        match (x1, x2, x3, x4)
        {
            (Num(nx1), Num(nx2), Num(nx3), Num(nx4)) =>
            {
                let nx1: Complex = if !nx1.real().is_finite()
                {
                    Complex::new(options.prec)
                }
                else
                {
                    1 + nx1.pow(2)
                };
                let nx2: Complex = if !nx2.real().is_finite()
                {
                    Complex::new(options.prec)
                }
                else
                {
                    1 + nx2.pow(2)
                };
                let nx3: Complex = if !nx3.real().is_finite()
                {
                    Complex::new(options.prec)
                }
                else
                {
                    1 + nx3.pow(2)
                };
                let nx4: Complex = if !nx4.real().is_finite()
                {
                    //TODO make for all and slope
                    Complex::new(options.prec)
                }
                else
                {
                    1 + nx4.pow(2)
                };
                let nx4 = nx4.sqrt();
                length += 2
                    * h.clone()
                    * (7 * (x0 + nx4.clone()) + (12 * nx2.sqrt()) + 32 * (nx1.sqrt() + nx3.sqrt()))
                    / 45;
                x0 = nx4
            }
            (Vector(nx1), Vector(nx2), Vector(nx3), Vector(nx4)) =>
            {
                let nx1: Complex = nx1
                    .iter()
                    .map(|n| n.clone().pow(2))
                    .fold(Complex::new(options.prec), |total, x| total + x)
                    .sqrt();
                let nx2: Complex = nx2
                    .iter()
                    .map(|n| n.clone().pow(2))
                    .fold(Complex::new(options.prec), |total, x| total + x)
                    .sqrt();
                let nx3: Complex = nx3
                    .iter()
                    .map(|n| n.clone().pow(2))
                    .fold(Complex::new(options.prec), |total, x| total + x)
                    .sqrt();
                let nx4 = nx4
                    .iter()
                    .map(|n| n.clone().pow(2))
                    .fold(Complex::new(options.prec), |total, x| total + x)
                    .sqrt();
                length +=
                    2 * h.clone() * (7 * (x0 + nx4.clone()) + (12 * nx2) + 32 * (nx1 + nx3)) / 45;
                x0 = nx4
            }
            (_, _, _, _) => return Err("not supported arc length data"),
        };
    }
    Ok(length)
}
//TODO ignore removables
#[allow(clippy::too_many_arguments)]
pub fn area(
    mut func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    mut start: Complex,
    end: Complex,
    points: usize,
    combine: bool,
) -> Result<NumStr, &'static str>
{
    let mut funcs = Vec::new();
    if combine && !func.is_empty() && func[0].str_is("{")
    {
        let mut brackets = 0;
        let mut last = 1;
        for (i, f) in func.iter().enumerate()
        {
            if let Str(s) = f
            {
                match s.as_str()
                {
                    "(" | "{" => brackets += 1,
                    ")" | "}" => brackets -= 1,
                    "," if brackets == 1 =>
                    {
                        funcs.push(func[last..i].to_vec());
                        last = i + 1;
                    }
                    _ =>
                    {}
                }
            }
        }
        if last != 1
        {
            func = func[last..func.len() - 1].to_vec();
        }
    }
    let mut areavec: Vec<Complex> = Vec::new();
    let div = Complex::with_val(options.prec, 0.5).pow(options.prec.0 / 2);
    let delta: Complex = (end.clone() - start.clone()) / points;
    let mut area: Complex = Complex::new(options.prec);
    let mut x0 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(start.clone()),
    )?;
    if !funcs.is_empty()
    {
        let mut nx0t = Complex::new(options.prec);
        for i in &funcs
        {
            nx0t += ((do_math_with_var(
                i.clone(),
                options,
                func_vars.clone(),
                &var,
                Num(start.clone() + div.clone()),
            )?
            .num()?
                - do_math_with_var(
                    i.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(start.clone()),
                )?
                .num()?)
                / div.clone())
            .pow(2);
        }
        x0 = Num(x0.num()? * nx0t.sqrt());
    }
    let h: Complex = delta.clone() / 4;
    for i in 0..points
    {
        if i + 1 == points
        {
            start = end.clone()
        }
        else
        {
            start += delta.clone();
        }
        let x1 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(start.clone() - 3 * h.clone()),
        )?;
        let x2 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(start.clone() - 2 * h.clone()),
        )?;
        let x3 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(start.clone() - h.clone()),
        )?;
        let x4 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(start.clone()),
        )?;
        match (x0, x1, x2, x3, x4.clone())
        {
            (Num(nx0), Num(nx1), Num(nx2), Num(nx3), Num(nx4)) if funcs.is_empty() =>
                {
                    area += 2 * h.clone() * (7 * (nx0 + nx4) + 12 * nx2 + 32 * (nx1 + nx3)) / 45;
                    x0 = x4;
                }
            (Num(nx0), Num(nx1), Num(nx2), Num(nx3), Num(nx4)) =>
                {
                    let mut nx1t = Complex::new(options.prec);
                    let mut nx2t = Complex::new(options.prec);
                    let mut nx3t = Complex::new(options.prec);
                    let mut nx4t = Complex::new(options.prec);
                    for i in &funcs
                    {
                        nx1t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(start.clone() - 3 * h.clone() + div.clone()))?
                            .num()?
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(start.clone() - 3 * h.clone()))?
                            .num()?)
                            / div.clone())
                            .pow(2);
                        nx2t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(start.clone() - 2 * h.clone() + div.clone()),
                        )?
                            .num()?
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(start.clone() - 2 * h.clone()))?
                            .num()?)
                            / div.clone())
                            .pow(2);
                        nx3t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(start.clone() - h.clone() + div.clone()),
                        )?
                            .num()?
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(start.clone() - h.clone()))?
                            .num()?)
                            / div.clone())
                            .pow(2);
                        nx4t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(start.clone() + div.clone()),
                        )?
                            .num()?
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(start.clone()))?
                            .num()?)
                            / div.clone())
                            .pow(2);
                    }
                    let x4 = nx4 * nx4t.sqrt();
                    area += 2
                        * h.clone()
                        * (7 * (nx0 + x4.clone())
                        + 12 * (nx2 * nx2t.sqrt())
                        + 32 * ((nx1 * nx1t.sqrt()) + (nx3 * nx3t.sqrt())))
                        / 45;
                    x0 = Num(x4);
                }
            (Vector(nx0), Vector(nx1), Vector(nx2), Vector(nx3), Vector(nx4))
            if areavec.is_empty() && !combine =>
                {
                    for i in 0..nx0.len()
                    {
                        areavec.push(
                            2 * h.clone()
                                * (7 * (nx0[i].clone() + nx4[i].clone())
                                + 12 * nx2[i].clone()
                                + 32 * (nx1[i].clone() + nx3[i].clone()))
                                / 45,
                        )
                    }
                    x0 = x4;
                }
            (Vector(nx0), Vector(nx1), Vector(nx2), Vector(nx3), Vector(nx4)) if !combine =>
                {
                    for (i, v) in areavec.iter_mut().enumerate()
                    {
                        *v += 2
                            * h.clone()
                            * (7 * (nx0[i].clone() + nx4[i].clone())
                            + 12 * nx2[i].clone()
                            + 32 * (nx1[i].clone() + nx3[i].clone()))
                            / 45
                    }
                    x0 = x4;
                }
            (_, _, _, _, _) => return Err("not supported area data, if parametric have the 2nd arg start and end with the { } brackets"),
        }
    }
    if areavec.is_empty()
    {
        Ok(Num(area))
    }
    else
    {
        Ok(Vector(areavec))
    }
}
pub fn slope(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    point: Complex,
    combine: bool,
) -> Result<NumStr, &'static str>
{
    let h = Complex::with_val(options.prec, 0.5).pow(options.prec.0 / 4);
    let n1 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(point.clone()),
    )?;
    let n2 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(point.clone() + h.clone()),
    )?;
    match (n1, n2)
    {
        (Num(mut n1), Num(mut n2)) =>
        {
            if n2.real().is_nan()
            {
                n2 = n1;
                n1 = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(point - h.clone()),
                )?
                .num()?;
            }
            if (n2.clone() - n1.clone()).abs().real().clone().log10() > -30
            {
                Ok(Num(Complex::with_val(options.prec, Infinity)))
            }
            else
            {
                Ok(Num((n2 - n1) / h))
            }
        }
        (Vector(n1), Vector(n2)) if !combine => Ok(Vector(
            n2.iter()
                .zip(n1)
                .map(|(f, i)| (f - i) / h.clone())
                .collect::<Vec<Complex>>(),
        )),
        (Vector(n1), Vector(n2)) if n1.len() == 1 => Ok(Num((n2[0].clone() - n1[0].clone()) / h)),
        (Vector(n1), Vector(n2)) if n1.len() == 2 => Ok(Num(
            (n2[1].clone() - n1[1].clone()) / (n2[0].clone() - n1[0].clone())
        )),
        (Vector(n1), Vector(n2)) =>
        {
            let num = n2.last().unwrap().clone() - n1.last().unwrap().clone();
            Ok(Vector(
                n1[0..n1.len() - 1]
                    .iter()
                    .zip(n2[0..n2.len() - 1].to_vec())
                    .map(|(i, f)| num.clone() / (f - i))
                    .collect::<Vec<Complex>>(),
            ))
        }
        (_, _) => Err("not supported slope data"),
    }
}
#[derive(Copy, Clone, PartialEq)]
pub enum LimSide
{
    Left,
    Right,
    Both,
}
pub fn limit(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    point: Complex,
    side: LimSide,
) -> Result<NumStr, &'static str>
{
    if point.clone().real().is_infinite()
    {
        let (h1, h2);
        let positive = point.real().is_sign_positive();
        if positive
        {
            h1 = Complex::with_val(options.prec, 2).pow(options.prec.0 / 4);
            h2 = Complex::with_val(options.prec, 2).pow((options.prec.0 / 4) as f64 + 7.0 / 0.94)
                - 3;
        }
        else
        {
            h1 = -Complex::with_val(options.prec, 2).pow(options.prec.0 / 4);
            h2 = 3 - Complex::with_val(options.prec, 2)
                .pow((options.prec.0 / 4) as f64 + 7.0 / 0.94);
        }
        let n1 = do_math_with_var(func.clone(), options, func_vars.clone(), &var, Num(h1))?;
        let n2 = do_math_with_var(func.clone(), options, func_vars.clone(), &var, Num(h2))?;
        match (n1, n2)
        {
            (Num(n1), Num(n2)) =>
            {
                if (n1.clone() - n2.clone()).abs().real().clone().log10() <= -10
                {
                    Ok(Num(Complex::with_val(options.prec, n1)))
                }
                else if n2.real().is_infinite() || n2.imag().is_infinite()
                {
                    Ok(Num(
                        match (n2.real().is_infinite(), n2.imag().is_infinite())
                        {
                            (true, true) =>
                            {
                                if n1.real().is_sign_positive()
                                {
                                    Complex::with_val(options.prec, (Infinity, Infinity))
                                }
                                else
                                {
                                    -Complex::with_val(options.prec, (Infinity, Infinity))
                                }
                            }
                            (true, false) =>
                            {
                                if n1.real().is_sign_positive()
                                {
                                    Complex::with_val(options.prec, Infinity)
                                }
                                else
                                {
                                    -Complex::with_val(options.prec, Infinity)
                                }
                            }
                            (false, true) =>
                            {
                                if n1.imag().is_sign_positive()
                                {
                                    Complex::with_val(options.prec, (0, Infinity))
                                }
                                else
                                {
                                    -Complex::with_val(options.prec, (0, Infinity))
                                }
                            }
                            (false, false) => Complex::with_val(options.prec, Nan),
                        },
                    ))
                }
                else
                {
                    let n3 = do_math_with_var(
                        func.clone(),
                        options,
                        func_vars.clone(),
                        &var,
                        Num(
                            if positive
                            {
                                Complex::with_val(options.prec, 2)
                                    .pow((options.prec.0 / 4) as f64 + 13.0 / 0.7)
                                    - 7
                            }
                            else
                            {
                                7 - Complex::with_val(options.prec, 2)
                                    .pow((options.prec.0 / 4) as f64 + 13.0 / 0.7)
                            },
                        ),
                    )?
                    .num()?;
                    let sign = n1.real().is_sign_positive() == n2.real().is_sign_positive()
                        && n2.real().is_sign_positive() == n3.real().is_sign_positive()
                        && n1.imag().is_sign_positive() == n2.imag().is_sign_positive()
                        && n2.imag().is_sign_positive() == n3.imag().is_sign_positive();
                    let n1r = n1.real().clone().abs();
                    let n2r = n2.real().clone().abs();
                    let n3r = n3.real().clone().abs();
                    let n1i = n1.imag().clone().abs();
                    let n2i = n2.imag().clone().abs();
                    let n3i = n3.imag().clone().abs();
                    Ok(Num(
                        if !sign
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            match (n3r > n2r && n2r > n1r, n3i > n2i && n2i > n1i)
                            {
                                (true, true) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                }
                                (true, false) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, Infinity)
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, Infinity)
                                    }
                                }
                                (false, true) =>
                                {
                                    if n1.imag().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, (0, Infinity))
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, (0, Infinity))
                                    }
                                }
                                (false, false) => Complex::with_val(options.prec, Nan),
                            }
                        },
                    ))
                }
            }
            (Vector(v1), Vector(v2)) =>
            {
                let mut v3: Vec<Complex> = Vec::new();
                let mut vec = Vec::with_capacity(v1.len());
                for (i, (n1, n2)) in v1.iter().zip(v2).enumerate()
                {
                    vec.push(
                        if (n1.clone() - n2.clone()).abs().real().clone().log10() <= -10
                        {
                            Complex::with_val(options.prec, n1)
                        }
                        else if n2.real().is_infinite() || n2.imag().is_infinite()
                        {
                            match (n2.real().is_infinite(), n2.imag().is_infinite())
                            {
                                (true, true) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                }
                                (true, false) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, Infinity)
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, Infinity)
                                    }
                                }
                                (false, true) =>
                                {
                                    if n1.imag().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, (0, Infinity))
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, (0, Infinity))
                                    }
                                }
                                (false, false) => Complex::with_val(options.prec, Nan),
                            }
                        }
                        else
                        {
                            if v3.is_empty()
                            {
                                v3 = do_math_with_var(
                                    func.clone(),
                                    options,
                                    func_vars.clone(),
                                    &var,
                                    Num(
                                        if positive
                                        {
                                            Complex::with_val(options.prec, 2)
                                                .pow((options.prec.0 / 4) as f64 + 13.0 / 0.7)
                                                - 7
                                        }
                                        else
                                        {
                                            7 - Complex::with_val(options.prec, 2)
                                                .pow((options.prec.0 / 4) as f64 + 13.0 / 0.7)
                                        },
                                    ),
                                )?
                                .vec()?;
                            }
                            let sign = n1.real().is_sign_positive() == n2.real().is_sign_positive()
                                && n2.real().is_sign_positive() == v3[i].real().is_sign_positive()
                                && n1.imag().is_sign_positive() == n2.imag().is_sign_positive()
                                && n2.imag().is_sign_positive() == v3[i].imag().is_sign_positive();
                            let n1r = n1.real().clone().abs();
                            let n2r = n2.real().clone().abs();
                            let n3r = v3[i].real().clone().abs();
                            let n1i = n1.imag().clone().abs();
                            let n2i = n2.imag().clone().abs();
                            let n3i = v3[i].imag().clone().abs();
                            if !sign
                            {
                                Complex::with_val(options.prec, Nan)
                            }
                            else
                            {
                                match (n3r > n2r && n2r > n1r, n3i > n2i && n2i > n1i)
                                {
                                    (true, true) =>
                                    {
                                        if n1.real().is_sign_positive()
                                        {
                                            Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                        else
                                        {
                                            -Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                    }
                                    (true, false) =>
                                    {
                                        if n1.real().is_sign_positive()
                                        {
                                            Complex::with_val(options.prec, Infinity)
                                        }
                                        else
                                        {
                                            -Complex::with_val(options.prec, Infinity)
                                        }
                                    }
                                    (false, true) =>
                                    {
                                        if n1.imag().is_sign_positive()
                                        {
                                            Complex::with_val(options.prec, (0, Infinity))
                                        }
                                        else
                                        {
                                            -Complex::with_val(options.prec, (0, Infinity))
                                        }
                                    }
                                    (false, false) => Complex::with_val(options.prec, Nan),
                                }
                            }
                        },
                    )
                }
                Ok(Vector(vec))
            }
            (_, _) => Err("unsupported lim data"),
        }
    }
    else
    {
        match side
        {
            LimSide::Left => limsided(func, func_vars, options, var, point, false),
            LimSide::Right => limsided(func, func_vars, options, var, point, true),
            LimSide::Both =>
            {
                let left = limsided(
                    func.clone(),
                    func_vars.clone(),
                    options,
                    var.clone(),
                    point.clone(),
                    false,
                )?;
                let right = limsided(func, func_vars, options, var, point, true)?;
                match (left, right)
                {
                    (Num(left), Num(right)) =>
                    {
                        if left.real().is_infinite()
                            && right.real().is_infinite()
                            && left.real().is_sign_positive() == right.real().is_sign_positive()
                            && left.imag().is_sign_positive() == right.imag().is_sign_positive()
                        {
                            Ok(Num(left))
                        }
                        else if (left.clone() - right.clone()).abs().real().clone().log10() <= -10
                        {
                            Ok(Num((left + right) / 2))
                        }
                        else
                        {
                            Ok(Num(Complex::with_val(options.prec, Nan)))
                        }
                    }
                    (Vector(left), Vector(right)) =>
                    {
                        let mut vec = Vec::with_capacity(left.len());
                        for (left, right) in left.iter().zip(right)
                        {
                            vec.push(
                                if left.real().is_infinite()
                                    && right.real().is_infinite()
                                    && left.real().is_sign_positive()
                                        == right.real().is_sign_positive()
                                    && left.imag().is_sign_positive()
                                        == right.imag().is_sign_positive()
                                {
                                    left.clone()
                                }
                                else if (left.clone() - right.clone())
                                    .abs()
                                    .real()
                                    .clone()
                                    .log10()
                                    <= -10
                                {
                                    (left - right) / 2
                                }
                                else
                                {
                                    Complex::with_val(options.prec, Nan)
                                },
                            )
                        }
                        Ok(Vector(vec))
                    }
                    (_, _) => Err("lim err"),
                }
            }
        }
    }
}
fn limsided(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    point: Complex,
    right: bool,
) -> Result<NumStr, &'static str>
{
    let h1 = Complex::with_val(options.prec, 0.5).pow((options.prec.0 / 4) as f64 + 7.0 / 0.94);
    let h2 = Complex::with_val(options.prec, 0.5).pow(options.prec.0 / 4);
    let n1 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(point.clone() + if right { h1 } else { -h1 }),
    )?;
    let n2 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(point.clone() + if right { h2 } else { -h2 }),
    )?;
    match (n1, n2)
    {
        (Num(n1), Num(n2)) => Ok(Num(
            if (n2.clone() - n1.clone()).abs().real().clone().log10() <= -10
            {
                n1
            }
            else
            {
                let h3 = Complex::with_val(options.prec, 0.5)
                    .pow((options.prec.0 / 4) as f64 + 13.0 / 0.7);
                let n3 = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(point.clone() + if right { h3 } else { -h3 }),
                )?
                .num()?;
                let sign = n1.real().is_sign_positive() == n2.real().is_sign_positive()
                    && n2.real().is_sign_positive() == n3.real().is_sign_positive()
                    && n1.imag().is_sign_positive() == n2.imag().is_sign_positive()
                    && n2.imag().is_sign_positive() == n3.imag().is_sign_positive();
                let n1r = n1.real().clone().abs();
                let n2r = n2.real().clone().abs();
                let n3r = n3.real().clone().abs();
                let n1i = n1.imag().clone().abs();
                let n2i = n2.imag().clone().abs();
                let n3i = n3.imag().clone().abs();
                if !sign
                {
                    Complex::with_val(options.prec, Nan)
                }
                else
                {
                    match (n3r > n1r && n1r > n2r, n3i > n1i && n1i > n2i)
                    {
                        (true, true) =>
                        {
                            if n1.real().is_sign_positive()
                            {
                                Complex::with_val(options.prec, (Infinity, Infinity))
                            }
                            else
                            {
                                -Complex::with_val(options.prec, (Infinity, Infinity))
                            }
                        }
                        (true, false) =>
                        {
                            if n1.real().is_sign_positive()
                            {
                                Complex::with_val(options.prec, Infinity)
                            }
                            else
                            {
                                -Complex::with_val(options.prec, Infinity)
                            }
                        }
                        (false, true) =>
                        {
                            if n1.imag().is_sign_positive()
                            {
                                Complex::with_val(options.prec, (0, Infinity))
                            }
                            else
                            {
                                -Complex::with_val(options.prec, (0, Infinity))
                            }
                        }
                        (false, false) => Complex::with_val(options.prec, Nan),
                    }
                }
            },
        )),
        (Vector(n1), Vector(n2)) =>
        {
            let mut n3: Vec<Complex> = Vec::new();
            let mut vec = Vec::with_capacity(n1.len());
            for (i, (n1, n2)) in n1.iter().zip(n2).enumerate()
            {
                vec.push(
                    if (n2.clone() - n1.clone()).abs().real().clone().log10() <= -10
                    {
                        n1.clone()
                    }
                    else
                    {
                        if n3.is_empty()
                        {
                            let h3 = Complex::with_val(options.prec, 0.5)
                                .pow((options.prec.0 / 4) as f64 + 13.0 / 0.7);
                            n3 = do_math_with_var(
                                func.clone(),
                                options,
                                func_vars.clone(),
                                &var,
                                Num(point.clone() - h3.clone()),
                            )?
                            .vec()?;
                        }
                        let sign = n1.real().is_sign_positive() == n2.real().is_sign_positive()
                            && n2.real().is_sign_positive() == n3[i].real().is_sign_positive()
                            && n1.imag().is_sign_positive() == n2.imag().is_sign_positive()
                            && n2.imag().is_sign_positive() == n3[i].imag().is_sign_positive();
                        let n1r = n1.real().clone().abs();
                        let n2r = n2.real().clone().abs();
                        let n3r = n3[i].real().clone().abs();
                        let n1i = n1.imag().clone().abs();
                        let n2i = n2.imag().clone().abs();
                        let n3i = n3[i].imag().clone().abs();
                        if !sign
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            match (n3r > n1r && n1r > n2r, n3i > n1i && n1i > n2i)
                            {
                                (true, true) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                }
                                (true, false) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, Infinity)
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, Infinity)
                                    }
                                }
                                (false, true) =>
                                {
                                    if n1.imag().is_sign_positive()
                                    {
                                        Complex::with_val(options.prec, (0, Infinity))
                                    }
                                    else
                                    {
                                        -Complex::with_val(options.prec, (0, Infinity))
                                    }
                                }
                                (false, false) => Complex::with_val(options.prec, Nan),
                            }
                        }
                    },
                )
            }
            Ok(Vector(vec))
        }
        (_, _) => Err("unsupported lim data"),
    }
}
//https://github.com/IstvanMezo/LambertW-function/blob/master/complex%20Lambert.cpp
pub fn lambertw(z: Complex, k: isize) -> Complex
{
    if z.is_zero()
    {
        return if k == 0
        {
            Complex::new(z.prec())
        }
        else
        {
            -Complex::with_val(z.prec(), Infinity)
        };
    }
    if z.imag().is_zero() && (k == 0 || k == -1)
    {
        let e: Float = -1 / Float::with_val(z.prec().0, 1).exp();
        if z.real() == &e
        {
            return Complex::with_val(z.prec(), -1);
        }
    }
    let prec = Float::with_val(z.prec().0, 0.1).pow(z.prec().0 / 2);
    let mut w = initpoint(z.clone(), k);
    let mut wprev = w.clone();
    {
        let zexp = w.clone().exp();
        let zexpz = w.clone() * zexp.clone();
        let zexpz_d = zexp.clone() + zexpz.clone();
        let zexpz_dd = (2 * zexp) + zexpz.clone();
        w -= 2 * ((zexpz.clone() - z.clone()) * zexpz_d.clone())
            / ((2 * zexpz_d.pow(2)) - ((zexpz - z.clone()) * zexpz_dd))
    }
    for _ in 0..50
    {
        if (w.clone() - wprev.clone()).abs().real() > &prec
        {
            wprev = w.clone();
            let zexp = w.clone().exp();
            let zexpz = w.clone() * zexp.clone();
            let zexpz_d = zexp.clone() + zexpz.clone();
            let zexpz_dd = (2 * zexp) + zexpz.clone();
            w -= 2 * ((zexpz.clone() - z.clone()) * zexpz_d.clone())
                / ((2 * zexpz_d.pow(2)) - ((zexpz - z.clone()) * zexpz_dd))
        }
        else
        {
            break;
        }
    }
    w
}
fn initpoint(z: Complex, k: isize) -> Complex
{
    let pi = Float::with_val(z.prec().0, Pi);
    let e = Float::with_val(z.prec().0, 1).exp();
    {
        let test: Complex = z.clone() + (1 / e.clone());
        if test.abs().real() <= &1.0001
        {
            let p1: Complex = 2 * e * z.clone() + 2;
            let p = p1.clone().sqrt();
            if k == 0
            {
                return p.clone() - (p1 / 3) + ((11 * p.pow(3)) / 72) - 1;
            }
            else if (k == 1 && z.imag() < &0) || (k == -1 && z.imag() > &0)
            {
                return -1 - p.clone() - (p1 / 3) - ((11 * p.pow(3)) / 72);
            }
        }
    }
    {
        let test: Complex = z.clone() - 0.5;
        if test.abs().real() <= &0.5001
        {
            if k == 0
            {
                return (0.35173371 * (0.1237166 + 7.061302897 * z.clone()))
                    / (2 + 0.827184 * (1 + 2 * z));
            }
            else if k == -1
            {
                return (Complex::with_val(z.prec(), (-2.2591588985, -4.22096))
                    * (Complex::with_val(z.prec(), (-14.073271, 33.767687754)) * z.clone()
                        + Complex::with_val(z.prec(), (-12.7127, 19.071643))
                            * (1 + 2 * z.clone())))
                    / (2 + Complex::with_val(z.prec(), (-17.23103, 10.629721)) * (1 + 2 * z));
            }
        }
    }
    let two_pi_k_i = Complex::with_val(z.prec(), (0, 2 * pi * k));
    let zln = z.clone().ln() + two_pi_k_i;
    zln.clone() - zln.ln()
}
