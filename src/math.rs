use crate::{
    complex::{
        add, and, cofactor, cubic, determinant, div, eigenvalues, eq, gamma, ge, gt, identity,
        inverse, le, lt, minors, mvec, ne, nth_prime, or, quadratic, recursion, rem, root, shl,
        shr, slog, sort, sub, sum, tetration, to, to_polar, trace, transpose, variance, NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    AngleType::{Degrees, Gradians, Radians},
    Options,
};
use libc::rand;
use rug::{
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    integer::IsPrime,
    ops::Pow,
    Complex, Float,
};
pub fn do_math(
    mut function: Vec<NumStr>,
    options: Options,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
) -> Result<NumStr, &'static str>
{
    if function.len() == 1 && !function[0].str_is("rnd")
    {
        return Ok(function[0].clone());
    }
    if function.is_empty()
    {
        return Err(" ");
    }
    for (i, v) in func_vars.clone().iter().enumerate()
    {
        if v.1.len() != 1 && !v.0.contains('(')
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
            for v in &func_vars
            {
                if *s == v.0 && !v.0.contains('(')
                {
                    if v.1.len() == 1
                    {
                        if let Str(_) = v.1[0]
                        {
                        }
                        else
                        {
                            function[i] = v.1[0].clone();
                        }
                    }
                    break;
                }
            }
        }
        i += 1;
    }
    // use std::io::Write;
    // for i in &function
    // {
    //     match i
    //     {
    //         Num(n) => print!(
    //             "{}",
    //             crate::print::get_output(options, &crate::Colors::default(), n).0
    //         ),
    //         Str(s) => print!("{}", s),
    //         _ =>
    //         {}
    //     }
    // }
    // print!("\n\x1b[G");
    // std::io::stdout().flush().unwrap();
    i = 0;
    'outer: while i < function.len()
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
                    let mut mat = Vec::<Vec<Complex>>::new();
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
                                    | "normP"
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
                                    | "quad"
                                    | "quadratic"
                                    | "cubic"
                                    | "percentilerank"
                                    | "percentile"
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
                                    function.insert(
                                        i,
                                        do_math(
                                            v[..place[0]].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?,
                                    );
                                    for (k, l) in place.iter().enumerate()
                                    {
                                        function.insert(i + k + 1, Str(','.to_string()));
                                        function.insert(
                                            i + k + 2,
                                            do_math(
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
                                            )?,
                                        );
                                        i += 1;
                                    }
                                    continue 'outer;
                                }
                            }
                            else if matches!(
                                k.as_str(),
                                "sum"
                                    | "summation"
                                    | "prod"
                                    | "product"
                                    | "Σ"
                                    | "Π"
                                    | "vec"
                                    | "mat"
                                    | "piecewise"
                            )
                            {
                                i = j - 1;
                                continue;
                            }
                        }
                    }
                    function[i] = do_math(v, options, func_vars.clone())?;
                    function.drain(i + 1..j);
                }
                _ =>
                {}
            }
        }
        i += 1;
    }
    i = 0;
    let to_deg = match options.deg
    {
        Degrees => Complex::with_val(options.prec, 180) / Complex::with_val(options.prec, Pi),
        Radians => Complex::with_val(options.prec, 1),
        Gradians => Complex::with_val(options.prec, 200) / Complex::with_val(options.prec, Pi),
    };
    while i < function.len() - 1
    {
        if let Str(s) = &function[i].clone()
        {
            if s != "rnd"
                && ((s.len() > 1 && s.chars().next().unwrap().is_alphabetic())
                    || matches!(s.as_str(), "C" | "P" | "I"))
            {
                if matches!(
                    s.as_str(),
                    "sum"
                        | "product"
                        | "prod"
                        | "summation"
                        | "Σ"
                        | "Π"
                        | "vec"
                        | "mat"
                        | "piecewise"
                )
                {
                    let mut place = Vec::new();
                    let mut count = 0;
                    for (f, n) in function[i + 2..].iter().enumerate()
                    {
                        if let Str(w) = n
                        {
                            if w == "," && (count == 0 || (s == "piecewise" && count == 1))
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
                    if s == "piecewise" && !place.is_empty()
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
                                .real()
                                    == &1.0
                            {
                                //TODO move to parse.rs
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
                            Num(Complex::with_val(options.prec, Nan))
                        };
                        function.drain(i + 1..=*place.last().unwrap());
                    }
                    else if place.len() == 4
                    {
                        if let Str(var) = &function[place[0] - 1]
                        {
                            let start = do_math(
                                function[place[1] + 1..place[2]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
                            .num()?;
                            let end = do_math(
                                function[place[2] + 1..place[3]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
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
                            let start = start.real().to_f64() as isize;
                            let end = end.real().to_f64() as isize;
                            function[i] = match s.as_str()
                            {
                                "vec" | "mat" => mvec(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    var,
                                    start,
                                    end,
                                    s == "vec",
                                    options,
                                )?,
                                _ => sum(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    var,
                                    start,
                                    end,
                                    !(s == "sum" || s == "summation" || s == "Σ"),
                                    options,
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
                                    do_math(
                                        function[i + 2..place[0]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )
                                }
                                {
                                    Ok(Num(a)) => Num(a.clone()),
                                    Ok(Vector(a)) => Num(a
                                        .iter()
                                        .fold(Complex::new(options.prec), |sum, val| sum + val)),
                                    Ok(Matrix(a)) => Num(a
                                        .iter()
                                        .flatten()
                                        .fold(Complex::new(options.prec), |sum, val| sum + val)),
                                    _ => return Err("sum err"),
                                }
                            }
                            "product" | "prod" | "Π" =>
                            {
                                function[i] =
                                    match if place.is_empty()
                                    {
                                        Ok(function[i + 1].clone())
                                    }
                                    else
                                    {
                                        do_math(
                                            function[i + 2..place[0]].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )
                                    }
                                    {
                                        Ok(Num(a)) => Num(a.clone()),
                                        Ok(Vector(a)) => Num(a.iter().fold(
                                            Complex::with_val(options.prec, 1),
                                            |sum, val| sum * val,
                                        )),
                                        Ok(Matrix(a)) => Num(a.iter().flatten().fold(
                                            Complex::with_val(options.prec, 1),
                                            |sum, val| sum * val,
                                        )),
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
                            function.drain(i + 1..=place[0]);
                        }
                    }
                    else
                    {
                        return Err("not enough args for sum/prod");
                    }
                }
                else
                {
                    match function[i + 1].clone()
                    {
                        Matrix(a) =>
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
                                "flatten" =>
                                {
                                    Vector(a.into_iter().flatten().collect::<Vec<Complex>>())
                                }
                                "cofactor" | "cofactors" | "cof" => Matrix(cofactor(&a)?),
                                "minor" | "minors" => Matrix(minors(&a)?),
                                "adjugate" | "adj" => Matrix(transpose(&cofactor(&a)?)?),
                                "inverse" | "inv" => Matrix(inverse(&a)?),
                                "transpose" | "trans" => Matrix(transpose(&a)?),
                                "len" | "length" => Num(Complex::with_val(options.prec, a.len())),
                                "wid" | "width" => Num(Complex::with_val(options.prec, a[0].len())),
                                "tr" | "trace" => Num(trace(&a)),
                                "det" | "determinant" => Num(determinant(&a)?),
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
                                                            let n2 =
                                                                n.clone().real().to_f64() as usize;
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
                                    let mut n = Complex::new(options.prec);
                                    for j in a.iter().flatten()
                                    {
                                        n += j.clone().abs().pow(2);
                                    }
                                    Num(n.sqrt())
                                }
                                "mean" => Num(a
                                    .iter()
                                    .flatten()
                                    .fold(Complex::new(options.prec), |sum, val| sum + val)
                                    / (a.len() * a[0].len())),
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
                                    let a =
                                        sort(a.iter().flatten().cloned().collect::<Vec<Complex>>());
                                    if a.len() % 2 == 0
                                    {
                                        Vector(vec![
                                            a[a.len() / 2 - 1].clone(),
                                            a[a.len() / 2].clone(),
                                        ])
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
                                        if !(a.imag().is_zero() && a.real() == &1)
                                        {
                                            res = false
                                        }
                                    }
                                    Num(Complex::with_val(options.prec, res as u8))
                                }
                                "any" =>
                                {
                                    let mut res = false;
                                    for a in a.iter().flatten()
                                    {
                                        if a.imag().is_zero() && a.real() == &1
                                        {
                                            res = true
                                        }
                                    }
                                    Num(Complex::with_val(options.prec, res as u8))
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
                                        for _ in 0..a[1].real().to_f64() as usize
                                        {
                                            vec.push(a[0].clone())
                                        }
                                    }
                                    Vector(sort(vec))
                                }
                                _ => do_functions(
                                    function[i + 1].clone(),
                                    options,
                                    &mut function,
                                    i,
                                    &to_deg,
                                    s,
                                )?,
                            };
                            function.remove(i + 1);
                        }
                        Vector(a) =>
                        {
                            function[i] = match s.as_str()
                            {
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
                                            (half1[half1.len() / 2 - 1].clone()
                                                + half1[half1.len() / 2].clone())
                                                / 2,
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len() - 1].clone() + half2[0].clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].clone()
                                            },
                                            (half2[half2.len() / 2 - 1].clone()
                                                + half2[half2.len() / 2].clone())
                                                / 2,
                                        ])
                                    }
                                    else
                                    {
                                        Vector(vec![
                                            half1[half1.len() / 2].clone(),
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len() - 1].clone() + half2[0].clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].clone()
                                            },
                                            half2[half2.len() / 2].clone(),
                                        ])
                                    }
                                }
                                "percentile" =>
                                {
                                    if function.len() < i + 3
                                    {
                                        return Err("not enough input");
                                    }
                                    let b = function[i + 3].num()?;
                                    function.drain(i + 2..=i + 3);
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
                                    if function.len() < i + 3
                                    {
                                        return Err("not enough input");
                                    }
                                    let mut cf = 0;
                                    let mut f = 0;
                                    let b = function[i + 3].num()?;
                                    function.drain(i + 2..=i + 3);
                                    for a in sort(a.clone())
                                    {
                                        if a.real() < b.real()
                                        {
                                            cf += 1;
                                        }
                                        else if a == b
                                        {
                                            f += 1;
                                        }
                                        else
                                        {
                                            break;
                                        }
                                    }
                                    Num(100
                                        * (Complex::with_val(options.prec, cf)
                                            + (0.5 * Complex::with_val(options.prec, f)))
                                        / a.len())
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
                                                Complex::with_val(options.prec, count),
                                            ]);
                                            last = a;
                                            count = 0;
                                        }
                                        count += 1;
                                    }
                                    mat.push(vec![
                                        last.clone(),
                                        Complex::with_val(options.prec, count),
                                    ]);
                                    Matrix(mat)
                                }
                                "standarddeviation" | "σ" =>
                                {
                                    Num(variance(&a, options.prec).sqrt())
                                }
                                "variance" | "var" => Num(variance(&a, options.prec)),
                                "all" =>
                                {
                                    let mut res = true;
                                    for a in a
                                    {
                                        if !(a.imag().is_zero() && a.real() == &1)
                                        {
                                            res = false
                                        }
                                    }
                                    Num(Complex::with_val(options.prec, res as u8))
                                }
                                "any" =>
                                {
                                    let mut res = false;
                                    for a in a
                                    {
                                        if a.imag().is_zero() && a.real() == &1
                                        {
                                            res = true
                                        }
                                    }
                                    Num(Complex::with_val(options.prec, res as u8))
                                }
                                "sort" => Vector(sort(a)),
                                "mean" => Num(a
                                    .iter()
                                    .fold(Complex::new(options.prec), |sum, val| sum + val)
                                    / a.len()),
                                "median" =>
                                {
                                    let a = sort(a);
                                    if a.len() % 2 == 0
                                    {
                                        Num((a[a.len() / 2 - 1].clone() + a[a.len() / 2].clone())
                                            / 2)
                                    }
                                    else
                                    {
                                        Num(a[a.len() / 2].clone())
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
                                "len" | "length" => Num(Complex::with_val(options.prec, a.len())),
                                "norm" =>
                                {
                                    let mut n = Complex::new(options.prec);
                                    for i in a
                                    {
                                        n += i.abs().pow(2);
                                    }
                                    Num(n.sqrt())
                                }
                                "normalize" =>
                                {
                                    let mut n = Complex::new(options.prec);
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
                                            let c: Complex =
                                                a[0].clone().pow(2) + a[1].clone().pow(2);
                                            let d: Complex =
                                                b[0].clone().pow(2) + b[1].clone().pow(2);
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
                                            let mut dot = Complex::new(options.prec);
                                            for i in
                                                a.iter().zip(b.vec()?.iter()).map(|(a, b)| a * b)
                                            {
                                                dot += i;
                                            }
                                            let mut norm = Complex::new(options.prec);
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
                                        let mut n = Complex::new(options.prec);
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
                                "split" => Matrix(
                                    a.iter()
                                        .map(|a| {
                                            vec![
                                                (*a.real()).clone().into(),
                                                (*a.imag()).clone().into(),
                                            ]
                                        })
                                        .collect::<Vec<Vec<Complex>>>(),
                                ),
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
                                                let n = num.real().to_f64() as usize;
                                                for i in 1..=n
                                                {
                                                    if n % i == 0
                                                    {
                                                        vec.push(Complex::with_val(
                                                            options.prec,
                                                            i,
                                                        ));
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
                                    options,
                                    &mut function,
                                    i,
                                    &to_deg,
                                    s,
                                )?,
                            };
                            function.remove(i + 1);
                        }
                        _ =>
                        {
                            function[i] = match s.as_str()
                            {
                                "normP" =>
                                {
                                    if i + 5 < function.len()
                                    {
                                        let mu = function[i + 1].num()?;
                                        let sigma = function[i + 3].num()?;
                                        let x = function[i + 5].num()?;
                                        function.drain(i + 2..i + 6);
                                        let n: Complex = (x - mu).pow(2);
                                        let n: Complex = -n / (2 * sigma.clone().pow(2));
                                        let tau: Complex = 2 * Complex::with_val(options.prec, Pi);
                                        Num(n.exp() / (sigma * tau.sqrt()))
                                    }
                                    else
                                    {
                                        return Err("not enough args");
                                    }
                                }
                                "cubic" =>
                                {
                                    if i + 7 < function.len()
                                    {
                                        let a = function[i + 1].num()?;
                                        let b = function[i + 3].num()?;
                                        let c = function[i + 5].num()?;
                                        let d = function[i + 7].num()?;
                                        function.drain(i + 2..i + 8);
                                        Vector(cubic(a, b, c, d))
                                    }
                                    else
                                    {
                                        return Err("not enough args");
                                    }
                                }
                                "quad" | "quadratic" =>
                                {
                                    if i + 5 < function.len()
                                    {
                                        let a = function[i + 1].num()?;
                                        let b = function[i + 3].num()?;
                                        let c = function[i + 5].num()?;
                                        function.drain(i + 2..i + 6);
                                        Vector(quadratic(a, b, c))
                                    }
                                    else
                                    {
                                        return Err("not enough args");
                                    }
                                }
                                "split" =>
                                {
                                    let a = function[i + 1].num()?;
                                    Vector(vec![
                                        (*a.real()).clone().into(),
                                        (*a.imag()).clone().into(),
                                    ])
                                }
                                "I" => Matrix(identity(
                                    function[i + 1].num()?.real().to_f64() as usize,
                                    options.prec,
                                )),
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
                                            let n = a.real().to_f64() as usize;
                                            for i in 1..=n
                                            {
                                                if n % i == 0
                                                {
                                                    vec.push(Complex::with_val(options.prec, i));
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
                                    options,
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
                "rnd" =>
                {
                    function[i] = Num(Complex::with_val(options.prec, unsafe { rand() })
                        / Complex::with_val(options.prec, libc::RAND_MAX))
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
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                ".." => function[i] = to(&function[i - 1], &function[i + 1])?,
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
    i = function.len().saturating_sub(2);
    while i != 0
    {
        if let Str(s) = &function[i]
        {
            match s.as_str()
            {
                "^" => function[i] = function[i - 1].pow(&function[i + 1])?,
                "^^" => function[i] = function[i - 1].func(&function[i + 1], tetration)?,
                "//" => function[i] = function[i - 1].func(&function[i + 1], root)?,
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
                "/" => function[i] = function[i - 1].func(&function[i + 1], div)?,
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
                "<=" => function[i] = function[i - 1].func(&function[i + 1], le)?,
                ">" => function[i] = function[i - 1].func(&function[i + 1], gt)?,
                ">=" => function[i] = function[i - 1].func(&function[i + 1], ge)?,
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
    if let Str(_) = &function[0]
    {
        function.remove(0);
    }
    if let Some(Str(_)) = &function.last()
    {
        function.pop();
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
    if function.len() > k + 3 && function[k + 2].str_is(",")
    {
        let b = function[k + 3].clone();
        function.drain(k + 2..k + 4);
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(functions(a, Some(b), to_deg.clone(), s, options)?)),
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
                            options,
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
                            options,
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
                    vec.push(functions(a.clone(), Some(i), to_deg.clone(), s, options)?)
                }
                Ok(Vector(vec))
            }
            (Vector(a), Num(b)) =>
            {
                let mut vec = Vec::new();
                for i in a
                {
                    vec.push(functions(i, Some(b.clone()), to_deg.clone(), s, options)?)
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
                        vec.push(functions(a.clone(), Some(j), to_deg.clone(), s, options)?)
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
                        vec.push(functions(j, Some(b.clone()), to_deg.clone(), s, options)?)
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
                            options,
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
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                Ok(Matrix(mat))
            }
            _ => Err("str err2"),
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
}
fn functions(
    a: Complex,
    c: Option<Complex>,
    to_deg: Complex,
    s: &str,
    options: Options,
) -> Result<Complex, &'static str>
{
    let b;
    let n = match s
    {
        "sin" => (a / to_deg).sin(),
        "csc" => (a / to_deg).sin().recip(),
        "cos" => (a / to_deg).cos(),
        "sec" => (a / to_deg).cos().recip(),
        "tan" => (a / to_deg).tan(),
        "cot" => (a / to_deg).tan().recip(),
        "asin" | "arcsin" =>
        {
            b = a.clone().asin() * to_deg;
            if a.imag().is_zero() && a.real() >= &1
            {
                Complex::with_val(options.prec, (b.real(), -b.imag()))
            }
            else
            {
                b
            }
        }
        "acsc" | "arccsc" =>
        {
            b = a.clone().recip().asin() * to_deg;
            if a.imag().is_zero()
            {
                Complex::with_val(options.prec, (b.real(), -b.imag()))
            }
            else
            {
                b
            }
        }
        "acos" | "arccos" =>
        {
            b = a.clone().acos() * to_deg;
            if a.imag().is_zero() && a.real() >= &1
            {
                Complex::with_val(options.prec, (b.real(), -b.imag()))
            }
            else
            {
                b
            }
        }
        "asec" | "arcsec" =>
        {
            b = a.clone().recip().acos() * to_deg;
            if a.imag().is_zero()
            {
                Complex::with_val(options.prec, (b.real(), -b.imag()))
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
                (a + (b * Complex::with_val(options.prec, (0, 1)))).arg() * to_deg
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
        "asech" | "arcsech" =>
        {
            b = a.clone().recip().acosh();
            if a.imag().is_zero() && a.real().is_sign_negative()
            {
                Complex::with_val(options.prec, (b.real(), -b.imag()))
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
                Complex::with_val(options.prec, (b.real(), -b.imag()))
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
                Complex::with_val(options.prec, (b.real(), -b.imag()))
            }
            else
            {
                b
            }
        }
        "cis" =>
        {
            (a.clone() / to_deg.clone()).cos()
                + (a / to_deg).sin() * Complex::with_val(options.prec, (0.0, 1.0))
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
        "exp" | "aln" => a.exp(),
        "log" =>
        {
            let a = a.ln();
            if let Some(b) = c
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
                let c: Float = b.real().clone() / 2;
                if b.is_zero() && !a.is_zero()
                {
                    Complex::with_val(a.prec(), Nan)
                }
                else if b.imag().is_zero()
                    && !c.fract().is_zero()
                    && b.real().clone().fract().is_zero()
                    && a.imag().is_zero()
                {
                    Complex::with_val(
                        options.prec,
                        a.real() / a.real().clone().abs()
                            * a.real().clone().abs().pow(b.real().clone().recip()),
                    )
                }
                else
                {
                    a.pow(b.recip())
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
                (gamma(&a) / gamma(&d)).into()
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
                        options.prec,
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
                    Complex::with_val(options.prec, gamma(&c) / (gamma(&d) * gamma(&e)))
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
                Complex::with_val(options.prec, gamma(a.real()))
            }
            else
            {
                return Err("complex gamma not supported");
            }
        }
        "sqrt" | "asquare" => a.sqrt(),
        "abs" | "norm" => a.abs(),
        "deg" | "degree" => match options.deg
        {
            Radians =>
            {
                a * Complex::with_val(options.prec, 180) / Complex::with_val(options.prec, Pi)
            }
            Gradians => a * 180.0 / 200.0,
            Degrees => a,
        },
        "rad" | "radian" => match options.deg
        {
            Radians => a,
            Gradians =>
            {
                a * Complex::with_val(options.prec, Pi) / Complex::with_val(options.prec, 200)
            }
            Degrees =>
            {
                a * Complex::with_val(options.prec, Pi) / Complex::with_val(options.prec, 180)
            }
        },
        "grad" | "gradian" => match options.deg
        {
            Radians =>
            {
                a * Complex::with_val(options.prec, 200) / Complex::with_val(options.prec, Pi)
            }
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
                Complex::with_val(options.prec, a.clone() / a.abs())
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
                    Complex::with_val(
                        options.prec,
                        a.real() / a.real().clone().abs()
                            * a.real()
                                .clone()
                                .abs()
                                .pow(Float::with_val(a.prec().0, 3).recip()),
                    )
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
        "doublefact" =>
        {
            if !a.imag().is_zero()
            {
                return Err("complex factorial not supported");
            }
            let a = a.real().clone();
            let two = Complex::with_val(options.prec, 2);
            let pi = Complex::with_val(options.prec, Pi);
            let gam: Float = a.clone() / 2 + 1;
            Complex::with_val(
                options.prec,
                two.pow(a.clone() / 2 + (1 - (pi.clone() * a.clone()).cos()) / 4)
                    * pi.clone().pow(((pi * a.clone()).cos() - 1) / 4)
                    * gamma(&gam),
            )
        }
        "fact" =>
        {
            if a.imag().is_zero()
            {
                let b: Float = a.real().clone() + 1;
                Complex::with_val(options.prec, gamma(&b))
            }
            else
            {
                return Err("complex factorial not supported");
            }
        }
        "subfact" =>
        {
            if !a.imag().is_zero()
                || a.real().is_sign_negative()
                || !a.real().clone().fract().is_zero()
            {
                return Err("complex/fractional subfactorial not supported");
            }
            let b: Float = a.real().clone() + 1;
            Complex::with_val(
                options.prec,
                (gamma(&b) / Float::with_val(options.prec.0, 1).exp()).round(),
            )
        }
        "sinc" => a.clone().sin() / a,
        "conj" | "conjugate" => a.conj(),
        "normD" =>
        {
            if a.imag().is_zero()
            {
                let two = Float::with_val(options.prec.0, 2);
                ((-a / two.clone().sqrt()).real().clone().erfc() / two).into()
            }
            else
            {
                return Err("complex erf not supported");
            }
        }
        "erf" =>
        {
            if a.imag().is_zero()
            {
                Complex::with_val(options.prec, a.real().clone().erf())
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
                Complex::with_val(options.prec, a.real().clone().erfc())
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
                Complex::with_val(options.prec, a.real().clone().ai())
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
                if a.real().is_sign_negative() && a.real().clone().fract().is_zero()
                {
                    Complex::with_val(options.prec, Infinity)
                }
                else
                {
                    Complex::with_val(options.prec, a.real().clone().digamma())
                }
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
                Complex::with_val(options.prec, a.real().clone().zeta())
            }
            else
            {
                return Err("complex zeta not supported");
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
                return Err("cant get a complex prime");
            }
        }
        "isprime" | "is_prime" =>
        {
            if a.imag().is_zero() && a.real().clone().fract() == 0.0
            {
                Complex::with_val(
                    options.prec,
                    (a.real().to_integer().unwrap().is_probably_prime(100) != IsPrime::No) as u8,
                )
            }
            else
            {
                return Err("cant get a complex prime");
            }
        }
        _ =>
        {
            return Err("wrong input type");
        }
    };
    if n.imag().is_zero()
    {
        Ok(Complex::with_val(n.prec(), n.real()))
    }
    else
    {
        Ok(n)
    }
}