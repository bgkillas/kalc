use crate::complex::{
    NumStr,
    NumStr::{Matrix, Num, Str, Vector},
};
use rug::{float::Constant::Pi, ops::Pow, Complex, Float};
use std::ops::{Shl, Shr};
pub fn do_math(func: Vec<NumStr>, deg: u8, prec: u32) -> Result<NumStr, ()>
{
    if func.len() == 1
    {
        return Ok(func[0].clone());
    }
    if func.is_empty()
    {
        return Err(());
    }
    let mut function = func;
    let mut i = 0;
    let mut single;
    let mut count = 0;
    let (mut j, mut v, mut vec, mut mat);
    let mut len = 0;
    let mut place = Vec::new();
    'outer: while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s == "{"
            {
                j = i + 1;
                count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err(());
                    }
                    match &function[j]
                    {
                        Str(s) if s == "{" => count += 1,
                        Str(s) if s == "}" => count -= 1,
                        _ =>
                        {}
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err(());
                }
                v = function[i + 1..j - 1].to_vec();
                single = 0;
                count = 0;
                vec = Vec::new();
                mat = Vec::new();
                for (f, n) in v.iter().enumerate()
                {
                    if let Str(s) = n
                    {
                        if s == "," && count == 0
                        {
                            let z = do_math(v[single..f].to_vec(), deg, prec)?;
                            match z
                            {
                                Num(n) => vec.push(n),
                                Vector(n) if len == n.len() || single == 0 =>
                                {
                                    len = n.len();
                                    mat.push(n)
                                }
                                _ => return Err(()),
                            }
                            single = f + 1;
                        }
                        else if s == "{"
                        {
                            count += 1;
                        }
                        else if s == "}"
                        {
                            count -= 1;
                        }
                    }
                }
                if single != v.len()
                {
                    let z = do_math(v[single..].to_vec(), deg, prec)?;
                    match z
                    {
                        Num(n) => vec.push(n),
                        Vector(n) if len == n.len() || single == 0 => mat.push(n),
                        _ => return Err(()),
                    }
                }
                function.drain(i..j);
                if !mat.is_empty()
                {
                    if vec.is_empty()
                    {
                        function.insert(i, Matrix(mat));
                    }
                    else
                    {
                        return Err(());
                    }
                }
                else
                {
                    function.insert(i, Vector(vec));
                }
            }
            else if s == "("
            {
                j = i + 1;
                count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err(());
                    }
                    match &function[j]
                    {
                        Str(s) if s == "(" => count += 1,
                        Str(s) if s == ")" => count -= 1,
                        _ =>
                        {}
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err(());
                }
                v = function[i + 1..j - 1].to_vec();
                if i != 0
                {
                    if let Str(k) = &function[i - 1]
                    {
                        if k == "log"
                            || k == "root"
                            || k == "atan"
                            || k == "arctan"
                            || k == "atan2"
                            || k == "bi"
                            || k == "binomial"
                            || k == "angle"
                            || k == "cross"
                            || k == "dot"
                            || k == "part"
                        {
                            count = 0;
                            for (f, n) in v.iter().enumerate()
                            {
                                if let Str(s) = n
                                {
                                    if s == "," && count == 0
                                    {
                                        place.push(f);
                                    }
                                    else if s == "(" || s == "{"
                                    {
                                        count += 1;
                                    }
                                    else if s == ")" || s == "}"
                                    {
                                        count -= 1;
                                    }
                                }
                            }
                            if !place.is_empty()
                            {
                                function.drain(i..j);
                                function.insert(i, do_math(v[..place[0]].to_vec(), deg, prec)?);
                                for (k, l) in place.iter().enumerate()
                                {
                                    function.insert(i + k + 1, Str(",".to_string()));
                                    function.insert(
                                        i + k + 2,
                                        do_math(v[l + 1..].to_vec(), deg, prec)?,
                                    );
                                    i += 1;
                                }
                                continue 'outer;
                            }
                        }
                        else if k == "sum" || k == "summation" || k == "prod" || k == "product"
                        {
                            i = j - 1;
                            continue;
                        }
                    }
                }
                function[i] = do_math(v, deg, prec)?;
                function.drain(i + 1..j);
            }
        }
        i += 1;
    }
    i = 0;
    let (mut a, mut b);
    let to_deg = if deg == 0
    {
        Complex::with_val(prec, 1)
    }
    else if deg == 1
    {
        Complex::with_val(prec, 180) / Complex::with_val(prec, Pi)
    }
    else
    {
        Complex::with_val(prec, 200) / Complex::with_val(prec, Pi)
    };
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s.len() > 1 && s.chars().next().unwrap().is_ascii_alphabetic()
            {
                if s == "sum" || s == "product" || s == "prod" || s == "summation"
                {
                    place.clear();
                    for (f, n) in function.iter().enumerate()
                    {
                        if let Str(s) = n
                        {
                            if s == "," && count == 1
                            {
                                place.push(f);
                            }
                            else if s == "(" || s == "{"
                            {
                                count += 1;
                            }
                            else if s == ")" || s == "}"
                            {
                                if count == 1
                                {
                                    place.push(f);
                                    break;
                                }
                                count -= 1;
                            }
                        }
                    }
                    if place.len() == 4
                    {
                        if let Str(l) = &function[place[0] + 1]
                        {
                            function[i] = Num(sum(
                                function[i + 2..place[0]].to_vec(),
                                l,
                                do_math(function[place[1] + 1..place[2]].to_vec(), deg, prec)?
                                    .num()?
                                    .real()
                                    .to_f64() as i64,
                                do_math(function[place[2] + 1..place[3]].to_vec(), deg, prec)?
                                    .num()?
                                    .real()
                                    .to_f64() as i64,
                                !(s == "sum" || s == "summation"),
                                deg,
                                prec,
                            )?);
                            function.drain(i + 1..=place[3]);
                        }
                        else
                        {
                            return Err(());
                        }
                    }
                    else
                    {
                        return Err(());
                    }
                }
                else if let Matrix(a) = function[i + 1].clone()
                {
                    function[i] = match s.as_str()
                    {
                        "len" | "length" => Num(Complex::with_val(prec, a.len())),
                        "wid" | "width" => Num(Complex::with_val(prec, a[0].len())),
                        "tr" | "trace" =>
                        {
                            let mut n = Complex::new(prec);
                            for (i, j) in a.iter().enumerate()
                            {
                                if j.len() == i
                                {
                                    break;
                                }
                                n += j[i].clone();
                            }
                            Num(n)
                        }
                        "det" | "determinant" =>
                        {
                            if a.len() == a[0].len()
                            {
                                if a.len() == 2
                                {
                                    Num(a[0][0].clone() * a[1][1].clone()
                                        - a[1][0].clone() * a[0][1].clone())
                                }
                                else if a.len() == 3
                                {
                                    Num(a[0][0].clone()
                                        * (a[1][1].clone() * a[2][2].clone()
                                            - a[1][2].clone() * a[2][1].clone())
                                        + a[0][1].clone()
                                            * (a[1][2].clone() * a[2][0].clone()
                                                - a[1][0].clone() * a[2][2].clone())
                                        + a[0][2].clone()
                                            * (a[1][0].clone() * a[2][1].clone()
                                                - a[1][1].clone() * a[2][0].clone()))
                                }
                                else
                                {
                                    return Err(());
                                }
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        "part" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                if function.len() > i + 5 && function[i + 4].str_is(",")
                                {
                                    let b = function[i + 3].num()?;
                                    let c = function[i + 5].num()?;
                                    function.drain(i + 2..i + 6);
                                    let n1 = b.clone().real().to_f64() as usize;
                                    let n2 = c.clone().real().to_f64() as usize;
                                    if n1 <= a.len() && n1 != 0 && n2 <= a[0].len() && n2 != 0
                                    {
                                        Num(a[n1 - 1][n2 - 1].clone())
                                    }
                                    else
                                    {
                                        return Err(());
                                    }
                                }
                                else
                                {
                                    let b = function[i + 3].num()?;
                                    function.drain(i + 2..i + 4);
                                    let n = b.clone().real().to_f64() as usize;
                                    if n <= a.len() && n != 0
                                    {
                                        Vector(a[n - 1].clone())
                                    }
                                    else
                                    {
                                        return Err(());
                                    }
                                }
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        _ =>
                        {
                            i += 1;
                            continue;
                        }
                    };
                    function.remove(i + 1);
                }
                else if let Vector(a) = function[i + 1].clone()
                {
                    function[i] = match s.as_str()
                    {
                        "len" | "length" => Num(Complex::with_val(prec, a.len())),
                        "abs" =>
                        {
                            let mut n = Complex::new(prec);
                            for i in a.iter().map(|x| x.clone().pow(2)).collect::<Vec<Complex>>()
                            {
                                n += i;
                            }
                            Num(n.sqrt())
                        }
                        "car" | "cartesian" =>
                        {
                            if a.len() == 2
                            {
                                let t = a[1].clone() / to_deg.clone();
                                Vector(vec![
                                    a[0].clone() * t.clone().cos(),
                                    a[0].clone() * t.clone().sin(),
                                ])
                            }
                            else if a.len() == 3
                            {
                                let t1 = a[1].clone() / to_deg.clone();
                                let t2 = a[2].clone() / to_deg.clone();
                                Vector(vec![
                                    a[0].clone() * t1.clone().sin() * t2.clone().cos(),
                                    a[0].clone() * t1.clone().sin() * t2.clone().sin(),
                                    a[0].clone() * t1.clone().cos(),
                                ])
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        "polar" | "pol" => Vector(to_polar(a.clone(), to_deg.clone())),
                        "angle" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                let b = function[i + 3].vec()?;
                                function.drain(i + 2..i + 4);
                                if a.len() == 3 && b.len() == 3
                                {
                                    let c: Complex = a[0].clone().pow(2)
                                        + a[1].clone().pow(2)
                                        + a[2].clone().pow(2);
                                    let d: Complex = b[0].clone().pow(2)
                                        + b[1].clone().pow(2)
                                        + b[2].clone().pow(2);
                                    Num(((a[0].clone() * b[0].clone()
                                        + a[1].clone() * b[1].clone()
                                        + a[2].clone() * b[2].clone())
                                        / (c.sqrt() * d.sqrt()))
                                    .acos()
                                        * to_deg.clone())
                                }
                                else if a.len() == 2 && b.len() == 2
                                {
                                    let c: Complex = a[0].clone().pow(2) + a[1].clone().pow(2);
                                    let d: Complex = b[0].clone().pow(2) + b[1].clone().pow(2);
                                    Num(((a[0].clone() * b[0].clone()
                                        + a[1].clone() * b[1].clone())
                                        / (c.sqrt() * d.sqrt()))
                                    .acos()
                                        * to_deg.clone())
                                }
                                else
                                {
                                    return Err(());
                                }
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        "cross" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                let b = function[i + 3].vec()?;
                                function.drain(i + 2..i + 4);
                                if a.len() == 3 && b.len() == 3
                                {
                                    Vector(vec![
                                        a[1].clone() * &b[2] - a[2].clone() * &b[1],
                                        a[2].clone() * &b[0] - a[0].clone() * &b[2],
                                        a[0].clone() * &b[1] - a[1].clone() * &b[0],
                                    ])
                                }
                                else if a.len() == 2 && b.len() == 2
                                {
                                    Num(a[0].clone() * &b[1] - a[1].clone() * &b[0])
                                }
                                else
                                {
                                    return Err(());
                                }
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        "dot" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                let mut n = Complex::new(prec);
                                for i in a
                                    .iter()
                                    .zip(function[i + 3].vec()?.iter())
                                    .map(|(a, b)| a * b)
                                {
                                    n += i;
                                }
                                function.drain(i + 2..i + 4);
                                Num(n)
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        "part" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                let b = function[i + 3].num()?;
                                function.drain(i + 2..i + 4);
                                let n = b.clone().real().to_f64() as usize;
                                if n <= a.len() && n != 0
                                {
                                    Num(a[n - 1].clone())
                                }
                                else
                                {
                                    return Err(());
                                }
                            }
                            else
                            {
                                return Err(());
                            }
                        }
                        _ =>
                        {
                            i += 1;
                            continue;
                        }
                    };
                    function.remove(i + 1);
                }
                else
                {
                    a = function[i + 1].num()?;
                    function[i] = Num(match s.to_string().as_str()
                    {
                        "sin" => (a / to_deg.clone()).sin(),
                        "csc" => (a / to_deg.clone()).sin().recip(),
                        "cos" => (a / to_deg.clone()).cos(),
                        "sec" => (a / to_deg.clone()).cos().recip(),
                        "tan" => (a / to_deg.clone()).tan(),
                        "cot" => (a / to_deg.clone()).tan().recip(),
                        "asin" | "arcsin" =>
                        {
                            b = a.clone().asin() * to_deg.clone();
                            if a.imag() == &0.0 && a.real() >= &1.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "acsc" | "arccsc" =>
                        {
                            b = a.clone().recip().asin() * to_deg.clone();
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "acos" | "arccos" =>
                        {
                            b = a.clone().acos() * to_deg.clone();
                            if a.imag() == &0.0 && a.real() >= &1.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "asec" | "arcsec" =>
                        {
                            b = a.clone().recip().acos() * to_deg.clone();
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "atan" | "arctan" | "atan2" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                b = function[i + 3].num()?;
                                function.drain(i + 2..i + 4);
                                let i = Complex::with_val(prec, (0, 1));
                                ((a.clone() + b.clone() * i.clone())
                                    / (a.clone() + b.clone() * i.clone()).abs())
                                .ln()
                                    * -i
                                    * to_deg.clone()
                            }
                            else
                            {
                                a.atan() * to_deg.clone()
                            }
                        }
                        "acot" | "arccot" => a.recip().atan() * to_deg.clone(),
                        "sinh" => a.sinh(),
                        "csch" => a.sinh().recip(),
                        "cosh" => a.cosh(),
                        "sech" => a.cosh().recip(),
                        "tanh" => a.tanh(),
                        "coth" => a.tanh().recip(),
                        "asinh" | "arcsinh" => a.asinh(),
                        "acsch" | "arccsch" => a.recip().asinh(),
                        "acosh" | "arccosh" => a.acosh(),
                        "asech" | "arcsech" =>
                        {
                            b = a.clone().recip().acosh();
                            if a.imag() == &0.0 && a.real() < &0.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "atanh" | "arctanh" =>
                        {
                            b = a.clone().atanh();
                            if a.imag() == &0.0 && a.real() >= &1.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "acoth" | "arccoth" =>
                        {
                            b = a.clone().recip().atanh();
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, (b.real(), -b.imag()))
                            }
                            else
                            {
                                b
                            }
                        }
                        "cis" =>
                        {
                            (a.clone() / to_deg.clone()).cos()
                                + (a / to_deg.clone()).sin() * Complex::with_val(prec, (0.0, 1.0))
                        }
                        "ln" | "aexp" =>
                        {
                            if a.imag() == &0.0
                            {
                                a = Complex::with_val(prec, a.real());
                            }
                            a.ln()
                        }
                        "ceil" => Complex::with_val(
                            prec,
                            (a.real().clone().ceil(), a.imag().clone().ceil()),
                        ),
                        "floor" => Complex::with_val(
                            prec,
                            (a.real().clone().floor(), a.imag().clone().floor()),
                        ),
                        "round" => Complex::with_val(
                            prec,
                            (a.real().clone().round(), a.imag().clone().round()),
                        ),
                        "recip" => a.recip(),
                        "exp" | "aln" => a.exp(),
                        "log" =>
                        {
                            if a.imag() == &0.0
                            {
                                a = Complex::with_val(prec, a.real());
                            }
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                b = function[i + 3].num()?;
                                if b.imag() == &0.0
                                {
                                    b = Complex::with_val(prec, b.real());
                                }
                                function.remove(i + 3);
                                function.remove(i + 2);
                                b.ln() / a.ln()
                            }
                            else
                            {
                                a.ln()
                            }
                        }
                        "root" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                b = function[i + 3].num()?;
                                function.drain(i + 2..i + 4);
                                match b.imag() == &0.0
                                    && (b.real().to_f64() / 2.0).fract() != 0.0
                                    && &b.real().clone().trunc() == b.real()
                                    && a.imag() == &0.0
                                {
                                    true => Complex::with_val(
                                        prec,
                                        a.real() / a.real().clone().abs()
                                            * a.real().clone().abs().pow(b.real().clone().recip()),
                                    ),
                                    false => a.pow(b.recip()),
                                }
                            }
                            else
                            {
                                a.sqrt()
                            }
                        }
                        "bi" | "binomial" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                b = function[i + 3].num()?;
                                function.drain(i + 2..i + 4);
                                if a.imag() != &0.0 && b.imag() != &0.0
                                {
                                    Complex::new(prec)
                                }
                                else if a.real().clone().fract() == 0.0
                                    && b.real().clone().fract() == 0.0
                                {
                                    Complex::with_val(
                                        prec,
                                        a.real()
                                            .to_integer()
                                            .unwrap()
                                            .binomial(b.real().to_f64() as u32),
                                    )
                                }
                                else
                                {
                                    let c: Float = a.real().clone() + 1;
                                    let d: Float = b.real().clone() + 1;
                                    let e: Float = a.real().clone() - b.real().clone() + 1;
                                    Complex::with_val(prec, c.gamma() / (d.gamma() * e.gamma()))
                                }
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "gamma" =>
                        {
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, a.real().clone().gamma())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "sqrt" | "asquare" => a.sqrt(),
                        "abs" => a.abs(),
                        "deg" | "degree" =>
                        {
                            if deg == 0
                            {
                                a * Complex::with_val(prec, 180) / Complex::with_val(prec, Pi)
                            }
                            else if deg == 2
                            {
                                a * 180.0 / 200.0
                            }
                            else
                            {
                                a
                            }
                        }
                        "rad" | "radians" =>
                        {
                            if deg == 0
                            {
                                a
                            }
                            else if deg == 2
                            {
                                a * Complex::with_val(prec, Pi) / Complex::with_val(prec, 200)
                            }
                            else
                            {
                                a * Complex::with_val(prec, Pi) / Complex::with_val(prec, 180)
                            }
                        }
                        "grad" | "gradians" =>
                        {
                            if deg == 0
                            {
                                a * Complex::with_val(prec, 200) / Complex::with_val(prec, Pi)
                            }
                            else if deg == 2
                            {
                                a
                            }
                            else
                            {
                                a * 200.0 / 180.0
                            }
                        }
                        "re" | "real" => Complex::with_val(prec, a.real()),
                        "im" | "imag" => Complex::with_val(prec, a.imag()),
                        "sgn" | "sign" => Complex::with_val(prec, a.clone() / a.abs()),
                        "arg" => a.arg(),
                        "cbrt" | "acube" =>
                        {
                            if a.imag() == &0.0
                            {
                                if a.real() == &0.0
                                {
                                    Complex::new(prec)
                                }
                                else
                                {
                                    Complex::with_val(
                                        prec,
                                        a.real() / a.real().clone().abs()
                                            * a.real().clone().abs().pow(3f64.recip()),
                                    )
                                }
                            }
                            else
                            {
                                a.pow(3f64.recip())
                            }
                        }
                        "frac" | "fract" => Complex::with_val(
                            prec,
                            (a.real().clone().fract(), a.imag().clone().fract()),
                        ),
                        "int" | "trunc" => Complex::with_val(
                            prec,
                            (a.real().clone().trunc(), a.imag().clone().trunc()),
                        ),
                        "square" | "asqrt" => a.pow(2),
                        "cube" | "acbrt" => a.pow(3),
                        "fact" =>
                        {
                            if a.imag() == &0.0
                            {
                                let b: Float = a.real().clone() + 1;
                                Complex::with_val(prec, b.gamma())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "subfact" =>
                        {
                            if a.imag() != &0.0 || a.real() < &0.0
                            {
                                return Err(());
                            }
                            Complex::with_val(prec, subfact(a.real().to_f64()))
                        }
                        "sinc" => a.clone().sin() / a,
                        "conj" | "conjugate" => a.conj(),
                        "erf" =>
                        {
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, a.real().clone().erf())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "erfc" =>
                        {
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, a.real().clone().erfc())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "ai" =>
                        {
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, a.real().clone().ai())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "digamma" =>
                        {
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, a.real().clone().digamma())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        "zeta" =>
                        {
                            if a.imag() == &0.0
                            {
                                Complex::with_val(prec, a.real().clone().zeta())
                            }
                            else
                            {
                                Complex::new(prec)
                            }
                        }
                        _ =>
                        {
                            i += 1;
                            continue;
                        }
                    });
                    function.remove(i + 1);
                }
            }
        }
        i += 1;
    }
    if function.len() > 1
    {
        i = function.len() - 2;
        while i != 0
        {
            if !function[i].str_is("^")
            {
                i -= 1;
                continue;
            }
            function[i] = function[i - 1].pow(&function[i + 1])?;
            function.remove(i + 1);
            function.remove(i - 1);
            i -= 1;
        }
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "*" => function[i] = function[i - 1].mul(&function[i + 1])?,
                "/" => function[i] = function[i - 1].div(&function[i + 1])?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            }
        }
        else
        {
            i += 1;
            continue;
        }
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "+" => function[i] = function[i - 1].add(&function[i + 1])?,
                "-" => function[i] = function[i - 1].sub(&function[i + 1])?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            }
        }
        else
        {
            i += 1;
            continue;
        }
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "%" =>
                {
                    function[i] = {
                        a = function[i - 1].num()?;
                        b = function[i + 1].num()?;
                        if a.imag() == &0.0 && b.imag() == &0.0
                        {
                            Num(Complex::with_val(prec, a.real() % b.real()))
                        }
                        else
                        {
                            let c = -a.clone() / b.clone();
                            Num(a + b * (c.real().clone().ceil() + c.imag().clone().ceil()))
                        }
                    }
                }
                "<" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        (function[i - 1].num()?.abs().real() < function[i + 1].num()?.abs().real())
                            as i32,
                    ))
                }
                ">" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        (function[i - 1].num()?.abs().real() > function[i + 1].num()?.abs().real())
                            as i32,
                    ))
                }
                ">=" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        (function[i - 1].num()?.abs().real() >= function[i + 1].num()?.abs().real())
                            as i32,
                    ))
                }
                "<=" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        (function[i - 1].num()?.abs().real() <= function[i + 1].num()?.abs().real())
                            as i32,
                    ))
                }
                "==" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        (function[i - 1].num()? == function[i + 1].num()?) as i32,
                    ))
                }
                "!=" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        (function[i - 1].num()? != function[i + 1].num()?) as i32,
                    ))
                }
                ">>" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        function[i - 1].num()?.shr(
                            function[i + 1]
                                .num()?
                                .real()
                                .to_u32_saturating()
                                .unwrap_or(0),
                        ),
                    ))
                }
                "<<" =>
                {
                    function[i] = Num(Complex::with_val(
                        prec,
                        function[i - 1].num()?.shl(
                            function[i + 1]
                                .num()?
                                .real()
                                .to_u32_saturating()
                                .unwrap_or(0),
                        ),
                    ))
                }
                _ =>
                {
                    i += 1;
                    continue;
                }
            }
        }
        else
        {
            i += 1;
            continue;
        }
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "&&" =>
                {
                    a = function[i - 1].num()?;
                    b = function[i - 1].num()?;
                    function[i] = Num(Complex::with_val(
                        prec,
                        (a.imag() == &0.0
                            && b.imag() == &0.0
                            && a.real() == &1.0
                            && b.real() == &1.0) as i32,
                    ))
                }
                "||" =>
                {
                    a = function[i - 1].num()?;
                    b = function[i - 1].num()?;
                    function[i] = Num(Complex::with_val(
                        prec,
                        (a.imag() == &0.0
                            && b.imag() == &0.0
                            && (a.real() == &1.0 || b.real() == &1.0))
                            as i32,
                    ))
                }
                _ =>
                {
                    i += 1;
                    continue;
                }
            }
        }
        else
        {
            i += 1;
            continue;
        }
        function.remove(i + 1);
        function.remove(i - 1);
    }
    Ok(function[0].clone())
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
        if a[1].eq0()
        {
            if a[0].eq0()
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
    else if a[1].eq0()
    {
        if a[0].eq0()
        {
            if a[2].eq0()
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
fn subfact(a: f64) -> f64
{
    if a == 0.0
    {
        return 1.0;
    }
    if a.fract() != 0.0
    {
        return f64::NAN;
    }
    let mut prev = 1.0;
    let mut curr = 0.0;
    let mut next;
    for i in 2..=(a as usize)
    {
        next = (i - 1) as f64 * (prev + curr);
        prev = curr;
        curr = next;
    }
    curr
}
fn sum(
    function: Vec<NumStr>,
    var: &str,
    start: i64,
    end: i64,
    product: bool,
    deg: u8,
    prec: u32,
) -> Result<Complex, ()>
{
    let mut value: Complex = Complex::new(prec);
    let mut func;
    let mut math;
    for z in start..=end
    {
        func = function.clone();
        for k in func.iter_mut()
        {
            if k.str_is(var)
            {
                *k = Num(Complex::with_val(prec, z));
            }
        }
        math = do_math(func, deg, prec)?.num()?;
        if !product
        {
            value += math;
        }
        else if z == start
        {
            value = math;
        }
        else
        {
            value *= math;
        }
    }
    Ok(value)
}