use crate::{
    complex::{
        add, and, cofactor, determinant, eq, ge, gt, hyperoperation, inverse, le, lt, minors, mvec,
        ne, nth_prime, or, rem, shl, shr, slog, sort, sub, sum, tetration, to_polar, transpose,
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    options::{
        AngleType,
        AngleType::{Degrees, Gradians, Radians},
    },
};
use rug::{float::Constant::Pi, ops::Pow, Complex, Float};

pub fn do_math(mut function: Vec<NumStr>, deg: AngleType, prec: u32)
    -> Result<NumStr, &'static str>
{
    if function.len() == 1
    {
        return Ok(function[0].clone());
    }
    if function.is_empty()
    {
        return Err(" ");
    }
    let mut i = 0;
    'outer: while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s == "{"
            {
                let mut j = i + 1;
                let mut count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err("curly bracket err");
                    }
                    if let Str(s) = &function[j]
                    {
                        match s.as_str()
                        {
                            "{" => count += 1,
                            "}" => count -= 1,
                            _ =>
                            {}
                        }
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err("no interior vector");
                }
                let mut single = 0;
                let v = function[i + 1..j - 1].to_vec();
                let mut vec = Vec::new();
                let mut mat = Vec::<Vec<Complex>>::new();
                for (f, n) in v.iter().enumerate()
                {
                    if let Str(s) = n
                    {
                        match s.as_str()
                        {
                            "," if count == 0 =>
                            {
                                let z = do_math(v[single..f].to_vec(), deg, prec)?;
                                match z
                                {
                                    Num(n) => vec.push(n),
                                    Vector(n) => mat.push(n),
                                    _ => return Err("broken matrix"),
                                }
                                single = f + 1;
                            }
                            "{" | "(" => count += 1,
                            "}" | ")" => count -= 1,
                            _ =>
                            {}
                        }
                    }
                }
                if single != v.len()
                {
                    let z = do_math(v[single..].to_vec(), deg, prec)?;
                    match z
                    {
                        Num(n) => vec.push(n),
                        Vector(n) => mat.push(n),
                        _ => return Err("broken matrix"),
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
                        return Err("vector err");
                    }
                }
                else
                {
                    function.insert(i, Vector(vec));
                }
            }
            else if s == "("
            {
                let mut j = i + 1;
                let mut count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err("round bracket err");
                    }
                    if let Str(s) = &function[j]
                    {
                        match s.as_str()
                        {
                            "(" => count += 1,
                            ")" => count -= 1,
                            _ =>
                            {}
                        }
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err("no interior bracket");
                }
                let v = function[i + 1..j - 1].to_vec();
                if i != 0
                {
                    if let Str(k) = &function[i - 1]
                    {
                        if matches!(
                            k.as_str(),
                            "log"
                                | "slog"
                                | "root"
                                | "atan"
                                | "arctan"
                                | "atan2"
                                | "bi"
                                | "binomial"
                                | "angle"
                                | "cross"
                                | "dot"
                                | "part"
                                | "proj"
                                | "project"
                                | "link"
                                | "C"
                                | "P"
                                | "H"
                        )
                        {
                            count = 0;
                            let mut place = Vec::new();
                            for (f, n) in v.iter().enumerate()
                            {
                                if let Str(s) = n
                                {
                                    match s.as_str()
                                    {
                                        "," if count == 0 => place.push(f),
                                        "(" | "{" => count += 1,
                                        ")" | "}" => count -= 1,
                                        _ =>
                                        {}
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
                        else if matches!(
                            k.as_str(),
                            "sum" | "summation" | "prod" | "product" | "Σ" | "Π" | "vec" | "mat"
                        )
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
            if (s.len() > 1 && s.chars().next().unwrap().is_alphabetic())
                || matches!(s.as_str(), "C" | "P" | "H" | "I")
            {
                if matches!(
                    s.as_str(),
                    "sum" | "product" | "prod" | "summation" | "Σ" | "Π" | "vec" | "mat"
                )
                {
                    let mut place = Vec::new();
                    let mut count = 0;
                    for (f, n) in function[i + 2..].iter().enumerate()
                    {
                        if let Str(s) = n
                        {
                            if s == "," && count == 0
                            {
                                place.push(f + i + 2);
                            }
                            else if s == "(" || s == "{"
                            {
                                count += 1;
                            }
                            else if s == ")" || s == "}"
                            {
                                if count == 0
                                {
                                    place.push(f + i + 2);
                                    break;
                                }
                                count -= 1;
                            }
                        }
                    }
                    if place.len() == 4
                    {
                        if let Str(var) = &function[place[0] - 1]
                        {
                            let start =
                                do_math(function[place[1] + 1..place[2]].to_vec(), deg, prec)?
                                    .num()?;
                            let end =
                                do_math(function[place[2] + 1..place[3]].to_vec(), deg, prec)?
                                    .num()?;
                            if !start.imag().is_zero() || !end.imag().is_zero()
                            {
                                return Err("imag start/end");
                            }
                            if !start.real().clone().fract().is_zero()
                                || !end.real().clone().fract().is_zero()
                            {
                                return Err("fractional start/end");
                            }
                            let start = start.real().to_f64() as usize;
                            let end = end.real().to_f64() as usize;
                            function[i] = match s.as_str()
                            {
                                "vec" | "mat" => mvec(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    var,
                                    start,
                                    end,
                                    s == "vec",
                                    deg,
                                    prec,
                                )?,
                                _ => sum(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    var,
                                    start,
                                    end,
                                    !(s == "sum" || s == "summation" || s == "Σ"),
                                    deg,
                                    prec,
                                )?,
                            };
                            function.drain(i + 1..=place[3]);
                        }
                        else
                        {
                            return Err("failed to get var for sum/prod");
                        }
                    }
                    else if place.len() == 1 || place.is_empty()
                    {
                        match s.as_str()
                        {
                            "sum" | "summation" | "Σ" =>
                            {
                                function[i] = match if place.is_empty()
                                {
                                    Ok(function[i + 1].clone())
                                }
                                else
                                {
                                    do_math(function[i + 2..place[0]].to_vec(), deg, prec)
                                }
                                {
                                    Ok(Num(a)) => Num(a.clone()),
                                    Ok(Vector(a)) =>
                                    {
                                        Num(a.iter().fold(Complex::new(prec), |sum, val| sum + val))
                                    }
                                    Ok(Matrix(a)) => Num(a
                                        .iter()
                                        .flatten()
                                        .fold(Complex::new(prec), |sum, val| sum + val)),
                                    _ => return Err("sum err"),
                                }
                            }
                            "product" | "prod" | "Π" =>
                            {
                                function[i] = match if place.is_empty()
                                {
                                    Ok(function[i + 1].clone())
                                }
                                else
                                {
                                    do_math(function[i + 2..place[0]].to_vec(), deg, prec)
                                }
                                {
                                    Ok(Num(a)) => Num(a.clone()),
                                    Ok(Vector(a)) => Num(a
                                        .iter()
                                        .fold(Complex::with_val(prec, 1), |sum, val| sum * val)),
                                    Ok(Matrix(a)) => Num(a
                                        .iter()
                                        .flatten()
                                        .fold(Complex::with_val(prec, 1), |sum, val| sum * val)),
                                    _ => return Err("sum err"),
                                }
                            }
                            _ => return Err("sum err"),
                        }
                        if place.is_empty()
                        {
                            function.remove(i + 1);
                        }
                        else
                        {
                            function.drain(i + 2..place[0]);
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
                        "max" =>
                        {
                            let mut vec = Vec::new();
                            for j in a
                            {
                                let mut max = j[0].clone();
                                for i in j
                                {
                                    if i.real() > max.real()
                                    {
                                        max = i
                                    }
                                }
                                vec.push(max)
                            }
                            Vector(vec)
                        }
                        "min" =>
                        {
                            let mut vec = Vec::new();
                            for j in a
                            {
                                let mut min = j[0].clone();
                                for i in j
                                {
                                    if i.real() < min.real()
                                    {
                                        min = i
                                    }
                                }
                                vec.push(min)
                            }
                            Vector(vec)
                        }
                        "flatten" => Vector(a.into_iter().flatten().collect::<Vec<Complex>>()),
                        "cofactor" | "cofactors" | "cof" =>
                        {
                            if a.len() == a[0].len() && a.len() > 1
                            {
                                Matrix(cofactor(&a))
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
                                Matrix(minors(&a))
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
                                Matrix(transpose(&cofactor(&a)))
                            }
                            else
                            {
                                return Err("non square matrix");
                            }
                        }
                        "inverse" | "inv" => Matrix(inverse(&a)?),
                        "transpose" | "trans" => Matrix(transpose(&a)),
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
                                Num(determinant(&a))
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
                                    match (function[i + 3].clone(), function[i + 5].clone())
                                    {
                                        (Num(b), Num(c)) =>
                                        {
                                            function.drain(i + 2..i + 6);
                                            let n1 = b.clone().real().to_f64() as usize;
                                            let n2 = c.clone().real().to_f64() as usize;
                                            if n1 <= a.len()
                                                && n1 != 0
                                                && n2 <= a[0].len()
                                                && n2 != 0
                                            {
                                                Num(a[n1 - 1][n2 - 1].clone())
                                            }
                                            else
                                            {
                                                return Err("not in matrix");
                                            }
                                        }
                                        (Vector(b), Num(c)) | (Num(c), Vector(b)) =>
                                        {
                                            function.drain(i + 2..i + 6);
                                            let n2 = c.clone().real().to_f64() as usize;
                                            let mut vec = Vec::new();
                                            for n in b
                                            {
                                                let n1 = n.clone().real().to_f64() as usize;
                                                if n1 <= a.len()
                                                    && n1 != 0
                                                    && n2 <= a[0].len()
                                                    && n2 != 0
                                                {
                                                    vec.push(a[n1 - 1][n2 - 1].clone())
                                                }
                                                else
                                                {
                                                    return Err("not in matrix");
                                                }
                                            }
                                            Vector(vec)
                                        }
                                        (Vector(b), Vector(c)) =>
                                        {
                                            function.drain(i + 2..i + 6);
                                            let mut mat = Vec::new();
                                            for g in b
                                            {
                                                let mut vec = Vec::new();
                                                let n1 = g.clone().real().to_f64() as usize;
                                                for n in c.clone()
                                                {
                                                    let n2 = n.clone().real().to_f64() as usize;
                                                    if n1 <= a.len()
                                                        && n1 != 0
                                                        && n2 <= a[0].len()
                                                        && n2 != 0
                                                    {
                                                        vec.push(a[n1 - 1][n2 - 1].clone())
                                                    }
                                                    else
                                                    {
                                                        return Err("not in matrix");
                                                    }
                                                }
                                                mat.push(vec);
                                            }
                                            Matrix(mat)
                                        }
                                        _ => return Err("wrong part num"),
                                    }
                                }
                                else
                                {
                                    match function[i + 3].clone()
                                    {
                                        Num(b) =>
                                        {
                                            function.drain(i + 2..i + 4);
                                            let n = b.clone().real().to_f64() as usize;
                                            if n <= a.len() && n != 0
                                            {
                                                Vector(a[n - 1].clone())
                                            }
                                            else
                                            {
                                                return Err("out of range");
                                            }
                                        }
                                        Vector(b) =>
                                        {
                                            function.drain(i + 2..i + 4);
                                            let mut vec = Vec::new();
                                            for i in b
                                            {
                                                let n = i.clone().real().to_f64() as usize;
                                                if n <= a.len() && n != 0
                                                {
                                                    vec.push(a[n - 1].clone());
                                                }
                                                else
                                                {
                                                    return Err("out of range");
                                                }
                                            }
                                            Matrix(vec)
                                        }
                                        _ => return Err("non num/vec"),
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
                            for j in a.iter().flatten()
                            {
                                n += j.clone().abs().pow(2);
                            }
                            Num(n.sqrt())
                        }
                        "mean" => Num(a
                            .iter()
                            .flatten()
                            .fold(Complex::new(prec), |sum, val| sum + val)
                            / (a.len() * a[0].len())),
                        "mode" =>
                        {
                            let mut most = (vec![], 0);
                            for i in a.iter().flatten()
                            {
                                let mut count = 0;
                                for j in a.iter().flatten()
                                {
                                    if i == j
                                    {
                                        count += 1;
                                    }
                                }
                                if count > most.1
                                {
                                    most = (vec![i.clone()], count);
                                }
                                if count == most.1 && !most.0.iter().any(|j| i == j)
                                {
                                    most.0.push(i.clone())
                                }
                            }
                            if most.0.len() == 1
                            {
                                Num(most.0[0].clone())
                            }
                            else
                            {
                                Vector(most.0)
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
                else if let Vector(a) = function[i + 1].clone()
                {
                    function[i] = match s.as_str()
                    {
                        "sort" => Vector(sort(a)),
                        "mean" =>
                        {
                            Num(a.iter().fold(Complex::new(prec), |sum, val| sum + val) / a.len())
                        }
                        "median" =>
                        {
                            let a = sort(a);
                            if a.len() % 2 == 0
                            {
                                Vector(vec![a[a.len() / 2 - 1].clone(), a[a.len() / 2].clone()])
                            }
                            else
                            {
                                Num(a[a.len() / 2].clone())
                            }
                        }
                        "mode" =>
                        {
                            let mut most = (vec![], 0);
                            for i in &a
                            {
                                let mut count = 0;
                                for j in &a
                                {
                                    if i == j
                                    {
                                        count += 1;
                                    }
                                }
                                if count > most.1
                                {
                                    most = (vec![i.clone()], count);
                                }
                                if count == most.1 && !most.0.iter().any(|j| i == j)
                                {
                                    most.0.push(i.clone())
                                }
                            }
                            if most.0.len() == 1
                            {
                                Num(most.0[0].clone())
                            }
                            else
                            {
                                Vector(most.0)
                            }
                        }
                        "max" =>
                        {
                            let mut max = a[0].clone();
                            for i in a
                            {
                                if i.real() > max.real()
                                {
                                    max = i
                                }
                            }
                            Num(max)
                        }
                        "min" =>
                        {
                            let mut min = a[0].clone();
                            for i in a
                            {
                                if i.real() < min.real()
                                {
                                    min = i
                                }
                            }
                            Num(min)
                        }
                        "reverse" => Vector(a.iter().rev().cloned().collect()),
                        "link" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                let b = function[i + 3].vec()?;
                                function.drain(i + 2..i + 4);
                                let mut a = a;
                                a.extend(b);
                                Vector(a)
                            }
                            else
                            {
                                return Err("no args");
                            }
                        }
                        "len" | "length" => Num(Complex::with_val(prec, a.len())),
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
                                match function[i + 3].clone()
                                {
                                    Num(b) =>
                                    {
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
                                    Vector(b) =>
                                    {
                                        function.drain(i + 2..i + 4);
                                        let mut vec = Vec::new();
                                        for i in b
                                        {
                                            let n = i.clone().real().to_f64() as usize;
                                            if n <= a.len() && n != 0
                                            {
                                                vec.push(a[n - 1].clone());
                                            }
                                            else
                                            {
                                                return Err("out of range");
                                            }
                                        }
                                        Vector(vec)
                                    }
                                    _ => return Err("non num/vec"),
                                }
                            }
                            else
                            {
                                return Err("no args");
                            }
                        }
                        "factors" | "factor" =>
                        {
                            let mut mat = Vec::new();
                            for num in a
                            {
                                if num.imag().clone().is_zero()
                                {
                                    if num.real().clone().fract().is_zero()
                                    {
                                        let mut vec = Vec::new();
                                        let n = num.real().to_f64() as u128;
                                        for i in 1..=n
                                        {
                                            if n % i == 0
                                            {
                                                vec.push(Complex::with_val(prec, i));
                                            }
                                        }
                                        mat.push(vec);
                                    }
                                    else
                                    {
                                        return Err("fractional factors not supported");
                                    }
                                }
                                else
                                {
                                    return Err("complex factors not supported");
                                }
                            }
                            Matrix(mat)
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
                    function[i] = match s.as_str()
                    {
                        "H" =>
                        {
                            if function.len() > i + 5
                            {
                                let a = function[i + 1].num()?;
                                let b = function[i + 3].num()?;
                                let c = function[i + 5].num()?;
                                if a.imag().is_zero()
                                    && b.imag().is_zero()
                                    && c.imag().is_zero()
                                    && a.real().clone().fract().is_zero()
                                    && b.real().clone().fract().is_zero()
                                    && c.real().clone().fract().is_zero()
                                    && a.real() > &0
                                    && b.real() > &0
                                    && c.real() > &0
                                {
                                    function.drain(i + 2..i + 6);
                                    Num(hyperoperation(a.real(), &b, &c))
                                }
                                else
                                {
                                    return Err("undefined hyperoperation");
                                }
                            }
                            else
                            {
                                return Err("no args");
                            }
                        }
                        "split" =>
                        {
                            let a = function[i + 1].num()?;
                            Vector(vec![
                                Complex::with_val(prec, a.real()),
                                Complex::with_val(prec, a.imag()),
                            ])
                        }
                        "I" =>
                        {
                            let a = function[i + 1].num()?.real().to_f64() as usize;
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
                            Matrix(mat)
                        }
                        "rotate" =>
                        {
                            let a = function[i + 1].num()? / to_deg.clone();
                            Matrix(vec![
                                vec![a.clone().cos(), -a.clone().sin()],
                                vec![a.clone().sin(), a.cos()],
                            ])
                        }
                        "factors" | "factor" =>
                        {
                            let a = function[i + 1].num()?;
                            if a.imag().clone().is_zero()
                            {
                                if a.real().clone().fract().is_zero()
                                {
                                    let mut vec = Vec::new();
                                    let n = a.real().to_f64() as u128;
                                    for i in 1..=n
                                    {
                                        if n % i == 0
                                        {
                                            vec.push(Complex::with_val(prec, i));
                                        }
                                    }
                                    Vector(vec)
                                }
                                else
                                {
                                    return Err("fractional factors not supported");
                                }
                            }
                            else
                            {
                                return Err("complex factors not supported");
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
            }
        }
        i += 1;
    }
    i = function.len().saturating_sub(2);
    while i != 0
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "^" => function[i] = function[i - 1].pow(&function[i + 1])?,
                "^^" => function[i] = function[i - 1].func(&function[i + 1], tetration)?,
                _ =>
                {
                    i -= 1;
                    continue;
                }
            }
            i -= 1;
        }
        else
        {
            i -= 1;
            continue;
        }
        function.remove(i + 2);
        function.remove(i);
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
                "+" => function[i] = function[i - 1].func(&function[i + 1], add)?,
                "-" => function[i] = function[i - 1].func(&function[i + 1], sub)?,
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
                "%" => function[i] = function[i - 1].func(&function[i + 1], rem)?,
                "<" => function[i] = function[i - 1].func(&function[i + 1], lt)?,
                ">" => function[i] = function[i - 1].func(&function[i + 1], gt)?,
                ">=" => function[i] = function[i - 1].func(&function[i + 1], le)?,
                "<=" => function[i] = function[i - 1].func(&function[i + 1], ge)?,
                "==" => function[i] = function[i - 1].func(&function[i + 1], eq)?,
                "!=" => function[i] = function[i - 1].func(&function[i + 1], ne)?,
                ">>" => function[i] = function[i - 1].func(&function[i + 1], shr)?,
                "<<" => function[i] = function[i - 1].func(&function[i + 1], shl)?,
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
                "&&" => function[i] = function[i - 1].func(&function[i + 1], and)?,
                "||" => function[i] = function[i - 1].func(&function[i + 1], or)?,
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
    if function.len() > k + 3 && function[k + 2].str_is(",")
    {
        let b = function[k + 3].clone();
        function.drain(k + 2..k + 4);
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(functions(a, Some(b), to_deg.clone(), s, deg)?)),
            (Vector(a), Vector(b)) =>
            {
                let mut mat = Vec::new();
                for a in a
                {
                    let mut vec = Vec::new();
                    for b in &b
                    {
                        vec.push(functions(
                            a.clone(),
                            Some(b.clone()),
                            to_deg.clone(),
                            s,
                            deg,
                        )?)
                    }
                    mat.push(vec);
                }
                Ok(Matrix(mat))
            }
            (Matrix(a), Matrix(b)) if a.len() == b.len() && a[0].len() == b[0].len() =>
            {
                let mut mat = Vec::new();
                for i in 0..a.len()
                {
                    let mut vec = Vec::new();
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
                let mut vec = Vec::new();
                for i in b
                {
                    vec.push(functions(a.clone(), Some(i), to_deg.clone(), s, deg)?)
                }
                Ok(Vector(vec))
            }
            (Vector(a), Num(b)) =>
            {
                let mut vec = Vec::new();
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
                    let mut vec = Vec::new();
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
                    let mut vec = Vec::new();
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
                    let mut vec = Vec::new();
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
                    let mut vec = Vec::new();
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
                    let mut vec = Vec::new();
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
                let mut vec = Vec::new();
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
            if a.imag().is_zero() && a.real() >= &1
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
            if a.imag().is_zero()
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
            if a.imag().is_zero() && a.real() >= &1
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
            if a.imag().is_zero()
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
            if a.imag().is_zero() && a.real() < &0
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
            if a.imag().is_zero() && a.real() >= &1
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
            if a.imag().is_zero()
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
            if a.imag().is_zero()
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
            let a = if a.imag().is_zero()
            {
                Complex::with_val(prec, a.real()).ln()
            }
            else
            {
                a.ln()
            };
            if let Some(b) = c
            {
                let b = if b.imag().is_zero()
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
        "slog" =>
        {
            if let Some(b) = c
            {
                if a.real() > &1
                {
                    slog(&a, &b)
                }
                else
                {
                    return Err("slog undefined <=1");
                }
            }
            else
            {
                return Err("no args");
            }
        }
        "root" =>
        {
            if let Some(b) = c
            {
                match b.imag().is_zero()
                    && (b.real().to_f64() / 2.0).fract() != 0.0
                    && b.real().clone().fract().is_zero()
                    && a.imag().is_zero()
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
        "P" =>
        {
            if let Some(b) = c
            {
                if !a.imag().is_zero() || !b.imag().is_zero()
                {
                    return Err("pick complex not supported");
                }
                let d: Float = a.real().clone() - b.real() + 1;
                let a: Float = a.real().clone() + 1;
                (a.gamma() / d.gamma()).into()
            }
            else
            {
                return Err("no args");
            }
        }
        "C" | "bi" | "binomial" =>
        {
            if let Some(b) = c
            {
                if !a.imag().is_zero() || !b.imag().is_zero()
                {
                    return Err("binomial complex not supported");
                }
                else if a.real().clone().fract().is_zero() && b.real().clone().fract().is_zero()
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
            if a.imag().is_zero()
            {
                Complex::with_val(prec, a.real().clone().gamma())
            }
            else
            {
                return Err("complex gamma not supported");
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
            if a.imag().is_zero()
            {
                if a.real().is_zero()
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
        "doublefact" =>
        {
            if !a.imag().is_zero()
            {
                return Err("complex factorial not supported");
            }
            let a = a.real().clone();
            let two = Complex::with_val(prec, 2);
            let pi = Complex::with_val(prec, Pi);
            let gam: Float = a.clone() / 2 + 1;
            Complex::with_val(
                prec,
                two.pow(a.clone() / 2 + (1 - (pi.clone() * a.clone()).cos()) / 4)
                    * pi.clone().pow(((pi * a.clone()).cos() - 1) / 4)
                    * gam.gamma(),
            )
        }
        "fact" =>
        {
            if a.imag().is_zero()
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
            if !a.imag().is_zero() || a.real() < &0 || !a.real().clone().fract().is_zero()
            {
                return Err("complex/fractional subfactorial not supported");
            }
            let b: Float = a.real().clone() + 1;
            Complex::with_val(prec, (b.gamma() / Float::with_val(prec.0, 1).exp()).round())
        }
        "sinc" => a.clone().sin() / a,
        "conj" | "conjugate" => a.conj(),
        "erf" =>
        {
            if a.imag().is_zero()
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
            if a.imag().is_zero()
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
            if a.imag().is_zero()
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
            if a.imag().is_zero()
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
            if a.imag().is_zero()
            {
                Complex::with_val(prec, a.real().clone().zeta())
            }
            else
            {
                return Err("complex zeta not supported");
            }
        }
        "prime" =>
        {
            if a.imag().is_zero()
            {
                Complex::with_val(prec, nth_prime(a.real().to_f64() as u128))
            }
            else
            {
                return Err("cant get a complex prime");
            }
        }
        _ =>
        {
            return Err("unreachable7");
        }
    })
}