use crate::{
    complex::{
        cofactor, determinant, inverse, minors, mvec, nth_prime, subfact, sum, to_polar, transpose,
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
    let mut count;
    let (mut j, mut v, mut vec, mut mat);
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
                v = function[i + 1..j - 1].to_vec();
                single = 0;
                count = 0;
                vec = Vec::new();
                mat = Vec::<Vec<Complex>>::new();
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
                j = i + 1;
                count = 1;
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
                v = function[i + 1..j - 1].to_vec();
                if i != 0
                {
                    if let Str(k) = &function[i - 1]
                    {
                        if matches!(
                            k.as_str(),
                            "log"
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
                        )
                        {
                            count = 0;
                            place.clear();
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
            if (s.len() > 1 && s.chars().next().unwrap().is_alphabetic())
                || matches!(s.as_str(), "C" | "P")
            {
                if matches!(
                    s.as_str(),
                    "sum" | "product" | "prod" | "summation" | "Σ" | "Π" | "vec" | "mat"
                )
                {
                    place.clear();
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
                            function[i] = match s.as_str()
                            {
                                "vec" | "mat" => mvec(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    var,
                                    do_math(function[place[1] + 1..place[2]].to_vec(), deg, prec)?
                                        .num()?
                                        .real()
                                        .to_f64() as u64,
                                    do_math(function[place[2] + 1..place[3]].to_vec(), deg, prec)?
                                        .num()?
                                        .real()
                                        .to_f64() as u64,
                                    s == "vec",
                                    deg,
                                    prec,
                                )?,
                                _ => sum(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    var,
                                    do_math(function[place[1] + 1..place[2]].to_vec(), deg, prec)?
                                        .num()?
                                        .real()
                                        .to_f64() as u64,
                                    do_math(function[place[2] + 1..place[3]].to_vec(), deg, prec)?
                                        .num()?
                                        .real()
                                        .to_f64() as u64,
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
                        "add" =>
                        {
                            let mut num = Complex::new(prec);
                            for i in a
                            {
                                for n in i
                                {
                                    num += n
                                }
                            }
                            Num(num)
                        }
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
                        "reverse" =>
                        {
                            let mut a = a;
                            a.reverse();
                            Vector(a)
                        }
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
                        "add" =>
                        {
                            let mut num = Complex::new(prec);
                            for n in a
                            {
                                num += n
                            }
                            Num(num)
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
                                if num.imag().clone() == 0.0
                                {
                                    if num.real().clone().fract() == 0.0
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
                        "iden" =>
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
                            a = function[i + 1].num()? / to_deg.clone();
                            Matrix(vec![
                                vec![a.clone().cos(), -a.clone().sin()],
                                vec![a.clone().sin(), a.cos()],
                            ])
                        }
                        "factors" | "factor" =>
                        {
                            a = function[i + 1].num()?;
                            if a.imag().clone() == 0.0
                            {
                                if a.real().clone().fract() == 0.0
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
                    b = function[i + 1].num()?;
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
                    b = function[i + 1].num()?;
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
        "P" =>
        {
            if let Some(b) = c
            {
                if a.imag() != &0.0 && b.imag() != &0.0
                {
                    return Err("binomial complex not supported");
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
        "prime" =>
        {
            if a.imag() == &0.0
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