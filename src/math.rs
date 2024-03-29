use crate::{
    complex::{
        add, and, area, atan, between, cofactor, cubic, determinant, digamma, div, eigenvalues, eq,
        erf, erfc, gamma, gcd, ge, gt, identity, incomplete_beta, incomplete_gamma, inverse,
        lambertw, length, limit, minors, mvec, ne, nth_prime, or, quadratic, recursion, rem, root,
        shl, shr, slog, slope, sort, sub, subfactorial, sum, tetration, to, to_polar, trace,
        transpose, variance,
        LimSide::{Both, Left, Right},
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    AngleType::{Degrees, Gradians, Radians},
    Options, Units,
};
use rug::{
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    integer::IsPrime,
    ops::Pow,
    Complex, Float, Integer,
};
use std::{cmp::Ordering, ops::Rem};
pub fn do_math(
    mut function: Vec<NumStr>,
    options: Options,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
) -> Result<NumStr, &'static str>
{
    if function.is_empty()
    {
        return Err(" ");
    }
    for (i, v) in func_vars.clone().iter().enumerate()
    {
        if v.1.len() != 1 && !v.0.ends_with(')')
        {
            if let Ok(n) = do_math(v.1.clone(), options, func_vars[..i].to_vec())
            {
                func_vars[i] = (v.0.clone(), vec![n]);
            }
        }
    }
    let mut i = 0;
    while i < function.len()
    {
        if let Str(s) = &function[i]
        {
            if s == "rnd"
            {
                function[i] = Num((
                    Complex::with_val(options.prec, fastrand::u64(..)) / u64::MAX,
                    None,
                ))
            }
            else
            {
                let s = s.clone();
                recursively_get_var(&mut function, &func_vars, &i, &s);
            }
        }
        i += 1;
    }
    i = 0;
    while i < function.len()
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "{" =>
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
                    let mut mat = Vec::<Vec<(Complex, Option<Units>)>>::new();
                    for (f, n) in v.iter().enumerate()
                    {
                        if let Str(s) = n
                        {
                            match s.as_str()
                            {
                                "," if count == 0 =>
                                {
                                    let z =
                                        do_math(v[single..f].to_vec(), options, func_vars.clone())?;
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
                        let z = do_math(v[single..].to_vec(), options, func_vars.clone())?;
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
                "(" =>
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
                    if i != 0
                    {
                        if let Str(k) = &function[i - 1]
                        {
                            if matches!(
                                k.as_str(),
                                "next"
                                    | "log"
                                    | "exp"
                                    | "zeta"
                                    | "ζ"
                                    | "polygamma"
                                    | "digamma"
                                    | "ψ"
                                    | "multinomial"
                                    | "gcd"
                                    | "gcf"
                                    | "lcm"
                                    | "ssrt"
                                    | "W"
                                    | "productlog"
                                    | "lambertw"
                                    | "slog"
                                    | "root"
                                    | "atan"
                                    | "arctan"
                                    | "atan2"
                                    | "normP"
                                    | "normD"
                                    | "betaP"
                                    | "betaC"
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
                                    | "gamma"
                                    | "ph"
                                    | "pochhammer"
                                    | "Β"
                                    | "B"
                                    | "beta"
                                    | "I"
                                    | "quad"
                                    | "quadratic"
                                    | "cubic"
                                    | "percentilerank"
                                    | "percentile"
                            )
                            {
                                let v = function[i + 1..j - 1].to_vec();
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
                                    let mut func = vec![function[i - 1].clone()];
                                    function.drain(i..j);
                                    func.push(do_math(
                                        v[..place[0]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?);
                                    for (k, l) in place.iter().enumerate()
                                    {
                                        func.push(do_math(
                                            v[l + 1
                                                ..if k + 1 != place.len()
                                                {
                                                    place[k + 1]
                                                }
                                                else
                                                {
                                                    v.len()
                                                }]
                                                .to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?);
                                    }
                                    function[i - 1] = do_math(func, options, func_vars.clone())?;
                                }
                                else
                                {
                                    let v = vec![
                                        function[i - 1].clone(),
                                        do_math(
                                            function[i + 1..j - 1].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?,
                                    ];
                                    function[i - 1] = do_math(v, options, func_vars.clone())?;
                                    function.drain(i..j);
                                }
                                continue;
                            }
                            else if matches!(
                                k.as_str(),
                                "sum"
                                    | "area"
                                    | "∫"
                                    | "length"
                                    | "slope"
                                    | "summation"
                                    | "prod"
                                    | "product"
                                    | "Σ"
                                    | "Π"
                                    | "vec"
                                    | "mat"
                                    | "piecewise"
                                    | "pw"
                                    | "D"
                                    | "integrate"
                                    | "arclength"
                                    | "lim"
                                    | "limit"
                            )
                            {
                                i = j - 1;
                                continue;
                            }
                            else if k.len() > 1 && k.chars().next().unwrap().is_alphabetic()
                                || matches!(k.as_str(), "C" | "B" | "P" | "I" | "W" | "D")
                            {
                                let v = vec![
                                    function[i - 1].clone(),
                                    do_math(
                                        function[i + 1..j - 1].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?,
                                ];
                                function[i - 1] = do_math(v, options, func_vars.clone())?;
                                function.drain(i..j);
                                continue;
                            }
                        }
                    }
                    let v = function[i + 1..j - 1].to_vec();
                    function[i] = do_math(v, options, func_vars.clone())?;
                    function.drain(i + 1..j);
                }
                _ =>
                {}
            }
        }
        i += 1;
    }
    if function.len() == 1
    {
        return Ok(function[0].clone());
    }
    i = 0;
    let to_deg = match options.deg
    {
        Degrees => 180 / Complex::with_val(options.prec, Pi),
        Radians => Complex::with_val(options.prec, 1),
        Gradians => 200 / Complex::with_val(options.prec, Pi),
    };
    while i < function.len() - 1
    {
        if let Str(s) = &function[i].clone()
        {
            if s.len() > 1 && s.chars().next().unwrap().is_alphabetic()
                || matches!(s.as_str(), "C" | "B" | "P" | "I" | "W" | "D" | "∫")
            {
                if matches!(
                    s.as_str(),
                    "sum"
                        | "area"
                        | "∫"
                        | "length"
                        | "slope"
                        | "product"
                        | "prod"
                        | "summation"
                        | "Σ"
                        | "Π"
                        | "vec"
                        | "mat"
                        | "piecewise"
                        | "pw"
                        | "D"
                        | "integrate"
                        | "arclength"
                        | "lim"
                        | "limit"
                )
                {
                    let mut place = Vec::new();
                    let mut count = 0;
                    for (f, n) in function[i + 2..].iter().enumerate()
                    {
                        if let Str(w) = n
                        {
                            if w == ","
                                && (count == 0 || ((s == "piecewise" || s == "pw") && count == 1))
                            {
                                place.push(f + i + 2);
                            }
                            else if w == "(" || w == "{"
                            {
                                count += 1;
                            }
                            else if w == ")" || w == "}"
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
                    match (
                        s.as_str(),
                        if place.is_empty()
                        {
                            Str(String::new())
                        }
                        else
                        {
                            function[place[0] - 1].clone()
                        },
                    )
                    {
                        ("lim" | "limit", Str(var)) if place.len() == 3 || place.len() == 4 =>
                        {
                            function[i] = limit(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .0,
                                if place.len() == 4
                                {
                                    match (do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0
                                    .real()
                                    .to_f64() as isize)
                                        .cmp(&0)
                                    {
                                        Ordering::Less => Left,
                                        Ordering::Greater => Right,
                                        Ordering::Equal => Both,
                                    }
                                }
                                else
                                {
                                    Both
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("length" | "arclength", Str(var))
                            if place.len() == 4 || place.len() == 5 =>
                        {
                            function[i] = Num((
                                length(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    options,
                                    var.to_string(),
                                    do_math(
                                        function[place[1] + 1..place[2]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0,
                                    do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0,
                                    if place.len() == 5
                                    {
                                        do_math(
                                            function[place[3] + 1..place[4]].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?
                                        .num()?
                                        .0
                                        .real()
                                        .to_f64() as usize
                                    }
                                    else
                                    {
                                        options.prec as usize / 4
                                    },
                                )?,
                                None,
                            ));
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("∫" | "area" | "integrate", Str(var))
                            if place.len() == 4 || place.len() == 5 || place.len() == 6 =>
                        {
                            function[i] = area(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .0,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .0,
                                if place.len() == 4
                                {
                                    options.prec as usize / 4
                                }
                                else
                                {
                                    do_math(
                                        function[place[3] + 1..place[4]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0
                                    .real()
                                    .to_f64() as usize
                                },
                                place.len() != 6,
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("slope" | "D", Str(var))
                            if place.len() == 3
                                || place.len() == 4
                                || place.len() == 5
                                || place.len() == 6 =>
                        {
                            function[i] = slope(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .0,
                                place.len() != 6,
                                if place.len() >= 5
                                {
                                    do_math(
                                        function[place[3] + 1..place[4]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0
                                    .real()
                                    .to_f64() as u32
                                }
                                else
                                {
                                    1
                                },
                                if place.len() >= 4
                                {
                                    match (do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0
                                    .real()
                                    .to_f64() as isize)
                                        .cmp(&0)
                                    {
                                        Ordering::Less => Left,
                                        Ordering::Greater => Right,
                                        Ordering::Equal => Both,
                                    }
                                }
                                else
                                {
                                    Both
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("pw" | "piecewise", _) =>
                        {
                            let mut ans = None;
                            let mut start = i + 3;
                            for (i, end) in place[0..place.len() - 1].iter().enumerate()
                            {
                                if i % 2 == 0
                                    && do_math(
                                        function[*end + 1..place[i + 1] - 1].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .0
                                    .real()
                                        == &1.0
                                {
                                    ans = Some(recursion(
                                        func_vars.clone(),
                                        function[start..*end].to_vec(),
                                        options,
                                    )?);
                                    break;
                                }
                                else
                                {
                                    start = end + 2;
                                }
                            }
                            function[i] = if let Some(n) = ans
                            {
                                n
                            }
                            else
                            {
                                Num((Complex::with_val(options.prec, Nan), None))
                            };
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        (
                            "sum" | "product" | "prod" | "summation" | "Σ" | "Π" | "vec" | "mat",
                            Str(var),
                        ) if place.len() == 4 =>
                        {
                            let start = do_math(
                                function[place[1] + 1..place[2]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
                            .num()?
                            .0;
                            let end = do_math(
                                function[place[2] + 1..place[3]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
                            .num()?
                            .0;
                            if !start.imag().is_zero() || !end.imag().is_zero()
                            {
                                return Err("imag start/end");
                            }
                            if !start.real().clone().fract().is_zero()
                                || !end.real().clone().fract().is_zero()
                            {
                                return Err("fractional start/end");
                            }
                            let start = start.real().to_f64() as isize;
                            let end = end.real().to_f64() as isize;
                            function[i] = match s.as_str()
                            {
                                "vec" | "mat" => mvec(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    &var,
                                    start,
                                    end,
                                    s == "vec",
                                    options,
                                )?,
                                _ => sum(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    &var,
                                    start,
                                    end,
                                    !(s == "sum" || s == "summation" || s == "Σ"),
                                    options,
                                )?,
                            };
                            function.drain(i + 1..=place[3]);
                        }
                        ("sum" | "summation" | "Σ", _) if place.len() <= 1 =>
                        {
                            function[i] = match if place.is_empty()
                            {
                                Ok(function.remove(i + 1).clone())
                            }
                            else
                            {
                                do_math(
                                    function.drain(i + 1..=place[0]).collect::<Vec<NumStr>>(),
                                    options,
                                    func_vars.clone(),
                                )
                            }
                            {
                                Ok(Num(a)) => Num(a.clone()),
                                Ok(Vector(a)) => Num((
                                    a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + val.0.clone()
                                    }),
                                    None,
                                )),
                                Ok(Matrix(a)) => Num((
                                    a.iter()
                                        .flatten()
                                        .fold(Complex::new(options.prec), |sum, val| {
                                            sum + val.0.clone()
                                        }),
                                    None,
                                )),
                                _ => return Err("sum err"),
                            };
                        }
                        ("product" | "prod" | "Π", _) if place.len() <= 1 =>
                        {
                            function[i] = match if place.is_empty()
                            {
                                Ok(function.remove(i + 1).clone())
                            }
                            else
                            {
                                do_math(
                                    function.drain(i + 1..=place[0]).collect::<Vec<NumStr>>(),
                                    options,
                                    func_vars.clone(),
                                )
                            }
                            {
                                Ok(Num(a)) => Num(a.clone()),
                                Ok(Vector(a)) => Num((
                                    a.iter()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.0.clone()
                                        }),
                                    None,
                                )),
                                Ok(Matrix(a)) => Num((
                                    a.iter()
                                        .flatten()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.0.clone()
                                        }),
                                    None,
                                )),
                                _ => return Err("prod err"),
                            };
                        }
                        (_, _) => return Err("arg/var err with sum/prod/vec/slope or similar"),
                    }
                }
                else
                {
                    let arg = function.remove(i + 1);
                    function[i] = match arg.clone()
                    {
                        Matrix(a) => match s.as_str()
                        {
                            "max" =>
                            {
                                let mut vec = Vec::new();
                                for j in a
                                {
                                    let mut max = j[0].clone();
                                    for i in j
                                    {
                                        if i.0.real() > max.0.real()
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
                                        if i.0.real() < min.0.real()
                                        {
                                            min = i
                                        }
                                    }
                                    vec.push(min)
                                }
                                Vector(vec)
                            }
                            "flatten" => Vector(a.into_iter().flatten().collect::<Vec<(
                                Complex,
                                Option<Units>,
                            )>>(
                            )),
                            "cofactor" | "cofactors" | "cof" => Matrix(cofactor(&a)?),
                            "minor" | "minors" => Matrix(minors(&a)?),
                            "adjugate" | "adj" => Matrix(transpose(&cofactor(&a)?)?),
                            "inverse" | "inv" => Matrix(inverse(&a)?),
                            "transpose" | "trans" => Matrix(transpose(&a)?),
                            "len" => Num((Complex::with_val(options.prec, a.len()), None)),
                            "wid" | "width" =>
                            {
                                Num((Complex::with_val(options.prec, a[0].len()), None))
                            }
                            "tr" | "trace" => Num((trace(&a), None)),
                            "det" | "determinant" => Num(determinant(&a)?),
                            "part" =>
                            {
                                if function.len() > i + 2
                                {
                                    match (function.remove(i + 1), function.remove(i + 1))
                                    {
                                        (Num(b), Num(c)) =>
                                        {
                                            let b = b.0;
                                            let c = c.0;
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
                                            let c = c.0;
                                            let n2 = c.clone().real().to_f64() as usize;
                                            let mut vec = Vec::new();
                                            for n in b
                                            {
                                                let n1 = n.0.clone().real().to_f64() as usize;
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
                                            let mut mat = Vec::new();
                                            for g in b
                                            {
                                                let mut vec = Vec::new();
                                                let n1 = g.0.clone().real().to_f64() as usize;
                                                for n in c.clone()
                                                {
                                                    let n2 = n.0.clone().real().to_f64() as usize;
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
                                else if function.len() > i + 1
                                {
                                    match function.remove(i + 1)
                                    {
                                        Num(b) =>
                                        {
                                            let b = b.0;
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
                                            let mut vec = Vec::new();
                                            for i in b
                                            {
                                                let n = i.0.clone().real().to_f64() as usize;
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
                                else
                                {
                                    return Err("no arg");
                                }
                            }
                            "norm" =>
                            {
                                let mut n = Complex::new(options.prec);
                                for j in a.iter().flatten()
                                {
                                    n += j.0.clone().abs().pow(2);
                                }
                                Num((n.sqrt(), None))
                            }
                            "mean" | "μ" => Num((
                                a.iter()
                                    .flatten()
                                    .fold(Complex::new(options.prec), |sum, val| {
                                        sum + val.0.clone()
                                    })
                                    / (a.len() * a[0].len()),
                                None,
                            )),
                            "mode" =>
                            {
                                let mut most = (Vec::new(), 0);
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
                            "median" =>
                            {
                                let a = sort(a.iter().flatten().cloned().collect::<Vec<(
                                    Complex,
                                    Option<Units>,
                                )>>(
                                ));
                                if a.len() % 2 == 0
                                {
                                    Vector(vec![a[a.len() / 2 - 1].clone(), a[a.len() / 2].clone()])
                                }
                                else
                                {
                                    Num(a[a.len() / 2].clone())
                                }
                            }
                            "all" =>
                            {
                                let mut res = true;
                                for a in a.iter().flatten()
                                {
                                    if !(a.0.imag().is_zero() && a.0.real() == &1)
                                    {
                                        res = false
                                    }
                                }
                                Num((Complex::with_val(options.prec, res as u8), None))
                            }
                            "any" =>
                            {
                                let mut res = false;
                                for a in a.iter().flatten()
                                {
                                    if a.0.imag().is_zero() && a.0.real() == &1
                                    {
                                        res = true
                                    }
                                }
                                Num((Complex::with_val(options.prec, res as u8), None))
                            }
                            "eigenvalues" => Vector(eigenvalues(&a)?),
                            "tolist" =>
                            {
                                let mut vec = Vec::new();
                                for a in a
                                {
                                    if a.len() != 2
                                    {
                                        return Err("bad list");
                                    }
                                    for _ in 0..a[1].0.real().to_f64() as usize
                                    {
                                        vec.push(a[0].clone())
                                    }
                                }
                                if vec.is_empty()
                                {
                                    return Err("bad list");
                                }
                                Vector(sort(vec))
                            }
                            "roll" =>
                            {
                                let mut sum: Integer = Integer::new();
                                for i in a
                                {
                                    if i.len() != 2
                                    {
                                        return Err("bad dice data");
                                    }
                                    let a = i[0].0.real().to_f64();
                                    if a > u64::MAX as f64
                                    {
                                        return Err("dice too large");
                                    }
                                    let n = a as u64;
                                    if n == 0
                                    {
                                        return Err("bad dice data");
                                    }
                                    let max = u64::MAX - u64::MAX.rem(n);
                                    let end = i[1].0.real().to_f64() as u64;
                                    let mut i = 0;
                                    while i < end
                                    {
                                        let rnd = fastrand::u64(..);
                                        if rnd < max
                                        {
                                            sum += rnd.rem(n) + 1;
                                            i += 1;
                                        }
                                    }
                                }
                                Num((Complex::with_val(options.prec, sum), None))
                            }
                            "dice" =>
                            {
                                let mut faces = Vec::new();
                                for a in a
                                {
                                    if a.len() != 2
                                    {
                                        return Err("bad list");
                                    }
                                    for _ in 0..a[1].0.real().to_f64() as usize
                                    {
                                        faces.push(a[0].0.real().to_f64() as usize)
                                    }
                                }
                                if faces.is_empty()
                                {
                                    return Err("bad list");
                                }
                                if faces.iter().any(|c| c == &0)
                                {
                                    return Err("bad face value");
                                }
                                let mut last = vec![Integer::from(1); faces[0]];
                                if faces.len() == 1
                                {
                                    Vector(
                                        last.iter()
                                            .map(|a| (Complex::with_val(options.prec, a), None))
                                            .collect::<Vec<(Complex, Option<Units>)>>(),
                                    )
                                }
                                else
                                {
                                    let mut current = last.clone();
                                    for i in 1..faces.len()
                                    {
                                        current = Vec::new();
                                        for p in 0..=faces[0..=i].iter().sum::<usize>() - i - 1
                                        {
                                            let value = last[if (p + 1) > faces[i]
                                            {
                                                p + 1 - faces[i]
                                            }
                                            else
                                            {
                                                0
                                            }
                                                ..=p.min(faces[0..i].iter().sum::<usize>() - i)]
                                                .iter()
                                                .sum::<Integer>();
                                            current.push(value)
                                        }
                                        last.clone_from(&current)
                                    }
                                    current.splice(0..0, vec![Integer::new(); faces.len() - 1]);
                                    Vector(
                                        current
                                            .iter()
                                            .map(|a| (Complex::with_val(options.prec, a), None))
                                            .collect::<Vec<(Complex, Option<Units>)>>(),
                                    )
                                }
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                        Vector(a) => match s.as_str()
                        {
                            "tolist" =>
                            {
                                let mut vec = Vec::new();
                                for (i, a) in a.iter().enumerate()
                                {
                                    let num = (Complex::with_val(options.prec, i + 1), None);
                                    for _ in 1..=a.0.real().to_f64() as usize
                                    {
                                        vec.push(num.clone())
                                    }
                                }
                                if vec.is_empty()
                                {
                                    return Err("bad list");
                                }
                                Vector(sort(vec))
                            }
                            "roll" =>
                            {
                                let mut sum: Integer = Integer::new();
                                let mut i = 0;
                                while i < a.len()
                                {
                                    let a = a[i].0.real().to_f64();
                                    if a > u64::MAX as f64
                                    {
                                        return Err("dice too large");
                                    }
                                    let n = a as u64;
                                    if n == 0
                                    {
                                        return Err("bad dice data");
                                    }
                                    let max = u64::MAX - u64::MAX.rem(n);
                                    let rnd = fastrand::u64(..);
                                    if rnd < max
                                    {
                                        sum += rnd.rem(n) + 1;
                                        i += 1;
                                    }
                                }
                                Num((Complex::with_val(options.prec, sum), None))
                            }
                            "dice" =>
                            {
                                let faces = a
                                    .iter()
                                    .map(|c| c.0.real().to_f64() as usize)
                                    .collect::<Vec<usize>>();
                                if faces.iter().any(|c| c == &0)
                                {
                                    return Err("bad face value");
                                }
                                let mut last = vec![Integer::from(1); faces[0]];
                                if faces.len() == 1
                                {
                                    Vector(
                                        last.iter()
                                            .map(|a| (Complex::with_val(options.prec, a), None))
                                            .collect::<Vec<(Complex, Option<Units>)>>(),
                                    )
                                }
                                else
                                {
                                    let mut current = Vec::new();
                                    for i in 1..faces.len()
                                    {
                                        current = Vec::new();
                                        for p in 0..=faces[0..=i].iter().sum::<usize>() - i - 1
                                        {
                                            let value = last[if (p + 1) > faces[i]
                                            {
                                                p + 1 - faces[i]
                                            }
                                            else
                                            {
                                                0
                                            }
                                                ..=p.min(faces[0..i].iter().sum::<usize>() - i)]
                                                .iter()
                                                .sum::<Integer>();
                                            current.push(value)
                                        }
                                        last.clone_from(&current);
                                    }
                                    current.splice(0..0, vec![Integer::new(); a.len() - 1]);
                                    Vector(
                                        current
                                            .iter()
                                            .map(|a| (Complex::with_val(options.prec, a), None))
                                            .collect::<Vec<(Complex, Option<Units>)>>(),
                                    )
                                }
                            }
                            "quartiles" =>
                            {
                                if a.len() < 2
                                {
                                    return Err("not enough data");
                                }
                                let a = sort(a);
                                let half1 = &a[0..a.len() / 2];
                                let half2 = if a.len() % 2 == 0
                                {
                                    &a[a.len() / 2..a.len()]
                                }
                                else
                                {
                                    &a[a.len() / 2 + 1..a.len()]
                                };
                                if half1.len() % 2 == 0
                                {
                                    Vector(vec![
                                        (
                                            (half1[half1.len() / 2 - 1].0.clone()
                                                + half1[half1.len() / 2].0.clone())
                                                / 2,
                                            None,
                                        ),
                                        (
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len() - 1].0.clone()
                                                    + half2[0].0.clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].0.clone()
                                            },
                                            None,
                                        ),
                                        (
                                            (half2[half2.len() / 2 - 1].0.clone()
                                                + half2[half2.len() / 2].0.clone())
                                                / 2,
                                            None,
                                        ),
                                    ])
                                }
                                else
                                {
                                    Vector(vec![
                                        (half1[half1.len() / 2].0.clone(), None),
                                        (
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len() - 1].0.clone()
                                                    + half2[0].0.clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].0.clone()
                                            },
                                            None,
                                        ),
                                        (half2[half2.len() / 2].0.clone(), None),
                                    ])
                                }
                            }
                            "percentile" =>
                            {
                                if function.len() < i + 1
                                {
                                    return Err("not enough input");
                                }
                                let b = function.remove(i + 1).num()?.0;
                                let r: Float = (b.real().clone() / 100) * a.len();
                                let r = r.ceil().to_f64() as usize;
                                if r > a.len()
                                {
                                    return Err("bad input");
                                }
                                Num(sort(a)[r.saturating_sub(1)].clone())
                            }
                            "percentilerank" =>
                            {
                                if function.len() < i + 1
                                {
                                    return Err("not enough input");
                                }
                                let mut cf = 0;
                                let mut f = 0;
                                let b = function.remove(i + 1).num()?.0;
                                for a in sort(a.clone())
                                {
                                    if a.0.real() < b.real()
                                    {
                                        cf += 1;
                                    }
                                    else if a.0 == b
                                    {
                                        f += 1;
                                    }
                                    else
                                    {
                                        break;
                                    }
                                }
                                Num((
                                    100 * (cf + Complex::with_val(options.prec, f) / 2) / a.len(),
                                    None,
                                ))
                            }
                            "tofreq" =>
                            {
                                if a.is_empty()
                                {
                                    return Err("bad list");
                                }
                                let mut a = sort(a);
                                let mut last = a[0].clone();
                                let mut count = 1;
                                a.remove(0);
                                let mut mat = Vec::new();
                                for a in a
                                {
                                    if a != last
                                    {
                                        mat.push(vec![
                                            last.clone(),
                                            (Complex::with_val(options.prec, count), None),
                                        ]);
                                        last = a;
                                        count = 0;
                                    }
                                    count += 1;
                                }
                                mat.push(vec![
                                    last.clone(),
                                    (Complex::with_val(options.prec, count), None),
                                ]);
                                Matrix(mat)
                            }
                            "standarddeviation" | "σ" =>
                            {
                                Num((variance(&a, options.prec).0.sqrt(), None))
                            }
                            "variance" | "var" => Num(variance(&a, options.prec)),
                            "all" =>
                            {
                                let mut res = true;
                                for a in a
                                {
                                    if !(a.0.imag().is_zero() && a.0.real() == &1)
                                    {
                                        res = false
                                    }
                                }
                                Num((Complex::with_val(options.prec, res as u8), None))
                            }
                            "any" =>
                            {
                                let mut res = false;
                                for a in a
                                {
                                    if a.0.imag().is_zero() && a.0.real() == &1
                                    {
                                        res = true
                                    }
                                }
                                Num((Complex::with_val(options.prec, res as u8), None))
                            }
                            "sort" => Vector(sort(a)),
                            "mean" | "μ" => Num((
                                a.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.0.clone()
                                }) / a.len(),
                                None,
                            )),
                            "median" =>
                            {
                                let a = sort(a);
                                if a.len() % 2 == 0
                                {
                                    Num((
                                        (a[a.len() / 2 - 1].0.clone() + a[a.len() / 2].0.clone())
                                            / 2,
                                        None,
                                    ))
                                }
                                else
                                {
                                    Num((a[a.len() / 2].0.clone(), None))
                                }
                            }
                            "mode" =>
                            {
                                let mut most = (Vec::new(), 0);
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
                                    if i.0.real() > max.0.real()
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
                                    if i.0.real() < min.0.real()
                                    {
                                        min = i
                                    }
                                }
                                Num(min)
                            }
                            "reverse" => Vector(a.iter().rev().cloned().collect()),
                            "link" =>
                            {
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).vec()?;
                                    let mut a = a;
                                    a.extend(b);
                                    Vector(a)
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "len" => Num((Complex::with_val(options.prec, a.len()), None)),
                            "norm" =>
                            {
                                let mut n = Complex::new(options.prec);
                                for i in a
                                {
                                    n += i.0.abs().pow(2);
                                }
                                Num((n.sqrt(), None))
                            }
                            "normalize" =>
                            {
                                let mut n = Complex::new(options.prec);
                                for i in a.clone()
                                {
                                    n += i.0.pow(2);
                                }
                                Vector(
                                    a.iter()
                                        .map(|x| (x.0.clone() / n.clone().sqrt(), None))
                                        .collect(),
                                )
                            }
                            "car" | "cartesian" =>
                            {
                                if a.len() == 2
                                {
                                    let t = a[1].0.clone() / to_deg.clone();
                                    Vector(vec![
                                        (a[0].0.clone() * t.clone().cos(), a[0].1),
                                        (a[0].0.clone() * t.clone().sin(), a[0].1),
                                    ])
                                }
                                else if a.len() == 3
                                {
                                    let t1 = a[1].0.clone() / to_deg.clone();
                                    let t2 = a[2].0.clone() / to_deg.clone();
                                    Vector(vec![
                                        (
                                            a[0].0.clone() * t1.clone().sin() * t2.clone().cos(),
                                            a[0].1,
                                        ),
                                        (
                                            a[0].0.clone() * t1.clone().sin() * t2.clone().sin(),
                                            a[0].1,
                                        ),
                                        (a[0].0.clone() * t1.clone().cos(), a[0].1),
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
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).vec()?;
                                    if a.len() == 3 && b.len() == 3
                                    {
                                        let c: Complex = a[0].0.clone().pow(2)
                                            + a[1].0.clone().pow(2)
                                            + a[2].0.clone().pow(2);
                                        let d: Complex = b[0].0.clone().pow(2)
                                            + b[1].0.clone().pow(2)
                                            + b[2].0.clone().pow(2);
                                        Num((
                                            ((a[0].0.clone() * b[0].0.clone()
                                                + a[1].0.clone() * b[1].0.clone()
                                                + a[2].0.clone() * b[2].0.clone())
                                                / (c.sqrt() * d.sqrt()))
                                            .acos()
                                                * to_deg.clone(),
                                            None,
                                        ))
                                    }
                                    else if a.len() == 2 && b.len() == 2
                                    {
                                        let c: Complex =
                                            a[0].0.clone().pow(2) + a[1].0.clone().pow(2);
                                        let d: Complex =
                                            b[0].0.clone().pow(2) + b[1].0.clone().pow(2);
                                        Num((
                                            ((a[0].0.clone() * b[0].0.clone()
                                                + a[1].0.clone() * b[1].0.clone())
                                                / (c.sqrt() * d.sqrt()))
                                            .acos()
                                                * to_deg.clone(),
                                            None,
                                        ))
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
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).vec()?;
                                    if a.len() == 3 && b.len() == 3
                                    {
                                        Vector(vec![
                                            (
                                                a[1].0.clone() * &b[2].0 - a[2].0.clone() * &b[1].0,
                                                None,
                                            ),
                                            (
                                                a[2].0.clone() * &b[0].0 - a[0].0.clone() * &b[2].0,
                                                None,
                                            ),
                                            (
                                                a[0].0.clone() * &b[1].0 - a[1].0.clone() * &b[0].0,
                                                None,
                                            ),
                                        ])
                                    }
                                    else if a.len() == 2 && b.len() == 2
                                    {
                                        Num((
                                            a[0].0.clone() * &b[1].0 - a[1].0.clone() * &b[0].0,
                                            None,
                                        ))
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
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).clone();
                                    if b.vec()?.len() == a.len()
                                    {
                                        let mut dot = Complex::new(options.prec);
                                        for i in a
                                            .iter()
                                            .zip(b.vec()?.iter())
                                            .map(|(a, b)| a.0.clone() * b.0.clone())
                                        {
                                            dot += i;
                                        }
                                        let mut norm = Complex::new(options.prec);
                                        for i in b.vec()?
                                        {
                                            norm += i.0.abs().pow(2);
                                        }
                                        Num((dot / norm, None)).mul(&b)?
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
                                if function.len() > i + 1
                                {
                                    let mut n = Complex::new(options.prec);
                                    for i in a
                                        .iter()
                                        .zip(function.remove(i + 1).vec()?.iter())
                                        .map(|(a, b)| a.0.clone() * b.0.clone())
                                    {
                                        n += i;
                                    }
                                    Num((n, None))
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "part" =>
                            {
                                if function.len() > i + 1
                                {
                                    match function.remove(i + 1)
                                    {
                                        Num(b) =>
                                        {
                                            let b = b.0;
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
                                            let mut vec = Vec::new();
                                            for i in b
                                            {
                                                let n = i.0.clone().real().to_f64() as usize;
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
                            "split" => Matrix(
                                a.iter()
                                    .map(|a| {
                                        vec![
                                            (a.0.real().clone().into(), None),
                                            (a.0.imag().clone().into(), None),
                                        ]
                                    })
                                    .collect::<Vec<Vec<(Complex, Option<Units>)>>>(),
                            ),
                            "factors" | "factor" =>
                            {
                                let mut fail = false;
                                let mut mat = Vec::new();
                                for num in a
                                {
                                    let num = num.0;
                                    if num.imag().clone().is_zero()
                                    {
                                        if num.real().clone().fract().is_zero()
                                        {
                                            let mut vec = Vec::new();
                                            let n = num.real().to_f64() as usize;
                                            for i in 1..=n
                                            {
                                                if n % i == 0
                                                {
                                                    vec.push((
                                                        Complex::with_val(options.prec, i),
                                                        None,
                                                    ));
                                                }
                                            }
                                            mat.push(vec);
                                        }
                                        else
                                        {
                                            fail = true;
                                            break;
                                        }
                                    }
                                    else
                                    {
                                        fail = true;
                                        break;
                                    }
                                }
                                if fail
                                {
                                    Num((Complex::with_val(options.prec, Nan), None))
                                }
                                else
                                {
                                    Matrix(mat)
                                }
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                        _ => match s.as_str()
                        {
                            "multinomial" =>
                            {
                                let mut numerator: Complex = arg.num()?.0 + 1;
                                let mut divisor = gamma(numerator.clone());
                                while i + 1 < function.len()
                                {
                                    let temp = function.remove(i + 1).num()?.0;
                                    numerator += temp.clone();
                                    let temp = temp.clone() + 1;
                                    divisor *= gamma(temp);
                                }
                                Num((gamma(numerator) / divisor, None))
                            }
                            "Β" | "B" | "beta" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let a = arg.num()?.0;
                                    let b = function.remove(i + 1).num()?.0;
                                    if i + 1 < function.len()
                                    {
                                        let x = function.remove(i + 1).num()?.0;
                                        Num((incomplete_beta(a, b, x), None))
                                    }
                                    else if a.imag().is_zero() && b.imag().is_zero()
                                    {
                                        Num((
                                            gamma(a.clone()) * gamma(b.clone())
                                                / gamma(a + b.clone()),
                                            None,
                                        ))
                                    }
                                    else
                                    {
                                        Num((
                                            incomplete_beta(
                                                Complex::with_val(options.prec, 1),
                                                a,
                                                b,
                                            ),
                                            None,
                                        ))
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "I" | "betaC" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let a = arg.num()?.0;
                                    let b = function.remove(i + 1).num()?.0;
                                    let x = function.remove(i + 1).num()?.0;
                                    Num((
                                        gamma(x.clone() + b.clone())
                                            * incomplete_beta(a, b.clone(), x.clone())
                                            / (gamma(x) * gamma(b)),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "betaP" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let alpha = arg.num()?.0;
                                    let beta = function.remove(i + 1).num()?.0;
                                    let x = function.remove(i + 1).num()?.0;
                                    let c: Complex = 1 - x.clone();
                                    Num((
                                        gamma(alpha.clone() + beta.clone())
                                            * x.pow(alpha.clone() - 1)
                                            * c.pow(beta.clone() - 1)
                                            / (gamma(alpha) * gamma(beta)),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "normP" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let mu = arg.num()?.0;
                                    let sigma = function.remove(i + 1).num()?.0;
                                    let x = function.remove(i + 1).num()?.0;
                                    let n: Complex = (x - mu).pow(2);
                                    let n: Complex = -n / (2 * sigma.clone().pow(2));
                                    let tau: Complex = 2 * Complex::with_val(options.prec, Pi);
                                    Num((n.exp() / (sigma * tau.sqrt()), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "normD" =>
                            {
                                let mut a = arg.num()?.0;
                                if i + 2 < function.len()
                                {
                                    a -= function.remove(i + 1).num()?.0;
                                    a /= function.remove(i + 1).num()?.0;
                                }
                                if a.imag().is_zero()
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num((
                                        ((-a / two.clone().sqrt()).real().clone().erfc() / two)
                                            .into(),
                                        None,
                                    ))
                                }
                                else
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num((erf(-a / two.clone().sqrt()) / two, None))
                                }
                            }
                            "cubic" =>
                            {
                                if i + 4 < function.len()
                                {
                                    let a = arg.num()?.0;
                                    let b = function.remove(i + 1).num()?.0;
                                    let c = function.remove(i + 1).num()?.0;
                                    let d = function.remove(i + 1).num()?.0;
                                    let real = !function.remove(i + 1).num()?.0.is_zero();
                                    let n = cubic(a, b, c, d, real);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 3 < function.len()
                                {
                                    let a = arg.num()?.0;
                                    let b = function.remove(i + 1).num()?.0;
                                    let c = function.remove(i + 1).num()?.0;
                                    let d = function.remove(i + 1).num()?.0;
                                    Vector(cubic(a, b, c, d, false))
                                }
                                else if i + 2 < function.len()
                                {
                                    let b = arg.num()?.0;
                                    let c = function.remove(i + 1).num()?.0;
                                    let d = function.remove(i + 1).num()?.0;
                                    Vector(cubic(
                                        Complex::with_val(options.prec, 1),
                                        b,
                                        c,
                                        d,
                                        false,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "quad" | "quadratic" =>
                            {
                                if i + 3 < function.len()
                                {
                                    let a = arg.num()?.0;
                                    let b = function.remove(i + 1).num()?.0;
                                    let c = function.remove(i + 1).num()?.0;
                                    let real = !function.remove(i + 1).num()?.0.is_zero();
                                    let n = quadratic(a, b, c, real);
                                    if n.is_empty()
                                    {
                                        Num((Complex::with_val(options.prec, Nan), None))
                                    }
                                    else if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 2 < function.len()
                                {
                                    let a = arg.num()?.0;
                                    let b = function.remove(i + 1).num()?.0;
                                    let c = function.remove(i + 1).num()?.0;
                                    Vector(quadratic(a, b, c, false))
                                }
                                else if i + 1 < function.len()
                                {
                                    let b = arg.num()?.0;
                                    let c = function.remove(i + 1).num()?.0;
                                    Vector(quadratic(
                                        Complex::with_val(options.prec, 1),
                                        b,
                                        c,
                                        false,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "split" =>
                            {
                                let a = arg.num()?.0;
                                Vector(vec![
                                    (a.real().clone().into(), None),
                                    (a.imag().clone().into(), None),
                                ])
                            }
                            "iden" | "identity" => Matrix(identity(
                                arg.num()?.0.real().to_f64() as usize,
                                options.prec,
                            )),
                            "rotate" =>
                            {
                                let a = arg.num()?.0 / to_deg.clone();
                                Matrix(vec![
                                    vec![(a.clone().cos(), None), (-a.clone().sin(), None)],
                                    vec![(a.clone().sin(), None), (a.cos(), None)],
                                ])
                            }
                            "factors" | "factor" =>
                            {
                                let a = arg.num()?.0;
                                if a.imag().clone().is_zero()
                                {
                                    if a.real().clone().fract().is_zero()
                                    {
                                        let mut vec = Vec::new();
                                        let n = a.real().to_f64() as usize;
                                        for i in 1..=n
                                        {
                                            if n % i == 0
                                            {
                                                vec.push((
                                                    Complex::with_val(options.prec, i),
                                                    None,
                                                ));
                                            }
                                        }
                                        Vector(vec)
                                    }
                                    else
                                    {
                                        Num((Complex::with_val(options.prec, Nan), None))
                                    }
                                }
                                else
                                {
                                    Num((Complex::with_val(options.prec, Nan), None))
                                }
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                    }
                }
            }
        }
        i += 1;
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "%" => function[i - 1].func(&function[i + 1], rem)?,
                ".." => to(&function[i - 1], &function[i + 1])?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
        }
        else
        {
            i += 1;
        }
    }
    i = function.len().saturating_sub(2);
    while i != 0
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "×" => function[i - 1].mul(&function[i + 1])?,
                "^" => function[i - 1].pow(&function[i + 1])?,
                "^^" => function[i - 1].func(&function[i + 1], tetration)?,
                "//" => function[i - 1].func(&function[i + 1], root)?,
                _ =>
                {
                    i -= 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
            i = i.saturating_sub(2);
        }
        else
        {
            i -= 1;
        }
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "*" => function[i - 1].mul(&function[i + 1])?,
                "/" => function[i - 1].func(&function[i + 1], div)?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
        }
        else
        {
            i += 1;
        }
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "±" => function[i - 1].pm(&function[i + 1])?,
                "+" => function[i - 1].func(&function[i + 1], add)?,
                "-" => function[i - 1].func(&function[i + 1], sub)?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
        }
        else
        {
            i += 1;
        }
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "<" =>
                {
                    if i + 3 < function.len()
                    {
                        if let Str(s) = &function[i + 2]
                        {
                            match (
                                s.as_str(),
                                &function[i - 1],
                                &function[i + 1],
                                &function[i + 3],
                            )
                            {
                                ("<", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            left.0.clone(),
                                            center.0.clone(),
                                            right.0.clone(),
                                            false,
                                            false,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                ("<=", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            left.0.clone(),
                                            center.0.clone(),
                                            right.0.clone(),
                                            false,
                                            true,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                    function[i] = function[i + 1].func(&function[i - 1], gt)?
                }
                "<=" =>
                {
                    if i + 3 < function.len()
                    {
                        if let Str(s) = &function[i + 2]
                        {
                            match (
                                s.as_str(),
                                &function[i - 1],
                                &function[i + 1],
                                &function[i + 3],
                            )
                            {
                                ("<", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            left.0.clone(),
                                            center.0.clone(),
                                            right.0.clone(),
                                            true,
                                            false,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                ("<=", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            left.0.clone(),
                                            center.0.clone(),
                                            right.0.clone(),
                                            true,
                                            true,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                    function[i] = function[i + 1].func(&function[i - 1], ge)?
                }
                ">" =>
                {
                    if i + 3 < function.len()
                    {
                        if let Str(s) = &function[i + 2]
                        {
                            match (
                                s.as_str(),
                                &function[i - 1],
                                &function[i + 1],
                                &function[i + 3],
                            )
                            {
                                (">", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            right.0.clone(),
                                            center.0.clone(),
                                            left.0.clone(),
                                            false,
                                            false,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                (">=", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            right.0.clone(),
                                            center.0.clone(),
                                            left.0.clone(),
                                            true,
                                            false,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                    function[i] = function[i - 1].func(&function[i + 1], gt)?
                }
                ">=" =>
                {
                    if i + 3 < function.len()
                    {
                        if let Str(s) = &function[i + 2]
                        {
                            match (
                                s.as_str(),
                                &function[i - 1],
                                &function[i + 1],
                                &function[i + 3],
                            )
                            {
                                (">", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            right.0.clone(),
                                            center.0.clone(),
                                            left.0.clone(),
                                            false,
                                            true,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                (">=", Num(left), Num(center), Num(right)) =>
                                {
                                    function[i] = Num((
                                        between(
                                            right.0.clone(),
                                            center.0.clone(),
                                            left.0.clone(),
                                            true,
                                            true,
                                        ),
                                        None,
                                    ));
                                    function.drain(i + 1..=i + 3);
                                    function.remove(i - 1);
                                    continue;
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                    function[i] = function[i - 1].func(&function[i + 1], ge)?
                }
                "==" => function[i] = function[i - 1].func(&function[i + 1], eq)?,
                "!=" => function[i] = function[i - 1].func(&function[i + 1], ne)?,
                ">>" => function[i] = function[i - 1].func(&function[i + 1], shr)?,
                "<<" => function[i] = function[i - 1].func(&function[i + 1], shl)?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
        }
        else
        {
            i += 1;
        }
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "&&" => function[i - 1].func(&function[i + 1], and)?,
                "||" => function[i - 1].func(&function[i + 1], or)?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
        }
        else
        {
            i += 1;
        }
    }
    if function.len() == 1
    {
        Ok(function[0].clone())
    }
    else
    {
        Err("failed to compute")
    }
}
fn recursively_get_var(
    function: &mut Vec<NumStr>,
    func_vars: &Vec<(String, Vec<NumStr>)>,
    i: &usize,
    s: &String,
)
{
    for v in func_vars
    {
        if *s == v.0 && !v.0.ends_with(')') && v.1.len() == 1
        {
            if let Str(s) = &v.1[0]
            {
                recursively_get_var(function, func_vars, i, s)
            }
            else
            {
                function[*i] = v.1[0].clone();
            }
        }
    }
}
fn do_functions(
    a: NumStr,
    options: Options,
    function: &mut Vec<NumStr>,
    k: usize,
    to_deg: &Complex,
    s: &str,
) -> Result<NumStr, &'static str>
{
    if function.len() > k + 1
    {
        match (a.clone(), function[k + 1].clone())
        {
            (Num(a), Num(b)) =>
            {
                function.remove(k + 1);
                return Ok(Num(functions(a, Some(b), to_deg.clone(), s, options)?));
            }
            (Vector(a), Vector(b)) =>
            {
                function.remove(k + 1);
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
                            options,
                        )?)
                    }
                    mat.push(vec);
                }
                return Ok(Matrix(mat));
            }
            (Matrix(a), Matrix(b)) if a.len() == b.len() && a[0].len() == b[0].len() =>
            {
                function.remove(k + 1);
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
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Num(a), Vector(b)) =>
            {
                function.remove(k + 1);
                let mut vec = Vec::new();
                for i in b
                {
                    vec.push(functions(a.clone(), Some(i), to_deg.clone(), s, options)?)
                }
                return Ok(Vector(vec));
            }
            (Vector(a), Num(b)) =>
            {
                function.remove(k + 1);
                let mut vec = Vec::new();
                for i in a
                {
                    vec.push(functions(i, Some(b.clone()), to_deg.clone(), s, options)?)
                }
                return Ok(Vector(vec));
            }
            (Num(a), Matrix(b)) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in b
                {
                    let mut vec = Vec::new();
                    for j in i
                    {
                        vec.push(functions(a.clone(), Some(j), to_deg.clone(), s, options)?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Matrix(a), Num(b)) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in a
                {
                    let mut vec = Vec::new();
                    for j in i
                    {
                        vec.push(functions(j, Some(b.clone()), to_deg.clone(), s, options)?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Matrix(a), Vector(b)) if a.len() == b.len() =>
            {
                function.remove(k + 1);
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
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Vector(a), Matrix(b)) if a.len() == b.len() =>
            {
                function.remove(k + 1);
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
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            _ =>
            {}
        }
    }
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
                    vec.push(functions(j, None, to_deg.clone(), s, options)?)
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
                vec.push(functions(i, None, to_deg.clone(), s, options)?)
            }
            Ok(Vector(vec))
        }
        Num(a) => Ok(Num(functions(a, None, to_deg.clone(), s, options)?)),
        _ => Err("str err1"),
    }
}
fn functions(
    mut a: (Complex, Option<Units>),
    mut c: Option<(Complex, Option<Units>)>,
    to_deg: Complex,
    s: &str,
    options: Options,
) -> Result<(Complex, Option<Units>), &'static str>
{
    if a.0.imag().is_zero() && !a.0.imag().is_sign_positive()
    {
        a.0 = Complex::with_val(a.0.prec(), a.0.real())
    }
    let n = if matches!(s, "root" | "sqrt" | "asquare")
    {
        if let Some(ref b) = c
        {
            if b.0.imag().is_zero() && !b.0.imag().is_sign_positive()
            {
                c = Some((Complex::with_val(b.0.prec(), b.0.real()), b.1))
            }
        }
        match s
        {
            "sqrt" | "asquare" => (
                a.0.sqrt(),
                if let Some(a) = a.1
                {
                    Some(a.root(2.0))
                }
                else
                {
                    None
                },
            ),
            "root" =>
            {
                if let Some(b) = c
                {
                    let b = b.0;
                    let c: Float = b.real().clone() / 2;
                    if b.is_zero() && !a.0.is_zero()
                    {
                        (Complex::with_val(a.0.prec(), Nan), None)
                    }
                    else if b.imag().is_zero()
                        && !c.fract().is_zero()
                        && b.real().clone().fract().is_zero()
                        && a.0.imag().is_zero()
                    {
                        (
                            {
                                let a = a.0.real();
                                let ab = a.clone().abs();
                                Complex::with_val(
                                    options.prec,
                                    a / ab.clone() * ab.pow(b.real().clone().recip()),
                                )
                            },
                            if let Some(a) = a.1
                            {
                                Some(a.root(b.real().to_f64()))
                            }
                            else
                            {
                                None
                            },
                        )
                    }
                    else
                    {
                        (
                            a.0.pow(b.clone().recip()),
                            if let Some(a) = a.1
                            {
                                Some(a.root(b.real().to_f64()))
                            }
                            else
                            {
                                None
                            },
                        )
                    }
                }
                else
                {
                    (
                        a.0.sqrt(),
                        if let Some(a) = a.1
                        {
                            Some(a.root(2.0))
                        }
                        else
                        {
                            None
                        },
                    )
                }
            }
            _ => return Err("unreachable"),
        }
    }
    else
    {
        let a = a.0;
        let mut d = None;
        if let Some(ref b) = c
        {
            if b.0.imag().is_zero() && !b.0.imag().is_sign_positive()
            {
                d = Some(Complex::with_val(b.0.prec(), b.0.real()))
            }
            else
            {
                d = Some(b.0.clone())
            }
        }
        (
            match s
            {
                "sin" => (a / to_deg).sin(),
                "csc" => (a / to_deg).sin().recip(),
                "cos" => (a / to_deg).cos(),
                "sec" => (a / to_deg).cos().recip(),
                "tan" => (a / to_deg).tan(),
                "cot" => (a / to_deg).tan().recip(),
                "asin" | "arcsin" => a.clone().asin() * to_deg,
                "acsc" | "arccsc" => a.clone().recip().asin() * to_deg,
                "acos" | "arccos" => a.clone().acos() * to_deg,
                "asec" | "arcsec" => a.clone().recip().acos() * to_deg,
                "atan2" =>
                {
                    if let Some(b) = d
                    {
                        atan(b, a) * to_deg
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "atan" | "arctan" =>
                {
                    if let Some(b) = d
                    {
                        atan(a, b) * to_deg
                    }
                    else
                    {
                        a.atan() * to_deg
                    }
                }
                "acot" | "arccot" => a.recip().atan() * to_deg,
                "sinh" => a.sinh(),
                "csch" => a.sinh().recip(),
                "cosh" => a.cosh(),
                "sech" => a.cosh().recip(),
                "tanh" => a.tanh(),
                "coth" => a.tanh().recip(),
                "asinh" | "arcsinh" => a.asinh(),
                "acsch" | "arccsch" => a.recip().asinh(),
                "acosh" | "arccosh" => a.acosh(),
                "asech" | "arcsech" => a.clone().recip().acosh(),
                "atanh" | "arctanh" => a.clone().atanh(),
                "acoth" | "arccoth" => a.clone().recip().atanh(),
                "cis" =>
                {
                    let b = a / to_deg.clone();
                    b.clone().cos() + b.sin() * Complex::with_val(options.prec, (0.0, 1.0))
                }
                "ln" | "aexp" => a.ln(),
                "ceil" => Complex::with_val(
                    options.prec,
                    (a.real().clone().ceil(), a.imag().clone().ceil()),
                ),
                "floor" => Complex::with_val(
                    options.prec,
                    (a.real().clone().floor(), a.imag().clone().floor()),
                ),
                "round" => Complex::with_val(
                    options.prec,
                    (a.real().clone().round(), a.imag().clone().round()),
                ),
                "recip" => a.recip(),
                "exp" | "aln" =>
                {
                    if let Some(b) = d
                    {
                        a.pow(b)
                    }
                    else
                    {
                        a.exp()
                    }
                }
                "W" | "productlog" | "lambertw" =>
                {
                    if let Some(b) = d
                    {
                        lambertw(b, a.real().to_f64() as isize)
                    }
                    else
                    {
                        lambertw(a, 0)
                    }
                }
                "next" =>
                {
                    if let Some(b) = d
                    {
                        let mut real: Float = a.real().clone();
                        let imag: Float = a.imag().clone();
                        if b.real().is_infinite()
                        {
                            if b.real().is_sign_positive()
                            {
                                real.next_up()
                            }
                            else
                            {
                                real.next_down()
                            }
                        }
                        else
                        {
                            real.next_toward(b.real());
                        }
                        Complex::with_val(options.prec, (real, imag))
                    }
                    else
                    {
                        let mut real: Float = a.real().clone();
                        let imag: Float = a.imag().clone();
                        real.next_up();
                        Complex::with_val(options.prec, (real, imag))
                    }
                }
                "log" =>
                {
                    let a = a.ln();
                    if let Some(b) = d
                    {
                        let b = b.ln();
                        if a.is_zero()
                        {
                            Complex::with_val(options.prec, Infinity)
                        }
                        else if b.real().is_infinite()
                        {
                            -Complex::with_val(options.prec, Infinity)
                        }
                        else
                        {
                            b / a
                        }
                    }
                    else
                    {
                        a
                    }
                }
                "ssrt" =>
                {
                    if let Some(b) = d
                    {
                        let b = b.ln();
                        b.clone() / lambertw(b, a.real().to_f64() as isize)
                    }
                    else
                    {
                        let a = a.ln();
                        a.clone() / lambertw(a, 0)
                    }
                }
                "slog" =>
                {
                    if let Some(b) = d
                    {
                        if a.real() > &1
                        {
                            slog(&a, &b)
                        }
                        else
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "P" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_sign_positive()
                            && a.real().clone().fract().is_zero()
                            && a.imag().is_zero()
                            && b.imag().is_zero()
                        {
                            if b.real().clone().fract().is_zero()
                            {
                                if b.real().clone() % 2 == 1
                                {
                                    gamma(b.clone() + 2)
                                }
                                else
                                {
                                    -gamma(b.clone() + 2)
                                }
                            }
                            else
                            {
                                let a = a + Complex::with_val(options.prec, (0, 1))
                                    * Float::with_val(options.prec, 0.5).pow(options.prec / 2);
                                (gamma(a.clone() + 1) / gamma(a.clone() - b + 1))
                                    .real()
                                    .clone()
                                    .into()
                            }
                        }
                        else
                        {
                            gamma(a.clone() + 1) / gamma(a.clone() - b + 1)
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "C" | "bi" | "binomial" =>
                {
                    if let Some(b) = d
                    {
                        if a.imag().is_zero()
                            && b.imag().is_zero()
                            && a.real().clone().fract().is_zero()
                            && b.real().clone().fract().is_zero()
                            && a.real().is_finite()
                        {
                            Complex::with_val(
                                options.prec,
                                a.real()
                                    .to_integer()
                                    .unwrap()
                                    .binomial(b.real().to_f64() as u32),
                            )
                        }
                        else
                        {
                            gamma(a.clone() + 1)
                                / (gamma(b.clone() + 1) * gamma(a.clone() - b.clone() + 1))
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "pochhammer" | "ph" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_sign_positive() && a.imag().is_zero() && b.imag().is_zero()
                        {
                            Complex::with_val(options.prec, -1).pow(b.clone())
                                * gamma(1 - a.clone())
                                / gamma(1 - a - b)
                        }
                        else
                        {
                            gamma(b.clone() + a.clone()) / gamma(a.clone())
                        }
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "gamma" | "Γ" =>
                {
                    if let Some(b) = d
                    {
                        incomplete_gamma(a, b)
                    }
                    else
                    {
                        gamma(a)
                    }
                }
                "abs" | "norm" => a.abs(),
                "deg" | "degree" => match options.deg
                {
                    Radians => a * 180 / Complex::with_val(options.prec, Pi),
                    Gradians => a * 180.0 / 200.0,
                    Degrees => a,
                },
                "rad" | "radian" => match options.deg
                {
                    Radians => a,
                    Gradians => a * Complex::with_val(options.prec, Pi) / 200,
                    Degrees => a * Complex::with_val(options.prec, Pi) / 180,
                },
                "grad" | "gradian" => match options.deg
                {
                    Radians => a * 200 / Complex::with_val(options.prec, Pi),
                    Gradians => a,
                    Degrees => a * 200.0 / 180.0,
                },
                "re" | "real" => Complex::with_val(options.prec, a.real()),
                "im" | "imag" => Complex::with_val(options.prec, a.imag()),
                "sgn" | "sign" =>
                {
                    if a.is_zero()
                    {
                        Complex::new(options.prec)
                    }
                    else
                    {
                        a.clone() / a.abs()
                    }
                }
                "arg" => a.arg(),
                "cbrt" | "acube" =>
                {
                    if a.imag().is_zero()
                    {
                        if a.real().is_zero()
                        {
                            Complex::new(options.prec)
                        }
                        else
                        {
                            (a.real() / a.real().clone().abs()
                                * a.real()
                                    .clone()
                                    .abs()
                                    .pow(Float::with_val(a.prec().0, 3).recip()))
                            .into()
                        }
                    }
                    else
                    {
                        a.clone().pow(Float::with_val(a.prec().0, 3).recip())
                    }
                }
                "frac" | "fract" => Complex::with_val(
                    options.prec,
                    (a.real().clone().fract(), a.imag().clone().fract()),
                ),
                "int" | "trunc" => Complex::with_val(
                    options.prec,
                    (a.real().clone().trunc(), a.imag().clone().trunc()),
                ),
                "square" | "asqrt" => a.pow(2),
                "cube" | "acbrt" => a.pow(3),
                "doublefact" | "doublefactorial" =>
                {
                    let two = Complex::with_val(options.prec, 2);
                    let pi = Complex::with_val(options.prec, Pi);
                    two.pow(a.clone() / 2 + (1 - (pi.clone() * a.clone()).cos()) / 4)
                        * pi.clone().pow(((pi * a.clone()).cos() - 1) / 4)
                        * gamma(a.clone() / 2 + 1)
                }
                "fact" | "factorial" => gamma(a.clone() + 1),
                "subfact" | "subfactorial" =>
                {
                    if !a.imag().is_zero()
                        || a.real().is_sign_negative()
                        || !a.real().clone().fract().is_zero()
                    {
                        subfactorial(a)
                    }
                    else if a.real().is_zero()
                    {
                        Complex::with_val(options.prec, 1)
                    }
                    else
                    {
                        (gamma(a.clone() + 1) / Float::with_val(options.prec, 1).exp())
                            .real()
                            .clone()
                            .round()
                            .into()
                    }
                }
                "sinc" => a.clone().sin() / a,
                "conj" | "conjugate" => a.conj(),
                "erf" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().erf().into()
                    }
                    else
                    {
                        erf(a)
                    }
                }
                "erfc" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().erfc().into()
                    }
                    else
                    {
                        erfc(a)
                    }
                }
                "erfi" =>
                {
                    let i = Complex::with_val(options.prec, (0, 1));
                    -i.clone() * erf(i * a)
                }
                "ai" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().ai().into()
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "trigamma" => digamma(a, 1),
                "digamma" | "polygamma" | "ψ" =>
                {
                    if let Some(b) = d
                    {
                        digamma(b, a.real().to_f64() as u32)
                    }
                    else if a.imag().is_zero()
                    {
                        if a.real().is_sign_negative() && a.real().clone().fract().is_zero()
                        {
                            Complex::with_val(options.prec, Infinity)
                        }
                        else
                        {
                            a.real().clone().digamma().into()
                        }
                    }
                    else
                    {
                        digamma(a, 0)
                    }
                }
                "zeta" | "ζ" =>
                {
                    if let Some(b) = d
                    {
                        let mut sum = Complex::new(options.prec);
                        for n in 0..=options.prec / 8
                        {
                            sum += 1 / (n + b.clone()).pow(a.clone())
                        }
                        sum
                    }
                    else if a.imag().is_zero()
                    {
                        a.real().clone().zeta().into()
                    }
                    else
                    {
                        let b = Complex::with_val(options.prec, 1);
                        let mut sum = Complex::new(options.prec);
                        for n in 0..=options.prec / 8
                        {
                            sum += 1 / (n + b.clone()).pow(a.clone())
                        }
                        sum
                    }
                }
                "prime" =>
                {
                    if a.imag().is_zero() && a.real().clone().fract() == 0.0
                    {
                        Complex::with_val(options.prec, nth_prime(a.real().to_f64() as usize))
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "lcm" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_finite() || !b.real().is_finite()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            let a = a.real().to_integer().unwrap();
                            let b = b.real().to_integer().unwrap();
                            Complex::with_val(options.prec, a.clone() * b.clone() / gcd(a, b))
                        }
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "gcd" | "gcf" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_finite() || !b.real().is_finite()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            Complex::with_val(
                                options.prec,
                                gcd(
                                    a.real().to_integer().unwrap(),
                                    b.real().to_integer().unwrap(),
                                ),
                            )
                        }
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "isprime" | "is_prime" =>
                {
                    if a.imag().is_zero() && a.real().clone().fract() == 0.0 && a.real().is_finite()
                    {
                        Complex::with_val(
                            options.prec,
                            (a.real().to_integer().unwrap().is_probably_prime(100) != IsPrime::No)
                                as u8,
                        )
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                _ =>
                {
                    return Err("wrong input type");
                }
            },
            None,
        )
    };
    if n.0.imag().is_zero() && !n.0.imag().is_sign_positive()
    {
        Ok((Complex::with_val(n.0.prec(), n.0.real()), n.1))
    }
    else
    {
        Ok(n)
    }
}
