use crate::{
    complex::NumStr::{
        Comma, Division, Exponent, Func, LeftBracket, LeftCurlyBracket, Matrix, Minus,
        Multiplication, Num, Plus, RightBracket, RightCurlyBracket, Vector,
    },
    math::do_math,
    misc::{do_math_with_var, place_funcvar, place_var},
    parse::simplify,
    Number, Options, Units,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rug::{
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    integer::IsPrime,
    ops::Pow,
    Complex, Float, Integer,
};
use std::cmp::Ordering;
#[derive(Clone, PartialEq)]
pub enum NumStr
{
    Num(Number),
    Func(String),
    Vector(Vec<Number>),
    Matrix(Vec<Vec<Number>>),
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Comma,
    Plus,
    Minus,
    PlusMinus,
    Multiplication,
    Division,
    InternalMultiplication,
    Tetration,
    Root,
    Exponent,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    Modulo,
    Range,
    Conversion,
    NearEqual,
    ShiftLeft,
    ShiftRight,
    And,
    Or,
    Not,
    Xor,
    Nand,
    Implies,
    Nor,
    Converse,
}
impl Number
{
    pub fn from(number: Complex, units: Option<Units>) -> Number
    {
        Self { number, units }
    }
    pub fn set_prec(&mut self, prec: u32)
    {
        self.number.set_prec(prec)
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
pub fn set_prec(function: &mut [NumStr], func_vars: &mut [(String, Vec<NumStr>)], prec: u32)
{
    function.iter_mut().for_each(|n| n.set_prec(prec));
    func_vars
        .iter_mut()
        .for_each(|(_, f)| f.iter_mut().for_each(|n| n.set_prec(prec)));
}
impl NumStr
{
    pub fn set_prec(&mut self, prec: u32)
    {
        match self
        {
            Num(n) => n.set_prec(prec),
            Vector(v) => v.iter_mut().for_each(|n| n.set_prec(prec)),
            Matrix(m) => m
                .iter_mut()
                .for_each(|v| v.iter_mut().for_each(|n| n.set_prec(prec))),
            _ =>
            {}
        }
    }
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
            (Matrix(a), Vector(b)) if a.iter().all(|a| a.len() == b.len()) => Vector(
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
            (Matrix(a), Matrix(b)) if a.iter().all(|a| a.len() == b.len()) => Matrix(
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
                    .map(|b| add(b, a))
                    .chain(b.iter().map(|b| sub(b, a)))
                    .collect(),
            ),
            (Vector(a), Vector(b)) if a.len() == b.len() => Vector(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| add(a, b))
                    .chain(a.iter().zip(b.iter()).map(|(a, b)| sub(a, b)))
                    .collect(),
            ),
            (Matrix(a), Num(b)) => Vector(
                a.iter()
                    .flat_map(|a| {
                        a.iter()
                            .map(|a| add(a, b))
                            .chain(a.iter().map(|a| sub(a, b)))
                            .collect::<Vec<Number>>()
                    })
                    .collect::<Vec<Number>>(),
            ),
            (Num(b), Matrix(a)) => Vector(
                a.iter()
                    .flat_map(|a| {
                        a.iter()
                            .map(|a| add(b, a))
                            .chain(a.iter().map(|a| sub(b, a)))
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
                        pow_nth(a, b)
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
            (Matrix(a), Num(b)) if a.iter().all(|c| a.len() == c.len()) =>
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
            (Matrix(a), Vector(b)) if a.iter().all(|a| a.len() == b.len()) => Matrix(
                a.iter()
                    .map(|a| a.iter().zip(b.iter()).map(|(a, b)| p(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a[0].len() == b[0].len() && a.len() == b.len() => Matrix(
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
            (Matrix(a), Vector(b)) if a.iter().all(|a| a.len() == b.len()) => Matrix(
                a.iter()
                    .map(|a| a.iter().zip(b.iter()).map(|(a, b)| func(a, b)).collect())
                    .collect(),
            ),
            (Matrix(a), Matrix(b)) if a[0].len() == b[0].len() && a.len() == b.len() => Matrix(
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
            Func(s2) => s == s2,
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
pub fn xor(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    Number::from(
        Complex::with_val(
            a.prec(),
            (a.imag().is_zero() && b.imag().is_zero() && ((a.real() == &1) ^ (b.real() == &1)))
                as u8,
        ),
        None,
    )
}
pub fn nand(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    Number::from(
        Complex::with_val(
            a.prec(),
            (a.imag().is_zero() && b.imag().is_zero() && (a.real() != &1 || b.real() != &1)) as u8,
        ),
        None,
    )
}
pub fn nor(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    Number::from(
        Complex::with_val(
            a.prec(),
            (a.imag().is_zero() && b.imag().is_zero() && (a.real() != &1 && b.real() != &1)) as u8,
        ),
        None,
    )
}
pub fn implies(a: &Number, b: &Number) -> Number
{
    let a = &a.number;
    let b = &b.number;
    Number::from(
        Complex::with_val(
            a.prec(),
            (a.imag().is_zero() && b.imag().is_zero() && (a.real() != &1 || b.real() == &1)) as u8,
        ),
        None,
    )
}
pub fn not(a: &NumStr) -> Result<NumStr, &'static str>
{
    match a
    {
        Num(a) =>
        {
            let a = &a.number;
            Ok(Num(Number::from(
                Complex::with_val(a.prec(), (a.imag().is_zero() && a.real() != &1) as u8),
                None,
            )))
        }
        Vector(v) =>
        {
            let mut o = Vec::new();
            for a in v
            {
                o.push(Number::from(
                    Complex::with_val(a.number.prec(), a.number.is_zero() && a.number.real() != &1),
                    None,
                ));
            }
            Ok(Vector(o))
        }
        Matrix(m) =>
        {
            let mut k = Vec::new();
            for v in m
            {
                let mut o = Vec::new();
                for a in v
                {
                    o.push(Number::from(
                        Complex::with_val(
                            a.number.prec(),
                            a.number.is_zero() && a.number.real() != &1,
                        ),
                        None,
                    ));
                }
                k.push(o)
            }
            Ok(Matrix(k))
        }
        _ => Err("bad not input"),
    }
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
            if b == 2
            {
                a.sqrt()
            }
            else if b.imag().is_zero()
                && b.real().clone() % 2 == 0
                && b.real().clone().fract().is_zero()
                && a.imag().is_zero()
                && a.real().is_sign_negative()
            {
                -pow_nth(-a, b.recip())
            }
            else
            {
                pow_nth(a, b.recip())
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
        a.number.clone() * pow_nth(Complex::with_val(a.number.prec(), 2), b.number.clone()),
        None,
    )
}
pub fn shr(a: &Number, b: &Number) -> Number
{
    Number::from(
        a.number.clone() * pow_nth(Complex::with_val(a.number.prec(), 2), -b.number.clone()),
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
pub fn about_eq(a: &Number, b: &Number) -> Number
{
    let ua = a.units;
    let ub = b.units;
    let a = a.number.clone();
    let b = b.number.clone();
    Number::from(
        Complex::with_val(
            a.prec(),
            ((a.real().is_sign_positive() == b.real().is_sign_positive()
                && a.imag().is_sign_positive() == b.imag().is_sign_positive()
                && a.real().clone().abs().log10().floor()
                    == b.real().clone().abs().log10().floor()
                && a.imag().clone().abs().log10().floor()
                    == b.imag().clone().abs().log10().floor())
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
    let oop = z.prec();
    let op = oop.0 / 2;
    let prec = n * op;
    z.set_prec(prec);
    let h: Float = Float::with_val(prec, 0.5).pow(op / 2);
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
    let mut n = sum * Float::with_val(prec, 2).pow(op / 2 * n);
    n.set_prec(oop);
    n
}
pub fn gamma(a: Complex) -> Complex
{
    if !a.imag().is_zero()
    {
        gamma0(a)
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
            if b == 0
            {
                Complex::with_val(a.prec(), 1)
            }
            else if b.real().is_sign_positive()
            {
                (1..b
                    .real()
                    .to_integer()
                    .unwrap_or_default()
                    .to_usize()
                    .unwrap_or_default())
                    .fold(a.clone(), |tetration, _| pow_nth(a.clone(), tetration))
            }
            else if b == -1
            {
                Complex::new(a.prec())
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
        pow_nth(a.clone(), tetration_recursion(a, b - 1))
    }
    else if b.real() <= &-1
    {
        tetration_recursion(a.clone(), b + 1).ln() / a.ln()
    }
    else
    {
        let aln = a.abs().clone().ln();
        1 + b.clone() * (aln.clone() * (2 + b.clone()) - b) / (1 + aln)
    }
}
pub fn slog(a: &Complex, b: &Complex) -> Complex
{
    if b.real() <= &0
    {
        let z = &pow_nth(a.clone(), b.clone());
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
        b.clone() * (2 * a.clone() + b * (1 - a.clone())) / (1 + a) - 1
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
        let abs: Complex = sqr(a.clone()) + sqr(b.clone());
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
        a
    }
    else if a.len() == 2
    {
        if a[1].number.is_zero()
        {
            if a[0].number.is_zero()
            {
                vec![
                    Number::from(Complex::new(a[0].number.prec()), None),
                    Number::from(
                        Complex::new(a[0].number.prec()),
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
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
            let mut n: Complex = sqr(a[0].number.clone()) + sqr(a[1].number.clone());
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
            let nxy: Complex = sqr(a[0].number.clone()) + sqr(a[1].number.clone());
            let mut n: Complex = nxy.clone() + sqr(a[2].number.clone());
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
        let nxy: Complex = sqr(a[0].number.clone()) + sqr(a[1].number.clone());
        let mut n: Complex = nxy.clone() + sqr(a[2].number.clone());
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
pub fn to_cyl(mut a: Vec<Number>, to_deg: Complex) -> Vec<Number>
{
    if a.len() == 1
    {
        a.push(Number::from(Complex::new(a[0].number.prec()), None));
    }
    if a.len() != 2 && a.len() != 3
    {
        a
    }
    else if a.len() == 2
    {
        if a[1].number.is_zero()
        {
            if a[0].number.is_zero()
            {
                vec![
                    Number::from(Complex::new(a[0].number.prec()), None),
                    Number::from(
                        Complex::new(a[0].number.prec()),
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
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
            let mut n: Complex = sqr(a[0].number.clone()) + sqr(a[1].number.clone());
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
                    Number::from(
                        Complex::new(a[0].number.prec()),
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
                    Number::from(Complex::new(a[0].number.prec()), None),
                ]
            }
            else
            {
                vec![
                    Number::from(Complex::new(a[0].number.prec()), None),
                    Number::from(
                        Complex::new(a[0].number.prec()),
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    ),
                    Number::from(a[2].number.clone().abs(), a[2].units),
                ]
            }
        }
        else
        {
            let nxy: Complex = sqr(a[0].number.clone()) + sqr(a[1].number.clone());
            vec![
                Number::from(nxy.sqrt(), a[0].units),
                Number::from(
                    atan(a[0].number.clone(), a[1].number.clone()) * to_deg.clone(),
                    Some(Units {
                        angle: 1.0,
                        ..Units::default()
                    }),
                ),
                a[2].clone(),
            ]
        }
    }
    else
    {
        let nxy: Complex = sqr(a[0].number.clone()) + sqr(a[1].number.clone());
        vec![
            Number::from(nxy.sqrt(), a[0].units),
            Number::from(
                atan(a[0].number.clone(), a[1].number.clone()) * to_deg.clone(),
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            a[2].clone(),
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
    mut start: Float,
    mut end: Float,
    product: bool,
    options: Options,
) -> Result<NumStr, &'static str>
{
    if start > end
    {
        (start, end) = (end, start)
    }
    if end.is_infinite()
    {
        let mut j = 0;
        let mut n = start
            .to_integer()
            .unwrap_or_default()
            .to_isize()
            .unwrap_or_default();
        let k = if end.is_sign_positive() { 1 } else { -1 };
        let mut last = Num(Number::from(Complex::with_val(options.prec, Nan), None));
        let mut value = do_math_with_var(
            function.clone(),
            options,
            func_vars.clone(),
            var,
            Num(Number::from(Complex::with_val(options.prec, n), None)),
        )?;
        while last != value
        {
            if j > 10000
            {
                return Ok(Num(Number::from(
                    Complex::with_val(options.prec, Nan),
                    None,
                )));
            }
            j += 1;
            n += k;
            last = value;
            let math = do_math_with_var(
                function.clone(),
                options,
                func_vars.clone(),
                var,
                Num(Number::from(Complex::with_val(options.prec, n), None)),
            )?;
            if product
            {
                value = last.mul(&math)?;
            }
            else
            {
                value = last.func(&math, add)?;
            }
        }
        Ok(value)
    }
    else if start.is_infinite()
    {
        return Err("unsupported due to lack of example for convergence");
    }
    else
    {
        let start = start
            .to_integer()
            .unwrap_or_default()
            .to_isize()
            .unwrap_or_default();
        let end = end
            .to_integer()
            .unwrap_or_default()
            .to_isize()
            .unwrap_or_default();
        let mut value = do_math_with_var(
            function.clone(),
            options,
            func_vars.clone(),
            var,
            Num(Number::from(Complex::with_val(options.prec, start), None)),
        )?;
        for z in start + 1..=end
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
pub fn trace(a: &[Vec<Number>]) -> Number
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
    Number::from(n, a[0][0].units)
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
            a[0][0].units.map(|b| b.pow(a.len() as f64)),
        ))
    }
    else
    {
        Err("not square")
    }
}
pub fn transpose(a: &[Vec<Number>]) -> Vec<Vec<Number>>
{
    let mut max = 0;
    for i in a
    {
        if i.len() > max
        {
            max = i.len()
        }
    }
    let mut b = vec![vec![Number::from(Complex::new(1), None); a.len()]; max];
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
    if a.iter().all(|j| a.len() == j.len())
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
    if a.iter().all(|j| a.len() == j.len()) && !a.is_empty()
    {
        let mut result = vec![vec![Number::from(Complex::new(1), None); a[0].len()]; a.len()];
        for (i, k) in result.iter_mut().enumerate()
        {
            for (j, l) in k.iter_mut().enumerate()
            {
                *l = if (i + j) % 2 == 1
                {
                    let a = determinant(&submatrix(a, i, j))?;
                    Number::from(-a.number, a.units)
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
pub fn nth_prime(n: Integer) -> Integer
{
    let mut count = Integer::from(1);
    let mut num = Integer::from(3);
    if n == 0
    {
        num = Integer::new()
    }
    else if n == 1
    {
        num = Integer::from(2);
    }
    while count < n
    {
        if num.is_probably_prime(100) != IsPrime::No
        {
            count += 1;
        }
        if count < n
        {
            num += 2;
        }
    }
    num
}
pub fn prime_factors(mut n: Integer) -> Vec<(Integer, isize)>
{
    if n < 2
    {
        return Vec::new();
    }
    let mut mat = Vec::new();
    let mut prime = Integer::from(2);
    let mut k = 0;
    loop
    {
        let (temp, rem) = n.clone().div_rem(prime.clone());
        if rem == 0
        {
            k += 1;
            if temp == 1
            {
                mat.push((prime.clone(), k));
                break;
            }
            n = temp;
        }
        else
        {
            if k != 0
            {
                mat.push((prime.clone(), k));
            }
            prime = prime.next_prime();
            k = 0;
        }
    }
    mat
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
pub fn sort_mat(mut a: Vec<Vec<Number>>, prec: u32) -> Vec<Vec<Number>>
{
    a.sort_by(|x, y| {
        if x.is_empty() || y.is_empty()
        {
            Ordering::Equal
        }
        else
        {
            x.iter()
                .fold(Float::new(prec), |sum, val| sum + val.number.real())
                .partial_cmp(
                    &y.iter()
                        .fold(Float::new(prec), |sum, val| sum + val.number.real()),
                )
                .unwrap_or(Ordering::Equal)
        }
    });
    a.sort_by(|x, y| x.len().partial_cmp(&y.len()).unwrap_or(Ordering::Equal));
    a
}
pub fn eigenvalues(mat: &[Vec<Number>], real: bool) -> Result<NumStr, &'static str>
{
    if !mat.is_empty() && (0..mat.len()).all(|j| mat.len() == mat[j].len())
    {
        match mat.len()
        {
            1 => Ok(Num(mat[0][0].clone())),
            2 =>
            {
                let pr = mat[0][0].number.prec().0;
                let mut mat = Matrix(mat.into());
                mat.set_prec(pr * 2);
                let mat = mat.mat()?;
                let mut v = Vector(quadratic(
                    Number::from(Complex::with_val(mat[0][0].number.prec(), 1), None),
                    Number::from(
                        -mat[0][0].number.clone() - mat[1][1].number.clone(),
                        mat[0][0].units,
                    ),
                    Number::from(
                        mat[0][0].number.clone() * mat[1][1].number.clone()
                            - mat[0][1].number.clone() * mat[1][0].number.clone(),
                        mat[0][0].units.map(|a| a.pow(2.0)),
                    ),
                    real,
                ));
                v.set_prec(pr);
                Ok(v)
            }
            3 =>
            {
                let pr = mat[0][0].number.prec().0;
                let mut mat = Matrix(mat.into());
                mat.set_prec(pr * 2);
                let mat = mat.mat()?;
                let mut v = Vector(cubic(
                    Number::from(Complex::with_val(mat[0][0].number.prec(), -1), None),
                    Number::from(
                        mat[2][2].number.clone()
                            + mat[1][1].number.clone()
                            + mat[0][0].number.clone(),
                        mat[0][0].units,
                    ),
                    Number::from(
                        -mat[0][0].number.clone() * mat[1][1].number.clone()
                            - mat[0][0].number.clone() * mat[2][2].number.clone()
                            + mat[0][1].number.clone() * mat[1][0].number.clone()
                            + mat[0][2].number.clone() * mat[2][0].number.clone()
                            - mat[1][1].number.clone() * mat[2][2].number.clone()
                            + mat[1][2].number.clone() * mat[2][1].number.clone(),
                        mat[0][0].units.map(|a| a.pow(2.0)),
                    ),
                    Number::from(
                        mat[0][0].number.clone()
                            * mat[1][1].number.clone()
                            * mat[2][2].number.clone()
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
                        mat[0][0].units.map(|a| a.pow(3.0)),
                    ),
                    real,
                ));
                v.set_prec(pr);
                Ok(v)
            }
            4 =>
            {
                let pr = mat[0][0].number.prec().0;
                let mut mat = Matrix(mat.into());
                mat.set_prec(pr * 2);
                let mat = mat.mat()?;
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
                let mut v = Vector(quartic(
                    Number::from(Complex::with_val(a.prec(), 1), None),
                    Number::from(
                        -a.clone() - f.clone() - k.clone() - q.clone(),
                        mat[0][0].units,
                    ),
                    Number::from(
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
                        mat[0][0].units.map(|a| a.pow(2.0)),
                    ),
                    Number::from(
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
                        mat[0][0].units.map(|a| a.pow(3.0)),
                    ),
                    Number::from(
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
                        mat[0][0].units.map(|a| a.pow(4.0)),
                    ),
                    real,
                ));
                v.set_prec(pr);
                Ok(v)
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
            2..5 =>
            {
                let p = mat[0][0].number.prec().0;
                let mut l = eigenvalues(mat, real)?.vec()?;
                let mut i = 0;
                while i + 1 < l.len()
                {
                    let mut has = false;
                    for v in l[i + 1..].iter().cloned()
                    {
                        if (v.number - l[i].number.clone()).abs().real().clone().log2()
                            < -(p as i32 / 8)
                        {
                            l.remove(i);
                            has = true;
                            break;
                        }
                    }
                    if !has
                    {
                        i += 1;
                    }
                }
                let v = l
                    .iter()
                    .filter_map(|l| {
                        Matrix(identity(mat.len(), l.number.prec().0))
                            .mul(&Num(l.clone()))
                            .map(|n| {
                                Matrix(mat.to_vec())
                                    .func(&n, sub)
                                    .map(|m| Some(kernel(m.mat().unwrap()).unwrap()))
                                    .unwrap_or(None)
                            })
                            .unwrap_or(None)
                    })
                    .flatten()
                    .collect::<Vec<Vec<Number>>>();
                Ok(Matrix(v))
            }
            _ => Err("unsupported"),
        }
    }
    else
    {
        Err("not square")
    }
}
pub fn rcf(mat: Vec<Vec<Number>>) -> Result<NumStr, &'static str>
{
    Err("")
}
pub fn jcf(mat: Vec<Vec<Number>>) -> Result<NumStr, &'static str>
{
    let pr = mat[0][0].number.prec().0;
    let l = mat.len();
    let beta = transpose(&generalized_eigenvectors(&mat, false)?.mat()?);
    let i = identity(l, pr);
    let mut o = i.clone();
    let mut d = change_basis(mat, &i, &beta)?.mat()?;
    for (i, r) in o.iter_mut().enumerate()
    {
        r[i].number = d[i..d.len() - 1]
            .iter()
            .enumerate()
            .map_while(|(j, r)| {
                if -r[i + j + 1].number.clone().abs().real().clone().ln() < pr / 16
                {
                    Some(r[i + j + 1].number.clone())
                }
                else
                {
                    None
                }
            })
            .fold(Complex::with_val(pr, 1), |sum, val| sum * val.clone());
    }
    let l = o.len();
    for k in 1..l
    {
        d = change_basis(d, &i, &o)?.mat()?;
        o = i.clone();
        let mut sum = Complex::new(pr);
        for (i, r) in o[1..l - k].iter_mut().enumerate()
        {
            let i = i + 1;
            sum += d[i - 1][i + k].number.clone();
            r[i + k].number = -sum.clone();
        }
    }
    change_basis(d, &i, &o)
}
pub fn generalized_eigenvectors(mat: &[Vec<Number>], real: bool) -> Result<NumStr, &'static str>
{
    if !mat.is_empty() && (0..mat.len()).all(|j| mat.len() == mat[j].len())
    {
        let one = Number::from(Complex::with_val(mat[0][0].number.prec(), 1), None);
        match mat.len()
        {
            1 => Ok(Num(one)),
            2..5 =>
            {
                let p = mat[0][0].number.prec().0;
                let mut l = eigenvalues(mat, real)?.vec()?;
                let mut i = 0;
                while i + 1 < l.len()
                {
                    let mut has = false;
                    for v in l[i + 1..].iter().cloned()
                    {
                        if (v.number - l[i].number.clone()).abs().real().clone().log2()
                            < -(p as i32 / 8)
                        {
                            l.remove(i);
                            has = true;
                            break;
                        }
                    }
                    if !has
                    {
                        i += 1;
                    }
                }
                let v = l
                    .iter()
                    .filter_map(|l| {
                        Matrix(identity(mat.len(), l.number.prec().0))
                            .mul(&Num(l.clone()))
                            .map(|n| {
                                Matrix(mat.to_vec())
                                    .func(&n, sub)
                                    .map(|m| {
                                        m.pow(&Num(Number::from(
                                            Complex::with_val(p, mat.len()),
                                            None,
                                        )))
                                        .map(|m| Some(kernel(m.mat().unwrap()).unwrap()))
                                        .unwrap_or(None)
                                    })
                                    .unwrap_or(None)
                            })
                            .unwrap_or(None)
                    })
                    .flatten()
                    .collect::<Vec<Vec<Number>>>();
                Ok(Matrix(v))
            }
            _ => Err("unsupported"),
        }
    }
    else
    {
        Err("not square")
    }
}
pub fn change_basis(
    a: Vec<Vec<Number>>,
    beta: &[Vec<Number>],
    gamma: &[Vec<Number>],
) -> Result<NumStr, &'static str>
{
    let m = Matrix(a);
    let tn = Matrix(inverse(&transpose(gamma))?);
    let mut c = Vec::new();
    for b in beta
    {
        c.push(tn.mul(&Vector(b.to_vec()))?.vec()?)
    }
    let tn = Matrix(inverse(&transpose(beta))?);
    let mut d = Vec::new();
    for g in gamma
    {
        d.push(tn.mul(&Vector(g.to_vec()))?.vec()?)
    }
    Matrix(c).mul(&m)?.mul(&Matrix(d))
}
pub fn coordinate(v: Vec<Number>, beta: Vec<Vec<Number>>) -> Result<NumStr, &'static str>
{
    let tn = Matrix(inverse(&transpose(&beta))?);
    tn.mul(&Vector(v))
}
pub fn rref(mut a: Vec<Vec<Number>>) -> Result<Vec<Vec<Number>>, &'static str>
{
    if a.is_empty() || a[0].is_empty() || a.iter().any(|b| a[0].len() != b.len())
    {
        return Err("invalid matrix");
    }
    let mut count = 0;
    for i in 0..a[0].len()
    {
        if let Some((n, v)) = a
            .clone()
            .iter()
            .enumerate()
            .find(|(j, b)| *j >= count && !b[i].number.is_zero())
        {
            for (a, r) in a.iter_mut().enumerate()
            {
                let c = r[i].number.clone() / v[i].number.clone();
                for (b, num) in r.iter_mut().enumerate()
                {
                    if a != n && !v[i].number.is_zero()
                    {
                        num.number -= v[b].number.clone() * c.clone();
                    }
                    else
                    {
                        num.number /= v[i].number.clone()
                    }
                }
            }
            if n != count
            {
                a.swap(count, n);
            }
            count += 1;
        }
    }
    Ok(a.to_vec())
}
pub fn kernel(a: Vec<Vec<Number>>) -> Result<Vec<Vec<Number>>, &'static str>
{
    let rref = rref(a)?;
    let mut ker = Vec::new();
    let mut leading_ones = Vec::new();
    for r in &rref
    {
        if let Some((pos, _)) = r.iter().enumerate().find(|(_, n)| !n.number.is_zero())
        {
            leading_ones.push(pos);
        }
    }
    let t = transpose(&rref);
    for (i, t) in t.iter().enumerate()
    {
        if !leading_ones.contains(&i)
        {
            let mut zero =
                vec![Number::from(Complex::new(rref[0][0].number.prec().0), None); rref[0].len()];
            for j in 0..i.min(leading_ones.len())
            {
                if leading_ones[j] < i
                {
                    zero[leading_ones[j]] = Number::from(-1.0 * t[j].number.clone(), None)
                }
            }
            zero[i] = Number::from(Complex::with_val(rref[0][0].number.prec().0, 1), None);
            ker.push(zero);
        }
    }
    Ok(ker)
}
pub fn range(a: Vec<Vec<Number>>) -> Result<Vec<Vec<Number>>, &'static str>
{
    let rref = rref(a.clone())?;
    let mut ran = Vec::new();
    let mut leading_ones = Vec::new();
    for r in &rref
    {
        if let Some((pos, _)) = r.iter().enumerate().find(|(_, n)| !n.number.is_zero())
        {
            leading_ones.push(pos);
        }
    }
    let t = transpose(&a);
    for i in leading_ones
    {
        ran.push(t[i].clone());
    }
    Ok(ran)
}
pub fn mul_units(a: Option<Units>, b: Option<Units>) -> Option<Units>
{
    match (a, b)
    {
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(a.mul(&b)),
        _ => None,
    }
}
pub fn div_units(a: Option<Units>, b: Option<Units>) -> Option<Units>
{
    match (a, b)
    {
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b.pow(-1.0)),
        (Some(a), Some(b)) => Some(a.div(&b)),
        _ => None,
    }
}
pub fn quadratic(a: Number, b: Number, c: Number, real: bool) -> Vec<Number>
{
    if a.number.is_zero()
    {
        return if b.number.is_zero()
        {
            vec![Number::from(Complex::new(a.number.prec()), None)]
        }
        else
        {
            let units = div_units(c.units, b.units);
            let b = b.number;
            let c = c.number;
            let mut r = -c / b;
            if -r.imag().clone().abs().log10() > a.number.prec().0 / 4
            {
                r = r.real().clone().into();
            }
            if real && !r.imag().is_zero()
            {
                vec![Number::from(Complex::with_val(a.number.prec(), Nan), None)]
            }
            else
            {
                vec![Number::from(r, units)]
            }
        };
    }
    let units = div_units(c.units, a.units).map(|a| a.root(2.0));
    let a = a.number;
    let b = b.number;
    let c = c.number;
    let p: Complex = sqr(b.clone());
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
            vec.push(Number::from(z1, units))
        }
        if z2.imag().is_zero()
        {
            vec.push(Number::from(z2, units))
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
        vec![Number::from(z1, units), Number::from(z2, units)]
    }
}
pub fn cubic(a: Number, b: Number, c: Number, d: Number, real: bool) -> Vec<Number>
{
    let units = div_units(d.units, a.units).map(|a| a.root(3.0));
    if a.number.is_zero()
    {
        return quadratic(b, c, d, real);
    }
    let d = d.number;
    if d.is_zero()
    {
        let mut vec = quadratic(a, b, c, real);
        vec.push(Number::from(Complex::new(d.prec()), vec[0].units));
        return vec;
    }
    let a = a.number;
    let b = b.number;
    let c = c.number;
    let prec = a.prec();
    let threerecip = Float::with_val(prec.0, 3).recip();
    if b.is_zero() && c.is_zero()
    {
        let reuse = pow_nth(d / a.clone(), threerecip.clone().into());
        let mut z1 = -reuse.clone();
        let mut z2 = reuse.clone() * Float::with_val(prec.0, -1).pow(threerecip.clone());
        let mut z3: Complex = -reuse * Float::with_val(prec.0, -1).pow(2 * threerecip);
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
                vec.push(Number::from(z1, units))
            }
            if z2.imag().is_zero()
            {
                vec.push(Number::from(z2, units))
            }
            if z3.imag().is_zero()
            {
                vec.push(Number::from(z3, units))
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
                Number::from(z1, units),
                Number::from(z2, units),
                Number::from(z3, units),
            ]
        };
    }
    let b = b / a.clone();
    let c = c / a.clone();
    let d = d / a.clone();
    // https://en.wikipedia.org/wiki/Cubic_equation#General_cubic_formula
    let d0: Complex = sqr(b.clone()) - 3 * c.clone();
    let d1: Complex = 2 * cube(b.clone()) - 9 * b.clone() * c.clone() + 27 * d.clone();
    let c: Complex = sqr(d1.clone()) - 4 * cube(d0.clone());
    let c: Complex = (d1 + c.sqrt()) / 2;
    let c = pow_nth(c, threerecip.clone().into());
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
            vec.push(Number::from(z1, units))
        }
        if z2.imag().is_zero()
        {
            vec.push(Number::from(z2, units))
        }
        if z3.imag().is_zero()
        {
            vec.push(Number::from(z3, units))
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
            Number::from(z1, units),
            Number::from(z2, units),
            Number::from(z3, units),
        ]
    }
}
pub fn quartic(div: Number, b: Number, c: Number, d: Number, e: Number, real: bool) -> Vec<Number>
{
    let units = div_units(e.units, div.units).map(|a| a.root(4.0));
    if e.number.is_zero()
    {
        let mut vec = cubic(div, b, c, d, real);
        vec.push(Number::from(Complex::new(e.number.prec()), vec[0].units));
        return vec;
    }
    let div = div.number;
    if div.is_zero()
    {
        return cubic(b, c, d, e, real);
    }
    let b = b.number;
    let c = c.number;
    let d = d.number;
    let e = e.number;
    let prec = div.prec();
    let threerecip = Float::with_val(prec.0, 3).recip();
    let a = b / div.clone();
    let b = c / div.clone();
    let c = d / div.clone();
    let d = e / div;
    // https://upload.wikimedia.org/wikipedia/commons/9/99/Quartic_Formula.svg
    let alpha: Complex = sqr(b.clone()) - 3 * a.clone() * c.clone() + 12 * d.clone();
    let phi: Complex = 2 * cube(b.clone()) - 9 * a.clone() * b.clone() * c.clone()
        + 27 * sqr(c.clone())
        + 27 * sqr(a.clone()) * d.clone()
        - 72 * b.clone() * d.clone();
    let omega: Complex = -4 * cube(alpha.clone()) + sqr(phi.clone());
    let omega: Complex = phi + omega.sqrt();
    let alpha: Complex = if alpha.is_zero()
    {
        Complex::new(prec)
    }
    else
    {
        Float::with_val(prec.0, 2).pow(threerecip.clone()) * alpha
            / (3 * pow_nth(omega.clone(), threerecip.clone().into()))
    };
    let beta: Complex = omega / 54;
    let beta: Complex = pow_nth(beta, threerecip.clone().into());
    let infirst: Complex = sqr(a.clone()) / 4 - 2 * b.clone() / 3 + alpha.clone() + beta.clone();
    let first: Complex = infirst.clone().sqrt() / 2;
    let third: Complex = -1 * cube(a.clone()) + 4 * a.clone() * b.clone() - 8 * c.clone();
    let third: Complex = if third.is_zero()
    {
        Complex::new(prec)
    }
    else
    {
        third / (first.clone() * 8)
    };
    let a4: Complex = -a.clone() / 4;
    let second: Complex = sqr(a.clone()) / 2 - 4 * b.clone() / 3 - alpha.clone() - beta.clone();
    let secondn: Complex = second.clone() - third.clone();
    let secondn: Complex = secondn.sqrt() / 2;
    let secondp: Complex = second + third.clone();
    let secondp: Complex = secondp.sqrt() / 2;
    let mut r1 = a4.clone() - first.clone() - secondn.clone();
    let mut r2 = a4.clone() - first.clone() + secondn.clone();
    let mut r3 = a4.clone() + first.clone() - secondp.clone();
    let mut r4 = a4.clone() + first.clone() + secondp.clone();
    if -r1.imag().clone().abs().log10() > a.prec().0 / 8
    {
        r1 = r1.real().clone().into();
    }
    if -r2.imag().clone().abs().log10() > a.prec().0 / 8
    {
        r2 = r2.real().clone().into();
    }
    if -r3.imag().clone().abs().log10() > a.prec().0 / 8
    {
        r3 = r3.real().clone().into();
    }
    if -r4.imag().clone().abs().log10() > a.prec().0 / 8
    {
        r4 = r4.real().clone().into();
    }
    if real
    {
        let mut vec = Vec::new();
        if r1.imag().is_zero()
        {
            vec.push(Number::from(r1, units))
        }
        if r2.imag().is_zero()
        {
            vec.push(Number::from(r2, units))
        }
        if r3.imag().is_zero()
        {
            vec.push(Number::from(r3, units))
        }
        if r4.imag().is_zero()
        {
            vec.push(Number::from(r4, units))
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
            Number::from(r1, units),
            Number::from(r2, units),
            Number::from(r3, units),
            Number::from(r4, units),
        ]
    }
}
pub fn variance(a: &[Number], mean: Option<Complex>, prec: u32) -> Number
{
    let mean = if let Some(n) = mean
    {
        n
    }
    else
    {
        a.iter()
            .fold(Complex::new(prec), |sum, val| sum + val.number.clone())
            / a.len()
    };
    let mut variance = Complex::new(prec);
    for a in a
    {
        variance += sqr(a.number.clone() - mean.clone())
    }
    Number::from(
        variance / (a.len().saturating_sub(1)),
        a[0].units.map(|a| a.pow(2.0)),
    )
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
            let mut cont = false;
            let mut bracket = 0;
            let mut pw = Vec::new();
            for f in &func
            {
                match f
                {
                    LeftBracket | LeftCurlyBracket => bracket += 1,
                    RightBracket | RightCurlyBracket =>
                    {
                        bracket -= 1;
                        if !pw.is_empty() && bracket == pw[0]
                        {
                            pw.remove(0);
                        }
                    }
                    Func(s) if s == "pw" => pw.insert(0, bracket),
                    _ =>
                    {
                        if !pw.is_empty() && f.str_is(&fv.0)
                        {
                            cont = true
                        }
                    }
                }
            }
            if cont
            {
                continue;
            }
            if fv.0.contains(',')
            {
                let mut vars = fv.0.split(',').collect::<Vec<&str>>();
                vars[0] = vars[0].split('(').next_back().unwrap();
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
                        let mut bracket = 0;
                        let mut k = 0;
                        let mut processed = Vec::new();
                        let mut last = 0;
                        for (i, n) in func[x + 2..].iter().enumerate()
                        {
                            match n
                            {
                                LeftBracket | LeftCurlyBracket => bracket += 1,
                                RightBracket | RightCurlyBracket =>
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
                                            processed.push(vec![Func(iden)]);
                                        }
                                        k = i;
                                        break;
                                    }
                                    bracket -= 1;
                                }
                                Comma if bracket == 0 =>
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
                                        processed.push(vec![Func(iden)]);
                                    }
                                    last = i + 1;
                                }
                                _ =>
                                {}
                            }
                        }
                        let mut i = 0;
                        while i < fv.1.len()
                        {
                            if let Func(s) = &fv.1[i]
                            {
                                for v in processed.iter().zip(vars.iter())
                                {
                                    if s == v.1
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
                let var = fv.0.split('(').next_back().unwrap();
                let var = &var[0..var.len().saturating_sub(1)];
                let mut x = func.len();
                while x > 0
                {
                    x -= 1;
                    if func[x].str_is(&fv.0)
                    {
                        let mut fv = fv.clone();
                        let mut bracket = 0;
                        let mut k = 0;
                        for (i, n) in func[x + 2..].iter().enumerate()
                        {
                            match n
                            {
                                LeftBracket | LeftCurlyBracket => bracket += 1,
                                RightBracket | RightCurlyBracket =>
                                {
                                    if bracket == 0
                                    {
                                        k = i;
                                    }
                                    bracket -= 1;
                                }
                                _ =>
                                {}
                            }
                        }
                        let iden = format!("@{}{}@", func_vars.len(), var);
                        let mut i = 0;
                        while i < fv.1.len()
                        {
                            if let Func(s) = &fv.1[i]
                            {
                                if *s == var
                                {
                                    fv.1[i] = Func(iden.clone());
                                }
                            }
                            i += 1;
                        }
                        func_vars.push((iden.clone(), func[x + 1..=k + x + 2].to_vec()));
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
        pow_nth(x.clone(), a.clone()) * pow_nth(f, b.clone())
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
    let p2: Complex = sqr(z.clone());
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
        sqr(z.clone()) + iter / (2 * erfc_recursion(z, iter + 1, max))
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
        gamma(s)
    }
    else if s.real().is_sign_positive()
        && !s.real().clone().fract().is_zero()
        && *z.real() <= 0.25
    {
        let p = z.prec().0 as usize / 4;
        gamma(s.clone()) - lower_incomplete_gamma_recursion(s, z, 0, p)
    }
    else
    {
        let p = z.prec().0 as usize / 4;
        incomplete_gamma_recursion(s, z, 0, p)
    }
}
pub fn lower_incomplete_gamma(s: Complex, z: Complex) -> Complex
{
    if s.real().is_sign_positive() && !s.real().clone().fract().is_zero() && *z.real() <= 1
    {
        let p = z.prec().0 as usize / 4;
        lower_incomplete_gamma_recursion(s, z, 0, p)
    }
    else
    {
        gamma(s.clone()) - incomplete_gamma(s, z)
    }
}
pub fn eta(s: Complex) -> Complex
{
    let prec = s.prec().0;
    let mut sum = Complex::new(prec);
    let two = Float::with_val(prec, 2);
    for n in 0..=(prec / 16).max(16)
    {
        let mut innersum = Complex::new(prec);
        let nb = Integer::from(n);
        for k in 0..=n
        {
            let num = nb.clone().binomial(k) * pow_nth(Complex::with_val(prec, 1 + k), -s.clone());
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
    eta(s.clone()) / (1 - pow_nth(Complex::with_val(s.prec(), 2), 1 - s))
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
            let num = binomial(n.clone() + 1, ic) * pow_nth(num, n.clone());
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
        && a.real() >= &0
        && b.real() >= &0
    {
        Complex::with_val(
            a.prec(),
            a.real().to_integer().unwrap_or_default().binomial(
                b.real()
                    .to_integer()
                    .unwrap_or_default()
                    .to_u32()
                    .unwrap_or_default(),
            ),
        )
    }
    else if a.real().is_sign_negative()
        && a.real().clone().fract().is_zero()
        && a.imag().is_zero()
        && b.imag().is_zero()
    {
        let prec = a.prec().0;
        let a = a + Complex::with_val(prec, (0, 1)) * Float::with_val(prec, 0.5).pow(prec / 2);
        (gamma(a.clone() + 1) / (gamma(b.clone() + 1) * gamma(a.clone() - b.clone() + 1)))
            .real()
            .clone()
            .into()
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
        (pow_nth(z.clone(), s.clone()) / z.clone().exp()) / incomplete_gamma_recursion(s, z, 1, max)
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
fn lower_incomplete_gamma_recursion(s: Complex, z: Complex, iter: usize, max: usize) -> Complex
{
    if iter == max
    {
        Complex::with_val(s.prec(), 1)
    }
    else if iter == 0
    {
        (pow_nth(z.clone(), s.clone()) / z.clone().exp())
            / lower_incomplete_gamma_recursion(s, z, 1, max)
    }
    else if iter % 2 == 1
    {
        s.clone() + iter
            - 1
            - ((s.clone() + iter - 1) * z.clone())
                / lower_incomplete_gamma_recursion(s, z, iter + 1, max)
    }
    else
    {
        s.clone() + iter - 1
            + (iter / 2 * z.clone() / lower_incomplete_gamma_recursion(s, z, iter + 1, max))
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
#[allow(clippy::too_many_arguments)]
pub fn surface_area(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    varx: String,
    mut startx: Complex,
    endx: Number,
    vary: String,
    starty_func: Vec<NumStr>,
    endy_func: Vec<NumStr>,
) -> Result<Number, &'static str>
{
    if starty_func.is_empty() || endy_func.is_empty()
    {
        return Err("bad start/end");
    }
    let points = options.prec as usize / 16;
    let unitsx = endx.units;
    let endx = endx.number;
    let deltax: Complex = (endx.clone() - startx.clone()) / (points - 1);
    let mut area: Complex = Complex::new(options.prec);
    if let (Ok(Num(start)), Ok(Num(end))) = (
        do_math(starty_func.clone(), options, func_vars.clone()),
        do_math(endy_func.clone(), options, func_vars.clone()),
    )
    {
        let starty = start.number;
        let endy = end.number;
        let unitsy = end.units;
        let deltay: Complex = (endy.clone() - starty.clone()) / (points - 1);
        let res = {
            let n = Num(Number::from(startx.clone(), unitsx));
            let func = place_var(func.clone(), &varx, n.clone());
            let func_vars = place_funcvar(func_vars.clone(), &varx, n);
            do_math_with_var(
                func.clone(),
                options,
                func_vars.clone(),
                &vary,
                Num(Number::from(starty.clone(), unitsy)),
            )?
        };
        match res
        {
            Num(n) =>
            {
                let mut data: Vec<Vec<Complex>> = vec![Vec::new(); points];
                let units = n.units.map(|n| n.pow(2.0));
                data[0].push(n.number);
                for (nx, row) in data.iter_mut().enumerate()
                {
                    if nx + 1 == points
                    {
                        startx.clone_from(&endx)
                    }
                    else if nx != 0
                    {
                        startx += deltax.clone();
                    }
                    let n = Num(Number::from(startx.clone(), unitsx));
                    let mut func = place_var(func.clone(), &varx, n.clone());
                    let mut func_vars = place_funcvar(func_vars.clone(), &varx, n);
                    simplify(&mut func, &mut func_vars, options);
                    let mut starty = starty.clone();
                    for ny in 0..points
                    {
                        if nx == 0 && ny == 0
                        {
                            continue;
                        }
                        if ny + 1 == points
                        {
                            starty.clone_from(&endy)
                        }
                        else if ny != 0
                        {
                            starty += deltay.clone();
                        }
                        row.push(
                            do_math_with_var(
                                func.clone(),
                                options,
                                func_vars.clone(),
                                &vary,
                                Num(Number::from(starty.clone(), unitsy)),
                            )?
                            .num()?
                            .number,
                        )
                    }
                }
                let k: Complex = sqr(deltax.clone()) * sqr(deltay.clone());
                for (nx, row) in data[..data.len() - 1].iter().enumerate()
                {
                    for (ny, z) in row[..row.len() - 1].iter().enumerate()
                    {
                        let a = row[ny + 1].clone();
                        let b = data[nx + 1][ny].clone();
                        {
                            let a = a.clone() - z;
                            let b = b.clone() - z;
                            let i = deltay.clone() * b;
                            let j = deltax.clone() * a;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            area += (i + j + k.clone()).sqrt() / 2;
                        }
                        {
                            let z = &data[nx + 1][ny + 1];
                            let a = a - z;
                            let b = b - z;
                            let i = deltax.clone() * b;
                            let j = deltay.clone() * a;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            area += (i + j + k.clone()).sqrt() / 2
                        }
                    }
                }
                Ok(Number::from(area, units))
            }
            Vector(v) if v.len() == 3 =>
            {
                let mut data: Vec<Vec<Vec<Complex>>> = vec![Vec::new(); points];
                let units = v[2].units.map(|n| n.pow(2.0));
                data[0].push(v.iter().map(|a| a.number.clone()).collect());
                for (nx, row) in data.iter_mut().enumerate()
                {
                    if nx + 1 == points
                    {
                        startx.clone_from(&endx)
                    }
                    else if nx != 0
                    {
                        startx += deltax.clone();
                    }
                    let n = Num(Number::from(startx.clone(), unitsx));
                    let mut func = place_var(func.clone(), &varx, n.clone());
                    let mut func_vars = place_funcvar(func_vars.clone(), &varx, n);
                    simplify(&mut func, &mut func_vars, options);
                    let mut starty = starty.clone();
                    for ny in 0..points
                    {
                        if nx == 0 && ny == 0
                        {
                            continue;
                        }
                        if ny + 1 == points
                        {
                            starty.clone_from(&endy)
                        }
                        else if ny != 0
                        {
                            starty += deltay.clone();
                        }
                        let v = do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &vary,
                            Num(Number::from(starty.clone(), unitsy)),
                        )?
                        .vec()?;
                        row.push(v.iter().map(|a| a.number.clone()).collect());
                    }
                }
                for (nx, row) in data[..data.len() - 1].iter().enumerate()
                {
                    for (ny, z) in row[..row.len() - 1].iter().enumerate()
                    {
                        let a = row[ny + 1].clone();
                        let b = data[nx + 1][ny].clone();
                        {
                            let a0 = a[0].clone() - z[0].clone();
                            let a1 = a[1].clone() - z[1].clone();
                            let a2 = a[2].clone() - z[2].clone();
                            let b0 = b[0].clone() - z[0].clone();
                            let b1 = b[1].clone() - z[1].clone();
                            let b2 = b[2].clone() - z[2].clone();
                            let i = a1.clone() * b2.clone() - a2.clone() * b1.clone();
                            let j = a2 * b0.clone() - a0.clone() * b2;
                            let k = a0 * b1 - a1 * b0;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            let k: Complex = sqr(k);
                            area += (i + j + k).sqrt() / 2;
                        }
                        {
                            let z = &data[nx + 1][ny + 1];
                            let a0 = a[0].clone() - z[0].clone();
                            let a1 = a[1].clone() - z[1].clone();
                            let a2 = a[2].clone() - z[2].clone();
                            let b0 = b[0].clone() - z[0].clone();
                            let b1 = b[1].clone() - z[1].clone();
                            let b2 = b[2].clone() - z[2].clone();
                            let i = a1.clone() * b2.clone() - a2.clone() * b1.clone();
                            let j = a2 * b0.clone() - a0.clone() * b2;
                            let k = a0 * b1 - a1 * b0;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            let k: Complex = sqr(k);
                            area += (i + j + k).sqrt() / 2;
                        }
                    }
                }
                Ok(Number::from(area, units))
            }
            _ => Err("bad input"),
        }
    }
    else
    {
        let n = Num(Number::from(startx.clone(), unitsx));
        let temp_func = place_var(func.clone(), &varx, n.clone());
        let temp_func_vars = place_funcvar(func_vars.clone(), &varx, n.clone());
        let starty = do_math_with_var(
            starty_func.clone(),
            options,
            temp_func_vars.clone(),
            &varx,
            n.clone(),
        )?
        .num()?
        .number;
        let end = do_math_with_var(
            endy_func.clone(),
            options,
            temp_func_vars.clone(),
            &varx,
            n.clone(),
        )?
        .num()?;
        let mut endy = end.number;
        let unitsy = end.units;
        let mut deltay: Complex = (endy.clone() - starty.clone()) / (points - 1);
        match do_math_with_var(
            temp_func.clone(),
            options,
            temp_func_vars.clone(),
            &vary,
            Num(Number::from(starty.clone(), unitsy)),
        )?
        {
            Num(n) =>
            {
                let mut data: Vec<Vec<[Complex; 2]>> = vec![Vec::new(); points];
                let units = n.units.map(|n| n.pow(2.0));
                data[0].push([starty.clone(), n.number]);
                for (nx, row) in data.iter_mut().enumerate()
                {
                    if nx + 1 == points
                    {
                        startx.clone_from(&endx)
                    }
                    else if nx != 0
                    {
                        startx += deltax.clone();
                    }
                    let n = Num(Number::from(startx.clone(), unitsx));
                    let mut func = place_var(func.clone(), &varx, n.clone());
                    let mut func_vars = place_funcvar(func_vars.clone(), &varx, n.clone());
                    simplify(&mut func, &mut func_vars, options);
                    let mut starty = starty.clone();
                    if nx != 0
                    {
                        starty = do_math_with_var(
                            starty_func.clone(),
                            options,
                            func_vars.clone(),
                            &varx,
                            n.clone(),
                        )?
                        .num()?
                        .number;
                        endy = do_math_with_var(
                            endy_func.clone(),
                            options,
                            func_vars.clone(),
                            &varx,
                            n.clone(),
                        )?
                        .num()?
                        .number;
                        deltay = (endy.clone() - starty.clone()) / (points - 1);
                    }
                    for ny in 0..points
                    {
                        if nx == 0 && ny == 0
                        {
                            continue;
                        }
                        if ny + 1 == points
                        {
                            starty.clone_from(&endy)
                        }
                        else if ny != 0
                        {
                            starty += deltay.clone();
                        }
                        row.push([
                            starty.clone(),
                            do_math_with_var(
                                func.clone(),
                                options,
                                func_vars.clone(),
                                &vary,
                                Num(Number::from(starty.clone(), unitsy)),
                            )?
                            .num()?
                            .number,
                        ])
                    }
                }
                for (nx, row) in data[..data.len() - 1].iter().enumerate()
                {
                    for (ny, z) in row[..row.len() - 1].iter().enumerate()
                    {
                        let a = row[ny + 1].clone();
                        let b = data[nx + 1][ny].clone();
                        {
                            let a1 = a[0].clone() - z[0].clone();
                            let a2 = a[1].clone() - z[1].clone();
                            let b0 = deltax.clone();
                            let b1 = b[0].clone() - z[0].clone();
                            let b2 = b[1].clone() - z[1].clone();
                            let i = a1.clone() * b2.clone() - a2.clone() * b1.clone();
                            let j = a2 * b0.clone();
                            let k = a1 * b0;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            let k: Complex = sqr(k);
                            area += (i + j + k).sqrt() / 2;
                        }
                        {
                            let z = &data[nx + 1][ny + 1];
                            let a0 = deltax.clone();
                            let a1 = a[0].clone() - z[0].clone();
                            let a2 = a[1].clone() - z[1].clone();
                            let b1 = b[0].clone() - z[0].clone();
                            let b2 = b[1].clone() - z[1].clone();
                            let i = a1.clone() * b2.clone() - a2.clone() * b1.clone();
                            let j = a0.clone() * b2;
                            let k = a0 * b1;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            let k: Complex = sqr(k);
                            area += (i + j + k).sqrt() / 2;
                        }
                    }
                }
                Ok(Number::from(area, units))
            }
            Vector(v) if v.len() == 3 =>
            {
                let mut data: Vec<Vec<Vec<Complex>>> = vec![Vec::new(); points];
                let units = v[2].units.map(|n| n.pow(2.0));
                data[0].push(v.iter().map(|a| a.number.clone()).collect());
                for (nx, row) in data.iter_mut().enumerate()
                {
                    if nx + 1 == points
                    {
                        startx.clone_from(&endx)
                    }
                    else if nx != 0
                    {
                        startx += deltax.clone();
                    }
                    let n = Num(Number::from(startx.clone(), unitsx));
                    let mut func = place_var(func.clone(), &varx, n.clone());
                    let mut func_vars = place_funcvar(func_vars.clone(), &varx, n.clone());
                    simplify(&mut func, &mut func_vars, options);
                    let mut starty = starty.clone();
                    if nx != 0
                    {
                        starty = do_math_with_var(
                            starty_func.clone(),
                            options,
                            func_vars.clone(),
                            &varx,
                            n.clone(),
                        )?
                        .num()?
                        .number;
                        endy = do_math_with_var(
                            endy_func.clone(),
                            options,
                            func_vars.clone(),
                            &varx,
                            n.clone(),
                        )?
                        .num()?
                        .number;
                        deltay = (endy.clone() - starty.clone()) / (points - 1);
                    }
                    for ny in 0..points
                    {
                        if nx == 0 && ny == 0
                        {
                            continue;
                        }
                        if ny + 1 == points
                        {
                            starty.clone_from(&endy)
                        }
                        else if ny != 0
                        {
                            starty += deltay.clone();
                        }
                        let v = do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &vary,
                            Num(Number::from(starty.clone(), unitsy)),
                        )?
                        .vec()?;
                        row.push(v.iter().map(|a| a.number.clone()).collect());
                    }
                }
                for (nx, row) in data[..data.len() - 1].iter().enumerate()
                {
                    for (ny, z) in row[..row.len() - 1].iter().enumerate()
                    {
                        let a = row[ny + 1].clone();
                        let b = data[nx + 1][ny].clone();
                        {
                            let a0 = a[0].clone() - z[0].clone();
                            let a1 = a[1].clone() - z[1].clone();
                            let a2 = a[2].clone() - z[2].clone();
                            let b0 = b[0].clone() - z[0].clone();
                            let b1 = b[1].clone() - z[1].clone();
                            let b2 = b[2].clone() - z[2].clone();
                            let i = a1.clone() * b2.clone() - a2.clone() * b1.clone();
                            let j = a2 * b0.clone() - a0.clone() * b2;
                            let k = a0 * b1 - a1 * b0;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            let k: Complex = sqr(k);
                            area += (i + j + k).sqrt() / 2;
                        }
                        {
                            let z = &data[nx + 1][ny + 1];
                            let a0 = a[0].clone() - z[0].clone();
                            let a1 = a[1].clone() - z[1].clone();
                            let a2 = a[2].clone() - z[2].clone();
                            let b0 = b[0].clone() - z[0].clone();
                            let b1 = b[1].clone() - z[1].clone();
                            let b2 = b[2].clone() - z[2].clone();
                            let i = a1.clone() * b2.clone() - a2.clone() * b1.clone();
                            let j = a2 * b0.clone() - a0.clone() * b2;
                            let k = a0 * b1 - a1 * b0;
                            let i: Complex = sqr(i);
                            let j: Complex = sqr(j);
                            let k: Complex = sqr(k);
                            area += (i + j + k).sqrt() / 2;
                        }
                    }
                }
                Ok(Number::from(area, units))
            }
            _ => Err("bad input"),
        }
    }
}
pub fn length(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    mut start: Complex,
    end: Number,
) -> Result<Number, &'static str>
{
    let points = options.prec as usize / 4;
    let units = end.units;
    let end = end.number;
    let delta: Complex = (end.clone() - start.clone()) / points;
    let delta2: Complex = sqr(delta.clone());
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
                let nl: Complex = sqr(xf.number.clone() - xi.number) + delta2.clone();
                length += nl.sqrt();
                x0 = Num(xf);
            }
            (Vector(xi), Vector(xf)) if xf.len() == 1 =>
            {
                let nl: Complex = sqr(xf[0].number.clone() - xi[0].number.clone()) + delta2.clone();
                length += nl.sqrt();
                x0 = Vector(xf);
            }
            (Vector(xi), Vector(xf)) =>
            {
                let nl: Complex = xi
                    .iter()
                    .zip(xf.clone())
                    .fold(Complex::new(options.prec), |sum, x| {
                        sum + sqr(x.1.number - x.0.number.clone())
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
    mut func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    mut start: Complex,
    mut end: Number,
    nth: Complex,
    combine: bool,
) -> Result<NumStr, &'static str>
{
    let mut negate = false;
    if start.real() > end.number.real() && nth == 1
    {
        negate = true;
        (end.number, start) = (start, end.number)
    }
    if start.real().is_infinite()
    {
        let neg = start.real().is_sign_negative();
        start = Complex::with_val(options.prec, 1) << (options.prec / 16);
        if neg
        {
            start *= -1
        }
    }
    if end.number.real().is_infinite()
    {
        let neg = end.number.real().is_sign_negative();
        end.number = Complex::with_val(options.prec, 1) << (options.prec / 16);
        if neg
        {
            end.number *= -1
        }
    }
    let points = options.prec as usize / 4;
    let units = end.units;
    let mut end = end.number;
    let mut funcs = Vec::new();
    if combine
        && !func.is_empty()
        && func[0] == LeftCurlyBracket
        && func[func.len() - 1] == RightCurlyBracket
    {
        let mut brackets = 0;
        let mut last = 1;
        for (i, f) in func.iter().enumerate()
        {
            match f
            {
                LeftBracket | LeftCurlyBracket => brackets += 1,
                RightBracket | RightCurlyBracket => brackets -= 1,
                Comma if brackets == 1 =>
                {
                    funcs.push(func[last..i].to_vec());
                    last = i + 1;
                }
                _ =>
                {}
            }
        }
        if last != 1
        {
            func = func[last..func.len().saturating_sub(1)].to_vec();
        }
    }
    let mut areavec: Vec<Number> = Vec::new();
    let div = Float::with_val(options.prec, 0.5).pow(options.prec / 2);
    let mut delta: Complex = (end.clone() - start.clone()) / points;
    let mut area: Complex = Complex::new(options.prec);
    let mut x0 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(Number::from(start.clone(), units)),
    )?;
    if start == end
    {
        return match x0 {
            Num(_) => Ok(Num(Number::from(Complex::new(options.prec), None))),
            Vector(_) => Ok(Vector(Vec::new())),
            _ => Err("not supported area data, if parametric have the 2nd arg start and end with the { } brackets"),
        };
    }
    {
        fn check_bounds(
            func: Vec<NumStr>,
            func_vars: Vec<(String, Vec<NumStr>)>,
            options: Options,
            var: String,
            delta: &mut Complex,
            units: Option<Units>,
            x0: &mut NumStr,
            start: &mut Complex,
            right: bool,
            compute: bool,
        ) -> Result<(), &'static str>
        {
            let mut has = false;
            if let Ok(a) = x0.num()
            {
                if !a.number.real().is_finite()
                {
                    let points = options.prec as usize / 4;
                    let end = if right
                    {
                        points * delta.clone() + start.clone()
                    }
                    else
                    {
                        start.clone() - points * delta.clone()
                    };
                    if right
                    {
                        *start += delta.clone() >> 4;
                    }
                    else
                    {
                        *start -= delta.clone() >> 4;
                    }
                    *delta = if right
                    {
                        (end.clone() - start.clone()) / points
                    }
                    else
                    {
                        (start.clone() - end.clone()) / points
                    };
                    *x0 = do_math_with_var(
                        func.clone(),
                        options,
                        func_vars.clone(),
                        &var,
                        Num(Number::from(start.clone(), units)),
                    )?;
                    has = true
                }
            }
            if !has && compute
            {
                *x0 = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(Number::from(start.clone(), units)),
                )?;
                check_bounds(
                    func, func_vars, options, var, delta, units, x0, start, right, false,
                )?
            }
            Ok(())
        }
        check_bounds(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            &mut delta,
            units,
            &mut x0,
            &mut start,
            true,
            false,
        )?;
        let mut endv = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(end.clone(), units)),
        )?;
        check_bounds(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            &mut delta,
            units,
            &mut endv,
            &mut end,
            false,
            false,
        )?;
        if nth == 1 && end.real().clone() - start.real().clone() > 2.0
        {
            let mut small_start = false;
            if start.real().is_sign_negative()
            {
                if let Ok(a) = x0.num()
                {
                    if a.number.real().clone().abs() < Float::with_val(options.prec, 1) >> 16
                    {
                        let a = do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(start.clone() * 2 - 1.141, units)),
                        )?
                        .num()?;
                        if a.number.real().clone().abs() < Float::with_val(options.prec, 1) >> 17
                        {
                            let a = do_math_with_var(
                                func.clone(),
                                options,
                                func_vars.clone(),
                                &var,
                                Num(Number::from(
                                    -(Complex::with_val(options.prec, 4) << (options.prec / 16)),
                                    units,
                                )),
                            )?
                            .num()?;
                            if a.number.real().clone().abs()
                                < Float::with_val(options.prec, 1) >> 18
                            {
                                small_start = true;
                            }
                        }
                    }
                }
            }
            let mut small_end = false;
            if end.real().is_sign_positive()
            {
                if let Ok(a) = endv.num()
                {
                    if a.number.real().clone().abs() < Float::with_val(options.prec, 1) >> 16
                    {
                        let a = do_math_with_var(
                            func.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(end.clone() * 2 + 1.141, units)),
                        )?
                        .num()?;
                        if a.number.real().clone().abs() < Float::with_val(options.prec, 1) >> 17
                        {
                            let a = do_math_with_var(
                                func.clone(),
                                options,
                                func_vars.clone(),
                                &var,
                                Num(Number::from(
                                    Complex::with_val(options.prec, 4) << (options.prec / 16),
                                    units,
                                )),
                            )?
                            .num()?;
                            if a.number.real().clone().abs()
                                < Float::with_val(options.prec, 1) >> 18
                            {
                                small_end = true
                            }
                        }
                    }
                }
            }
            let two = Num(Number::from(Complex::with_val(options.prec, 2), None));
            let one = Num(Number::from(Complex::with_val(options.prec, 1), None));
            fn change_var(
                func: &mut [NumStr],
                func_vars: &mut Vec<(String, Vec<NumStr>)>,
                from: String,
                fto: Vec<NumStr>,
            )
            {
                let to = format!("@!@{}@", from);
                for v in func.iter_mut()
                {
                    if *v == Func(from.clone())
                    {
                        *v = Func(to.clone())
                    }
                }
                for func in func_vars.iter_mut()
                {
                    for v in func.1.iter_mut()
                    {
                        if *v == Func(from.clone())
                        {
                            *v = Func(to.clone())
                        }
                    }
                }
                func_vars.push((to, fto));
            }
            match (small_start, small_end)
            {
                (true, true) =>
                {
                    change_var(
                        &mut func,
                        &mut func_vars,
                        var.clone(),
                        vec![
                            Func(var.clone()),
                            Division,
                            LeftBracket,
                            one.clone(),
                            Minus,
                            Func(var.clone()),
                            Exponent,
                            two.clone(),
                            RightBracket,
                        ],
                    );
                    func.insert(0, LeftBracket);
                    func.extend(vec![
                        RightBracket,
                        Multiplication,
                        LeftBracket,
                        one.clone(),
                        Plus,
                        Func(var.clone()),
                        Exponent,
                        two.clone(),
                        RightBracket,
                        Division,
                        LeftBracket,
                        one,
                        Minus,
                        Func(var.clone()),
                        Exponent,
                        two.clone(),
                        RightBracket,
                        Exponent,
                        two,
                    ]);
                    start = Complex::with_val(options.prec, -1);
                    end = Complex::with_val(options.prec, 1);
                    delta = 2 * end.clone() / points;
                    check_bounds(
                        func.clone(),
                        func_vars.clone(),
                        options,
                        var.clone(),
                        &mut delta,
                        units,
                        &mut x0,
                        &mut start,
                        true,
                        true,
                    )?;
                    check_bounds(
                        func.clone(),
                        func_vars.clone(),
                        options,
                        var.clone(),
                        &mut delta,
                        units,
                        &mut endv,
                        &mut end,
                        false,
                        true,
                    )?;
                }
                (false, true) =>
                {
                    change_var(
                        &mut func,
                        &mut func_vars,
                        var.clone(),
                        vec![
                            Num(Number::from(start, None)),
                            Plus,
                            Func(var.clone()),
                            Division,
                            LeftBracket,
                            one.clone(),
                            Minus,
                            Func(var.clone()),
                            RightBracket,
                        ],
                    );
                    func.insert(0, LeftBracket);
                    func.extend(vec![
                        RightBracket,
                        Division,
                        LeftBracket,
                        one,
                        Minus,
                        Func(var.clone()),
                        RightBracket,
                        Exponent,
                        two,
                    ]);
                    start = Complex::new(options.prec);
                    end = Complex::with_val(options.prec, 1);
                    delta = end.clone() / points;
                    check_bounds(
                        func.clone(),
                        func_vars.clone(),
                        options,
                        var.clone(),
                        &mut delta,
                        units,
                        &mut x0,
                        &mut start,
                        true,
                        true,
                    )?;
                    check_bounds(
                        func.clone(),
                        func_vars.clone(),
                        options,
                        var.clone(),
                        &mut delta,
                        units,
                        &mut endv,
                        &mut end,
                        false,
                        true,
                    )?;
                }
                (true, false) =>
                {
                    change_var(
                        &mut func,
                        &mut func_vars,
                        var.clone(),
                        vec![
                            Num(Number::from(end, None)),
                            Minus,
                            LeftBracket,
                            one.clone(),
                            Minus,
                            Func(var.clone()),
                            RightBracket,
                            Division,
                            Func(var.clone()),
                        ],
                    );
                    func.insert(0, LeftBracket);
                    func.extend(vec![
                        RightBracket,
                        Division,
                        Func(var.clone()),
                        Exponent,
                        two,
                    ]);
                    start = Complex::new(options.prec);
                    end = Complex::with_val(options.prec, 1);
                    delta = end.clone() / points;
                    check_bounds(
                        func.clone(),
                        func_vars.clone(),
                        options,
                        var.clone(),
                        &mut delta,
                        units,
                        &mut x0,
                        &mut start,
                        true,
                        true,
                    )?;
                    check_bounds(
                        func.clone(),
                        func_vars.clone(),
                        options,
                        var.clone(),
                        &mut delta,
                        units,
                        &mut endv,
                        &mut end,
                        false,
                        true,
                    )?;
                }
                _ =>
                {}
            }
        }
    }
    let yunits = if let Num(ref a) = x0 { a.units } else { None };
    if !funcs.is_empty()
    {
        let mut nx0t = Complex::new(options.prec);
        for i in &funcs
        {
            nx0t += sqr((do_math_with_var(
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
        }
    }
    let h: Complex = delta.clone() / 4;
    #[allow(clippy::type_complexity)]
    let data: Vec<Result<(Option<Complex>, Option<Vec<Number>>), &str>> = (0..points).into_par_iter().map(|i|
    {
        let point = if i + 1 == points
        {
            end.clone()
        }
        else
        {
            start.clone() + (i + 1) * delta.clone()
        };
        let x0 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(point.clone() - 4 * h.clone(), units)),
        )?;
        let x1 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(point.clone() - 3 * h.clone(), units)),
        )?;
        let x2 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(point.clone() - 2 * h.clone(), units)),
        )?;
        let x3 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(point.clone() - h.clone(), units)),
        )?;
        let x4 = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(Number::from(point.clone(), units)),
        )?;
        match (x0, x1, x2, x3, x4)
        {
            (Num(nx0), Num(nx1), Num(nx2), Num(nx3), Num(nx4)) if funcs.is_empty() =>
                {
                    let n0;
                    let n1;
                    let n2;
                    let n3;
                    let n4;
                    if nth != 1.0
                    {
                        let nt = pow_nth(end.clone() - point.clone() + delta.clone(), nth.clone() - 1);
                        n0 = nx0.number * nt;
                        let n: Complex = end.clone() - point.clone() + 3 * h.clone();
                        let nt = pow_nth(n, nth.clone() - 1);
                        n1 = nx1.number * nt;
                        let n: Complex = end.clone() - point.clone() + 2 * h.clone();
                        let nt = pow_nth(n, nth.clone() - 1);
                        n2 = nx2.number * nt;
                        let n: Complex = end.clone() - point.clone() + h.clone();
                        let nt = pow_nth(n, nth.clone() - 1);
                        n3 = nx3.number * nt;
                        let n: Complex = end.clone() - point.clone();
                        let nt = pow_nth(n, nth.clone() - 1);
                        n4 = nx4.number * nt;
                    } else {
                        n0 = nx0.number;
                        n1 = nx1.number;
                        n2 = nx2.number;
                        n3 = nx3.number;
                        n4 = nx4.number;
                    }
                    Ok((Some(2 * h.clone() * (7 * (n0 + n4) + 12 * n2 + 32 * (n1 + n3)) / 45), None))
                }
            (Num(nx0), Num(nx1), Num(nx2), Num(nx3), Num(nx4)) =>
                {
                    let mut nx1t = Complex::new(options.prec);
                    let mut nx2t = Complex::new(options.prec);
                    let mut nx3t = Complex::new(options.prec);
                    let mut nx4t = Complex::new(options.prec);
                    for i in &funcs
                    {
                        nx1t += sqr((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(
                                point.clone() - 3 * h.clone() + div.clone(),
                                units,
                            )),
                        )?
                            .num()?
                            .number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() - 3 * h.clone(), units)),
                        )?
                            .num()?
                            .number)
                            / div.clone());
                        nx2t += sqr((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(
                                point.clone() - 2 * h.clone() + div.clone(),
                                units,
                            )),
                        )?
                            .num()?
                            .number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() - 2 * h.clone(), units)),
                        )?
                            .num()?
                            .number)
                            / div.clone());
                        nx3t += sqr((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() - h.clone() + div.clone(), units)),
                        )?
                            .num()?
                            .number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() - h.clone(), units)),
                        )?
                            .num()?
                            .number)
                            / div.clone());
                        nx4t += sqr((do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone() + div.clone(), units)),
                        )?
                            .num()?
                            .number
                            - do_math_with_var(
                            i.clone(),
                            options,
                            func_vars.clone(),
                            &var,
                            Num(Number::from(point.clone(), units)),
                        )?
                            .num()?
                            .number)
                            / div.clone())
                    }
                    let nx1 = nx1.number * nx1t.sqrt();
                    let nx2 = nx2.number * nx2t.sqrt();
                    let nx3 = nx3.number * nx3t.sqrt();
                    let nx4 = nx4.number * nx4t.sqrt();
                    let n0;
                    let n1;
                    let n2;
                    let n3;
                    let n4;
                    if nth != 1.0
                    {
                        if i == 0
                        {
                            let nt = pow_nth(end.clone() - point.clone() + delta.clone(), nth.clone() - 1);
                            n0 = nx0.number * nt;
                        } else {
                            n0 = nx0.number;
                        }
                        let n: Complex = end.clone() - point.clone() + 3 * h.clone();
                        let nt: Complex = pow_nth(n, nth.clone() - 1);
                        n1 = nx1 * nt;
                        let n: Complex = end.clone() - point.clone() + 2 * h.clone();
                        let nt: Complex = pow_nth(n, nth.clone() - 1);
                        n2 = nx2 * nt;
                        let n: Complex = end.clone() - point.clone() + h.clone();
                        let nt: Complex = pow_nth(n, nth.clone() - 1);
                        n3 = nx3 * nt;
                        let n: Complex = end.clone() - point.clone();
                        let nt: Complex = pow_nth(n, nth.clone() - 1);
                        n4 = nx4 * nt;
                    } else {
                        n0 = nx0.number;
                        n1 = nx1;
                        n2 = nx2;
                        n3 = nx3;
                        n4 = nx4;
                    }
                    Ok((Some(2 * h.clone() * (7 * (n0 + n4.clone()) + 12 * n2 + 32 * (n1 + n3)) / 45), None))
                }
            (Vector(nx0), Vector(nx1), Vector(nx2), Vector(nx3), Vector(nx4))
            if !combine =>
                {
                    let mut areavec = Vec::new();
                    for i in 0..nx0.len()
                    {
                        let n0;
                        let n1;
                        let n2;
                        let n3;
                        let n4;
                        if nth != 1.0
                        {
                            let nt = pow_nth(end.clone() - point.clone() + delta.clone(), nth.clone() - 1);
                            n0 = nx0[i].number.clone() * nt;
                            let n: Complex = end.clone() - point.clone() + 3 * h.clone();
                            let nt = pow_nth(n, nth.clone() - 1);
                            n1 = nx1[i].number.clone() * nt;
                            let n: Complex = end.clone() - point.clone() + 2 * h.clone();
                            let nt = pow_nth(n, nth.clone() - 1);
                            n2 = nx2[i].number.clone() * nt;
                            let n: Complex = end.clone() - point.clone() + h.clone();
                            let nt = pow_nth(n, nth.clone() - 1);
                            n3 = nx3[i].number.clone() * nt;
                            let n: Complex = end.clone() - point.clone();
                            let nt = pow_nth(n, nth.clone() - 1);
                            n4 = nx4[i].number.clone() * nt;
                        } else {
                            n0 = nx0[i].number.clone();
                            n1 = nx1[i].number.clone();
                            n2 = nx2[i].number.clone();
                            n3 = nx3[i].number.clone();
                            n4 = nx4[i].number.clone();
                        }
                        areavec.push(Number::from(
                            2 * h.clone() * (7 * (n0 + n4) + 12 * n2 + 32 * (n1 + n3)) / 45,
                            match (units, nx1[i].units)
                            {
                                (Some(a), Some(b)) => Some(a.mul(&b)),
                                (Some(a), None) | (None, Some(a)) => Some(a),
                                (None, None) => None,
                            },
                        ))
                    }
                    Ok((None, Some(areavec)))
                }
            (_, _, _, _, _) => Err("not supported area data, if parametric have the 2nd arg start and end with the { } brackets"),
        }
    }).collect();
    for d in data
    {
        if let Ok(a) = d
        {
            if let Some(a) = a.0
            {
                area += a
            }
            else if let Some(a) = a.1
            {
                if areavec.is_empty()
                {
                    areavec = a
                }
                else
                {
                    for (a, b) in areavec.iter_mut().zip(a.iter())
                    {
                        a.number += b.number.clone()
                    }
                }
            }
        }
        else if let Err(s) = d
        {
            return Err(s);
        }
    }
    let g = gamma(nth.clone());
    if areavec.is_empty()
    {
        Ok(Num(Number::from(
            if negate { -area / g } else { area / g },
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
        Ok(Vector(
            areavec
                .iter()
                .map(|a| {
                    Number::from(
                        if negate
                        {
                            -a.number.clone() / g.clone()
                        }
                        else
                        {
                            a.number.clone() / g.clone()
                        },
                        a.units,
                    )
                })
                .collect::<Vec<Number>>(),
        ))
    }
}
pub fn iter(
    func: Vec<NumStr>,
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
    mut x: NumStr,
    n: Float,
    all: bool,
) -> Result<NumStr, &'static str>
{
    if n.is_infinite()
    {
        let mut last = Num(Number::from(Complex::with_val(options.prec, Nan), None));
        let mut j = 0;
        if all
        {
            if let Num(num) = x.clone()
            {
                let mut vec = vec![num];
                loop
                {
                    if j > 10000
                    {
                        return Ok(Num(Number::from(
                            Complex::with_val(options.prec, Nan),
                            None,
                        )));
                    }
                    j += 1;
                    last = x.clone();
                    x = do_math_with_var(func.clone(), options, func_vars.clone(), &var, x)?;
                    if last == x
                    {
                        break;
                    }
                    vec.push(x.num()?);
                }
                Ok(Vector(vec))
            }
            else if let Vector(v) = x.clone()
            {
                let mut vec = vec![v];
                loop
                {
                    if j > 10000
                    {
                        return Ok(Num(Number::from(
                            Complex::with_val(options.prec, Nan),
                            None,
                        )));
                    }
                    j += 1;
                    last = x.clone();
                    x = do_math_with_var(func.clone(), options, func_vars.clone(), &var, x)?;
                    if last == x
                    {
                        break;
                    }
                    vec.push(x.vec()?);
                }
                Ok(Matrix(vec))
            }
            else
            {
                return Err("unsupported iter");
            }
        }
        else
        {
            while last != x
            {
                if j > 10000
                {
                    return Ok(Num(Number::from(
                        Complex::with_val(options.prec, Nan),
                        None,
                    )));
                }
                j += 1;
                last = x.clone();
                x = do_math_with_var(func.clone(), options, func_vars.clone(), &var, x)?
            }
            Ok(x)
        }
    }
    else
    {
        let n = n
            .to_integer()
            .unwrap_or_default()
            .to_usize()
            .unwrap_or_default();
        if all
        {
            if let Num(num) = x.clone()
            {
                let mut vec = vec![num];
                for _ in 0..n
                {
                    x = do_math_with_var(func.clone(), options, func_vars.clone(), &var, x)?;
                    vec.push(x.num()?);
                }
                Ok(Vector(vec))
            }
            else if let Vector(v) = x.clone()
            {
                let mut vec = vec![v];
                for _ in 0..n
                {
                    x = do_math_with_var(func.clone(), options, func_vars.clone(), &var, x)?;
                    vec.push(x.vec()?);
                }
                Ok(Matrix(vec))
            }
            else
            {
                return Err("unsupported iter");
            }
        }
        else
        {
            for _ in 0..n
            {
                x = do_math_with_var(func.clone(), options, func_vars.clone(), &var, x)?
            }
            Ok(x)
        }
    }
}
pub fn solve(
    mut func: Vec<NumStr>,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    x: Number,
) -> Result<NumStr, &'static str>
{
    //newtons method, x-f(x)/f'(x)
    let units = x.units;
    let mut x = x.number;
    if x.real().is_nan()
    {
        let points = if x.imag().is_zero()
        {
            vec![
                Number::from(Complex::new(options.prec), None),
                Number::from(Complex::with_val(options.prec, -2), None),
                Number::from(Complex::with_val(options.prec, 2), None),
            ]
        }
        else
        {
            vec![
                Number::from(Complex::new(options.prec), None),
                Number::from(Complex::with_val(options.prec, -2), None),
                Number::from(Complex::with_val(options.prec, 2), None),
                Number::from(Complex::with_val(options.prec, (0, -2)), None),
                Number::from(Complex::with_val(options.prec, (0, 2)), None),
                Number::from(Complex::with_val(options.prec, (-2, 2)), None),
                Number::from(Complex::with_val(options.prec, (2, 2)), None),
                Number::from(Complex::with_val(options.prec, (-2, -2)), None),
                Number::from(Complex::with_val(options.prec, (2, -2)), None),
            ]
        };
        let mut values: Vec<Number> = Vec::new();
        let mut first = true;
        'main: for p in points
        {
            let v = solve(
                func.clone(),
                func_vars.clone(),
                options,
                var.clone(),
                p.clone(),
            )?
            .num()?;
            if !v.number.real().is_nan()
            {
                if first
                {
                    first = false;
                    func.insert(0, LeftBracket);
                    func.push(RightBracket);
                    func.push(Division);
                    func.push(LeftBracket);
                    func.push(LeftBracket);
                    func.push(Func(var.clone()));
                    func.push(Minus);
                    func.push(Num(v.clone()));
                    func.push(RightBracket);
                    func.push(RightBracket);
                }
                else
                {
                    for n1 in &values
                    {
                        if -(n1.number.real() - v.number.real().clone())
                            .clone()
                            .abs()
                            .log2()
                            > options.prec / 16
                            && -(n1.number.imag() - v.number.imag().clone())
                                .clone()
                                .abs()
                                .log2()
                                > options.prec / 16
                        {
                            continue 'main;
                        }
                    }
                    func.insert(func.len() - 1, Multiplication);
                    func.insert(func.len() - 1, LeftBracket);
                    func.insert(func.len() - 1, Func(var.clone()));
                    func.insert(func.len() - 1, Minus);
                    func.insert(func.len() - 1, Num(v.clone()));
                    func.insert(func.len() - 1, RightBracket);
                }
                values.push(v);
            }
        }
        Ok(
            if values.is_empty()
            {
                Num(Number::from(Complex::with_val(options.prec, Nan), None))
            }
            else if values.len() == 1
            {
                Num(values[0].clone())
            }
            else
            {
                Vector(values)
            },
        )
    }
    else
    {
        let op = options.prec;
        let prec;
        (prec, options.prec) = set_slope_prec(options.prec, 1);
        x.set_prec(options.prec);
        set_prec(&mut func, &mut func_vars, options.prec);
        for _ in 0..(op / 4).max(64)
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
                    None,
                    Some(prec),
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
        let k = slopesided(
            func,
            func_vars,
            options,
            var,
            n,
            false,
            1,
            true,
            Some(y.clone()),
            None,
            Some(prec),
        )?
        .num()?
        .number;
        x -= y.num()?.number / k.clone();
        if (last - x.clone()).abs().real().clone().log2() < op as i32 / -16 && k.real().is_finite()
        {
            x.set_prec(op);
            Ok(Num(Number::from(x, units)))
        }
        else
        {
            Ok(Num(Number::from(
                Complex::with_val(options.prec, Nan),
                None,
            )))
        }
    }
}
pub fn extrema(
    mut func: Vec<NumStr>,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    x: Number,
) -> Result<NumStr, &'static str>
{
    //newtons method, x-f'(x)/f''(x)
    let units = x.units;
    let mut x = x.number;
    let op = options.prec;
    let prec;
    (prec, options.prec) = set_slope_prec(options.prec, 2);
    x.set_prec(options.prec);
    set_prec(&mut func, &mut func_vars, options.prec);
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
        let yh = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var.clone(),
            Num(Number::from(
                x.clone() + Float::with_val(options.prec, 0.5).pow(prec),
                None,
            )),
        )?;
        x -= slopesided(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            n.clone(),
            false,
            1,
            true,
            Some(y.clone()),
            Some(yh.clone()),
            Some(prec),
        )?
        .num()?
        .number
            / slopesided(
                func.clone(),
                func_vars.clone(),
                options,
                var.clone(),
                n,
                false,
                2,
                true,
                Some(y),
                Some(yh),
                Some(prec),
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
    let yh = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var.clone(),
        Num(Number::from(
            x.clone() + Float::with_val(options.prec, 0.5).pow(prec),
            None,
        )),
    )?;
    let k = slopesided(
        func.clone(),
        func_vars.clone(),
        options,
        var.clone(),
        n.clone(),
        false,
        2,
        true,
        Some(y.clone()),
        Some(yh.clone()),
        Some(prec),
    )?
    .num()?
    .number;
    x -= slopesided(
        func.clone(),
        func_vars.clone(),
        options,
        var.clone(),
        n,
        false,
        1,
        true,
        Some(y),
        Some(yh),
        Some(prec),
    )?
    .num()?
    .number
        / k.clone();
    if (last - x.clone()).abs().real().clone().log2() < op as i32 / -16 && k.real().is_finite()
    {
        let n = Number::from(x, units);
        let mut v = Vector(vec![
            n.clone(),
            do_math_with_var(func, options, func_vars, &var, Num(n.clone()))?.num()?,
            Number::from(
                Complex::with_val(
                    options.prec,
                    if k.clone().abs().real().clone().log2() < op as i32 / -16
                    {
                        0
                    }
                    else if k.real().is_sign_positive()
                    {
                        1
                    }
                    else
                    {
                        -1
                    },
                ),
                None,
            ),
        ]);
        v.set_prec(op);
        Ok(v)
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
pub fn taylor(
    mut func: Vec<NumStr>,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    x: Option<Number>,
    a: Number,
    nth: usize,
) -> Result<NumStr, &'static str>
{
    fn fact(n: usize) -> Integer
    {
        let mut fact = Integer::from(1);
        for i in 1..=n
        {
            fact *= i
        }
        fact
    }
    let op = options.prec;
    let mut an = a.number;
    let mut prec;
    (prec, _) = set_slope_prec(options.prec, nth.min(8) as u32);
    (_, options.prec) = set_slope_prec(options.prec, nth as u32);
    set_prec(&mut func, &mut func_vars, options.prec);
    an.set_prec(options.prec);
    let a = Number::from(an.clone(), a.units);
    let val = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(a.clone()),
    )?;
    let mut val2 = do_math_with_var(
        func.clone(),
        options,
        func_vars.clone(),
        &var,
        Num(Number::from(
            an.clone() + Float::with_val(options.prec, 0.5).pow(prec),
            None,
        )),
    )?;
    if let Some(x) = x
    {
        let mut x = x.number;
        x.set_prec(options.prec);
        let mut sum = val.clone();
        for n in 1..=nth
        {
            if n % 8 == 0
            {
                (prec, _) = set_slope_prec(options.prec, nth.min(8 + n) as u32);
                val2 = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(Number::from(
                        an.clone() + Float::with_val(options.prec, 0.5).pow(prec),
                        None,
                    )),
                )?;
            }
            let d = slopesided(
                func.clone(),
                func_vars.clone(),
                options,
                var.clone(),
                a.clone(),
                false,
                n as u32,
                true,
                Some(val.clone()),
                Some(val2.clone()),
                Some(prec),
            )?;
            let v = d.func(
                &Num(Number::from(
                    fact(n) * (x.clone() - an.clone()).pow(-(n as i32)),
                    None,
                )),
                div,
            )?;
            sum = sum.func(&v, add)?;
        }
        sum.set_prec(op);
        Ok(sum)
    }
    else
    {
        let mut poly_mat = Vec::with_capacity(nth + 1);
        let mut poly = Vec::with_capacity(nth + 1);
        let is_vector = match val.clone()
        {
            Vector(a) =>
            {
                let empty = vec![Number::from(Complex::new(options.prec), None); a.len()];
                poly_mat.push(a);
                for _ in 1..=nth
                {
                    poly_mat.push(empty.clone())
                }
                true
            }
            Num(a) =>
            {
                let empty = Number::from(Complex::new(options.prec), None);
                poly.push(a);
                for _ in 1..=nth
                {
                    poly.push(empty.clone())
                }
                false
            }
            _ => return Err("unsupported type"),
        };
        for n in 1..=nth
        {
            if n % 8 == 0
            {
                (prec, _) = set_slope_prec(options.prec, nth.min(8 + n) as u32);
                val2 = do_math_with_var(
                    func.clone(),
                    options,
                    func_vars.clone(),
                    &var,
                    Num(Number::from(
                        an.clone() + Float::with_val(options.prec, 0.5).pow(prec),
                        None,
                    )),
                )?;
            }
            let d = slopesided(
                func.clone(),
                func_vars.clone(),
                options,
                var.clone(),
                a.clone(),
                false,
                n as u32,
                true,
                Some(val.clone()),
                Some(val2.clone()),
                Some(prec),
            )?;
            if is_vector
            {
                let d = d.vec()?;
                for (i, poly) in poly_mat.iter_mut().enumerate()
                {
                    if i > n
                    {
                        break;
                    }
                    for (d, poly) in d.iter().zip(poly.iter_mut())
                    {
                        *poly = add(
                            poly,
                            &Number::from(
                                (-a.clone().number).pow(n - i) * d.number.clone()
                                    / (fact(n - i) * fact(i)),
                                None,
                            ),
                        )
                    }
                }
            }
            else
            {
                let d = d.num()?.number;
                for (i, poly) in poly.iter_mut().enumerate()
                {
                    if i > n
                    {
                        break;
                    }
                    *poly = add(
                        poly,
                        &Number::from(
                            (-a.clone().number).pow(n - i) * d.clone() / (fact(n - i) * fact(i)),
                            None,
                        ),
                    )
                }
            }
        }
        if is_vector
        {
            poly_mat.reverse();
            let mut m = Matrix(transpose(&poly_mat));
            m.set_prec(op);
            Ok(m)
        }
        else
        {
            poly.reverse();
            let mut v = Vector(poly);
            v.set_prec(op);
            Ok(v)
        }
    }
}
fn set_slope_prec(prec: u32, nth: u32) -> (u32, u32)
{
    let prec = prec.clamp(256, 1024 * nth.max(1));
    (prec / (nth + 8), (nth / 8).max(1) * prec / 2)
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
) -> Result<NumStr, &'static str>
{
    let oop = options.prec;
    if nth == 0
    {
        do_math_with_var(func.clone(), options, func_vars.clone(), &var, Num(point))
    }
    else if options.prec <= 256
    {
        options.prec = 256;
        point.number.set_prec(options.prec);
        let mut n = slopesided(
            func, func_vars, options, var, point, combine, nth, true, None, None, None,
        )?;
        n.set_prec(oop);
        Ok(n)
    }
    else
    {
        let op = options.prec.clamp(256, 1024);
        let prec;
        (prec, options.prec) = set_slope_prec(options.prec, nth);
        point.number.set_prec(options.prec);
        let val = do_math_with_var(
            func.clone(),
            options,
            func_vars.clone(),
            &var,
            Num(point.clone()),
        )?;
        let left = slopesided(
            func.clone(),
            func_vars.clone(),
            options,
            var.clone(),
            point.clone(),
            combine,
            nth,
            false,
            Some(val.clone()),
            None,
            Some(prec),
        )?;
        let right = slopesided(
            func,
            func_vars,
            options,
            var,
            point,
            combine,
            nth,
            true,
            Some(val),
            None,
            Some(prec),
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
                        < op as i32 / -16)
                    || (left.imag().is_infinite()
                        && right.imag().is_infinite()
                        && (left.real().clone() - right.real().clone())
                            .abs()
                            .clone()
                            .log2()
                            < op as i32 / -16))
                    && left.real().is_sign_positive() == right.real().is_sign_positive()
                    && left.imag().is_sign_positive() == right.imag().is_sign_positive())
                    || (left.clone() - right.clone()).abs().real().clone().log2() < op as i32 / -16
                {
                    let mut n = Num(Number::from((left + right) / 2, units));
                    n.set_prec(oop);
                    Ok(n)
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
                                < op as i32 / -16)
                            || (left.imag().is_infinite()
                                && right.imag().is_infinite()
                                && (left.real().clone() - right.real().clone())
                                    .abs()
                                    .clone()
                                    .log2()
                                    < op as i32 / -16))
                            && left.real().is_sign_positive() == right.real().is_sign_positive()
                            && left.imag().is_sign_positive() == right.imag().is_sign_positive())
                            || (left.clone() - right.clone()).abs().real().clone().log2()
                                < op as i32 / -16
                        {
                            Number::from((left + right) / 2, units)
                        }
                        else
                        {
                            Number::from(Complex::with_val(options.prec, Nan), None)
                        }
                    })
                }
                let mut v = Vector(vec);
                v.set_prec(oop);
                Ok(v)
            }
            (_, _) => Err("lim err"),
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
    val2: Option<NumStr>,
    prec: Option<u32>,
) -> Result<NumStr, &'static str>
{
    if nth == 0
    {
        return do_math_with_var(func.clone(), options, func_vars.clone(), &var, Num(point));
    }
    let mut oop = 0;
    let units = point.units;
    let mut point = point.number;
    let prec = if let Some(prec) = prec
    {
        prec
    }
    else
    {
        oop = options.prec;
        options.prec = options.prec.clamp(256, 1024 * nth.max(1));
        let prec;
        (prec, options.prec) = set_slope_prec(options.prec, nth);
        point.set_prec(options.prec);
        prec
    };
    let h: Float = if right
    {
        Float::with_val(options.prec, 0.5).pow(prec)
    }
    else
    {
        -Float::with_val(options.prec, 0.5).pow(prec)
    };
    let num = Integer::from(nth);
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
            let bo = val2.is_some();
            for k in 0..nth
            {
                if bo && nth - k == 1
                {
                    if k % 2 == 0
                    {
                        sum += num.clone().binomial(k) * val2.clone().unwrap().num()?.number
                    }
                    else
                    {
                        sum -= num.clone().binomial(k) * val2.clone().unwrap().num()?.number
                    }
                }
                else if k % 2 == 0
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
                let mut n = Num(Number::from(
                    sum * Float::with_val(options.prec, 2).pow(nth * prec),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                ));
                if oop != 0
                {
                    n.set_prec(oop);
                }
                Ok(n)
            }
            else
            {
                let mut n = Num(Number::from(
                    -sum * Float::with_val(options.prec, 2).pow(nth * prec),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                ));
                if oop != 0
                {
                    n.set_prec(oop);
                }
                Ok(n)
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
            let mut v = Vector(
                sum.iter()
                    .map(|n| {
                        Number::from(
                            if right || nth % 2 == 0
                            {
                                n.number.clone() * Float::with_val(options.prec, 2).pow(nth * prec)
                            }
                            else
                            {
                                -n.number.clone() * Float::with_val(options.prec, 2).pow(nth * prec)
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
            );
            if oop != 0
            {
                v.set_prec(oop);
            }
            Ok(v)
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
                let mut n = Num(Number::from(
                    sum[0].number.clone() * Float::with_val(options.prec, 2).pow(nth * prec),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                ));
                if oop != 0
                {
                    n.set_prec(oop)
                }
                Ok(n)
            }
            else
            {
                let mut n = Num(Number::from(
                    -sum[0].number.clone() * Float::with_val(options.prec, 2).pow(nth * prec),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                ));
                if oop != 0
                {
                    n.set_prec(oop)
                }
                Ok(n)
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
                let mut n = Num(Number::from(
                    sum[1].number.clone() / sum[0].number.clone(),
                    match (yunits, units)
                    {
                        (Some(a), Some(b)) => Some(a.div(&b)),
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(Units::default().div(&b)),
                        (None, None) => None,
                    },
                ));
                if oop != 0
                {
                    n.set_prec(oop)
                }
                Ok(n)
            }
            else
            {
                let nf = &sum.last().unwrap().number;
                let mut v = Vector(
                    sum[0..sum.len().saturating_sub(1)]
                        .iter()
                        .map(|n| {
                            Number::from(
                                nf.clone() / n.number.clone(),
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
                );
                if oop != 0
                {
                    v.set_prec(oop)
                }
                Ok(v)
            }
        }
        _ => Err("not supported slope data"),
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
    mut func: Vec<NumStr>,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
    mut options: Options,
    var: String,
    point: Number,
    side: LimSide,
) -> Result<NumStr, &'static str>
{
    let xunits = point.units;
    let mut point = point.number;
    let oop = options.prec;
    options.prec = options.prec.clamp(256, 1024);
    if oop != options.prec
    {
        point.set_prec(options.prec);
        set_prec(&mut func, &mut func_vars, options.prec);
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
                let mut n = if (n1.clone() - n2.clone()).abs().real().clone().log2()
                    < options.prec as i32 / -16
                {
                    Num(Number::from(n2, units))
                }
                else if n1.real().is_sign_positive() != n2.real().is_sign_positive()
                    || n1.imag().is_sign_positive() != n2.imag().is_sign_positive()
                {
                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                }
                else if n2.real().is_infinite() || n2.imag().is_infinite()
                {
                    Num(Number::from(
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
                    ))
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
                    Num(Number::from(
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
                    ))
                };
                n.set_prec(oop);
                Ok(n)
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
                let mut v = Vector(vec);
                v.set_prec(oop);
                Ok(v)
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
                            let mut n = Num(Number::from((left + right) / 2, units));
                            n.set_prec(oop);
                            Ok(n)
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
                        let mut v = Vector(vec);
                        v.set_prec(options.prec);
                        Ok(v)
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
    let h1 = Float::with_val(options.prec, 0.5).pow(options.prec / 4);
    let h2 = Float::with_val(options.prec, 0.5).pow((options.prec / 3) as f64 + 7.0 / 0.94);
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
pub fn lambertw(z: Complex, k: Integer) -> Complex
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
        let zexpz = &w * zexp.clone();
        let zexpz_d = &zexp + zexpz.clone();
        let zexpz_dd = (2 * zexp) + &zexpz;
        w -=
            2 * ((zexpz.clone() - &z) * &zexpz_d) / ((2 * sqr(zexpz_d)) - ((zexpz - &z) * zexpz_dd))
    }
    w
}
fn initpoint(z: Complex, k: Integer) -> Complex
{
    let prec = z.prec();
    {
        let e = Float::with_val(prec.0, -1).exp();
        let test: Complex = z.clone() + &e;
        if test.abs().real() <= &1.005
        {
            if k == 0 && z.real() > &-0.5
            {
                let p1: Complex = 2 * z / e + 2;
                let p = p1.clone().sqrt();
                return p.clone() - (p1 / 3) + ((11 * cube(p)) / 72) - 1;
            }
            else if k == -1 && z.real().is_sign_negative() && z.imag().is_zero()
            {
                let p1: Complex = 2 * z / e + 2;
                let p = p1.clone().sqrt();
                return -p.clone() - (p1 / 3) - ((11 * cube(p)) / 72) - 1;
            }
        }
    }
    {
        let test: Complex = z.clone() - 0.5;
        if test.abs().real() <= &0.6
        {
            if k == 0
            {
                return (0.35173371 * (0.1237166 + 7.061302897 * z.clone()))
                    / (2 + 0.827184 * (1 + 2 * z));
            }
            else if k == -1
            {
                return (Complex::with_val(prec, (-2.2591588985, -4.22096))
                    * (Complex::with_val(prec, (-14.073271, -33.767687754)) * &z
                        + Complex::with_val(prec, (-12.7127, 19.071643)) * (1 + 2 * z.clone())))
                    / (2 + Complex::with_val(prec, (-17.23103, 10.629721)) * (1 + 2 * z));
            }
        }
    }
    let zln = z.ln() + Complex::with_val(prec, (0, 2 * Float::with_val(prec.0, Pi) * k));
    zln.clone() - zln.ln()
}
pub fn rand_gamma(k: Float, t: Float) -> Float
{
    let prec = k.prec();
    let mut sum = Float::new(prec);
    for _ in 1..=k
        .clone()
        .floor()
        .to_integer()
        .unwrap_or_default()
        .to_usize()
        .unwrap_or_default()
    {
        let u: Float = Float::with_val(prec, fastrand::u128(1..)) / u128::MAX;
        sum += u.ln();
    }
    let s = k.clone().fract();
    let e = Float::with_val(prec, 1).exp();
    let check = e.clone() / (e + s.clone());
    let mut eta: Float;
    loop
    {
        let u: Float = Float::with_val(prec, fastrand::u128(1..)) / u128::MAX;
        let v: Float = Float::with_val(prec, fastrand::u128(1..)) / u128::MAX;
        let w: Float = Float::with_val(prec, fastrand::u128(1..)) / u128::MAX;
        let n;
        if u <= check
        {
            eta = v.pow(1 / s.clone());
            n = w * eta.clone().pow(s.clone() - 1);
        }
        else
        {
            eta = 1 - v.ln();
            n = w * (-eta.clone()).exp();
        };
        if n <= eta.clone().pow(s.clone() - 1) * (-eta.clone()).exp()
        {
            break;
        }
    }
    let f: Float = eta - sum;
    t * f
}
pub fn rand_norm(m: Complex, s: Complex) -> Complex
{
    let prec = s.prec().0;
    let mut u: Float = Float::with_val(prec, fastrand::i128(i128::MIN + 2..i128::MAX)) / i128::MAX;
    let mut v: Float = Float::with_val(prec, fastrand::i128(i128::MIN + 2..i128::MAX)) / i128::MAX;
    let mut g: Float = u.clone().pow(2) + v.pow(2);
    while g >= 1
    {
        u = Float::with_val(prec, fastrand::i128(i128::MIN + 2..i128::MAX)) / i128::MAX;
        v = Float::with_val(prec, fastrand::i128(i128::MIN + 2..i128::MAX)) / i128::MAX;
        g = u.clone().pow(2) + v.pow(2);
    }
    let d: Float = -2 * g.clone().ln() / g;
    let z: Float = u * d.sqrt();
    m + z * s
}
pub fn regularized_incomplete_beta(x: Complex, a: Complex, b: Complex) -> Complex
{
    (gamma(a.clone() + b.clone())) * incomplete_beta(x, a.clone(), b.clone())
        / (gamma(a) * gamma(b))
}
pub fn sqr(z: Complex) -> Complex
{
    if z.imag().is_zero()
    {
        z.pow(2)
    }
    else
    {
        z.clone() * z
    }
}
pub fn cube(z: Complex) -> Complex
{
    if z.imag().is_zero()
    {
        z.pow(3)
    }
    else
    {
        z.clone() * &z * z
    }
}
pub fn pow_nth(z: Complex, n: Complex) -> Complex
{
    let zz = z.imag().is_zero();
    let nz = n.imag().is_zero();
    let nr = n.real();
    if nr.clone().fract().is_zero() && nz
    {
        if !zz && nr <= &256
        {
            if nr.is_zero()
            {
                Complex::with_val(z.prec(), 1)
            }
            else
            {
                let mut p = z.clone();
                for _ in 1..n
                    .real()
                    .to_integer()
                    .unwrap_or_default()
                    .abs()
                    .to_usize()
                    .unwrap_or_default()
                {
                    p *= &z;
                }
                if nr.is_sign_positive()
                {
                    p
                }
                else
                {
                    1 / p
                }
            }
        }
        else
        {
            z.pow(n)
        }
    }
    else if zz && nz && {
        let zr = z.real();
        zr.clone().fract().is_zero() && zr.is_sign_negative()
    }
    {
        z.pow(n)
    }
    else
    {
        (z.ln() * n).exp()
    }
}
pub fn hsv2rgb(mut hue: Float, sat: Float, val: Float) -> Vec<Number>
{
    if sat.is_zero()
    {
        return rgb2val(val.clone(), val.clone(), val);
    }
    hue *= 6;
    let i = hue
        .clone()
        .floor()
        .to_integer()
        .unwrap_or_default()
        .to_usize()
        .unwrap_or_default();
    let f = hue - i;
    let p = val.clone() * (1 - sat.clone());
    let q = val.clone() * (1 - sat.clone() * f.clone());
    let t = val.clone() * (1 - sat * (1 - f));
    match i % 6
    {
        0 => rgb2val(val, t, p),
        1 => rgb2val(q, val, p),
        2 => rgb2val(p, val, t),
        3 => rgb2val(p, q, val),
        4 => rgb2val(t, p, val),
        5 => rgb2val(val, p, q),
        _ => rgb2val(val, p, q),
    }
}
pub fn rgb2val(r: Float, g: Float, b: Float) -> Vec<Number>
{
    let r: Float = 255 * r;
    let g: Float = 255 * g;
    let b: Float = 255 * b;
    vec![
        Number::from(r.into(), None),
        Number::from(g.into(), None),
        Number::from(b.into(), None),
    ]
}