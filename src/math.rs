use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    options::{
        AngleType,
        AngleType::{Degrees, Gradians, Radians},
    },
};
use rug::{float::Constant::Pi, ops::Pow, Complex, Float};
use std::ops::{Shl, Shr};
pub fn do_math(func: Vec<NumStr>, deg: AngleType, prec: u32) -> Result<NumStr, &'static str>
{
    if func.len() == 1
    {
        return Ok(func[0].clone());
    }
    if func.is_empty()
    {
        return Err("no function");
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
                        return Err("idk");
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
                    return Err("no interior vector");
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
                                _ => return Err("probably unreachable"),
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
                        _ => return Err("probably not reachable"),
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
                        return Err("likely unreachable");
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
                        return Err("unsure");
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
                    return Err("no interior bracket");
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
                            || k == "max"
                            || k == "min"
                            || k == "proj"
                            || k == "project"
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
    let to_deg = match deg
    {
        Degrees => Complex::with_val(prec, 180) / Complex::with_val(prec, Pi),
        Radians => Complex::with_val(prec, 1),
        Gradians => Complex::with_val(prec, 200) / Complex::with_val(prec, Pi),
    };
    while i < function.len() - 1
    {
        if let Str(s) = &function[i].clone()
        {
            if s.len() > 1 && s.chars().next().unwrap().is_alphabetic()
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
                            function[i] = sum(
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
                            )?;
                            function.drain(i + 1..=place[3]);
                        }
                        else
                        {
                            return Err("failed to get var for sum/prod");
                        }
                    }
                    else
                    {
                        return Err("not enough args for sum/prod");
                    }
                }
                else if let Matrix(a) = function[i + 1].clone()
                {
                    function[i] = match s.as_str()
                    {
                        "cofactor" | "cofactors" | "cof" =>
                        {
                            if a.len() == a[0].len() && a.len() > 1
                            {
                                Matrix(cofactor(a))
                            }
                            else
                            {
                                return Err("non square matrix");
                            }
                        }
                        "minor" | "minors" =>
                        {
                            if a.len() == a[0].len() && a.len() > 1
                            {
                                Matrix(minors(a))
                            }
                            else
                            {
                                return Err("non square matrix");
                            }
                        }
                        "adjugate" | "adj" =>
                        {
                            if a.len() == a[0].len() && a.len() > 1
                            {
                                Matrix(transpose(cofactor(a)))
                            }
                            else
                            {
                                return Err("non square matrix");
                            }
                        }
                        "inverse" | "inv" => Matrix(inverse(a)?),
                        "transpose" | "trans" => Matrix(transpose(a)),
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
                                Num(determinant(a))
                            }
                            else
                            {
                                return Err("non square matrix");
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
                                        return Err("not in matrix");
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
                                        return Err("not in matrix");
                                    }
                                }
                            }
                            else
                            {
                                return Err("no arg");
                            }
                        }
                        "norm" =>
                        {
                            let mut n = Complex::new(prec);
                            for i in a
                            {
                                for j in i
                                {
                                    n += j.abs().pow(2);
                                }
                            }
                            Num(n.sqrt())
                        }
                        "abs" => Matrix(
                            a.iter()
                                .map(|a| a.iter().map(|a| a.clone().abs()).collect())
                                .collect(),
                        ),
                        _ => do_functions(
                            function[i + 1].clone(),
                            deg,
                            &mut function,
                            i,
                            &to_deg,
                            s,
                        )?,
                    };
                    function.remove(i + 1);
                }
                else if let Vector(a) = function[i + 1].clone()
                {
                    function[i] = match s.as_str()
                    {
                        "len" | "length" => Num(Complex::with_val(prec, a.len())),
                        "abs" => Vector(a.iter().map(|x| x.clone().abs()).collect()),
                        "norm" =>
                        {
                            let mut n = Complex::new(prec);
                            for i in a
                            {
                                n += i.abs().pow(2);
                            }
                            Num(n.sqrt())
                        }
                        "normalize" =>
                        {
                            let mut n = Complex::new(prec);
                            for i in a.clone()
                            {
                                n += i.pow(2);
                            }
                            Vector(a.iter().map(|x| x / n.clone().sqrt()).collect())
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
                                return Err("incorrect polar form");
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
                                    return Err("cant decern angles");
                                }
                            }
                            else
                            {
                                return Err("no args");
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
                                    return Err("cant cross");
                                }
                            }
                            else
                            {
                                return Err("no args");
                            }
                        }
                        "project" | "proj" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                let b = function[i + 3].clone();
                                if b.vec()?.len() == a.len()
                                {
                                    let mut dot = Complex::new(prec);
                                    for i in a.iter().zip(b.vec()?.iter()).map(|(a, b)| a * b)
                                    {
                                        dot += i;
                                    }
                                    let mut norm = Complex::new(prec);
                                    for i in b.vec()?
                                    {
                                        norm += i.abs().pow(2);
                                    }
                                    function.drain(i + 2..i + 4);
                                    Num(dot / norm).mul(&b)?
                                }
                                else
                                {
                                    return Err("cant project");
                                }
                            }
                            else
                            {
                                return Err("no args");
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
                                return Err("no args");
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
                                    return Err("out of range");
                                }
                            }
                            else
                            {
                                return Err("no args");
                            }
                        }
                        _ => do_functions(
                            function[i + 1].clone(),
                            deg,
                            &mut function,
                            i,
                            &to_deg,
                            s,
                        )?,
                    };
                    function.remove(i + 1);
                }
                else
                {
                    function[i] = if s == "rotate"
                    {
                        a = function[i + 1].num()? / to_deg.clone();
                        Matrix(vec![
                            vec![a.clone().cos(), -a.clone().sin()],
                            vec![a.clone().sin(), a.cos()],
                        ])
                    }
                    else
                    {
                        do_functions(function[i + 1].clone(), deg, &mut function, i, &to_deg, s)?
                    };
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
                "±" => function[i] = function[i - 1].pm(&function[i + 1])?,
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
fn do_functions(
    a: NumStr,
    deg: AngleType,
    function: &mut Vec<NumStr>,
    k: usize,
    to_deg: &Complex,
    s: &str,
) -> Result<NumStr, &'static str>
{
    let mut vec = Vec::new();
    if function.len() > k + 3 && function[k + 2].str_is(",")
    {
        let b = function[k + 3].clone();
        function.drain(k + 2..k + 4);
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(functions(a, Some(b), to_deg.clone(), s, deg)?)),
            (Vector(a), Vector(b)) if a.len() == b.len() =>
            {
                for i in 0..b.len()
                {
                    vec.push(functions(
                        a[i].clone(),
                        Some(b[i].clone()),
                        to_deg.clone(),
                        s,
                        deg,
                    )?)
                }
                Ok(Vector(vec))
            }
            (Matrix(a), Matrix(b)) if a.len() == b.len() && a[0].len() == b[0].len() =>
            {
                let mut mat = Vec::new();
                for i in 0..a.len()
                {
                    vec.clear();
                    for j in 0..a[0].len()
                    {
                        vec.push(functions(
                            a[i][j].clone(),
                            Some(b[i][j].clone()),
                            to_deg.clone(),
                            s,
                            deg,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            (Num(a), Vector(b)) =>
            {
                for i in b
                {
                    vec.push(functions(a.clone(), Some(i), to_deg.clone(), s, deg)?)
                }
                Ok(Vector(vec))
            }
            (Vector(a), Num(b)) =>
            {
                for i in a
                {
                    vec.push(functions(i, Some(b.clone()), to_deg.clone(), s, deg)?)
                }
                Ok(Vector(vec))
            }
            (Num(a), Matrix(b)) =>
            {
                let mut mat = Vec::new();
                for i in b
                {
                    vec.clear();
                    for j in i
                    {
                        vec.push(functions(a.clone(), Some(j), to_deg.clone(), s, deg)?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            (Matrix(a), Num(b)) =>
            {
                let mut mat = Vec::new();
                for i in a
                {
                    vec.clear();
                    for j in i
                    {
                        vec.push(functions(j, Some(b.clone()), to_deg.clone(), s, deg)?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            (Matrix(a), Vector(b)) if a.len() == b.len() =>
            {
                let mut mat = Vec::new();
                for i in 0..a.len()
                {
                    vec.clear();
                    for j in 0..a[0].len()
                    {
                        vec.push(functions(
                            a[i][j].clone(),
                            Some(b[i].clone()),
                            to_deg.clone(),
                            s,
                            deg,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            (Vector(a), Matrix(b)) if a.len() == b.len() =>
            {
                let mut mat = Vec::new();
                for i in 0..b.len()
                {
                    vec.clear();
                    for j in 0..b[0].len()
                    {
                        vec.push(functions(
                            a[i].clone(),
                            Some(b[i][j].clone()),
                            to_deg.clone(),
                            s,
                            deg,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            _ => Err("unhreachable"),
        }
    }
    else
    {
        match a
        {
            Matrix(a) =>
            {
                let mut mat = Vec::new();
                for i in a
                {
                    vec.clear();
                    for j in i
                    {
                        vec.push(functions(j, None, to_deg.clone(), s, deg)?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            Vector(a) =>
            {
                for i in a
                {
                    vec.push(functions(i, None, to_deg.clone(), s, deg)?)
                }
                Ok(Vector(vec))
            }
            Num(a) => Ok(Num(functions(a, None, to_deg.clone(), s, deg)?)),
            _ => Err("unreachable6"),
        }
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
fn submatrix(a: Vec<Vec<Complex>>, row: usize, col: usize) -> Vec<Vec<Complex>>
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
fn determinant(a: Vec<Vec<Complex>>) -> Complex
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
            det += x * determinant(sub_matrix) * if i % 2 == 0 { 1.0 } else { -1.0 };
        }
        det
    }
}
fn transpose(a: Vec<Vec<Complex>>) -> Vec<Vec<Complex>>
{
    let mut b = vec![vec![Complex::new(a[0][0].prec()); a.len()]; a[0].len()];
    for (i, l) in a.iter().enumerate()
    {
        for (j, n) in l.iter().enumerate()
        {
            b[j][i] = n.clone();
        }
    }
    b
}
fn minors(a: Vec<Vec<Complex>>) -> Vec<Vec<Complex>>
{
    let mut result = vec![vec![Complex::new(a[0][0].prec()); a[0].len()]; a.len()];
    for (i, k) in result.iter_mut().enumerate()
    {
        for (j, l) in k.iter_mut().enumerate()
        {
            *l = determinant(submatrix(a.clone(), i, j));
        }
    }
    result
}
fn cofactor(a: Vec<Vec<Complex>>) -> Vec<Vec<Complex>>
{
    let mut result = vec![vec![Complex::new(a[0][0].prec()); a[0].len()]; a.len()];
    for (i, k) in result.iter_mut().enumerate()
    {
        for (j, l) in k.iter_mut().enumerate()
        {
            *l = if (i + j) % 2 == 1
            {
                -determinant(submatrix(a.clone(), i, j))
            }
            else
            {
                determinant(submatrix(a.clone(), i, j))
            };
        }
    }
    result
}
pub fn inverse(a: Vec<Vec<Complex>>) -> Result<Vec<Vec<Complex>>, &'static str>
{
    if a.len() == a[0].len() && a.len() > 1
    {
        Matrix(transpose(cofactor(a.clone())))
            .div(&Num(determinant(a)))?
            .mat()
    }
    else
    {
        Err("not square")
    }
}
fn functions(
    a: Complex,
    c: Option<Complex>,
    to_deg: Complex,
    s: &str,
    deg: AngleType,
) -> Result<Complex, &'static str>
{
    let b;
    let prec = to_deg.prec();
    Ok(match s
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
            if let Some(b) = c
            {
                let i = Complex::with_val(prec, (0, 1));
                ((a.clone() + b.clone() * i.clone()) / (a.clone() + b.clone() * i.clone()).abs())
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
                Complex::with_val(prec, a.real()).ln()
            }
            else
            {
                a.ln()
            }
        }
        "ceil" => Complex::with_val(prec, (a.real().clone().ceil(), a.imag().clone().ceil())),
        "floor" => Complex::with_val(prec, (a.real().clone().floor(), a.imag().clone().floor())),
        "round" => Complex::with_val(prec, (a.real().clone().round(), a.imag().clone().round())),
        "recip" => a.recip(),
        "exp" | "aln" => a.exp(),
        "log" =>
        {
            let a = if a.imag() == &0.0
            {
                Complex::with_val(prec, a.real()).ln()
            }
            else
            {
                a.ln()
            };
            if let Some(b) = c
            {
                let b = if b.imag() == &0.0
                {
                    Complex::with_val(prec, b.real()).ln()
                }
                else
                {
                    b.ln()
                };
                b / a
            }
            else
            {
                a
            }
        }
        "root" =>
        {
            if let Some(b) = c
            {
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
            if let Some(b) = c
            {
                if a.imag() != &0.0 && b.imag() != &0.0
                {
                    return Err("binomial complex not supported");
                }
                else if a.real().clone().fract() == 0.0 && b.real().clone().fract() == 0.0
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
                return Err("no args");
            }
        }
        "gamma" | "Γ" =>
        {
            if a.imag() == &0.0
            {
                Complex::with_val(prec, a.real().clone().gamma())
            }
            else
            {
                return Err("complex gamma not supported");
            }
        }
        "max" =>
        {
            if let Some(b) = c
            {
                Complex::with_val(
                    prec,
                    (
                        if a.real() > b.real()
                        {
                            a.real()
                        }
                        else
                        {
                            b.real()
                        },
                        if a.imag() > b.imag()
                        {
                            a.imag()
                        }
                        else
                        {
                            b.imag()
                        },
                    ),
                )
            }
            else
            {
                return Err("no args");
            }
        }
        "min" =>
        {
            if let Some(b) = c
            {
                Complex::with_val(
                    prec,
                    (
                        if a.real() < b.real()
                        {
                            a.real()
                        }
                        else
                        {
                            b.real()
                        },
                        if a.imag() < b.imag()
                        {
                            a.imag()
                        }
                        else
                        {
                            b.imag()
                        },
                    ),
                )
            }
            else
            {
                return Err("no args");
            }
        }
        "sqrt" | "asquare" => a.sqrt(),
        "abs" | "norm" => a.abs(),
        "deg" | "degree" => match deg
        {
            Radians => a * Complex::with_val(prec, 180) / Complex::with_val(prec, Pi),
            Gradians => a * 180.0 / 200.0,
            Degrees => a,
        },
        "rad" | "radian" => match deg
        {
            Radians => a,
            Gradians => a * Complex::with_val(prec, Pi) / Complex::with_val(prec, 200),
            Degrees => a * Complex::with_val(prec, Pi) / Complex::with_val(prec, 180),
        },
        "grad" | "gradian" => match deg
        {
            Radians => a * Complex::with_val(prec, 200) / Complex::with_val(prec, Pi),
            Gradians => a,
            Degrees => a * 200.0 / 180.0,
        },
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
        "frac" | "fract" =>
        {
            Complex::with_val(prec, (a.real().clone().fract(), a.imag().clone().fract()))
        }
        "int" | "trunc" =>
        {
            Complex::with_val(prec, (a.real().clone().trunc(), a.imag().clone().trunc()))
        }
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
                return Err("complex factorial not supported");
            }
        }
        "subfact" =>
        {
            if a.imag() != &0.0 || a.real() < &0.0
            {
                return Err("complex/fractional subfactorial not supported");
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
                return Err("complex erf not supported");
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
                return Err("complex erfc not supported");
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
                return Err("complex ai not supported");
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
                return Err("complex digamma not supported");
            }
        }
        "zeta" | "ζ" =>
        {
            if a.imag() == &0.0
            {
                Complex::with_val(prec, a.real().clone().zeta())
            }
            else
            {
                return Err("complex zeta not supported");
            }
        }
        _ =>
        {
            return Err("unreachable7");
        }
    })
}