use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    math::do_math,
    misc::do_math_with_var,
    Number, Options, Units,
};
use rug::{
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    ops::Pow,
    Complex, Float, Integer,
};
use std::cmp::Ordering;
#[derive(Clone, PartialEq)]
pub enum NumStr
{
    Num(Number),
    Str(String),
    Vector(Vec<Number>),
    Matrix(Vec<Vec<Number>>),
}
impl Number
{
    pub fn from(number: Complex, units: Option<Units>) -> Number
    {
        Self { number, units }
    }
}
pub fn add(a: &Number, b: &Number) -> Number
{
    Number::from(
        a.number.clone() + b.number.clone(),
        if a.units == b.units { a.units } else { None },
    )
}
pub fn sub(a: &Number, b: &Number) -> Number
{
    Number::from(
        a.number.clone() - b.number.clone(),
        if a.units == b.units { a.units } else { None },
    )
}
impl NumStr
{
    pub fn mul(&self, b: &Self) -> Result<Self, &'static str>
    {
        fn m(a: &Number, b: &Number) -> Number
        {
            Number::from(
                {
                    let a = a.number.clone();
                    let b = b.number.clone();
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
                                (true, true) | (false, false) =>
                                {
                                    Complex::with_val(a.prec(), Infinity)
                                }
                                (false, true) | (true, false) =>
                                {
                                    -Complex::with_val(a.prec(), Infinity)
                                }
                            }
                        }
                    }
                    else
                    {
                        a * b.clone()
                    }
                },
                match (a.units, b.units)
                {
                    (Some(a), Some(b)) => Some(a.mul(&b)),
                    (Some(a), None) | (None, Some(a)) => Some(a),
                    (None, None) => None,
                },
            )
        }
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Num(m(a, b)),
            (Num(b), Vector(a)) | (Vector(a), Num(b)) =>
            {
                Vector(a.iter().map(|a| m(a, b)).collect())
            }
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                Vector(a.iter().zip(b.iter()).map(|(a, b)| m(a, b)).collect())
            }
            (Num(b), Matrix(a)) | (Matrix(a), Num(b)) => Matrix(
                a.iter()
                    .map(|a| a.iter().map(|a| m(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Vector(b)) if a[0].len() == b.len() => Vector(
                a.iter()
                    .map(|a| {
                        let mut iter = a.iter().zip(b.iter()).map(|(a, b)| m(a, b));
                        let mut sum = iter.next().unwrap();
                        for val in iter
                        {
                            sum = add(&sum, &val)
                        }
                        sum
                    })
                    .collect::<Vec<Number>>(),
            ),
            (Vector(a), Matrix(b)) if a.len() == b.len() => Vector(
                transpose(b)
                    .iter()
                    .map(|b| {
                        let mut iter = b.iter().zip(a.iter()).map(|(a, b)| m(a, b));
                        let mut sum = iter.next().unwrap();
                        for val in iter
                        {
                            sum = add(&sum, &val)
                        }
                        sum
                    })
                    .collect::<Vec<Number>>(),
            ),
            (Matrix(a), Matrix(b)) if a[0].len() == b.len() => Matrix(
                a.iter()
                    .map(|a| {
                        transpose(b)
                            .iter()
                            .map(|b| {
                                let mut iter = a.iter().zip(b.iter()).map(|(a, b)| m(a, b));
                                let mut sum = iter.next().unwrap();
                                for val in iter
                                {
                                    sum = add(&sum, &val)
                                }
                                sum
                            })
                            .collect::<Vec<Number>>()
                    })
                    .collect(),
            ),
            _ => return Err("mul err"),
        })
    }
    pub fn pm(&self, b: &Self) -> Result<Self, &'static str>
    {
        Ok(match (self, b)
        {
            (Num(a), Num(b)) => Vector(vec![add(a, b), sub(a, b)]),
            (Num(a), Vector(b)) => Vector(
                b.iter()
                    .map(|b| add(a, b))
                    .chain(b.iter().map(|b| sub(a, b)))
                    .collect(),
            ),
            (Vector(b), Num(a)) => Vector(
                b.iter()
                    .map(|b| add(a, b))
                    .chain(b.iter().map(|b| sub(a, b)))
                    .collect(),
            ),
            (Vector(a), Vector(b)) if a.len() == b.len() => Vector(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| add(a, b))
                    .chain(a.iter().zip(b.iter()).map(|(a, b)| sub(a, b)))
                    .collect(),
            ),
            (Matrix(a), Num(b)) | (Num(b), Matrix(a)) => Vector(
                a.iter()
                    .flat_map(|a| {
                        a.iter()
                            .map(|a| add(a, b))
                            .chain(a.iter().map(|a| sub(a, b)))
                            .collect::<Vec<Number>>()
                    })
                    .collect::<Vec<Number>>(),
            ),
            _ => return Err("plus-minus unsupported"),
        })
    }
    pub fn pow(&self, b: &Self) -> Result<Self, &'static str>
    {
        fn p(a: &Number, b: &Number) -> Number
        {
            Number::from(
                {
                    let a = a.number.clone();
                    let b = b.number.clone();
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
                },
                match (a.units, b.units)
                {
                    (Some(a), None) => Some(a.pow(b.number.real().to_f64())),
                    _ => None,
                },
            )
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
                let b = b.number.clone();
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
                                        Number::from(
                                            Complex::with_val(a[0][0].number.prec(), 1),
                                            None,
                                        )
                                    }
                                    else
                                    {
                                        Number::from(Complex::new(a[0][0].number.prec()), None)
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
                        let c = b
                            .real()
                            .clone()
                            .abs()
                            .to_integer()
                            .unwrap_or_default()
                            .to_usize()
                            .unwrap_or_default();
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
                    return Err("no imag/fractional support for matrix powers");
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
                            .collect::<Vec<Number>>()
                    })
                    .collect(),
            ),
            _ => return Err("pow err"),
        })
    }
    pub fn func<F>(&self, b: &Self, func: F) -> Result<Self, &'static str>
    where
        F: Fn(&Number, &Number) -> Number,
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
                    .map(|(a, b)| a.iter().map(|a| func(b, a)).collect())
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
                            .collect::<Vec<Number>>()
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
    pub fn num(&self) -> Result<Number, &'static str>
    {
        match self
        {
            Num(n) => Ok(n.clone()),
            _ => Err("failed to get number"),
        }
    }
    pub fn vec(&self) -> Result<Vec<Number>, &'static str>
    {
        match self
        {
            Vector(v) => Ok(v.clone()),
            _ => Err("failed to get vector"),
        }
    }
    pub fn mat(&self) -> Result<Vec<Vec<Number>>, &'static str>
    {
        match self
        {
            Matrix(m) => Ok(m.clone()),
            _ => Err("failed to get matrix"),
        }
    }
}
pub fn and(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    Number::from(
        Complex::with_val(
            a.prec(),
            (a.imag().is_zero() && b.imag().is_zero() && a.real() == &1 && b.real() == &1) as u8,
        ),
        None,
    )
}
pub fn or(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    Number::from(
        Complex::with_val(
            a.prec(),
            (a.imag().is_zero() && b.imag().is_zero() && (a.real() == &1 || b.real() == &1)) as u8,
        ),
        None,
    )
}
pub fn div(a: &Number, b: &Number) -> Number
{
    Number::from(
        {
            let a = a.number.clone();
            let b = b.number.clone();
            if b.is_zero() || a.real().is_infinite()
            {
                if a.is_zero() || b.real().is_infinite()
                {
                    Complex::with_val(a.prec(), Nan)
                }
                else if a.real().is_sign_positive() == b.real().is_sign_positive()
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
        },
        match (a.units, b.units)
        {
            (Some(a), Some(b)) => Some(a.div(&b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(Units::default().div(&b)),
            (None, None) => None,
        },
    )
}
pub fn root(a: &Number, b: &Number) -> Number
{
    Number::from(
        {
            let a = a.number.clone();
            let b = b.number.clone();
            let c: Float = b.real().clone() / 2;
            match b.imag().is_zero()
                && !c.fract().is_zero()
                && b.real().clone().fract().is_zero()
                && a.imag().is_zero()
            {
                true if a.real().is_sign_positive() => a.pow(b.real().clone().recip()),
                true => -(-a).pow(b.real().clone().recip()),
                false => a.pow(b.clone().recip()),
            }
        },
        match (a.units, b.units)
        {
            (Some(a), None) => Some(a.root(b.number.real().to_f64())),
            _ => None,
        },
    )
}
pub fn unity(y: Complex, x: Complex) -> Vec<Number>
{
    if x.real().is_zero()
    {
        return Vec::new();
    }
    let mut vec: Vec<Number> = Vec::new();
    let taui: Complex = 2 * Complex::with_val(x.prec(), (0, Pi));
    let r: Float = x.imag().clone().pow(2) / 2;
    let r: Float = x.real().clone() / 2 + r / x.real().clone();
    let n = (x.imag().clone() * y.real() / x.real() - y.imag()) / taui.imag().clone();
    let left: Float = -r.clone() + n.clone();
    let right: Float = r + n;
    for k in if left < right
    {
        left.clone()
            .trunc()
            .to_integer()
            .unwrap_or_default()
            .to_i128()
            .unwrap_or_default()
            ..=if left.clone().fract().is_zero() && right.clone().fract().is_zero()
            {
                right
                    .clone()
                    .trunc()
                    .to_integer()
                    .unwrap_or_default()
                    .to_i128()
                    .unwrap_or_default()
                    - 1
            }
            else
            {
                right
                    .clone()
                    .trunc()
                    .to_integer()
                    .unwrap_or_default()
                    .to_i128()
                    .unwrap_or_default()
            }
    }
    else
    {
        right
            .clone()
            .trunc()
            .to_integer()
            .unwrap_or_default()
            .to_i128()
            .unwrap_or_default()
            ..=if left.clone().fract().is_zero() && right.clone().fract().is_zero()
            {
                left.clone()
                    .trunc()
                    .to_integer()
                    .unwrap_or_default()
                    .to_i128()
                    .unwrap_or_default()
                    - 1
            }
            else
            {
                left.clone()
                    .trunc()
                    .to_integer()
                    .unwrap_or_default()
                    .to_i128()
                    .unwrap_or_default()
            }
    }
    {
        let r: Complex = (y.clone() + k * taui.clone()) / x.clone();
        let r: Complex = r.exp();
        vec.push(Number::from(r, None))
    }
    vec
}
pub fn shl(a: &Number, b: &Number) -> Number
{
    Number::from(
        a.number.clone() * Complex::with_val(a.number.prec(), 2).pow(b.number.clone()),
        None,
    )
}
pub fn shr(a: &Number, b: &Number) -> Number
{
    Number::from(
        a.number.clone() * Complex::with_val(a.number.prec(), 2).pow(-b.number.clone()),
        None,
    )
}
pub fn ne(a: &Number, b: &Number) -> Number
{
    let ua = a.units;
    let ub = b.units;
    let a = a.number.clone();
    let b = b.number.clone();
    let c: Complex = a.clone() - b.clone();
    let int = Integer::from(10).pow(a.prec().0 / 4);
    let re: Float = c.real().clone() * int.clone();
    let re: Float = re.round() / int.clone();
    let im: Float = c.imag().clone() * int.clone();
    let im: Float = im.round() / int;
    Number::from(
        Complex::with_val(
            a.prec(),
            ((!(re.is_zero()
                || (a.real().is_infinite()
                    && b.real().is_infinite()
                    && a.real().is_sign_positive() == b.real().is_sign_positive()))
                || !(im.is_zero()
                    || (a.imag().is_infinite()
                        && b.imag().is_infinite()
                        && a.imag().is_sign_positive() == b.imag().is_sign_positive())))
                || match (ua, ub)
                {
                    (Some(a), Some(b)) => a != b,
                    (Some(a), None) | (None, Some(a)) => !a.is_none(),
                    (None, None) => false,
                }) as u8,
        ),
        None,
    )
}
pub fn eq(a: &Number, b: &Number) -> Number
{
    let ua = a.units;
    let ub = b.units;
    let a = a.number.clone();
    let b = b.number.clone();
    let c: Complex = a.clone() - b.clone();
    let int = Integer::from(10).pow(a.prec().0 / 4);
    let re: Float = c.real().clone() * int.clone();
    let re: Float = re.round() / int.clone();
    let im: Float = c.imag().clone() * int.clone();
    let im: Float = im.round() / int;
    Number::from(
        Complex::with_val(
            a.prec(),
            ((re.is_zero()
                || (a.real().is_infinite()
                    && b.real().is_infinite()
                    && a.real().is_sign_positive() == b.real().is_sign_positive())
                    && im.is_zero()
                || (a.imag().is_infinite()
                    && b.imag().is_infinite()
                    && a.imag().is_sign_positive() == b.imag().is_sign_positive()))
                && match (ua, ub)
                {
                    (Some(a), Some(b)) => a == b,
                    (Some(a), None) | (None, Some(a)) => a.is_none(),
                    (None, None) => true,
                }) as u8,
        ),
        None,
    )
}
pub fn ge(a: &Number, b: &Number) -> Number
{
    Number::from(
        Complex::with_val(a.number.prec(), (a.number.real() >= b.number.real()) as u8),
        None,
    )
}
pub fn gt(a: &Number, b: &Number) -> Number
{
    Number::from(
        Complex::with_val(a.number.prec(), (a.number.real() > b.number.real()) as u8),
        None,
    )
}
pub fn rem(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    let c = a.clone() / b.clone();
    let c = Complex::with_val(
        a.prec(),
        (c.real().clone().floor(), c.imag().clone().floor()),
    );
    Number::from(a - b * c, None)
}
pub fn digamma(mut z: Complex, mut n: u32) -> Complex
{
    n += 1;
    let op = z.prec().0 / 2;
    let prec = n * op;
    z.set_prec(prec);
    let h: Complex = Complex::with_val(prec, 0.5).pow(op / 2);
    let num = Integer::from(n);
    let mut sum = Complex::new(prec);
    for k in 0..=n
    {
        if k % 2 == 0
        {
            sum += num.clone().binomial(k) * gamma(z.clone() + h.clone() * (n - k)).ln()
        }
        else
        {
            sum -= num.clone().binomial(k) * gamma(z.clone() + h.clone() * (n - k)).ln()
        }
    }
    sum * Complex::with_val(prec, 2).pow(op / 2 * n)
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
pub fn tetration(a: &Number, b: &Number) -> Number
{
    let a = a.number.clone();
    let b = b.number.clone();
    Number::from(
        if b.real().clone().fract().is_zero()
        {
            if b.real().is_sign_positive()
            {
                (1..b
                    .real()
                    .to_integer()
                    .unwrap_or_default()
                    .to_usize()
                    .unwrap_or_default())
                    .fold(a.clone(), |tetration, _| a.clone().pow(tetration))
            }
            else if b == -1
            {
                Complex::with_val(a.prec(), 0)
            }
            else
            {
                Complex::with_val(a.prec(), (Infinity, Nan))
            }
        }
        else
        {
            tetration_recursion(a.clone(), b.clone())
        },
        None,
    )
}
fn tetration_recursion(a: Complex, b: Complex) -> Complex
{
    if b.real().is_sign_positive()
    {
        a.clone().pow(tetration_recursion(a, b - 1))
    }
    else if b.real() <= &-1
    {
        tetration_recursion(a.clone(), b + 1).ln() / a.ln()
    }
    else
    {
        let aln = a.clone().ln();
        1 + b.clone() * (aln.clone() * (2 + b.clone()) - b) / (1 + aln)
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
pub fn atan(a: Complex, b: Complex) -> Complex
{
    if a.imag().is_zero() && b.imag().is_zero()
    {
        Complex::with_val(a.prec(), (a.real(), b.real())).arg()
    }
    else
    {
        let i = Complex::with_val(a.prec(), (0, 1));
        let abs: Complex = a.clone().pow(2) + b.clone().pow(2);
        -i.clone() * ((a + b * i) / abs.sqrt()).ln()
    }
}
pub fn to_polar(mut a: Vec<Number>, to_deg: Complex) -> Vec<Number>
{
    if a.len() == 1
    {
        a.push(Number::from(Complex::new(a[0].number.prec()), None));
    }
    if a.len() != 2 && a.len() != 3
    {
        Vec::new()
    }
    else if a.len() == 2
    {
        if a[1].number.is_zero()
        {
            if a[0].number.is_zero()
            {
                vec![
                    Number::from(Complex::new(a[0].number.prec()), None),
                    Number::from(Complex::new(a[0].number.prec()), None),
                ]
            }
            else
            {
                vec![
                    Number::from(a[0].number.clone().abs(), a[0].units),
                    Number::from(
                        if a[0].number.real().is_sign_positive()
                        {
                            Complex::new(a[0].number.prec())
                        }
                        else
                        {
                            to_deg * Float::with_val(a[0].number.prec().0, Pi)
                        },
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
                ]
            }
        }
        else
        {
            let mut n: Complex = a[0].number.clone().pow(2) + a[1].number.clone().pow(2);
            n = n.sqrt();
            vec![
                Number::from(n.clone(), a[0].units),
                Number::from(
                    atan(a[0].number.clone(), a[1].number.clone()) * to_deg,
                    Some(Units {
                        angle: 1.0,
                        ..Units::default()
                    }),
                ),
            ]
        }
    }
    else if a[1].number.is_zero()
    {
        if a[0].number.is_zero()
        {
            if a[2].number.is_zero()
            {
                vec![
                    Number::from(Complex::new(a[0].number.prec()), None),
                    Number::from(Complex::new(a[0].number.prec()), None),
                    Number::from(Complex::new(a[0].number.prec()), None),
                ]
            }
            else
            {
                vec![
                    Number::from(a[2].number.clone().abs(), a[2].units),
                    Number::from(
                        Complex::new(a[0].number.prec()),
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
                    Number::from(
                        Complex::new(a[0].number.prec()),
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
                ]
            }
        }
        else
        {
            let nxy: Complex = a[0].number.clone().pow(2) + a[1].number.clone().pow(2);
            let mut n: Complex = nxy.clone() + a[2].number.clone().pow(2);
            n = n.sqrt();
            vec![
                Number::from(n.clone(), a[0].units),
                Number::from(
                    atan(a[2].number.clone(), nxy.sqrt()) * to_deg.clone(),
                    Some(Units {
                        angle: 1.0,
                        ..Units::default()
                    }),
                ),
                Number::from(
                    Complex::new(a[0].number.prec()),
                    Some(Units {
                        angle: 1.0,
                        ..Units::default()
                    }),
                ),
            ]
        }
    }
    else
    {
        let nxy: Complex = a[0].number.clone().pow(2) + a[1].number.clone().pow(2);
        let mut n: Complex = nxy.clone() + a[2].number.clone().pow(2);
        n = n.sqrt();
        vec![
            Number::from(n.clone(), a[0].units),
            Number::from(
                atan(a[2].number.clone(), nxy.sqrt()) * to_deg.clone(),
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            Number::from(
                atan(a[0].number.clone(), a[1].number.clone()) * to_deg.clone(),
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
        ]
    }
}
pub fn to(a: &NumStr, b: &NumStr) -> Result<NumStr, &'static str>
{
    Ok(match (a, b)
    {
        (Num(a), Num(b)) =>
        {
            let prec = a.number.prec();
            let a = a
                .number
                .real()
                .to_integer()
                .unwrap_or_default()
                .to_isize()
                .unwrap_or_default();
            let b = b
                .number
                .real()
                .to_integer()
                .unwrap_or_default()
                .to_isize()
                .unwrap_or_default();
            let vec: Vec<Number> = if a < b
            {
                (a..=b)
                    .map(|a| Number::from(Complex::with_val(prec, a), None))
                    .collect()
            }
            else
            {
                (b..=a)
                    .rev()
                    .map(|a| Number::from(Complex::with_val(prec, a), None))
                    .collect()
            };
            if vec.is_empty()
            {
                return Err("start range greater then end range");
            }
            Vector(vec)
        }
        (Vector(a), Num(b)) =>
        {
            let prec = b.number.prec();
            let b = b
                .number
                .real()
                .to_integer()
                .unwrap_or_default()
                .to_isize()
                .unwrap_or_default();
            let mat: Vec<Vec<Number>> = a
                .iter()
                .map(|a| {
                    let a = a
                        .number
                        .real()
                        .to_integer()
                        .unwrap_or_default()
                        .to_isize()
                        .unwrap_or_default();
                    if a < b
                    {
                        (a..=b)
                            .map(|a| Number::from(Complex::with_val(prec, a), None))
                            .collect()
                    }
                    else
                    {
                        (b..=a)
                            .rev()
                            .map(|a| Number::from(Complex::with_val(prec, a), None))
                            .collect()
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
            let prec = a.number.prec();
            let a = a
                .number
                .real()
                .to_integer()
                .unwrap_or_default()
                .to_isize()
                .unwrap_or_default();
            let mat: Vec<Vec<Number>> = b
                .iter()
                .map(|b| {
                    let b = b
                        .number
                        .real()
                        .to_integer()
                        .unwrap_or_default()
                        .to_isize()
                        .unwrap_or_default();
                    if a < b
                    {
                        (a..=b)
                            .map(|a| Number::from(Complex::with_val(prec, a), None))
                            .collect()
                    }
                    else
                    {
                        (b..=a)
                            .rev()
                            .map(|a| Number::from(Complex::with_val(prec, a), None))
                            .collect()
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
                Num(Number::from(Complex::with_val(options.prec, z), None)),
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
                Num(Number::from(Complex::with_val(options.prec, z), None)),
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
    let mut value = do_math_with_var(
        function.clone(),
        options,
        func_vars.clone(),
        var,
        Num(Number::from(
            Complex::with_val(options.prec, if start < end { start } else { end }),
            None,
        )),
    )?;
    for z in if start < end
    {
        start + 1..=end
    }
    else
    {
        end + 1..=start
    }
    {
        let math = do_math_with_var(
            function.clone(),
            options,
            func_vars.clone(),
            var,
            Num(Number::from(Complex::with_val(options.prec, z), None)),
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
pub fn submatrix(a: &[Vec<Number>], row: usize, col: usize) -> Vec<Vec<Number>>
{
    a.iter()
        .enumerate()
        .filter(|&(i, _)| i != row)
        .map(|(_, r)| {
            r.iter()
                .enumerate()
                .filter(|&(j, _)| j != col)
                .map(|(_, value)| value.clone())
                .collect::<Vec<Number>>()
        })
        .collect()
}
pub fn trace(a: &[Vec<Number>]) -> Complex
{
    let mut n = Complex::new(a[0][0].number.prec());
    for (i, j) in a.iter().enumerate()
    {
        if j.len() == i
        {
            break;
        }
        n += j[i].number.clone();
    }
    n
}
pub fn identity(a: usize, prec: u32) -> Vec<Vec<Number>>
{
    let mut mat = Vec::with_capacity(a);
    for i in 0..a
    {
        let mut vec = Vec::with_capacity(a);
        for j in 0..a
        {
            if i == j
            {
                vec.push(Number::from(Complex::with_val(prec, 1), None));
            }
            else
            {
                vec.push(Number::from(Complex::new(prec), None));
            }
        }
        mat.push(vec);
    }
    mat
}
pub fn determinant(a: &[Vec<Number>]) -> Result<Number, &'static str>
{
    if !a.is_empty() && (0..a.len()).all(|j| a.len() == a[j].len())
    {
        Ok(Number::from(
            match a.len()
            {
                1 => a[0][0].number.clone(),
                2 =>
                {
                    a[0][0].number.clone() * a[1][1].number.clone()
                        - a[1][0].number.clone() * a[0][1].number.clone()
                }
                3 =>
                {
                    a[0][0].number.clone()
                        * (a[1][1].number.clone() * a[2][2].number.clone()
                            - a[1][2].number.clone() * a[2][1].number.clone())
                        + a[0][1].number.clone()
                            * (a[1][2].number.clone() * a[2][0].number.clone()
                                - a[1][0].number.clone() * a[2][2].number.clone())
                        + a[0][2].number.clone()
                            * (a[1][0].number.clone() * a[2][1].number.clone()
                                - a[1][1].number.clone() * a[2][0].number.clone())
                }
                _ =>
                {
                    let mut det = Complex::new(a[0][0].number.prec());
                    for (i, x) in a[0].iter().enumerate()
                    {
                        let mut sub_matrix = a[1..].to_vec();
                        for row in &mut sub_matrix
                        {
                            row.remove(i);
                        }
                        det += x.number.clone()
                            * determinant(&sub_matrix)?.number
                            * if i % 2 == 0 { 1.0 } else { -1.0 };
                    }
                    det
                }
            },
            None,
        ))
    }
    else
    {
        Err("not square")
    }
}
pub fn transpose(a: &[Vec<Number>]) -> Vec<Vec<Number>>
{
    let mut b = vec![vec![Number::from(Complex::new(1), None); a.len()]; a[0].len()];
    for (i, l) in a.iter().enumerate()
    {
        for (j, n) in l.iter().enumerate()
        {
            b[j][i].clone_from(n);
        }
    }
    b
}
pub fn minors(a: &[Vec<Number>]) -> Result<Vec<Vec<Number>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        let mut result = vec![vec![Number::from(Complex::new(1), None); a[0].len()]; a.len()];
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
pub fn cofactor(a: &[Vec<Number>]) -> Result<Vec<Vec<Number>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        let mut result = vec![vec![Number::from(Complex::new(1), None); a[0].len()]; a.len()];
        for (i, k) in result.iter_mut().enumerate()
        {
            for (j, l) in k.iter_mut().enumerate()
            {
                *l = Number::from(
                    if (i + j) % 2 == 1
                    {
                        -determinant(&submatrix(a, i, j))?.number
                    }
                    else
                    {
                        determinant(&submatrix(a, i, j))?.number
                    },
                    None,
                );
            }
        }
        Ok(result)
    }
    else
    {
        Err("not square")
    }
}
pub fn inverse(a: &[Vec<Number>]) -> Result<Vec<Vec<Number>>, &'static str>
{
    if (0..a.len()).all(|j| a.len() == a[j].len())
    {
        Matrix(transpose(&cofactor(a)?))
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
pub fn sort(mut a: Vec<Number>) -> Vec<Number>
{
    a.sort_by(|x, y| {
        x.number
            .real()
            .partial_cmp(y.number.real())
            .unwrap_or(Ordering::Equal)
    });
    a
}
pub fn sort_mat(mut a: Vec<Vec<Number>>) -> Vec<Vec<Number>>
{
    a.sort_by(|x, y| {
        x[0].number
            .real()
            .partial_cmp(y[0].number.real())
            .unwrap_or(Ordering::Equal)
    });
    a
}
pub fn eigenvalues(mat: &[Vec<Number>], real: bool) -> Result<NumStr, &'static str>
{
    if !mat.is_empty() && (0..mat.len()).all(|j| mat.len() == mat[j].len())
    {
        match mat.len()
        {
            1 => Ok(Num(mat[0][0].clone())),
            2 => Ok(Vector(quadratic(
                Complex::with_val(mat[0][0].number.prec(), 1),
                -mat[0][0].number.clone() - mat[1][1].number.clone(),
                mat[0][0].number.clone() * mat[1][1].number.clone()
                    - mat[0][1].number.clone() * mat[1][0].number.clone(),
                real,
            ))),
            3 => Ok(Vector(cubic(
                Complex::with_val(mat[0][0].number.prec(), -1),
                mat[2][2].number.clone() + mat[1][1].number.clone() + mat[0][0].number.clone(),
                -mat[0][0].number.clone() * mat[1][1].number.clone()
                    - mat[0][0].number.clone() * mat[2][2].number.clone()
                    + mat[0][1].number.clone() * mat[1][0].number.clone()
                    + mat[0][2].number.clone() * mat[2][0].number.clone()
                    - mat[1][1].number.clone() * mat[2][2].number.clone()
                    + mat[1][2].number.clone() * mat[2][1].number.clone(),
                mat[0][0].number.clone() * mat[1][1].number.clone() * mat[2][2].number.clone()
                    - mat[0][0].number.clone()
                        * mat[1][2].number.clone()
                        * mat[2][1].number.clone()
                    - mat[0][1].number.clone()
                        * mat[1][0].number.clone()
                        * mat[2][2].number.clone()
                    + mat[0][1].number.clone()
                        * mat[1][2].number.clone()
                        * mat[2][0].number.clone()
                    + mat[0][2].number.clone()
                        * mat[1][0].number.clone()
                        * mat[2][1].number.clone()
                    - mat[0][2].number.clone()
                        * mat[1][1].number.clone()
                        * mat[2][0].number.clone(),
                real,
            ))),
            4 =>
            {
                let a = mat[0][0].number.clone();
                let b = mat[0][1].number.clone();
                let c = mat[0][2].number.clone();
                let d = mat[0][3].number.clone();
                let e = mat[1][0].number.clone();
                let f = mat[1][1].number.clone();
                let g = mat[1][2].number.clone();
                let h = mat[1][3].number.clone();
                let i = mat[2][0].number.clone();
                let j = mat[2][1].number.clone();
                let k = mat[2][2].number.clone();
                let l = mat[2][3].number.clone();
                let m = mat[3][0].number.clone();
                let n = mat[3][1].number.clone();
                let p = mat[3][2].number.clone();
                let q = mat[3][3].number.clone();
                Ok(Vector(quartic(
                    Complex::with_val(a.prec(), 1),
                    -a.clone() - f.clone() - k.clone() - q.clone(),
                    a.clone() * f.clone() + a.clone() * k.clone() + a.clone() * q.clone()
                        - b.clone() * e.clone()
                        - c.clone() * i.clone()
                        - d.clone() * m.clone()
                        + f.clone() * k.clone()
                        + f.clone() * q.clone()
                        - g.clone() * j.clone()
                        - h.clone() * n.clone()
                        + k.clone() * q.clone()
                        - l.clone() * p.clone(),
                    -a.clone() * f.clone() * k.clone() - a.clone() * f.clone() * q.clone()
                        + a.clone() * g.clone() * j.clone()
                        + a.clone() * h.clone() * n.clone()
                        - a.clone() * k.clone() * q.clone()
                        + a.clone() * l.clone() * p.clone()
                        + b.clone() * e.clone() * k.clone()
                        + b.clone() * e.clone() * q.clone()
                        - b.clone() * g.clone() * i.clone()
                        - b.clone() * h.clone() * m.clone()
                        - c.clone() * e.clone() * j.clone()
                        + c.clone() * f.clone() * i.clone()
                        + c.clone() * i.clone() * q.clone()
                        - c.clone() * l.clone() * m.clone()
                        - d.clone() * e.clone() * n.clone()
                        + d.clone() * f.clone() * m.clone()
                        - d.clone() * i.clone() * p.clone()
                        + d.clone() * k.clone() * m.clone()
                        - f.clone() * k.clone() * q.clone()
                        + f.clone() * l.clone() * p.clone()
                        + g.clone() * j.clone() * q.clone()
                        - g.clone() * l.clone() * n.clone()
                        - h.clone() * j.clone() * p.clone()
                        + h.clone() * k.clone() * n.clone(),
                    a.clone() * f.clone() * k.clone() * q.clone()
                        - a.clone() * f.clone() * l.clone() * p.clone()
                        - a.clone() * g.clone() * j.clone() * q.clone()
                        + a.clone() * g.clone() * l.clone() * n.clone()
                        + a.clone() * h.clone() * j.clone() * p.clone()
                        - a.clone() * h.clone() * k.clone() * n.clone()
                        - b.clone() * e.clone() * k.clone() * q.clone()
                        + b.clone() * e.clone() * l.clone() * p.clone()
                        + b.clone() * g.clone() * i.clone() * q.clone()
                        - b.clone() * g.clone() * l.clone() * m.clone()
                        - b.clone() * h.clone() * i.clone() * p.clone()
                        + b.clone() * h.clone() * k.clone() * m.clone()
                        + c.clone() * e.clone() * j.clone() * q.clone()
                        - c.clone() * e.clone() * l.clone() * n.clone()
                        - c.clone() * f.clone() * i.clone() * q.clone()
                        + c.clone() * f.clone() * l.clone() * m.clone()
                        + c.clone() * h.clone() * i.clone() * n.clone()
                        - c.clone() * h.clone() * j.clone() * m.clone()
                        - d.clone() * e.clone() * j.clone() * p.clone()
                        + d.clone() * e.clone() * k.clone() * n.clone()
                        + d.clone() * f.clone() * i.clone() * p.clone()
                        - d.clone() * f.clone() * k.clone() * m.clone()
                        - d.clone() * g.clone() * i.clone() * n.clone()
                        + d.clone() * g.clone() * j.clone() * m.clone(),
                    real,
                )))
            }
            _ => Err("unsupported"),
        }
    }
    else
    {
        Err("not square")
    }
}
pub fn eigenvectors(mat: &[Vec<Number>], real: bool) -> Result<NumStr, &'static str>
{
    if !mat.is_empty() && (0..mat.len()).all(|j| mat.len() == mat[j].len())
    {
        let one = Number::from(Complex::with_val(mat[0][0].number.prec(), 1), None);
        match mat.len()
        {
            1 => Ok(Num(one)),
            2 => Ok(Matrix(
                quadratic(
                    -mat[1][0].number.clone(),
                    mat[0][0].number.clone() - mat[1][1].number.clone(),
                    mat[0][1].number.clone(),
                    real,
                )
                .iter()
                .rev()
                .map(|n| vec![n.clone(), one.clone()])
                .collect::<Vec<Vec<Number>>>(),
            )),
            3 =>
            {
                let l = eigenvalues(mat, real).unwrap().vec().unwrap();
                //x=(b(l-i)+hc)/(h(l-a)+bg)
                //y=(dx+f)/(l-e)
                let a = mat[0][0].number.clone();
                let b = mat[0][1].number.clone();
                let c = mat[0][2].number.clone();
                let d = mat[1][0].number.clone();
                let e = mat[1][1].number.clone();
                let f = mat[1][2].number.clone();
                let g = mat[2][0].number.clone();
                let h = mat[2][1].number.clone();
                let i = mat[2][2].number.clone();
                Ok(Matrix(
                    l.iter()
                        .map(|l| {
                            let l = l.number.clone();
                            let mut x = b.clone() * (l.clone() - i.clone()) + h.clone() * c.clone();
                            if !x.is_zero()
                            {
                                x /= h.clone() * (l.clone() - a.clone()) + b.clone() * g.clone()
                            };
                            let mut y = d.clone() * x.clone() + f.clone();
                            if !y.is_zero()
                            {
                                y /= l.clone() - e.clone();
                            }
                            vec![Number::from(x, None), Number::from(y, None), one.clone()]
                        })
                        .collect::<Vec<Vec<Number>>>(),
                ))
            }
            4 =>
            {
                let w = eigenvalues(mat, real).unwrap().vec().unwrap();
                let a = mat[0][0].number.clone();
                let b = mat[0][1].number.clone();
                let c = mat[0][2].number.clone();
                let d = mat[0][3].number.clone();
                let e = mat[1][0].number.clone();
                let f = mat[1][1].number.clone();
                let g = mat[1][2].number.clone();
                let h = mat[1][3].number.clone();
                let i = mat[2][0].number.clone();
                let j = mat[2][1].number.clone();
                let k = mat[2][2].number.clone();
                let l = mat[2][3].number.clone();
                let m = mat[3][0].number.clone();
                let n = mat[3][1].number.clone();
                let o = mat[3][2].number.clone();
                let p = mat[3][3].number.clone();
                Ok(Matrix(
                    w.iter()
                        .map(|w| {
                            let w = w.number.clone();
                            let v1 = o.clone() * (w.clone() - f.clone()) + g.clone() * n.clone();
                            let v2 = w.clone() - k.clone();
                            let mut x: Complex = v1.clone() * v2.clone() * d.clone()
                                + c.clone()
                                    * (j.clone() * o.clone() * h.clone()
                                        + j.clone() * g.clone() * (w.clone() - p.clone())
                                        + v1.clone() * l.clone())
                                + v2.clone()
                                    * b.clone()
                                    * (o.clone() * h.clone() + g.clone() * (w.clone() - p.clone()));
                            if !x.is_zero()
                            {
                                x /= v1.clone() * v2.clone() * (w.clone() - a.clone())
                                    - c.clone() * o.clone() * e.clone() * j.clone()
                                    + g.clone() * m.clone() * c.clone() * j.clone()
                                    - v2.clone() * b.clone() * o.clone() * e.clone()
                                    + v2.clone() * b.clone() * g.clone() * m.clone()
                                    - c.clone() * v1.clone() * i.clone();
                            }
                            let mut y: Complex = o.clone() * (h.clone() + e.clone() * x.clone())
                                + g.clone() * (w.clone() - p.clone() - m.clone() * x.clone());
                            if !y.is_zero()
                            {
                                y /= v1.clone();
                            }
                            let mut z = w.clone()
                                - p.clone()
                                - m.clone() * x.clone()
                                - n.clone() * y.clone();
                            if !z.is_zero()
                            {
                                z /= o.clone();
                            }
                            vec![
                                Number::from(x, None),
                                Number::from(y, None),
                                Number::from(z, None),
                                one.clone(),
                            ]
                        })
                        .collect::<Vec<Vec<Number>>>(),
                ))
            }
            _ => Err("unsupported"),
        }
    }
    else
    {
        Err("not square")
    }
}
pub fn quadratic(a: Complex, b: Complex, c: Complex, real: bool) -> Vec<Number>
{
    if a.is_zero()
    {
        return if b.is_zero()
        {
            vec![Number::from(Complex::new(a.prec()), None)]
        }
        else
        {
            let mut r = -c / b;
            if -r.imag().clone().abs().log10() > a.prec().0 / 4
            {
                r = r.real().clone().into();
            }
            if real && !r.imag().is_zero()
            {
                vec![Number::from(Complex::with_val(a.prec(), Nan), None)]
            }
            else
            {
                vec![Number::from(r, None)]
            }
        };
    }
    let p: Complex = b.clone().pow(2);
    let p: Complex = p - (4 * c * a.clone());
    let p = p.sqrt();
    let a: Complex = 2 * a;
    let mut z1 = (p.clone() - b.clone()) / a.clone();
    let mut z2 = (-p - b) / a.clone();
    if -z1.imag().clone().abs().log10() > a.prec().0 / 4
    {
        z1 = z1.real().clone().into();
    }
    if -z2.imag().clone().abs().log10() > a.prec().0 / 4
    {
        z2 = z2.real().clone().into();
    }
    if real
    {
        let mut vec = Vec::new();
        if z1.imag().is_zero()
        {
            vec.push(Number::from(z1, None))
        }
        if z2.imag().is_zero()
        {
            vec.push(Number::from(z2, None))
        }
        if vec.is_empty()
        {
            vec![Number::from(Complex::with_val(a.prec(), Nan), None)]
        }
        else
        {
            vec
        }
    }
    else
    {
        vec![Number::from(z1, None), Number::from(z2, None)]
    }
}
pub fn cubic(a: Complex, b: Complex, c: Complex, d: Complex, real: bool) -> Vec<Number>
{
    if a.is_zero()
    {
        return quadratic(b, c, d, real);
    }
    let prec = a.prec();
    let threerecip = Float::with_val(prec.0, 3).recip();
    if d.is_zero()
    {
        let mut vec = quadratic(a, b, c, real);
        vec.push(Number::from(Complex::new(d.prec()), None));
        return vec;
    }
    if b.is_zero() && c.is_zero()
    {
        let reuse = (d / a.clone()).pow(threerecip.clone());
        let mut z1 = -reuse.clone();
        let mut z2 = reuse.clone() * Complex::with_val(prec, -1).pow(threerecip.clone());
        let mut z3: Complex = -reuse * Complex::with_val(prec, -1).pow(2 * threerecip);
        if -z1.imag().clone().abs().log10() > a.prec().0 / 4
        {
            z1 = z1.real().clone().into();
        }
        if -z2.imag().clone().abs().log10() > a.prec().0 / 4
        {
            z2 = z2.real().clone().into();
        }
        if -z3.imag().clone().abs().log10() > a.prec().0 / 4
        {
            z3 = z3.real().clone().into();
        }
        return if real
        {
            let mut vec = Vec::new();
            if z1.imag().is_zero()
            {
                vec.push(Number::from(z1, None))
            }
            if z2.imag().is_zero()
            {
                vec.push(Number::from(z2, None))
            }
            if z3.imag().is_zero()
            {
                vec.push(Number::from(z3, None))
            }
            if vec.is_empty()
            {
                vec![Number::from(Complex::with_val(prec, Nan), None)]
            }
            else
            {
                vec
            }
        }
        else
        {
            vec![
                Number::from(z1, None),
                Number::from(z2, None),
                Number::from(z3, None),
            ]
        };
    }
    let b = b / a.clone();
    let c = c / a.clone();
    let d = d / a.clone();
    // https://en.wikipedia.org/wiki/Cubic_equation#General_cubic_formula
    let d0: Complex = b.clone().pow(2) - 3 * c.clone();
    let d1: Complex = 2 * b.clone().pow(3) - 9 * b.clone() * c.clone() + 27 * d.clone();
    let c: Complex = d1.clone().pow(2) - 4 * d0.clone().pow(3);
    let c: Complex = (d1 + c.sqrt()) / 2;
    let c = c.pow(threerecip.clone());
    let omega: Complex = Complex::with_val(prec, (-0.5, Float::with_val(prec.0, 3).sqrt() / 2));
    let mut z1: Complex = if d0.is_zero()
    {
        -(b.clone() + c.clone()) / 3
    }
    else
    {
        -(b.clone() + c.clone() + d0.clone() / c.clone()) / 3
    };
    let c0 = c.clone() * omega.clone();
    let mut z2: Complex = if d0.is_zero()
    {
        -(b.clone() + c0.clone()) / 3
    }
    else
    {
        -(b.clone() + c0.clone() + d0.clone() / c0) / 3
    };
    let c1 = c * omega.conj();
    let mut z3: Complex = if d0.is_zero()
    {
        -(b + c1.clone()) / 3
    }
    else
    {
        -(b + c1.clone() + d0 / c1) / 3
    };
    if -z1.imag().clone().abs().log10() > a.prec().0 / 4
    {
        z1 = z1.real().clone().into();
    }
    if -z2.imag().clone().abs().log10() > a.prec().0 / 4
    {
        z2 = z2.real().clone().into();
    }
    if -z3.imag().clone().abs().log10() > a.prec().0 / 4
    {
        z3 = z3.real().clone().into();
    }
    if real
    {
        let mut vec = Vec::new();
        if z1.imag().is_zero()
        {
            vec.push(Number::from(z1, None))
        }
        if z2.imag().is_zero()
        {
            vec.push(Number::from(z2, None))
        }
        if z3.imag().is_zero()
        {
            vec.push(Number::from(z3, None))
        }
        if vec.is_empty()
        {
            vec![Number::from(Complex::with_val(prec, Nan), None)]
        }
        else
        {
            vec
        }
    }
    else
    {
        vec![
            Number::from(z1, None),
            Number::from(z2, None),
            Number::from(z3, None),
        ]
    }
}
pub fn quartic(
    div: Complex,
    b: Complex,
    c: Complex,
    d: Complex,
    e: Complex,
    real: bool,
) -> Vec<Number>
{
    if e.is_zero()
    {
        let mut vec = cubic(div, b, c, d, real);
        vec.push(Number::from(Complex::new(e.prec()), None));
        return vec;
    }
    // https://upload.wikimedia.org/wikipedia/commons/9/99/Quartic_Formula.svg
    if div.is_zero()
    {
        return cubic(b, c, d, e, real);
    }
    let prec = div.prec();
    let threerecip = Float::with_val(prec.0, 3).recip();
    let a = b / div.clone();
    let b = c / div.clone();
    let c = d / div.clone();
    let d = e / div;
    let alpha: Complex = b.clone().pow(2) - 3 * a.clone() * c.clone() + 12 * d.clone();
    let phi: Complex = 2 * b.clone().pow(3) - 9 * a.clone() * b.clone() * c.clone()
        + 27 * c.clone().pow(2)
        + 27 * a.clone().pow(2) * d.clone()
        - 72 * b.clone() * d.clone();

    let omega: Complex = -4 * alpha.clone().pow(3) + phi.clone().pow(2);
    let omega: Complex = phi + omega.sqrt();

    let alpha: Complex = if alpha.is_zero()
    {
        Complex::new(prec)
    }
    else
    {
        Complex::with_val(prec, 2).pow(threerecip.clone()) * alpha
            / (3 * omega.clone().pow(threerecip.clone()))
    };

    let beta: Complex = omega / 54;
    let beta: Complex = beta.pow(threerecip.clone());

    let infirst: Complex = a.clone().pow(2) / 4 - 2 * b.clone() / 3 + alpha.clone() + beta.clone();

    let first: Complex = infirst.clone().sqrt() / 2;

    let third: Complex = -1 * a.clone().pow(3) + 4 * a.clone() * b.clone() - 8 * c.clone();
    let third: Complex = if third.is_zero()
    {
        Complex::new(prec)
    }
    else
    {
        third / (first.clone() * 8)
    };

    let a4: Complex = -a.clone() / 4;

    let second: Complex = a.clone().pow(2) / 2 - 4 * b.clone() / 3 - alpha.clone() - beta.clone();

    let secondn: Complex = second.clone() - third.clone();
    let secondn: Complex = secondn.sqrt() / 2;

    let secondp: Complex = second + third.clone();
    let secondp: Complex = secondp.sqrt() / 2;
    let mut r1 = a4.clone() - first.clone() - secondn.clone();
    let mut r2 = a4.clone() - first.clone() + secondn.clone();
    let mut r3 = a4.clone() + first.clone() - secondp.clone();
    let mut r4 = a4.clone() + first.clone() + secondp.clone();
    if -r1.imag().clone().abs().log10() > a.prec().0 / 4
    {
        r1 = r1.real().clone().into();
    }
    if -r2.imag().clone().abs().log10() > a.prec().0 / 4
    {
        r2 = r2.real().clone().into();
    }
    if -r3.imag().clone().abs().log10() > a.prec().0 / 4
    {
        r3 = r3.real().clone().into();
    }
    if -r4.imag().clone().abs().log10() > a.prec().0 / 4
    {
        r4 = r4.real().clone().into();
    }
    if real
    {
        let mut vec = Vec::new();
        if r1.imag().is_zero()
        {
            vec.push(Number::from(r1, None))
        }
        if r2.imag().is_zero()
        {
            vec.push(Number::from(r2, None))
        }
        if r3.imag().is_zero()
        {
            vec.push(Number::from(r3, None))
        }
        if r4.imag().is_zero()
        {
            vec.push(Number::from(r4, None))
        }
        if vec.is_empty()
        {
            vec![Number::from(Complex::with_val(prec, Nan), None)]
        }
        else
        {
            vec
        }
    }
    else
    {
        vec![
            Number::from(r1, None),
            Number::from(r2, None),
            Number::from(r3, None),
            Number::from(r4, None),
        ]
    }
}
pub fn variance(a: &[Number], prec: u32) -> Number
{
    let mean = a
        .iter()
        .fold(Complex::new(prec), |sum, val| sum + val.number.clone())
        / a.len();
    let mut variance = Complex::new(prec);
    for a in a
    {
        variance += (a.number.clone() - mean.clone()).pow(2)
    }
    Number::from(variance / (a.len().saturating_sub(1)), None)
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
                    let vl = vars.len().saturating_sub(1);
                    vars[vl] = &vars[vl][0..vars[vl].len().saturating_sub(1)];
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
                let var = &var[0..var.len().saturating_sub(1)];
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
pub fn erf(z: Complex) -> Complex
{
    1 - erfc(z)
}
pub fn erfc(z: Complex) -> Complex
{
    let p2: Complex = z.clone().pow(2);
    erfc_recursion(z.clone(), 0, z.prec().0 as usize / 4) / Complex::with_val(z.prec(), Pi).sqrt()
        * z
        / p2.exp()
}
fn erfc_recursion(z: Complex, iter: usize, max: usize) -> Complex
{
    if iter == max
    {
        Complex::with_val(z.prec(), 1)
    }
    else if iter == 0
    {
        erfc_recursion(z, 1, max).recip()
    }
    else if iter % 2 == 1
    {
        z.clone().pow(2) + iter / (2 * erfc_recursion(z, iter + 1, max))
    }
    else
    {
        1 + iter / (2 * erfc_recursion(z, iter + 1, max))
    }
}
fn gamma0(z: Complex) -> Complex
{
    let p = z.prec().0 as usize / 4;
    gamma0_recursion_first(z.clone(), 0, p) + gamma0_recursion_second(z, 0, p)
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
    if z.is_zero()
    {
        gamma0(s)
    }
    else
    {
        let p = z.prec().0 as usize / 4;
        incomplete_gamma_recursion(s, z, 0, p)
    }
}
pub fn eta(s: Complex) -> Complex
{
    let prec = s.prec().0;
    let mut sum = Complex::new(prec);
    let two = Complex::with_val(prec, 2);
    for n in 0..=(prec / 16).max(16)
    {
        let mut innersum = Complex::new(prec);
        let nb = Integer::from(n);
        for k in 0..=n
        {
            let num = nb.clone().binomial(k) * Complex::with_val(prec, 1 + k).pow(-s.clone());
            if k % 2 == 0
            {
                innersum += num;
            }
            else
            {
                innersum -= num;
            }
        }
        sum += innersum / two.clone().pow(n + 1)
    }
    sum
}
pub fn zeta(s: Complex) -> Complex
{
    eta(s.clone()) / (1 - Complex::with_val(s.prec(), 2).pow(1 - s))
}
pub fn euleriannumbers(n: Complex, k: i32) -> Complex
{
    if k < 0
    {
        Complex::with_val(n.prec(), Nan)
    }
    else if n.real().clone().fract() != 0 && n.imag().is_zero() && n.real().is_sign_positive()
    {
        let mut sum = Complex::new(n.prec());
        for i in 0..=k
        {
            let ic = Complex::with_val(n.prec(), i);
            let num: Complex = k - ic.clone() + 1;
            let num = binomial(n.clone() + 1, ic) * num.pow(n.clone());
            if i % 2 == 0
            {
                sum += num
            }
            else
            {
                sum -= num
            }
        }
        sum
    }
    else
    {
        Complex::with_val(
            n.prec(),
            euleriannumbersint(
                n.real()
                    .to_integer()
                    .unwrap_or_default()
                    .to_u32()
                    .unwrap_or_default(),
                k as u32,
            ),
        )
    }
}
pub fn euleriannumbersint(n: u32, k: u32) -> Integer
{
    let mut sum = Integer::new();
    for i in 0..=k
    {
        let num: Integer = k - Integer::from(i) + 1;
        let num = Integer::from(n + 1).binomial(i) * num.pow(n);
        if i % 2 == 0
        {
            sum += num
        }
        else
        {
            sum -= num
        }
    }
    sum
}
pub fn binomial(a: Complex, b: Complex) -> Complex
{
    if a.imag().is_zero()
        && b.imag().is_zero()
        && a.real().clone().fract().is_zero()
        && b.real().clone().fract().is_zero()
        && a.real().is_finite()
    {
        Complex::with_val(
            a.prec(),
            a.real().to_integer().unwrap().binomial(
                b.real()
                    .to_integer()
                    .unwrap_or_default()
                    .to_u32()
                    .unwrap_or_default(),
            ),
        )
    }
    else
    {
        gamma(a.clone() + 1) / (gamma(b.clone() + 1) * gamma(a.clone() - b.clone() + 1))
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
    subfactorial_recursion(z.clone(), 0, (z.prec().0 as usize / 4).max(32))
        + gamma(z.clone() + 1) / Float::with_val(z.prec().0, 1).exp()
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
    end: Number,
    points: usize,
) -> Result<Number, &'static str>
{
    let units = end.units;
    let mut end = end.number;
    if start.real() > end.real()
    {
        (start, end) = (end, start)
    }
    let delta: Complex = (end.clone() - start.clone()) / points;
    let mut x0: NumStr = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var.clone(),
        Num(Number::from(start.clone(), units)),
    )?;
    let length_units = match x0.clone()
    {
        Num(a) => a.units,
        Vector(a) =>
        {
            if a.iter().all(|b| b.units == a[0].units)
            {
                a[0].units
            }
            else
            {
                None
            }
        }
        _ => return Err("not supported arc length data"),
    };
    let mut length = Complex::new(options.prec);
    for i in 0..points
    {
        if i + 1 == points
        {
            start.clone_from(&end)
        }
        else
        {
            start += delta.clone();
        }
        let x1 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var.clone(),
            Num(Number::from(start.clone(), units)),
        )?;
        match (x0, x1)
        {
            (Num(xi), Num(xf)) =>
            {
                let nl: Complex = (xf.number.clone() - xi.number).pow(2) + delta.clone().pow(2);
                length += nl.sqrt();
                x0 = Num(xf);
            }
            (Vector(xi), Vector(xf)) if xf.len() == 1 =>
            {
                let nl: Complex =
                    (xf[0].number.clone() - xi[0].number.clone()).pow(2) + delta.clone().pow(2);
                length += nl.sqrt();
                x0 = Vector(xf);
            }
            (Vector(xi), Vector(xf)) =>
            {
                let nl: Complex = xi
                    .iter()
                    .zip(xf.clone())
                    .fold(Complex::new(options.prec), |sum, x| {
                        sum + (x.1.number - x.0.number.clone()).pow(2)
                    });
                length += nl.sqrt();
                x0 = Vector(xf);
            }
            (_, _) => return Err("not supported arc length data"),
        };
    }
    Ok(Number::from(length, length_units))
}
#[allow(clippy::too_many_arguments)]
pub fn area(
    mut func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    mut start: Complex,
    end: Number,
    points: usize,
    combine: bool,
) -> Result<NumStr, &'static str>
{
    let units = end.units;
    let end = end.number;
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
            func = func[last..func.len().saturating_sub(1)].to_vec();
        }
    }
    let mut areavec: Vec<Number> = Vec::new();
    let div = Complex::with_val(options.prec, 0.5).pow(options.prec / 2);
    let delta: Complex = (end.clone() - start.clone()) / points;
    let mut area: Complex = Complex::new(options.prec);
    let mut x0 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(Number::from(start.clone(), units)),
    )?;
    let yunits = if let Num(ref a) = x0 { a.units } else { None };
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
                Num(Number::from(start.clone() + div.clone(), units)),
            )?
            .num()?
            .number
                - do_math_with_var(
                    i.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(Number::from(start.clone(), units)),
                )?
                .num()?
                .number)
                / div.clone())
            .pow(2);
        }
        x0 = Num(Number::from(x0.num()?.number * nx0t.sqrt(), units));
    }
    let h: Complex = delta.clone() / 4;
    for i in 0..points
    {
        if i + 1 == points
        {
            start.clone_from(&end)
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
            Num(Number::from(start.clone() - 3 * h.clone(), units)),
        )?;
        let x2 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(start.clone() - 2 * h.clone(), units)),
        )?;
        let x3 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(start.clone() - h.clone(), units)),
        )?;
        let x4 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(start.clone(), units)),
        )?;
        match (x0, x1, x2, x3, x4.clone())
        {
            (Num(nx0), Num(nx1), Num(nx2), Num(nx3), Num(nx4)) if funcs.is_empty() =>
                {
                    area += 2 * h.clone() * (7 * (nx0.number + nx4.number) + 12 * nx2.number + 32 * (nx1.number + nx3.number)) / 45;
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
                            &var, Num( Number::from(start.clone() - 3 * h.clone() + div.clone(),units)))?
                            .num()?.number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num( Number::from(start.clone() - 3 * h.clone(),units)))?
                            .num()?.number)
                            / div.clone())
                            .pow(2);
                        nx2t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num( Number::from(start.clone() - 2 * h.clone() + div.clone(),units)),
                        )?
                            .num()?.number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num( Number::from(start.clone() - 2 * h.clone(),units)))?
                            .num()?.number)
                            / div.clone())
                            .pow(2);
                        nx3t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num( Number::from(start.clone() - h.clone() + div.clone(), units)),
                        )?
                            .num()?.number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(Number::from(start.clone() - h.clone(), units)))?
                            .num()?.number)
                            / div.clone())
                            .pow(2);
                        nx4t += ((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(start.clone() + div.clone(), units)),
                        )?
                            .num()?.number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var, Num(Number::from(start.clone(), units)))?
                            .num()?.number)
                            / div.clone())
                            .pow(2);
                    }
                    let x4 = nx4.number * nx4t.sqrt();
                    area += 2
                        * h.clone()
                        * (7 * (nx0.number + x4.clone())
                        + 12 * (nx2.number * nx2t.sqrt())
                        + 32 * ((nx1.number * nx1t.sqrt()) + (nx3.number * nx3t.sqrt())))
                        / 45;
                    x0 = Num(Number::from(x4, units));
                }
            (Vector(nx0), Vector(nx1), Vector(nx2), Vector(nx3), Vector(nx4))
            if areavec.is_empty() && !combine =>
                {
                    for i in 0..nx0.len()
                    {
                        areavec.push(Number::from
                            (      2 * h.clone()
                                * (7 * (nx0[i].number.clone() + nx4[i].number.clone())
                                + 12 * nx2[i].number.clone()
                                + 32 * (nx1[i].number.clone() + nx3[i].number.clone()))
                                / 45,
                                        match (units, nx1[i].units)
            {
                (Some(a), Some(b)) => Some(a.mul(&b)),
                (Some(a), None) | (None, Some(a)) => Some(a),
                (None, None) => None,
            }
)
                        )
                    }
                    x0 = x4;
                }
            (Vector(nx0), Vector(nx1), Vector(nx2), Vector(nx3), Vector(nx4)) if !combine =>
                {
                    for (i, v) in areavec.iter_mut().enumerate()
                    {
                        v.number += 2
                            * h.clone()
                            * (7 * (nx0[i].number.clone() + nx4[i].number.clone())
                            + 12 * nx2[i].number.clone()
                            + 32 * (nx1[i].number.clone() + nx3[i].number.clone()))
                            / 45
                    }
                    x0 = x4;
                }
            (_, _, _, _, _) => return Err("not supported area data, if parametric have the 2nd arg start and end with the { } brackets"),
        }
    }
    if areavec.is_empty()
    {
        Ok(Num(Number::from(
            area,
            match (units, yunits)
            {
                (Some(a), Some(b)) => Some(a.mul(&b)),
                (Some(a), None) | (None, Some(a)) => Some(a),
                (None, None) => None,
            },
        )))
    }
    else
    {
        Ok(Vector(areavec))
    }
}
pub fn solve(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    mut x: Complex,
) -> Result<NumStr, &'static str>
{
    //newtons method, x-f(x)/f'(x)
    let op = options.prec;
    options.prec = options.prec.clamp(256, 1024);
    x.set_prec(options.prec);
    for _ in 0..op / 4
    {
        let n = Number::from(x.clone(), None);
        let y = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var.clone(),
            Num(n.clone()),
        )?;
        x -= y.num()?.number
            / slopesided(
                func.clone(),
                func_vars.clone(),
                options,
                var.clone(),
                n,
                false,
                1,
                true,
                Some(y),
            )?
            .num()?
            .number
    }
    let last = x.clone();
    let n = Number::from(x.clone(), None);
    let y = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var.clone(),
        Num(n.clone()),
    )?;
    x -= y.num()?.number
        / slopesided(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            n,
            false,
            1,
            true,
            Some(y),
        )?
        .num()?
        .number;
    if (last - x.clone()).abs().real().clone().log2() < op as i32 / -16
    {
        Ok(Num(Number::from(x, None)))
    }
    else
    {
        Ok(Num(Number::from(
            Complex::with_val(options.prec, Nan),
            None,
        )))
    }
}
#[allow(clippy::too_many_arguments)]
pub fn slope(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    mut point: Number,
    combine: bool,
    nth: u32,
    side: LimSide,
) -> Result<NumStr, &'static str>
{
    match side
    {
        LimSide::Left => slopesided(
            func, func_vars, options, var, point, combine, nth, false, None,
        ),
        LimSide::Right => slopesided(
            func, func_vars, options, var, point, combine, nth, true, None,
        ),
        LimSide::Both =>
        {
            options.prec = options.prec.clamp(256, 1024);
            point.number.set_prec(options.prec);
            let left = slopesided(
                func.clone(),
                func_vars.clone(),
                options,
                var.clone(),
                point.clone(),
                combine,
                nth,
                false,
                None,
            )?;
            let right = slopesided(
                func, func_vars, options, var, point, combine, nth, true, None,
            )?;
            match (left, right)
            {
                (Num(left), Num(right)) =>
                {
                    let units = left.units;
                    let left = left.number;
                    let right = right.number;
                    if (((left.real().is_infinite()
                        && right.real().is_infinite()
                        && (left.imag().clone() - right.imag().clone())
                            .abs()
                            .clone()
                            .log2()
                            < options.prec as i32 / -16)
                        || (left.imag().is_infinite()
                            && right.imag().is_infinite()
                            && (left.real().clone() - right.real().clone())
                                .abs()
                                .clone()
                                .log2()
                                < options.prec as i32 / -16))
                        && left.real().is_sign_positive() == right.real().is_sign_positive()
                        && left.imag().is_sign_positive() == right.imag().is_sign_positive())
                        || (left.clone() - right.clone()).abs().real().clone().log2()
                            < options.prec as i32 / -16
                    {
                        Ok(Num(Number::from((left + right) / 2, units)))
                    }
                    else
                    {
                        Ok(Num(Number::from(
                            Complex::with_val(options.prec, Nan),
                            None,
                        )))
                    }
                }
                (Vector(left), Vector(right)) =>
                {
                    let mut vec = Vec::with_capacity(left.len());
                    for (left, right) in left.iter().zip(right)
                    {
                        vec.push({
                            let units = left.units;
                            let left = left.number.clone();
                            let right = right.number.clone();
                            if (((left.real().is_infinite()
                                && right.real().is_infinite()
                                && (left.imag().clone() - right.imag().clone())
                                    .abs()
                                    .clone()
                                    .log2()
                                    < options.prec as i32 / -16)
                                || (left.imag().is_infinite()
                                    && right.imag().is_infinite()
                                    && (left.real().clone() - right.real().clone())
                                        .abs()
                                        .clone()
                                        .log2()
                                        < options.prec as i32 / -16))
                                && left.real().is_sign_positive()
                                    == right.real().is_sign_positive()
                                && left.imag().is_sign_positive()
                                    == right.imag().is_sign_positive())
                                || (left.clone() - right.clone()).abs().real().clone().log2()
                                    < options.prec as i32 / -16
                            {
                                Number::from((left + right) / 2, units)
                            }
                            else
                            {
                                Number::from(Complex::with_val(options.prec, Nan), None)
                            }
                        })
                    }
                    Ok(Vector(vec))
                }
                (_, _) => Err("lim err"),
            }
        }
    }
}
#[allow(clippy::too_many_arguments)]
pub fn slopesided(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    point: Number,
    combine: bool,
    nth: u32,
    right: bool,
    val: Option<NumStr>,
) -> Result<NumStr, &'static str>
{
    let units = point.units;
    let mut point = point.number;
    options.prec = options.prec.clamp(256, 1024);
    let prec = options.prec / 8;
    options.prec = nth.max(1) * options.prec / 2;
    point.set_prec(options.prec);
    let h: Complex = if right
    {
        Complex::with_val(options.prec, 0.5).pow(prec)
    }
    else
    {
        -Complex::with_val(options.prec, 0.5).pow(prec)
    };
    let n = if let Some(n) = val
    {
        n
    }
    else
    {
        do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(point.clone(), units)),
        )?
    };
    let num = Integer::from(nth);
    match n
    {
        Num(sum) =>
        {
            let yunits = sum.units;
            let mut sum = sum.number;
            if nth % 2 == 1
            {
                sum *= -1;
            }
            for k in 0..nth
            {
                if k % 2 == 0
                {
                    sum += num.clone().binomial(k)
                        * do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() + h.clone() * (nth - k), units)),
                        )?
                        .num()?
                        .number;
                }
                else
                {
                    sum -= num.clone().binomial(k)
                        * do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() + h.clone() * (nth - k), units)),
                        )?
                        .num()?
                        .number;
                }
            }
            if right || nth % 2 == 0
            {
                Ok(Num(Number::from(
                    get_infinities(
                        sum * Float::with_val(options.prec, 2).pow(nth * prec),
                        prec,
                        options.prec,
                    ),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                )))
            }
            else
            {
                Ok(Num(Number::from(
                    -get_infinities(
                        sum * Float::with_val(options.prec, 2).pow(nth * prec),
                        prec,
                        options.prec,
                    ),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                )))
            }
        }
        Vector(mut sum) if !combine =>
        {
            let yunits = sum[0].units;
            if nth % 2 == 1
            {
                for n in sum.iter_mut()
                {
                    n.number *= -1;
                }
            }
            for k in 0..nth
            {
                let b = num.clone().binomial(k);
                let vec = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(Number::from(point.clone() + h.clone() * (nth - k), units)),
                )?
                .vec()?;
                if k % 2 == 0
                {
                    for (n, a) in sum.iter_mut().zip(vec)
                    {
                        n.number += a.number * b.clone()
                    }
                }
                else
                {
                    for (n, a) in sum.iter_mut().zip(vec)
                    {
                        n.number -= a.number * b.clone()
                    }
                }
            }
            Ok(Vector(
                sum.iter()
                    .map(|n| {
                        Number::from(
                            if right || nth % 2 == 0
                            {
                                get_infinities(
                                    n.number.clone()
                                        * Float::with_val(options.prec, 2).pow(nth * prec),
                                    prec,
                                    options.prec,
                                )
                            }
                            else
                            {
                                -get_infinities(
                                    n.number.clone()
                                        * Float::with_val(options.prec, 2).pow(nth * prec),
                                    prec,
                                    options.prec,
                                )
                            },
                            match (yunits, units)
                            {
                                (Some(a), Some(b)) => Some(a.div(&b)),
                                (Some(a), None) => Some(a),
                                (None, Some(b)) => Some(Units::default().div(&b)),
                                (None, None) => None,
                            },
                        )
                    })
                    .collect::<Vec<Number>>(),
            ))
        }
        Vector(mut sum) if sum.len() == 1 =>
        {
            let yunits = sum[0].units;
            if nth % 2 == 1
            {
                sum[0].number *= -1;
            }
            for k in 0..nth
            {
                if k % 2 == 0
                {
                    sum[0].number += num.clone().binomial(k)
                        * do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() + h.clone() * (nth - k), units)),
                        )?
                        .num()?
                        .number;
                }
                else
                {
                    sum[0].number -= num.clone().binomial(k)
                        * do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() + h.clone() * (nth - k), units)),
                        )?
                        .num()?
                        .number;
                }
            }
            if right || nth % 2 == 0
            {
                Ok(Num(Number::from(
                    get_infinities(
                        sum[0].number.clone() * Float::with_val(options.prec, 2).pow(nth * prec),
                        prec,
                        options.prec,
                    ),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                )))
            }
            else
            {
                Ok(Num(Number::from(
                    -get_infinities(
                        sum[0].number.clone() * Float::with_val(options.prec, 2).pow(nth * prec),
                        prec,
                        options.prec,
                    ),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                )))
            }
        }
        Vector(mut sum) =>
        {
            let yunits = sum[0].units;
            if nth % 2 == 1
            {
                for n in sum.iter_mut()
                {
                    n.number *= -1;
                }
            }
            for k in 0..nth
            {
                let b = num.clone().binomial(k);
                let vec = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(Number::from(point.clone() + h.clone() * (nth - k), units)),
                )?
                .vec()?;
                if k % 2 == 0
                {
                    for (n, a) in sum.iter_mut().zip(vec)
                    {
                        n.number += a.number * b.clone()
                    }
                }
                else
                {
                    for (n, a) in sum.iter_mut().zip(vec)
                    {
                        n.number -= a.number * b.clone()
                    }
                }
            }
            if sum.len() == 2
            {
                Ok(Num(Number::from(
                    get_infinities(
                        sum[1].number.clone() / sum[0].number.clone(),
                        prec,
                        options.prec,
                    ),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                )))
            }
            else
            {
                let nf = &sum.last().unwrap().number;
                Ok(Vector(
                    sum[0..sum.len().saturating_sub(1)]
                        .iter()
                        .map(|n| {
                            Number::from(
                                get_infinities(nf.clone() / n.number.clone(), prec, options.prec),
                                match (yunits, units)
                                {
                                    (Some(a), Some(b)) => Some(a.div(&b)),
                                    (Some(a), None) => Some(a),
                                    (None, Some(b)) => Some(Units::default().div(&b)),
                                    (None, None) => None,
                                },
                            )
                        })
                        .collect::<Vec<Number>>(),
                ))
            }
        }
        _ => Err("not supported slope data"),
    }
}
fn get_infinities(n: Complex, prec: u32, optionsprec: u32) -> Complex
{
    match (
        n.real().clone().abs().log2() > prec as i32 / 2 - 1,
        n.imag().clone().abs().log2() > prec as i32 / 2 - 1,
    )
    {
        (true, true) => match (n.real().is_sign_positive(), n.imag().is_sign_positive())
        {
            (true, true) => Complex::with_val(optionsprec, (Infinity, Infinity)),
            (true, false) => Complex::with_val(
                optionsprec,
                (Infinity, -Float::with_val(optionsprec, Infinity)),
            ),
            (false, true) => Complex::with_val(
                optionsprec,
                (-Float::with_val(optionsprec, Infinity), Infinity),
            ),
            (false, false) => -Complex::with_val(optionsprec, (Infinity, Infinity)),
        },
        (true, false) =>
        {
            if n.real().is_sign_positive()
            {
                Complex::with_val(optionsprec, (Infinity, n.imag()))
            }
            else
            {
                -Complex::with_val(optionsprec, (Infinity, -n.imag()))
            }
        }
        (false, true) =>
        {
            if n.imag().is_sign_positive()
            {
                Complex::with_val(optionsprec, (n.real(), Infinity))
            }
            else
            {
                -Complex::with_val(optionsprec, (-n.real(), Infinity))
            }
        }
        (false, false) => n,
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
    mut options: Options,
    var: String,
    point: Number,
    side: LimSide,
) -> Result<NumStr, &'static str>
{
    let xunits = point.units;
    let mut point = point.number;
    if options.prec < 256
    {
        options.prec = 256;
        point.set_prec(options.prec);
    }
    else if options.prec > 1024
    {
        options.prec = 1024;
        point.set_prec(options.prec);
    }
    if point.clone().real().is_infinite() || point.clone().imag().is_infinite()
    {
        let (h1, h2);
        let positive = point.real().is_sign_positive();
        if positive
        {
            h1 = Complex::with_val(options.prec, 2).pow(options.prec / 4);
            h2 = Complex::with_val(options.prec, 2).pow((options.prec / 3) as f64 + 7.0 / 0.94) - 3;
        }
        else
        {
            h1 = -Complex::with_val(options.prec, 2).pow(options.prec / 4);
            h2 = 3 - Complex::with_val(options.prec, 2).pow((options.prec / 3) as f64 + 7.0 / 0.94);
        }
        let n1 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(h1, xunits)),
        )?;
        let n2 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(h2, xunits)),
        )?;
        match (n1, n2)
        {
            (Num(n1), Num(n2)) =>
            {
                let units = n1.units;
                let n1 = n1.number;
                let n2 = n2.number;
                if (n1.clone() - n2.clone()).abs().real().clone().log2() < options.prec as i32 / -16
                {
                    Ok(Num(Number::from(n2, units)))
                }
                else if n1.real().is_sign_positive() != n2.real().is_sign_positive()
                    || n1.imag().is_sign_positive() != n2.imag().is_sign_positive()
                {
                    Ok(Num(Number::from(
                        Complex::with_val(options.prec, Nan),
                        None,
                    )))
                }
                else if n2.real().is_infinite() || n2.imag().is_infinite()
                {
                    Ok(Num(Number::from(
                        match (n2.real().is_infinite(), n2.imag().is_infinite())
                        {
                            (true, true) =>
                            {
                                match (n1.real().is_sign_positive(), n1.imag().is_sign_positive())
                                {
                                    (true, true) =>
                                    {
                                        Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                    (true, false) => Complex::with_val(
                                        options.prec,
                                        (Infinity, -Float::with_val(options.prec, Infinity)),
                                    ),
                                    (false, true) => Complex::with_val(
                                        options.prec,
                                        (-Float::with_val(options.prec, Infinity), Infinity),
                                    ),
                                    (false, false) =>
                                    {
                                        -Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                }
                            }
                            (true, false) =>
                            {
                                if n1.real().is_sign_positive()
                                {
                                    Complex::with_val(
                                        options.prec,
                                        (
                                            Infinity,
                                            if (n1.imag() - n2.imag().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                n2.imag().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                        ),
                                    )
                                }
                                else
                                {
                                    -Complex::with_val(
                                        options.prec,
                                        (
                                            Infinity,
                                            if (n1.imag() - n2.imag().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                -n2.imag().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                        ),
                                    )
                                }
                            }
                            (false, true) =>
                            {
                                if n1.imag().is_sign_positive()
                                {
                                    Complex::with_val(
                                        options.prec,
                                        (
                                            if (n1.real() - n2.real().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                n2.real().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                            Infinity,
                                        ),
                                    )
                                }
                                else
                                {
                                    -Complex::with_val(
                                        options.prec,
                                        (
                                            if (n1.real() - n2.real().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                -n2.real().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                            Infinity,
                                        ),
                                    )
                                }
                            }
                            (false, false) => Complex::with_val(options.prec, Nan),
                        },
                        units,
                    )))
                }
                else
                {
                    let n3 = do_math_with_var(
                        func.clone(),
                        options,
                        func_vars.clone(),
                        &var,
                        Num(Number::from(
                            if positive
                            {
                                Complex::with_val(options.prec, 2)
                                    .pow((options.prec / 2) as f64 + 13.0 / 0.7)
                                    - 7
                            }
                            else
                            {
                                7 - Complex::with_val(options.prec, 2)
                                    .pow((options.prec / 2) as f64 + 13.0 / 0.7)
                            },
                            xunits,
                        )),
                    )?
                    .num()?
                    .number;
                    let sign = n2.real().is_sign_positive() == n3.real().is_sign_positive()
                        && n2.imag().is_sign_positive() == n3.imag().is_sign_positive();
                    let n1r = n1.real().clone().abs();
                    let n2r = n2.real().clone().abs();
                    let n3r = n3.real().clone().abs();
                    let n1i = n1.imag().clone().abs();
                    let n2i = n2.imag().clone().abs();
                    let n3i = n3.imag().clone().abs();
                    Ok(Num(Number::from(
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
                                    match (
                                        n1.real().is_sign_positive(),
                                        n1.imag().is_sign_positive(),
                                    )
                                    {
                                        (true, true) =>
                                        {
                                            Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                        (true, false) => Complex::with_val(
                                            options.prec,
                                            (Infinity, -Float::with_val(options.prec, Infinity)),
                                        ),
                                        (false, true) => Complex::with_val(
                                            options.prec,
                                            (-Float::with_val(options.prec, Infinity), Infinity),
                                        ),
                                        (false, false) =>
                                        {
                                            -Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                    }
                                }
                                (true, false) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(
                                            options.prec,
                                            (
                                                Infinity,
                                                if (n2i - n3i.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    n3i
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                            ),
                                        )
                                    }
                                    else
                                    {
                                        -Complex::with_val(
                                            options.prec,
                                            (
                                                Infinity,
                                                if (n2i - n3i.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    -n3i
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                            ),
                                        )
                                    }
                                }
                                (false, true) =>
                                {
                                    if n1.imag().is_sign_positive()
                                    {
                                        Complex::with_val(
                                            options.prec,
                                            (
                                                if (n2r - n3r.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    n3r
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                                Infinity,
                                            ),
                                        )
                                    }
                                    else
                                    {
                                        -Complex::with_val(
                                            options.prec,
                                            (
                                                if (n2r - n3r.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    -n3r
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                                Infinity,
                                            ),
                                        )
                                    }
                                }
                                (false, false) => Complex::with_val(options.prec, Nan),
                            }
                        },
                        units,
                    )))
                }
            }
            (Vector(v1), Vector(v2)) =>
            {
                let mut v3: Vec<Number> = Vec::new();
                let mut vec = Vec::with_capacity(v1.len());
                for (i, (n1, n2)) in v1.iter().zip(v2).enumerate()
                {
                    let units = n1.units;
                    let n1 = n1.number.clone();
                    let n2 = n2.number.clone();
                    vec.push(Number::from(
                        if (n1.clone() - n2.clone()).abs().real().clone().log2()
                            < options.prec as i32 / -16
                        {
                            n2
                        }
                        else if n1.real().is_sign_positive() != n2.real().is_sign_positive()
                            || n1.imag().is_sign_positive() != n2.imag().is_sign_positive()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else if n2.real().is_infinite() || n2.imag().is_infinite()
                        {
                            match (n2.real().is_infinite(), n2.imag().is_infinite())
                            {
                                (true, true) =>
                                {
                                    match (
                                        n1.real().is_sign_positive(),
                                        n1.imag().is_sign_positive(),
                                    )
                                    {
                                        (true, true) =>
                                        {
                                            Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                        (true, false) => Complex::with_val(
                                            options.prec,
                                            (Infinity, -Float::with_val(options.prec, Infinity)),
                                        ),
                                        (false, true) => Complex::with_val(
                                            options.prec,
                                            (-Float::with_val(options.prec, Infinity), Infinity),
                                        ),
                                        (false, false) =>
                                        {
                                            -Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                    }
                                }
                                (true, false) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(
                                            options.prec,
                                            (
                                                Infinity,
                                                if (n1.imag() - n2.imag().clone()).abs().log10()
                                                    <= -10
                                                {
                                                    n2.imag().clone()
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                            ),
                                        )
                                    }
                                    else
                                    {
                                        -Complex::with_val(
                                            options.prec,
                                            (
                                                Infinity,
                                                if (n1.imag() - n2.imag().clone()).abs().log10()
                                                    <= -10
                                                {
                                                    -n2.imag().clone()
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                            ),
                                        )
                                    }
                                }
                                (false, true) =>
                                {
                                    if n1.imag().is_sign_positive()
                                    {
                                        Complex::with_val(
                                            options.prec,
                                            (
                                                if (n1.real() - n2.real().clone()).abs().log10()
                                                    <= -10
                                                {
                                                    n2.real().clone()
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                                Infinity,
                                            ),
                                        )
                                    }
                                    else
                                    {
                                        -Complex::with_val(
                                            options.prec,
                                            (
                                                if (n1.real() - n2.real().clone()).abs().log10()
                                                    <= -10
                                                {
                                                    -n2.real().clone()
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                                Infinity,
                                            ),
                                        )
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
                                    Num(Number::from(
                                        if positive
                                        {
                                            Complex::with_val(options.prec, 2)
                                                .pow((options.prec / 2) as f64 + 13.0 / 0.7)
                                                - 7
                                        }
                                        else
                                        {
                                            7 - Complex::with_val(options.prec, 2)
                                                .pow((options.prec / 2) as f64 + 13.0 / 0.7)
                                        },
                                        xunits,
                                    )),
                                )?
                                .vec()?;
                            }
                            let v3 = &v3[i].number;
                            let sign = n2.real().is_sign_positive() == v3.real().is_sign_positive()
                                && n2.imag().is_sign_positive() == v3.imag().is_sign_positive();
                            let n1r = n1.real().clone().abs();
                            let n2r = n2.real().clone().abs();
                            let n3r = v3.real().clone().abs();
                            let n1i = n1.imag().clone().abs();
                            let n2i = n2.imag().clone().abs();
                            let n3i = v3.imag().clone().abs();
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
                                        match (
                                            n1.real().is_sign_positive(),
                                            n1.imag().is_sign_positive(),
                                        )
                                        {
                                            (true, true) => Complex::with_val(
                                                options.prec,
                                                (Infinity, Infinity),
                                            ),
                                            (true, false) => Complex::with_val(
                                                options.prec,
                                                (
                                                    Infinity,
                                                    -Float::with_val(options.prec, Infinity),
                                                ),
                                            ),
                                            (false, true) => Complex::with_val(
                                                options.prec,
                                                (
                                                    -Float::with_val(options.prec, Infinity),
                                                    Infinity,
                                                ),
                                            ),
                                            (false, false) => -Complex::with_val(
                                                options.prec,
                                                (Infinity, Infinity),
                                            ),
                                        }
                                    }
                                    (true, false) =>
                                    {
                                        if n1.real().is_sign_positive()
                                        {
                                            Complex::with_val(
                                                options.prec,
                                                (
                                                    Infinity,
                                                    if (n2i - n3i.clone()).abs().log2()
                                                        < options.prec as i32 / -16
                                                    {
                                                        n3i
                                                    }
                                                    else
                                                    {
                                                        Float::new(options.prec)
                                                    },
                                                ),
                                            )
                                        }
                                        else
                                        {
                                            -Complex::with_val(
                                                options.prec,
                                                (
                                                    Infinity,
                                                    if (n2i - n3i.clone()).abs().log2()
                                                        < options.prec as i32 / -16
                                                    {
                                                        -n3i
                                                    }
                                                    else
                                                    {
                                                        Float::new(options.prec)
                                                    },
                                                ),
                                            )
                                        }
                                    }
                                    (false, true) =>
                                    {
                                        if n1.imag().is_sign_positive()
                                        {
                                            Complex::with_val(
                                                options.prec,
                                                (
                                                    if (n2r - n3r.clone()).abs().log2()
                                                        < options.prec as i32 / -16
                                                    {
                                                        n3r
                                                    }
                                                    else
                                                    {
                                                        Float::new(options.prec)
                                                    },
                                                    Infinity,
                                                ),
                                            )
                                        }
                                        else
                                        {
                                            -Complex::with_val(
                                                options.prec,
                                                (
                                                    if (n2r - n3r.clone()).abs().log2()
                                                        < options.prec as i32 / -16
                                                    {
                                                        -n3r
                                                    }
                                                    else
                                                    {
                                                        Float::new(options.prec)
                                                    },
                                                    Infinity,
                                                ),
                                            )
                                        }
                                    }
                                    (false, false) => Complex::with_val(options.prec, Nan),
                                }
                            }
                        },
                        units,
                    ))
                }
                Ok(Vector(vec))
            }
            (_, _) => Err("unsupported lim data"),
        }
    }
    else
    {
        let point = Number::from(point, xunits);
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
                        let units = left.units;
                        let left = left.number;
                        let right = right.number;
                        if (((left.real().is_infinite()
                            && right.real().is_infinite()
                            && (left.imag().clone() - right.imag().clone())
                                .abs()
                                .clone()
                                .log2()
                                < options.prec as i32 / -16)
                            || (left.imag().is_infinite()
                                && right.imag().is_infinite()
                                && (left.real().clone() - right.real().clone())
                                    .abs()
                                    .clone()
                                    .log2()
                                    < options.prec as i32 / -16))
                            && left.real().is_sign_positive() == right.real().is_sign_positive()
                            && left.imag().is_sign_positive() == right.imag().is_sign_positive())
                            || (left.clone() - right.clone()).abs().real().clone().log2()
                                < options.prec as i32 / -16
                        {
                            Ok(Num(Number::from((left + right) / 2, units)))
                        }
                        else
                        {
                            Ok(Num(Number::from(
                                Complex::with_val(options.prec, Nan),
                                None,
                            )))
                        }
                    }
                    (Vector(left), Vector(right)) =>
                    {
                        let mut vec = Vec::with_capacity(left.len());
                        for (left, right) in left.iter().zip(right)
                        {
                            let units = left.units;
                            let left = &left.number;
                            let right = &right.number;
                            vec.push(
                                if (((left.real().is_infinite()
                                    && right.real().is_infinite()
                                    && (left.imag().clone() - right.imag().clone())
                                        .abs()
                                        .clone()
                                        .log2()
                                        < options.prec as i32 / -16)
                                    || (left.imag().is_infinite()
                                        && right.imag().is_infinite()
                                        && (left.real().clone() - right.real().clone())
                                            .abs()
                                            .clone()
                                            .log2()
                                            < options.prec as i32 / -16))
                                    && left.real().is_sign_positive()
                                        == right.real().is_sign_positive()
                                    && left.imag().is_sign_positive()
                                        == right.imag().is_sign_positive())
                                    || (left.clone() - right.clone()).abs().real().clone().log2()
                                        < options.prec as i32 / -16
                                {
                                    Number::from((left + right.clone()) / 2, units)
                                }
                                else
                                {
                                    Number::from(Complex::with_val(options.prec, Nan), None)
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
    point: Number,
    right: bool,
) -> Result<NumStr, &'static str>
{
    let xunits = point.units;
    let point = point.number;
    let h1 = Complex::with_val(options.prec, 0.5).pow(options.prec / 4);
    let h2 = Complex::with_val(options.prec, 0.5).pow((options.prec / 3) as f64 + 7.0 / 0.94);
    let n1 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(Number::from(
            point.clone() + if right { h1 } else { -h1 },
            xunits,
        )),
    )?;
    let n2 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(Number::from(
            point.clone() + if right { h2 } else { -h2 },
            xunits,
        )),
    )?;
    match (n1, n2)
    {
        (Num(n1), Num(n2)) =>
        {
            let units = n1.units;
            let n1 = n1.number;
            let n2 = n2.number;
            Ok(Num(Number::from(
                if (n2.clone() - n1.clone()).abs().real().clone().log2() < options.prec as i32 / -16
                {
                    n1
                }
                else
                {
                    let h3 = Complex::with_val(options.prec, 0.5)
                        .pow((options.prec / 2) as f64 + 13.0 / 0.7);
                    let n3 = do_math_with_var(
                        func.clone(),
                        options,
                        func_vars.clone(),
                        &var,
                        Num(Number::from(
                            point.clone() + if right { h3 } else { -h3 },
                            xunits,
                        )),
                    )?
                    .num()?
                    .number;
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
                        match (n3r > n2r && n2r > n1r, n3i > n2i && n2i > n1i)
                        {
                            (true, true) =>
                            {
                                match (n1.real().is_sign_positive(), n1.imag().is_sign_positive())
                                {
                                    (true, true) =>
                                    {
                                        Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                    (true, false) => Complex::with_val(
                                        options.prec,
                                        (Infinity, -Float::with_val(options.prec, Infinity)),
                                    ),
                                    (false, true) => Complex::with_val(
                                        options.prec,
                                        (-Float::with_val(options.prec, Infinity), Infinity),
                                    ),
                                    (false, false) =>
                                    {
                                        -Complex::with_val(options.prec, (Infinity, Infinity))
                                    }
                                }
                            }
                            (true, false) =>
                            {
                                if n1.real().is_sign_positive()
                                {
                                    Complex::with_val(
                                        options.prec,
                                        (
                                            Infinity,
                                            if (n1.imag() - n2.imag().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                n2.imag().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                        ),
                                    )
                                }
                                else
                                {
                                    -Complex::with_val(
                                        options.prec,
                                        (
                                            Infinity,
                                            if (n1.imag() - n2.imag().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                -n2.imag().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                        ),
                                    )
                                }
                            }
                            (false, true) =>
                            {
                                if n1.imag().is_sign_positive()
                                {
                                    Complex::with_val(
                                        options.prec,
                                        (
                                            if (n1.real() - n2.real().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                n2.real().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                            Infinity,
                                        ),
                                    )
                                }
                                else
                                {
                                    -Complex::with_val(
                                        options.prec,
                                        (
                                            if (n1.real() - n2.real().clone()).abs().log2()
                                                < options.prec as i32 / -16
                                            {
                                                -n2.real().clone()
                                            }
                                            else
                                            {
                                                Float::new(options.prec)
                                            },
                                            Infinity,
                                        ),
                                    )
                                }
                            }
                            (false, false) => Complex::with_val(options.prec, Nan),
                        }
                    }
                },
                units,
            )))
        }
        (Vector(n1), Vector(n2)) =>
        {
            let mut n3: Vec<Number> = Vec::new();
            let mut vec = Vec::with_capacity(n1.len());
            for (i, (n1, n2)) in n1.iter().zip(n2).enumerate()
            {
                let units = n1.units;
                let n1 = &n1.number;
                let n2 = &n2.number;
                vec.push(Number::from(
                    if (n2.clone() - n1.clone()).abs().real().clone().log2()
                        < options.prec as i32 / -16
                    {
                        n1.clone()
                    }
                    else
                    {
                        if n3.is_empty()
                        {
                            let h3 = Complex::with_val(options.prec, 0.5)
                                .pow((options.prec / 2) as f64 + 13.0 / 0.7);
                            n3 = do_math_with_var(
                                func.clone(),
                                options,
                                func_vars.clone(),
                                &var,
                                Num(Number::from(point.clone() - h3.clone(), xunits)),
                            )?
                            .vec()?;
                        }
                        let n3 = &n3[i].number;
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
                            match (n3r > n2r && n2r > n1r, n3i > n2i && n2i > n1i)
                            {
                                (true, true) =>
                                {
                                    match (
                                        n1.real().is_sign_positive(),
                                        n1.imag().is_sign_positive(),
                                    )
                                    {
                                        (true, true) =>
                                        {
                                            Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                        (true, false) => Complex::with_val(
                                            options.prec,
                                            (Infinity, -Float::with_val(options.prec, Infinity)),
                                        ),
                                        (false, true) => Complex::with_val(
                                            options.prec,
                                            (-Float::with_val(options.prec, Infinity), Infinity),
                                        ),
                                        (false, false) =>
                                        {
                                            -Complex::with_val(options.prec, (Infinity, Infinity))
                                        }
                                    }
                                }
                                (true, false) =>
                                {
                                    if n1.real().is_sign_positive()
                                    {
                                        Complex::with_val(
                                            options.prec,
                                            (
                                                Infinity,
                                                if (n2i - n3i.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    n3i
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                            ),
                                        )
                                    }
                                    else
                                    {
                                        -Complex::with_val(
                                            options.prec,
                                            (
                                                Infinity,
                                                if (n2i - n3i.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    -n3i
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                            ),
                                        )
                                    }
                                }
                                (false, true) =>
                                {
                                    if n1.imag().is_sign_positive()
                                    {
                                        Complex::with_val(
                                            options.prec,
                                            (
                                                if (n2r - n3r.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    n3r
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                                Infinity,
                                            ),
                                        )
                                    }
                                    else
                                    {
                                        -Complex::with_val(
                                            options.prec,
                                            (
                                                if (n2r - n3r.clone()).abs().log2()
                                                    < options.prec as i32 / -16
                                                {
                                                    -n3r
                                                }
                                                else
                                                {
                                                    Float::new(options.prec)
                                                },
                                                Infinity,
                                            ),
                                        )
                                    }
                                }
                                (false, false) => Complex::with_val(options.prec, Nan),
                            }
                        }
                    },
                    units,
                ))
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
        let e: Float = -Float::with_val(z.prec().0, -1).exp();
        if z.real() == &e
        {
            return Complex::with_val(z.prec(), -1);
        }
    }
    let mut w = initpoint(z.clone(), k);
    for _ in 0..(z.prec().0 / 64).max(8)
    {
        let zexp = w.clone().exp();
        let zexpz = w.clone() * zexp.clone();
        let zexpz_d = zexp.clone() + zexpz.clone();
        let zexpz_dd = (2 * zexp) + zexpz.clone();
        w -= 2 * ((zexpz.clone() - z.clone()) * zexpz_d.clone())
            / ((2 * zexpz_d.pow(2)) - ((zexpz - z.clone()) * zexpz_dd))
    }
    w
}
fn initpoint(z: Complex, k: isize) -> Complex
{
    {
        let e = Float::with_val(z.prec().0, 1).exp();
        let test: Complex = z.clone() + e.clone().recip();
        if test.abs().real() <= &1.005
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
        if test.abs().real() <= &0.5005
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
    let two_pi_k_i = Complex::with_val(z.prec(), (0, 2 * Float::with_val(z.prec().0, Pi) * k));
    let zln = z.clone().ln() + two_pi_k_i;
    zln.clone() - zln.ln()
}
