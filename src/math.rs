use crate::{
    complex::{
        add, and, area, atan, binomial, cofactor, cubic, determinant, digamma, div, eigenvalues,
        eigenvectors, eq, erf, erfc, eta, euleriannumbers, euleriannumbersint, gamma, gcd, ge, gt,
        identity, incomplete_beta, incomplete_gamma, inverse, iter, lambertw, length, limit,
        minors, mvec, ne, nth_prime, or, quadratic, quartic, recursion, rem, root, shl, shr, slog,
        slope, solve, sort, sort_mat, sub, subfactorial, sum, tetration, to, to_polar, trace,
        transpose, unity, variance, zeta,
        LimSide::{Both, Left, Right},
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    AngleType::{Degrees, Gradians, Radians},
    Number, Options, Units,
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
use std::{cmp::Ordering, ops::Rem, time::SystemTime};
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
    let mut i = 0;
    while i < func_vars.len()
    {
        let v = func_vars[i].clone();
        if (v.1.len() != 1
            || (if let Str(s) = &v.1[0]
            {
                matches!(s.as_str(), "rnd" | "epoch")
            }
            else
            {
                false
            }))
            && !v.0.ends_with(')')
        {
            if let Ok(n) = do_math(v.1.clone(), options, func_vars[..i].to_vec())
            {
                for f in function.iter_mut()
                {
                    if let Str(s) = &f
                    {
                        if *s == v.0
                        {
                            *f = n.clone();
                        }
                    }
                }
                if i + 1 < func_vars.len()
                {
                    for fv in func_vars[i + 1..].iter_mut()
                    {
                        for f in fv.1.iter_mut()
                        {
                            if let Str(s) = &f
                            {
                                if *s == v.0
                                {
                                    *f = n.clone();
                                }
                            }
                        }
                    }
                }
                func_vars.remove(i);
                continue;
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
                    let mut mat = Vec::<Vec<Number>>::new();
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
                                    | "ζ"
                                    | "polygamma"
                                    | "digamma"
                                    | "inter"
                                    | "interpolate"
                                    | "lobf"
                                    | "lineofbestfit"
                                    | "ψ"
                                    | "rotate"
                                    | "multinomial"
                                    | "gcd"
                                    | "gcf"
                                    | "lcm"
                                    | "ssrt"
                                    | "W"
                                    | "unity"
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
                                    | "Ap"
                                    | "An"
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
                                    | "quartic"
                                    | "percentilerank"
                                    | "percentile"
                                    | "eigenvalues"
                                    | "eigenvectors"
                                    | "mod"
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
                                    | "solve"
                                    | "∫"
                                    | "length"
                                    | "slope"
                                    | "iter"
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
        if let Str(s) = &function[0]
        {
            if !matches!(s.as_str(), "rnd" | "epoch")
            {
                return Ok(function[0].clone());
            }
        }
        else
        {
            return Ok(function[0].clone());
        }
    }
    i = 0;
    let to_deg = match options.angle
    {
        Degrees => 180 / Complex::with_val(options.prec, Pi),
        Radians => Complex::with_val(options.prec, 1),
        Gradians => 200 / Complex::with_val(options.prec, Pi),
    };
    while i < function.len().saturating_sub(1)
    {
        if let Str(s) = &function[i].clone()
        {
            if (s.len() > 1
                && s.chars().next().unwrap().is_alphabetic()
                && !matches!(s.as_str(), "epoch" | "rnd"))
                || matches!(s.as_str(), "C" | "B" | "P" | "I" | "W" | "D" | "∫")
            {
                if matches!(
                    s.as_str(),
                    "sum"
                        | "area"
                        | "solve"
                        | "∫"
                        | "length"
                        | "slope"
                        | "iter"
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
                        ("iter", Str(var)) if place.len() == 4 || place.len() == 5 =>
                        {
                            function[i] = iter(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .number
                                .real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default(),
                                place.len() == 5,
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("solve", Str(var)) if place.len() == 2 || place.len() == 3 =>
                        {
                            function[i] = solve(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                if place.len() == 3
                                {
                                    do_math(
                                        function[place[1] + 1..place[2]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                }
                                else
                                {
                                    Number::from(Complex::new(options.prec), None)
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
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
                                .num()?,
                                if place.len() == 4
                                {
                                    match do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .number
                                    .real()
                                    .cmp0()
                                    {
                                        Some(Ordering::Less) => Left,
                                        Some(Ordering::Greater) => Right,
                                        _ => Both,
                                    }
                                }
                                else
                                {
                                    Both
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("length" | "arclength", Str(var)) if place.len() == 4 =>
                        {
                            function[i] = Num(length(
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
                                .number,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                                options.prec as usize / 4,
                            )?);
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("∫" | "area" | "integrate", Str(var))
                            if place.len() == 4 || place.len() == 5 =>
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
                                .number,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                                options.prec as usize / 4,
                                place.len() != 5,
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("slope" | "D", Str(var))
                            if place.len() == 3 || place.len() == 4 || place.len() == 5 =>
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
                                .num()?,
                                place.len() != 5,
                                if place.len() >= 4
                                {
                                    do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .number
                                    .real()
                                    .to_integer()
                                    .unwrap_or_default()
                                    .to_u32()
                                    .unwrap_or_default()
                                }
                                else
                                {
                                    1
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("pw" | "piecewise", _) if !place.is_empty() =>
                        {
                            let mut ans = None;
                            let mut start = i + 3;
                            for (i, end) in place[0..if place.len() % 2 == 1
                            {
                                place.len()
                            }
                            else
                            {
                                place.len().saturating_sub(1)
                            }]
                                .iter()
                                .enumerate()
                            {
                                if i + 1 == place.len()
                                    || (i % 2 == 0
                                        && do_math(
                                            function[*end + 1..place[i + 1] - 1].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?
                                        .num()?
                                        .number
                                        .real()
                                            == &1.0)
                                {
                                    ans = Some(recursion(
                                        func_vars.clone(),
                                        function[if i + 1 == place.len()
                                        {
                                            start.saturating_sub(1)
                                        }
                                        else
                                        {
                                            start
                                        }..*end]
                                            .to_vec(),
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
                                Num(Number::from(Complex::with_val(options.prec, Nan), None))
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
                            .number;
                            let end = do_math(
                                function[place[2] + 1..place[3]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
                            .num()?
                            .number;
                            if !start.imag().is_zero() || !end.imag().is_zero()
                            {
                                return Err("imag start/end");
                            }
                            if !start.real().clone().fract().is_zero()
                                || !end.real().clone().fract().is_zero()
                            {
                                return Err("fractional start/end");
                            }
                            let start = start
                                .real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_isize()
                                .unwrap_or_default();
                            let end = end
                                .real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_isize()
                                .unwrap_or_default();
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
                                Ok(Vector(a)) => Num(Number::from(
                                    a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + val.number.clone()
                                    }),
                                    None,
                                )),
                                Ok(Matrix(a)) => Num(Number::from(
                                    a.iter()
                                        .flatten()
                                        .fold(Complex::new(options.prec), |sum, val| {
                                            sum + val.number.clone()
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
                                Ok(Vector(a)) => Num(Number::from(
                                    a.iter()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.number.clone()
                                        }),
                                    None,
                                )),
                                Ok(Matrix(a)) => Num(Number::from(
                                    a.iter()
                                        .flatten()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.number.clone()
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
                            "lobf" | "lineofbestfit" =>
                            {
                                if function.len() > i + 1
                                {
                                    if !a.is_empty() && a.iter().all(|a| a.len() == 2)
                                    {
                                        let mut xsum = Complex::new(options.prec);
                                        let mut ysum = Complex::new(options.prec);
                                        let mut xxsum = Complex::new(options.prec);
                                        let mut xysum = Complex::new(options.prec);
                                        for row in &a
                                        {
                                            let x = row[0].number.clone();
                                            let y = row[1].number.clone();
                                            xsum += x.clone();
                                            ysum += y.clone();
                                            xxsum += x.clone() * x.clone();
                                            xysum += x * y;
                                        }
                                        let m: Complex = (a.len() * xysum
                                            - xsum.clone() * ysum.clone())
                                            / (a.len() * xxsum - xsum.clone().pow(2));
                                        let b = (ysum - m.clone() * xsum) / a.len();
                                        let x = function.remove(i + 1).num()?.number;
                                        Num(Number::from(m * x + b, a[0][1].units))
                                    }
                                    else
                                    {
                                        return Err("dimensions too high");
                                    }
                                }
                                else
                                {
                                    return Err("no x value given");
                                }
                            }
                            "inter" | "interpolate" =>
                            {
                                if function.len() > i + 1
                                {
                                    if !a.is_empty() && a.iter().all(|a| a.len() == 2)
                                    {
                                        let x = function.remove(i + 1).num()?.number;
                                        let mut sum = Complex::new(options.prec);
                                        for i in 0..a.len()
                                        {
                                            let mut prod = Complex::with_val(options.prec, 1);
                                            for j in 0..a.len()
                                            {
                                                if j != i
                                                {
                                                    prod *= (x.clone() - a[j][0].number.clone())
                                                        / (a[i][0].number.clone()
                                                            - a[j][0].number.clone())
                                                }
                                            }
                                            sum += prod * a[i][1].number.clone()
                                        }
                                        Num(Number::from(sum, a[0][1].units))
                                    }
                                    else
                                    {
                                        return Err("dimensions too high");
                                    }
                                }
                                else
                                {
                                    return Err("no x value given");
                                }
                            }
                            "sort" => Matrix(sort_mat(a)),
                            "max" =>
                            {
                                let mut vec = Vec::new();
                                for j in a
                                {
                                    let mut max = j[0].clone();
                                    for i in j
                                    {
                                        if i.number.real() > max.number.real()
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
                                        if i.number.real() < min.number.real()
                                        {
                                            min = i
                                        }
                                    }
                                    vec.push(min)
                                }
                                Vector(vec)
                            }
                            "flatten" => Vector(a.into_iter().flatten().collect::<Vec<Number>>()),
                            "cofactor" | "cofactors" | "cof" => Matrix(cofactor(&a)?),
                            "minor" | "minors" => Matrix(minors(&a)?),
                            "adjugate" | "adj" => Matrix(transpose(&cofactor(&a)?)),
                            "inverse" | "inv" => Matrix(inverse(&a)?),
                            "transpose" | "trans" => Matrix(transpose(&a)),
                            "len" =>
                            {
                                Num(Number::from(Complex::with_val(options.prec, a.len()), None))
                            }
                            "wid" | "width" => Num(Number::from(
                                Complex::with_val(options.prec, a[0].len()),
                                None,
                            )),
                            "tr" | "trace" => Num(Number::from(trace(&a), None)),
                            "det" | "determinant" => Num(determinant(&a)?),
                            "part" =>
                            {
                                if function.len() > i + 2
                                {
                                    match (function.remove(i + 1), function.remove(i + 1))
                                    {
                                        (Num(b), Num(c)) =>
                                        {
                                            let b = b.number;
                                            let c = c.number;
                                            let n1 =
                                                b.clone().real().to_integer().unwrap_or_default();
                                            let getcol = n1 == -1;
                                            let n1 = n1.to_usize().unwrap_or_default();
                                            let n2 = c
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if getcol
                                            {
                                                if a.iter().all(|a| n2 < a.len())
                                                {
                                                    Vector(
                                                        a.iter()
                                                            .map(|a| a[n2].clone())
                                                            .collect::<Vec<Number>>(),
                                                    )
                                                }
                                                else
                                                {
                                                    return Err("out of range");
                                                }
                                            }
                                            else if n1 < a.len() && n2 < a[n1].len()
                                            {
                                                Num(a[n1][n2].clone())
                                            }
                                            else
                                            {
                                                return Err("not in matrix");
                                            }
                                        }
                                        (Num(b), Vector(c)) =>
                                        {
                                            let b = b.number;
                                            let n1 =
                                                b.clone().real().to_integer().unwrap_or_default();
                                            let getcol = n1 == -1;
                                            let n1 = n1.to_usize().unwrap_or_default();
                                            if getcol
                                            {
                                                let mut mat = Vec::new();
                                                for n in c
                                                {
                                                    let n = n
                                                        .number
                                                        .clone()
                                                        .real()
                                                        .to_integer()
                                                        .unwrap_or_default()
                                                        .to_usize()
                                                        .unwrap_or_default();
                                                    if a.iter().all(|a| n < a.len())
                                                    {
                                                        mat.push(
                                                            a.iter()
                                                                .map(|a| a[n].clone())
                                                                .collect::<Vec<Number>>(),
                                                        )
                                                    }
                                                    else
                                                    {
                                                        return Err("out of range");
                                                    }
                                                }
                                                Matrix(transpose(&mat))
                                            }
                                            else
                                            {
                                                let mut vec = Vec::new();
                                                for n in c
                                                {
                                                    let n2 = n
                                                        .number
                                                        .clone()
                                                        .real()
                                                        .to_integer()
                                                        .unwrap_or_default()
                                                        .to_usize()
                                                        .unwrap_or_default();
                                                    if n1 < a.len() && n2 < a[n1].len()
                                                    {
                                                        vec.push(a[n1][n2].clone())
                                                    }
                                                    else
                                                    {
                                                        return Err("not in matrix");
                                                    }
                                                }
                                                Vector(vec)
                                            }
                                        }
                                        (Vector(b), Num(c)) =>
                                        {
                                            let c = c.number;
                                            let n2 = c
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            let mut vec = Vec::new();
                                            for n in b
                                            {
                                                let n1 = n
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n1 < a.len() && n2 < a[n1].len()
                                                {
                                                    vec.push(a[n1][n2].clone())
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
                                                let n1 = g
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                for n in c.clone()
                                                {
                                                    let n2 = n
                                                        .number
                                                        .clone()
                                                        .real()
                                                        .to_integer()
                                                        .unwrap_or_default()
                                                        .to_usize()
                                                        .unwrap_or_default();
                                                    if n1 < a.len() && n2 < a[n1].len()
                                                    {
                                                        vec.push(a[n1][n2].clone())
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
                                            let b = b.number;
                                            let n = b
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if n < a.len()
                                            {
                                                Vector(a[n].clone())
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
                                                let n = i
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n < a.len()
                                                {
                                                    vec.push(a[n].clone());
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
                                    n += j.number.clone().abs().pow(2);
                                }
                                Num(Number::from(n.sqrt(), None))
                            }
                            "mean" | "μ" => Num(Number::from(
                                a.iter()
                                    .flatten()
                                    .fold(Complex::new(options.prec), |sum, val| {
                                        sum + val.number.clone()
                                    })
                                    / a.iter().fold(0, |sum, a| sum + a.len()),
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
                                let a = sort(a.iter().flatten().cloned().collect::<Vec<Number>>());
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
                                    if !(a.number.imag().is_zero() && a.number.real() == &1)
                                    {
                                        res = false
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "any" =>
                            {
                                let mut res = false;
                                for a in a.iter().flatten()
                                {
                                    if a.number.imag().is_zero() && a.number.real() == &1
                                    {
                                        res = true
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "eigenvalues" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Str(_))
                                {
                                    function.remove(i + 1);
                                    eigenvalues(&a, true)?
                                }
                                else
                                {
                                    eigenvalues(&a, false)?
                                }
                            }
                            "eigenvectors" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Str(_))
                                {
                                    function.remove(i + 1);
                                    eigenvectors(&a, true)?
                                }
                                else
                                {
                                    eigenvectors(&a, false)?
                                }
                            }
                            "tolist" =>
                            {
                                let mut vec = Vec::new();
                                for a in a
                                {
                                    if a.len() != 2
                                    {
                                        return Err("bad list");
                                    }
                                    for _ in 0..a[1]
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_usize()
                                        .unwrap_or_default()
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
                            "weighted" =>
                            {
                                let mut sum = Integer::new();
                                for i in &a
                                {
                                    sum += i[1].number.real().to_integer().unwrap_or_default();
                                }
                                let n = sum.to_u64().unwrap_or_default();
                                let max = u64::MAX - u64::MAX.rem(n);
                                let mut rnd = u64::MAX;
                                while rnd >= max
                                {
                                    rnd = fastrand::u64(..);
                                }
                                rnd = rnd.rem(n) + 1;
                                let mut num =
                                    Number::from(Complex::with_val(options.prec, Nan), None);
                                for i in &a
                                {
                                    rnd = rnd.saturating_sub(
                                        i[1].number
                                            .real()
                                            .to_integer()
                                            .unwrap_or_default()
                                            .to_u64()
                                            .unwrap_or_default(),
                                    );
                                    if rnd == 0
                                    {
                                        num = i[0].clone();
                                        break;
                                    }
                                }
                                Num(num)
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
                                    let a = i[0].number.real().to_integer().unwrap_or_default();
                                    if a > u64::MAX || a == 0
                                    {
                                        return Err("dice too large or bad dice data");
                                    }
                                    let n = a.to_u64().unwrap_or_default();
                                    let max = u64::MAX - u64::MAX.rem(n);
                                    let end = i[1]
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_u64()
                                        .unwrap_or_default();
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
                                Num(Number::from(Complex::with_val(options.prec, sum), None))
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
                                    for _ in 0..a[1]
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_usize()
                                        .unwrap_or_default()
                                    {
                                        faces.push(
                                            a[0].number
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default(),
                                        )
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
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
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
                                    current.splice(
                                        0..0,
                                        vec![Integer::new(); faces.len().saturating_sub(1)],
                                    );
                                    Vector(
                                        current
                                            .iter()
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
                                    )
                                }
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                        Vector(a) => match s.as_str()
                        {
                            "transpose" | "trans" => Matrix(transpose(&[a])),
                            "tolist" =>
                            {
                                let mut vec = Vec::new();
                                for (i, a) in a.iter().enumerate()
                                {
                                    let num =
                                        Number::from(Complex::with_val(options.prec, i + 1), None);
                                    for _ in 1..=a
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_usize()
                                        .unwrap_or_default()
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
                                    let a = a[i].number.real().to_integer().unwrap_or_default();
                                    if a > u64::MAX as f64 || a == 0
                                    {
                                        return Err("dice too large or bad dice data");
                                    }
                                    let n = a.to_u64().unwrap_or_default();
                                    let max = u64::MAX - u64::MAX.rem(n);
                                    let rnd = fastrand::u64(..);
                                    if rnd < max
                                    {
                                        sum += rnd.rem(n) + 1;
                                        i += 1;
                                    }
                                }
                                Num(Number::from(Complex::with_val(options.prec, sum), None))
                            }
                            "dice" =>
                            {
                                let faces = a
                                    .iter()
                                    .map(|c| {
                                        c.number
                                            .real()
                                            .to_integer()
                                            .unwrap_or_default()
                                            .to_usize()
                                            .unwrap_or_default()
                                    })
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
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
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
                                    current.splice(
                                        0..0,
                                        vec![Integer::new(); a.len().saturating_sub(1)],
                                    );
                                    Vector(
                                        current
                                            .iter()
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
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
                                        Number::from(
                                            (half1[half1.len() / 2 - 1].number.clone()
                                                + half1[half1.len() / 2].number.clone())
                                                / 2,
                                            None,
                                        ),
                                        Number::from(
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len().saturating_sub(1)]
                                                    .number
                                                    .clone()
                                                    + half2[0].number.clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].number.clone()
                                            },
                                            None,
                                        ),
                                        Number::from(
                                            (half2[half2.len() / 2 - 1].number.clone()
                                                + half2[half2.len() / 2].number.clone())
                                                / 2,
                                            None,
                                        ),
                                    ])
                                }
                                else
                                {
                                    Vector(vec![
                                        Number::from(half1[half1.len() / 2].number.clone(), None),
                                        Number::from(
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len().saturating_sub(1)]
                                                    .number
                                                    .clone()
                                                    + half2[0].number.clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].number.clone()
                                            },
                                            None,
                                        ),
                                        Number::from(half2[half2.len() / 2].number.clone(), None),
                                    ])
                                }
                            }
                            "percentile" =>
                            {
                                if function.len() < i + 1
                                {
                                    return Err("not enough input");
                                }
                                let b = function.remove(i + 1).num()?.number;
                                let r: Float = (b.real().clone() / 100) * a.len();
                                let r = r
                                    .ceil()
                                    .to_integer()
                                    .unwrap_or_default()
                                    .to_usize()
                                    .unwrap_or_default();
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
                                let b = function.remove(i + 1).num()?.number;
                                for a in sort(a.clone())
                                {
                                    if a.number.real() < b.real()
                                    {
                                        cf += 1;
                                    }
                                    else if a.number == b
                                    {
                                        f += 1;
                                    }
                                    else
                                    {
                                        break;
                                    }
                                }
                                Num(Number::from(
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
                                            Number::from(
                                                Complex::with_val(options.prec, count),
                                                None,
                                            ),
                                        ]);
                                        last = a;
                                        count = 0;
                                    }
                                    count += 1;
                                }
                                mat.push(vec![
                                    last.clone(),
                                    Number::from(Complex::with_val(options.prec, count), None),
                                ]);
                                Matrix(mat)
                            }
                            "standarddeviation" | "σ" =>
                            {
                                Num(Number::from(variance(&a, options.prec).number.sqrt(), None))
                            }
                            "variance" | "var" => Num(variance(&a, options.prec)),
                            "all" =>
                            {
                                let mut res = true;
                                for a in a
                                {
                                    if !(a.number.imag().is_zero() && a.number.real() == &1)
                                    {
                                        res = false
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "any" =>
                            {
                                let mut res = false;
                                for a in a
                                {
                                    if a.number.imag().is_zero() && a.number.real() == &1
                                    {
                                        res = true
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "sort" => Vector(sort(a)),
                            "mean" | "μ" => Num(Number::from(
                                a.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.number.clone()
                                }) / a.len(),
                                None,
                            )),
                            "median" =>
                            {
                                let a = sort(a);
                                if a.len() % 2 == 0
                                {
                                    Num(Number::from(
                                        (a[a.len() / 2 - 1].number.clone()
                                            + a[a.len() / 2].number.clone())
                                            / 2,
                                        None,
                                    ))
                                }
                                else
                                {
                                    Num(Number::from(a[a.len() / 2].number.clone(), None))
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
                                    if i.number.real() > max.number.real()
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
                                    if i.number.real() < min.number.real()
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
                            "len" =>
                            {
                                Num(Number::from(Complex::with_val(options.prec, a.len()), None))
                            }
                            "norm" =>
                            {
                                let mut n = Complex::new(options.prec);
                                for i in a
                                {
                                    n += i.number.abs().pow(2);
                                }
                                Num(Number::from(n.sqrt(), None))
                            }
                            "normalize" =>
                            {
                                let mut n = Complex::new(options.prec);
                                for i in a.clone()
                                {
                                    n += i.number.pow(2);
                                }
                                Vector(
                                    a.iter()
                                        .map(|x| {
                                            Number::from(x.number.clone() / n.clone().sqrt(), None)
                                        })
                                        .collect(),
                                )
                            }
                            "car" | "cartesian" =>
                            {
                                if a.len() == 2
                                {
                                    let t = a[1].number.clone() / to_deg.clone();
                                    Vector(vec![
                                        Number::from(
                                            a[0].number.clone() * t.clone().cos(),
                                            a[0].units,
                                        ),
                                        Number::from(
                                            a[0].number.clone() * t.clone().sin(),
                                            a[0].units,
                                        ),
                                    ])
                                }
                                else if a.len() == 3
                                {
                                    let t1 = a[1].number.clone() / to_deg.clone();
                                    let t2 = a[2].number.clone() / to_deg.clone();
                                    Vector(vec![
                                        Number::from(
                                            a[0].number.clone()
                                                * t1.clone().sin()
                                                * t2.clone().cos(),
                                            a[0].units,
                                        ),
                                        Number::from(
                                            a[0].number.clone()
                                                * t1.clone().sin()
                                                * t2.clone().sin(),
                                            a[0].units,
                                        ),
                                        Number::from(
                                            a[0].number.clone() * t1.clone().cos(),
                                            a[0].units,
                                        ),
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
                                        let c: Complex = a[0].number.clone().pow(2)
                                            + a[1].number.clone().pow(2)
                                            + a[2].number.clone().pow(2);
                                        let d: Complex = b[0].number.clone().pow(2)
                                            + b[1].number.clone().pow(2)
                                            + b[2].number.clone().pow(2);
                                        Num(Number::from(
                                            ((a[0].number.clone() * b[0].number.clone()
                                                + a[1].number.clone() * b[1].number.clone()
                                                + a[2].number.clone() * b[2].number.clone())
                                                / (c.sqrt() * d.sqrt()))
                                            .acos()
                                                * to_deg.clone(),
                                            None,
                                        ))
                                    }
                                    else if a.len() == 2 && b.len() == 2
                                    {
                                        let c: Complex =
                                            a[0].number.clone().pow(2) + a[1].number.clone().pow(2);
                                        let d: Complex =
                                            b[0].number.clone().pow(2) + b[1].number.clone().pow(2);
                                        Num(Number::from(
                                            ((a[0].number.clone() * b[0].number.clone()
                                                + a[1].number.clone() * b[1].number.clone())
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
                                    let vec = to_polar(a, to_deg.clone());
                                    if vec.len() == 3
                                    {
                                        Vector(vec[1..=2].to_vec())
                                    }
                                    else if vec.len() == 2
                                    {
                                        Num(vec[1].clone())
                                    }
                                    else
                                    {
                                        return Err("cant decern angles");
                                    }
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
                                            Number::from(
                                                a[1].number.clone() * &b[2].number
                                                    - a[2].number.clone() * &b[1].number,
                                                None,
                                            ),
                                            Number::from(
                                                a[2].number.clone() * &b[0].number
                                                    - a[0].number.clone() * &b[2].number,
                                                None,
                                            ),
                                            Number::from(
                                                a[0].number.clone() * &b[1].number
                                                    - a[1].number.clone() * &b[0].number,
                                                None,
                                            ),
                                        ])
                                    }
                                    else if a.len() == 2 && b.len() == 2
                                    {
                                        Num(Number::from(
                                            a[0].number.clone() * &b[1].number
                                                - a[1].number.clone() * &b[0].number,
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
                                            .map(|(a, b)| a.number.clone() * b.number.clone())
                                        {
                                            dot += i;
                                        }
                                        let mut norm = Complex::new(options.prec);
                                        for i in b.vec()?
                                        {
                                            norm += i.number.abs().pow(2);
                                        }
                                        Num(Number::from(dot / norm, None)).mul(&b)?
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
                                        .map(|(a, b)| a.number.clone() * b.number.clone())
                                    {
                                        n += i;
                                    }
                                    Num(Number::from(n, None))
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
                                            let b = b.number;
                                            let n = b
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if n < a.len()
                                            {
                                                Num(a[n].clone())
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
                                                let n = i
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n < a.len()
                                                {
                                                    vec.push(a[n].clone());
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
                                            Number::from(a.number.real().clone().into(), None),
                                            Number::from(a.number.imag().clone().into(), None),
                                        ]
                                    })
                                    .collect::<Vec<Vec<Number>>>(),
                            ),
                            "factors" | "factor" =>
                            {
                                let mut fail = false;
                                let mut mat = Vec::new();
                                for num in a
                                {
                                    let num = num.number;
                                    if num.imag().clone().is_zero()
                                    {
                                        if num.real().clone().fract().is_zero()
                                        {
                                            let mut vec = Vec::new();
                                            let n = num
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            for i in 1..=n
                                            {
                                                if n % i == 0
                                                {
                                                    vec.push(Number::from(
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
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
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
                                let mut numerator: Complex = arg.num()?.number + 1;
                                let mut divisor = gamma(numerator.clone());
                                while i + 1 < function.len() && !matches!(&function[i + 1], Str(_))
                                {
                                    let temp = function.remove(i + 1).num()?.number;
                                    numerator += temp.clone();
                                    let temp = temp.clone() + 1;
                                    divisor *= gamma(temp);
                                }
                                Num(Number::from(gamma(numerator) / divisor, None))
                            }
                            "Β" | "B" | "beta" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    if i + 1 < function.len()
                                    {
                                        let x = function.remove(i + 1).num()?.number;
                                        Num(Number::from(incomplete_beta(a, b, x), None))
                                    }
                                    else if a.imag().is_zero() && b.imag().is_zero()
                                    {
                                        Num(Number::from(
                                            gamma(a.clone()) * gamma(b.clone())
                                                / gamma(a + b.clone()),
                                            None,
                                        ))
                                    }
                                    else
                                    {
                                        Num(Number::from(
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
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let x = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
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
                                    let alpha = arg.num()?.number;
                                    let beta = function.remove(i + 1).num()?.number;
                                    let x = function.remove(i + 1).num()?.number;
                                    let c: Complex = 1 - x.clone();
                                    Num(Number::from(
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
                                    let mu = arg.num()?.number;
                                    let sigma = function.remove(i + 1).num()?.number;
                                    let x = function.remove(i + 1).num()?.number;
                                    let n: Complex = (x - mu).pow(2);
                                    let n: Complex = -n / (2 * sigma.clone().pow(2));
                                    let tau: Complex = 2 * Complex::with_val(options.prec, Pi);
                                    Num(Number::from(n.exp() / (sigma * tau.sqrt()), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "normD" =>
                            {
                                let mut a = arg.num()?.number;
                                if i + 2 < function.len()
                                {
                                    a -= function.remove(i + 1).num()?.number;
                                    a /= function.remove(i + 1).num()?.number;
                                }
                                if a.imag().is_zero()
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num(Number::from(
                                        ((-a / two.clone().sqrt()).real().clone().erfc() / two)
                                            .into(),
                                        None,
                                    ))
                                }
                                else
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num(Number::from(erf(-a / two.clone().sqrt()) / two, None))
                                }
                            }
                            "quartic" =>
                            {
                                if i + 5 < function.len()
                                {
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let d = function.remove(i + 1).num()?.number;
                                    let e = function.remove(i + 1).num()?.number;
                                    function.remove(i + 1);
                                    let n = quartic(a, b, c, d, e, true);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 4 < function.len()
                                {
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let d = function.remove(i + 1).num()?.number;
                                    let e = function.remove(i + 1).num()?.number;
                                    let n = quartic(a, b, c, d, e, false);
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
                                    let b = arg.num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let d = function.remove(i + 1).num()?.number;
                                    let e = function.remove(i + 1).num()?.number;
                                    let n = quartic(
                                        Complex::with_val(options.prec, 1),
                                        b,
                                        c,
                                        d,
                                        e,
                                        false,
                                    );
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "cubic" =>
                            {
                                if i + 4 < function.len()
                                {
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let d = function.remove(i + 1).num()?.number;
                                    function.remove(i + 1);
                                    let n = cubic(a, b, c, d, true);
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
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let d = function.remove(i + 1).num()?.number;
                                    let n = cubic(a, b, c, d, false);
                                    if n.len() == 1
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
                                    let b = arg.num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let d = function.remove(i + 1).num()?.number;
                                    let n =
                                        cubic(Complex::with_val(options.prec, 1), b, c, d, false);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
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
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    function.remove(i + 1);
                                    let n = quadratic(a, b, c, true);
                                    if n.is_empty()
                                    {
                                        Num(Number::from(
                                            Complex::with_val(options.prec, Nan),
                                            None,
                                        ))
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
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let n = quadratic(a, b, c, false);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 1 < function.len()
                                {
                                    let b = arg.num()?.number;
                                    let c = function.remove(i + 1).num()?.number;
                                    let n =
                                        quadratic(Complex::with_val(options.prec, 1), b, c, false);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "cossin" =>
                            {
                                let (a, b) = arg.num()?.number.sin_cos(Complex::new(options.prec));
                                Vector(vec![Number::from(b, None), Number::from(a, None)])
                            }
                            "split" =>
                            {
                                let a = arg.num()?.number;
                                Vector(vec![
                                    Number::from(a.real().clone().into(), None),
                                    Number::from(a.imag().clone().into(), None),
                                ])
                            }
                            "iden" | "identity" => Matrix(identity(
                                arg.num()?
                                    .number
                                    .real()
                                    .to_integer()
                                    .unwrap_or_default()
                                    .to_usize()
                                    .unwrap_or_default(),
                                options.prec,
                            )),
                            "rotate" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let (sina, cosa) = (arg.num()?.number / to_deg.clone())
                                        .sin_cos(Complex::new(options.prec));
                                    let (sinb, cosb) = (function.remove(i + 1).num()?.number
                                        / to_deg.clone())
                                    .sin_cos(Complex::new(options.prec));
                                    let (sinc, cosc) = (function.remove(i + 1).num()?.number
                                        / to_deg.clone())
                                    .sin_cos(Complex::new(options.prec));
                                    Matrix(vec![
                                        vec![
                                            Number::from(cosa.clone() * cosb.clone(), None),
                                            Number::from(
                                                cosa.clone() * sinb.clone() * sinc.clone()
                                                    - sina.clone() * cosc.clone(),
                                                None,
                                            ),
                                            Number::from(
                                                cosa.clone() * sinb.clone() * cosc.clone()
                                                    + sina.clone() * sinc.clone(),
                                                None,
                                            ),
                                        ],
                                        vec![
                                            Number::from(sina.clone() * cosb.clone(), None),
                                            Number::from(
                                                sina.clone() * sinb.clone() * sinc.clone()
                                                    + cosa.clone() * cosc.clone(),
                                                None,
                                            ),
                                            Number::from(
                                                sina.clone() * sinb.clone() * cosc.clone()
                                                    - cosa.clone() * sinc.clone(),
                                                None,
                                            ),
                                        ],
                                        vec![
                                            Number::from(-sinb.clone(), None),
                                            Number::from(cosb.clone() * sinc.clone(), None),
                                            Number::from(cosb.clone() * cosc.clone(), None),
                                        ],
                                    ])
                                }
                                else
                                {
                                    let (sin, cos) = (arg.num()?.number / to_deg.clone())
                                        .sin_cos(Complex::new(options.prec));
                                    Matrix(vec![
                                        vec![
                                            Number::from(cos.clone(), None),
                                            Number::from(-sin.clone(), None),
                                        ],
                                        vec![Number::from(sin, None), Number::from(cos, None)],
                                    ])
                                }
                            }
                            "factors" | "factor" =>
                            {
                                let a = arg.num()?.number;
                                if a.imag().clone().is_zero()
                                {
                                    if a.real().clone().fract().is_zero()
                                    {
                                        let mut vec = Vec::new();
                                        let n = a
                                            .real()
                                            .to_integer()
                                            .unwrap_or_default()
                                            .to_usize()
                                            .unwrap_or_default();
                                        for i in 1..=n
                                        {
                                            if n % i == 0
                                            {
                                                vec.push(Number::from(
                                                    Complex::with_val(options.prec, i),
                                                    None,
                                                ));
                                            }
                                        }
                                        Vector(vec)
                                    }
                                    else
                                    {
                                        Num(Number::from(
                                            Complex::with_val(options.prec, Nan),
                                            None,
                                        ))
                                    }
                                }
                                else
                                {
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                                }
                            }
                            "unity" =>
                            {
                                let vec = if i + 1 < function.len()
                                    && !matches!(&function[i + 1], Str(_))
                                {
                                    unity(
                                        arg.num()?.number.ln(),
                                        function.remove(i + 1).num()?.number,
                                    )
                                }
                                else
                                {
                                    unity(Complex::new(options.prec), arg.num()?.number)
                                };
                                if vec.is_empty()
                                {
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                                }
                                else
                                {
                                    Vector(vec)
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
    i = 0;
    while i < function.len()
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "rnd" => Num(Number::from(
                    Complex::with_val(options.prec, fastrand::u64(..)) / u64::MAX,
                    None,
                )),
                "epoch" => Num(Number::from(
                    Complex::with_val(
                        options.prec,
                        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                        {
                            Ok(n) => n.as_nanos(),
                            _ => return Err("epoch fail"),
                        },
                    ) / 1000000000,
                    Some(Units {
                        second: 1.0,
                        ..Units::default()
                    }),
                )),
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
        }
    }
    i = 1;
    while i < function.len().saturating_sub(1)
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
    while i < function.len().saturating_sub(1)
    {
        if let Str(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "×" => function[i - 1].mul(&function[i + 1])?,
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
    while i < function.len().saturating_sub(1)
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
    while i < function.len().saturating_sub(1)
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
    if options.units
    {
        i = 1;
        while i < function.len().saturating_sub(1)
        {
            if let Str(s) = &function[i]
            {
                function[i] = match s.as_str()
                {
                    "->" => function[i - 1].func(&function[i + 1], div)?,
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
    }
    i = 1;
    while i < function.len().saturating_sub(1)
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
                            if matches!(s.as_str(), "<" | ">" | "==" | "<=" | "!=" | ">=")
                            {
                                function[i - 1] = function[i + 1].func(&function[i - 1], gt)?;
                                function[i] = Str("&&".to_string());
                                continue;
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
                            if matches!(s.as_str(), "<" | ">" | "==" | "<=" | "!=" | ">=")
                            {
                                function[i - 1] = function[i + 1].func(&function[i - 1], ge)?;
                                function[i] = Str("&&".to_string());
                                continue;
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
                            if matches!(s.as_str(), "<" | ">" | "==" | "<=" | "!=" | ">=")
                            {
                                function[i - 1] = function[i - 1].func(&function[i + 1], gt)?;
                                function[i] = Str("&&".to_string());
                                continue;
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
                            if matches!(s.as_str(), "<" | ">" | "==" | "<=" | "!=" | ">=")
                            {
                                function[i - 1] = function[i - 1].func(&function[i + 1], ge)?;
                                function[i] = Str("&&".to_string());
                                continue;
                            }
                        }
                    }
                    function[i] = function[i - 1].func(&function[i + 1], ge)?
                }
                "==" =>
                {
                    if i + 3 < function.len()
                    {
                        if let Str(s) = &function[i + 2]
                        {
                            if matches!(s.as_str(), "<" | ">" | "==" | "<=" | "!=" | ">=")
                            {
                                function[i - 1] = function[i - 1].func(&function[i + 1], eq)?;
                                function[i] = Str("&&".to_string());
                                continue;
                            }
                        }
                    }
                    function[i] = function[i - 1].func(&function[i + 1], eq)?
                }
                "!=" =>
                {
                    if i + 3 < function.len()
                    {
                        if let Str(s) = &function[i + 2]
                        {
                            if matches!(s.as_str(), "<" | ">" | "==" | "<=" | "!=" | ">=")
                            {
                                function[i - 1] = function[i - 1].func(&function[i + 1], ne)?;
                                function[i] = Str("&&".to_string());
                                continue;
                            }
                        }
                    }
                    function[i] = function[i - 1].func(&function[i + 1], ne)?
                }
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
    while i < function.len().saturating_sub(1)
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
            (Matrix(a), Matrix(b))
                if a.len() == b.len() && (0..a.len()).all(|i| a[i].len() == b[i].len()) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in 0..a.len()
                {
                    let mut vec = Vec::new();
                    for j in 0..a[i].len()
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
                    for j in 0..a[i].len()
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
                    for j in 0..b[i].len()
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
    mut a: Number,
    mut c: Option<Number>,
    to_deg: Complex,
    s: &str,
    options: Options,
) -> Result<Number, &'static str>
{
    if a.number.imag().is_zero() && !a.number.imag().is_sign_positive()
    {
        a.number = Complex::with_val(a.number.prec(), a.number.real())
    }
    let n = if matches!(
        s,
        "root"
            | "sqrt"
            | "asquare"
            | "exp"
            | "aln"
            | "square"
            | "asqrt"
            | "cube"
            | "acbrt"
            | "asin"
            | "arcsin"
            | "acsc"
            | "arccsc"
            | "acos"
            | "arccos"
            | "asec"
            | "arcsec"
            | "atan2"
            | "atan"
            | "arctan"
            | "acot"
            | "arccot"
            | "ceil"
            | "floor"
            | "round"
            | "frac"
            | "fract"
            | "cbrt"
            | "acube"
            | "units"
            | "int"
            | "trunc"
            | "recip"
            | "abs"
            | "norm"
    )
    {
        if let Some(ref b) = c
        {
            if b.number.imag().is_zero() && !b.number.imag().is_sign_positive()
            {
                c = Some(Number::from(
                    Complex::with_val(b.number.prec(), b.number.real()),
                    b.units,
                ))
            }
        }
        match s
        {
            "sqrt" | "asquare" => Number::from(a.number.sqrt(), a.units.map(|a| a.root(2.0))),
            "root" =>
            {
                if let Some(b) = c
                {
                    let b = b.number;
                    let c: Float = b.real().clone() / 2;
                    if b.is_zero() && !a.number.is_zero()
                    {
                        Number::from(Complex::with_val(a.number.prec(), Nan), None)
                    }
                    else if b.imag().is_zero()
                        && !c.fract().is_zero()
                        && b.real().clone().fract().is_zero()
                        && a.number.imag().is_zero()
                    {
                        Number::from(
                            {
                                let a = a.number.real();
                                let ab = a.clone().abs();
                                Complex::with_val(
                                    options.prec,
                                    a / ab.clone() * ab.pow(b.real().clone().recip()),
                                )
                            },
                            a.units.map(|a| a.root(b.real().to_f64())),
                        )
                    }
                    else
                    {
                        Number::from(
                            a.number.pow(b.clone().recip()),
                            a.units.map(|a| a.root(b.real().to_f64())),
                        )
                    }
                }
                else
                {
                    Number::from(a.number.sqrt(), a.units.map(|a| a.root(2.0)))
                }
            }
            "exp" | "aln" =>
            {
                if let Some(b) = c
                {
                    Number::from(
                        a.number.pow(b.number.clone()),
                        a.units.map(|a| a.pow(b.number.real().to_f64())),
                    )
                }
                else
                {
                    Number::from(a.number.exp(), None)
                }
            }
            "square" | "asqrt" => Number::from(a.number.pow(2), a.units.map(|a| a.pow(2.0))),
            "cube" | "acbrt" => Number::from(a.number.pow(3), a.units.map(|a| a.pow(3.0))),
            "asin" | "arcsin" => Number::from(
                a.number.clone().asin() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "acsc" | "arccsc" => Number::from(
                a.number.clone().recip().asin() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "acos" | "arccos" => Number::from(
                a.number.clone().acos() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "asec" | "arcsec" => Number::from(
                a.number.clone().recip().acos() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "atan2" =>
            {
                if let Some(b) = c
                {
                    Number::from(
                        atan(b.number, a.number) * to_deg,
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    )
                }
                else
                {
                    return Err("not enough args");
                }
            }
            "atan" | "arctan" =>
            {
                if let Some(b) = c
                {
                    Number::from(
                        atan(a.number, b.number) * to_deg,
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    )
                }
                else
                {
                    Number::from(
                        a.number.atan() * to_deg,
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    )
                }
            }
            "acot" | "arccot" => Number::from(
                a.number.recip().atan() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "ceil" => Number::from(
                Complex::with_val(
                    options.prec,
                    (
                        a.number.real().clone().ceil(),
                        a.number.imag().clone().ceil(),
                    ),
                ),
                a.units,
            ),
            "floor" => Number::from(
                Complex::with_val(
                    options.prec,
                    (
                        a.number.real().clone().floor(),
                        a.number.imag().clone().floor(),
                    ),
                ),
                a.units,
            ),
            "round" => Number::from(
                Complex::with_val(
                    options.prec,
                    (
                        a.number.real().clone().round(),
                        a.number.imag().clone().round(),
                    ),
                ),
                a.units,
            ),
            "cbrt" | "acube" => Number::from(
                if a.number.imag().is_zero()
                {
                    if a.number.real().is_zero()
                    {
                        Complex::new(options.prec)
                    }
                    else if a.number.real().is_sign_positive()
                    {
                        a.number.pow(Float::with_val(options.prec, 3).recip())
                    }
                    else
                    {
                        -(-a.number).pow(Float::with_val(options.prec, 3).recip())
                    }
                }
                else
                {
                    a.number
                        .clone()
                        .pow(Float::with_val(options.prec, 3).recip())
                },
                a.units.map(|a| a.root(3.0)),
            ),
            "abs" | "norm" => Number::from(a.number.abs(), a.units),
            "frac" | "fract" => Number::from(
                Complex::with_val(
                    options.prec,
                    (
                        a.number.real().clone().fract(),
                        a.number.imag().clone().fract(),
                    ),
                ),
                a.units,
            ),
            "int" | "trunc" => Number::from(
                Complex::with_val(
                    options.prec,
                    (
                        a.number.real().clone().trunc(),
                        a.number.imag().clone().trunc(),
                    ),
                ),
                a.units,
            ),
            "recip" => Number::from(a.number.recip(), a.units.map(|a| a.pow(-1.0))),
            "units" => Number::from(Complex::with_val(options.prec, 1), a.units),
            _ => return Err("unreachable"),
        }
    }
    else
    {
        let a = a.number;
        let mut d = None;
        if let Some(ref b) = c
        {
            if b.number.imag().is_zero() && !b.number.imag().is_sign_positive()
            {
                d = Some(Complex::with_val(b.number.prec(), b.number.real()))
            }
            else
            {
                d = Some(b.number.clone())
            }
        }
        Number::from(
            match s
            {
                "sin" => (a / to_deg).sin(),
                "csc" => (a / to_deg).sin().recip(),
                "cos" => (a / to_deg).cos(),
                "sec" => (a / to_deg).cos().recip(),
                "tan" => (a / to_deg).tan(),
                "cot" => (a / to_deg).tan().recip(),
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
                "W" | "productlog" | "lambertw" =>
                {
                    if let Some(b) = d
                    {
                        lambertw(b, a.real().to_integer().unwrap_or_default())
                    }
                    else
                    {
                        lambertw(a, Integer::new())
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
                            if b.is_zero()
                            {
                                Complex::with_val(options.prec, Nan)
                            }
                            else
                            {
                                Complex::with_val(options.prec, Infinity)
                            }
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
                        b.clone() / lambertw(b, a.real().to_integer().unwrap_or_default())
                    }
                    else
                    {
                        let a = a.ln();
                        a.clone() / lambertw(a, Integer::new())
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
                "Ap" =>
                {
                    if let Some(b) = d
                    {
                        if a.real().is_sign_negative()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            let mut sum = Complex::new(options.prec);
                            let n = a
                                .real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_u32()
                                .unwrap_or_default();
                            for k in 0..=n
                            {
                                sum += b.clone().pow(k) * euleriannumbersint(n, k)
                            }
                            sum
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "An" =>
                {
                    if let Some(b) = d
                    {
                        euleriannumbers(
                            a,
                            b.real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_i32()
                                .unwrap_or_default(),
                        )
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
                        binomial(a, b)
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
                "onlyreal" | "onlyre" | "ore" =>
                {
                    if -a.imag().clone().abs().log10() > a.prec().0 / 4
                    {
                        a.real().clone().into()
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "onlyimag" | "onlyim" | "oim" =>
                {
                    if -a.real().clone().abs().log10() > a.prec().0 / 4
                    {
                        a.imag().clone().into()
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "re" | "real" => a.real().clone().into(),
                "im" | "imag" => a.imag().clone().into(),
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
                        digamma(
                            b,
                            a.real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_u32()
                                .unwrap_or_default(),
                        )
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
                "eta" | "η" => eta(a),
                "zeta" | "ζ" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().zeta().into()
                    }
                    else
                    {
                        zeta(a)
                    }
                }
                "prime" =>
                {
                    if a.imag().is_zero() && a.real().clone().fract() == 0.0
                    {
                        Complex::with_val(
                            options.prec,
                            nth_prime(a.real().to_integer().unwrap_or_default()),
                        )
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "mod" =>
                {
                    if let Some(b) = d
                    {
                        let c = a.clone() / b.clone();
                        let c = Complex::with_val(
                            options.prec,
                            (c.real().clone().floor(), c.imag().clone().floor()),
                        );
                        a - b * c
                    }
                    else
                    {
                        return Err("not enough args");
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
    if n.number.imag().is_zero() && !n.number.imag().is_sign_positive()
    {
        Ok(Number::from(n.number.real().clone().into(), n.units))
    }
    else
    {
        Ok(n)
    }
}
