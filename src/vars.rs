use crate::Options;
use rug::{
    float::Constant::{Catalan, Euler, Pi},
    Float,
};
use std::collections::HashSet;
pub fn input_var(
    input: &str,
    vars: &[[String; 2]],
    sumrec: &mut Vec<(i32, String)>,
    options: Options,
) -> String
{
    let chars = input
        .replace('[', "(car{")
        .replace(']', "})")
        .chars()
        .collect::<Vec<char>>();
    let mut output = String::new();
    let mut stack_end = Vec::new();
    let mut stack_start = Vec::new();
    for c in &chars
    {
        match c
        {
            '(' => stack_end.push(')'),
            '{' => stack_end.push('}'),
            ')' | '}' =>
            {
                if let Some(top) = stack_end.last()
                {
                    if top == c
                    {
                        stack_end.pop();
                    }
                }
                else
                {
                    match c
                    {
                        ')' => stack_start.push('('),
                        '}' => stack_start.push('{'),
                        _ =>
                        {}
                    }
                }
            }
            _ =>
            {}
        }
    }
    let mut input = String::new();
    while let Some(top) = stack_start.pop()
    {
        input.push(top);
    }
    for i in &chars
    {
        input.push(*i)
    }
    while let Some(top) = stack_end.pop()
    {
        input.push(top);
    }
    let chars = input.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut sum = (0, String::new());
    let mut bracket = 0;
    'main: while i < chars.len()
    {
        let c = chars[i];
        let mut not_pushed = true;
        if !c.is_alphabetic()
        {
            if c == '('
            {
                bracket += 1;
            }
            else if c == ')'
            {
                if sum.0 == bracket
                {
                    sum.0 = 0;
                    sum.1 = String::new();
                    sumrec.pop();
                }
                bracket -= 1;
            }
            output.push(c);
            i += 1;
            continue;
        }
        let count = chars[i..]
            .iter()
            .position(|x| !x.is_alphabetic())
            .unwrap_or(chars.len() - i);
        let mut word = chars[i..i + count].iter().collect::<String>();
        if (word.ends_with('x') && word != "max") || word.ends_with('y')
        {
            word.pop();
        }
        if matches!(
            word.as_str(),
            "sum" | "summation" | "prod" | "production" | "vec" | "mat" | "Σ" | "Π"
        )
        {
            let mut place = 0;
            let mut count2 = 0;
            for c in &chars[i + count + 1..]
            {
                if c == &',' && count2 == 0
                {
                    place += 1;
                }
                else if c == &'(' || c == &'{'
                {
                    count2 += 1;
                }
                else if c == &')' || c == &'}'
                {
                    if count2 == 0
                    {
                        break;
                    }
                    count2 -= 1;
                }
            }
            if place == 3
            {
                let count2 = chars[i + count + 1..]
                    .iter()
                    .position(|x| x == &',')
                    .unwrap_or(0);
                sum.0 = bracket + 1;
                if count2 != 0
                {
                    sum.1 = chars[i + count + 1..i + count + count2 + 1]
                        .iter()
                        .collect::<String>();
                }
                sumrec.push(sum.clone())
            }
        }
        if is_func(&word)
        {
            i += word.len();
            output.push_str(&word)
        }
        else if sumrec.iter().any(|a| a.1 == word)
        {
            if matches!(chars[i - 1], '0'..='9' | ')' | '}' | ']' | 'x' | 'y')
            {
                output.push('*')
            }
            i += word.len();
            output.push_str(&word);
            if matches!(chars[i], '0'..='9' | ')' | '}' | ']' | 'x' | 'y')
            {
                output.push('*')
            }
        }
        else
        {
            for var in vars
            {
                let vl = var[0].chars().collect::<Vec<char>>().len();
                if var[0] != "e"
                    || (!options.small_e
                        || !(i != 0
                            && i + 1 != chars.len()
                            && chars[i - 1].is_numeric()
                            && (chars[i + 1].is_numeric() || chars[i + 1] == '-')))
                {
                    let j = i;
                    if var[0].contains('(')
                        && input.contains('(')
                        && i + vl - 1 <= chars.len()
                        && chars[i..i + vl - 1]
                            .iter()
                            .collect::<String>()
                            .split('(')
                            .next()
                            == var[0].split('(').next()
                    {
                        let mut count = 0;
                        for (f, c) in chars[i..].iter().enumerate()
                        {
                            if *c == '('
                            {
                                count += 1;
                            }
                            else if *c == ')'
                            {
                                count -= 1;
                                if count == 0
                                {
                                    i += f;
                                    break;
                                }
                            }
                        }
                        if i == j
                        {
                            i = input.len() - 1
                        }
                        if chars[j..i + 1].iter().collect::<String>() == var[0]
                        {
                            not_pushed = false;
                            output.push('(');
                            output.push_str(&input_var(&var[1], vars, sumrec, options));
                            output.push(')');
                        }
                        else
                        {
                            let mut k = 0;
                            for (f, c) in chars[j + 2..].iter().enumerate()
                            {
                                if *c == ')'
                                {
                                    k = f + j + 3;
                                    break;
                                }
                                else if f + j + 3 == chars.len()
                                {
                                    k = f + j + 4;
                                    break;
                                }
                            }
                            if k == 0
                            {
                                continue;
                            }
                            let v = var[0].chars().collect::<Vec<char>>();
                            if input.contains(',') && var[0].contains(',') && chars.len() > 4
                            {
                                not_pushed = false;
                                output.push('(');
                                let mut temp = &chars
                                    [j + var[0].chars().position(|c| c == '(').unwrap() + 1..i + 1];
                                if temp.ends_with(&[')'])
                                {
                                    temp = &temp[..temp.len() - 1];
                                }
                                let mut commas = Vec::new();
                                count = 0;
                                for (f, c) in temp.iter().enumerate()
                                {
                                    if c == &'(' || c == &'{' || c == &'['
                                    {
                                        count += 1;
                                    }
                                    else if c == &')' || c == &'}' || c == &']'
                                    {
                                        count -= 1;
                                    }
                                    else if c == &',' && count == 0
                                    {
                                        commas.push(f);
                                    }
                                }
                                if commas.len() == var[0].matches(',').count()
                                {
                                    let mut start = 0;
                                    let mut split = Vec::new();
                                    for end in commas
                                    {
                                        split.push(&temp[start..end]);
                                        start = end + 1;
                                    }
                                    split.push(&temp[start..]);
                                    let mut value = var[1].clone();
                                    for i in 0..split.len()
                                    {
                                        value = value.replace(
                                            v[v.len()
                                                - 2 * (i as i32 - split.len() as i32).unsigned_abs()
                                                    as usize],
                                            &format!("({})", &split[i].iter().collect::<String>(),),
                                        );
                                    }
                                    output.push_str(&input_var(&value, vars, sumrec, options));
                                    output.push(')');
                                }
                            }
                            else
                            {
                                not_pushed = false;
                                output.push('(');
                                let mut temp =
                                    &chars[j + var[0].split('(').next().unwrap().len() + 1..i + 1];
                                if temp.ends_with(&[')'])
                                {
                                    temp = &temp[..temp.len() - 1];
                                }
                                output.push_str(
                                    &input_var(&var[1], vars, sumrec, options).replace(
                                        v[v.len() - 2],
                                        &format!(
                                            "({})",
                                            &input_var(
                                                &temp.iter().collect::<String>(),
                                                vars,
                                                sumrec,
                                                options
                                            ),
                                        ),
                                    ),
                                );
                                output.push(')');
                            }
                        }
                    }
                    else if i + vl <= chars.len()
                        && chars[i..i + vl].iter().collect::<String>() == var[0]
                    {
                        i += vl;
                        output.push('(');
                        output.push_str(&input_var(&var[1], vars, sumrec, options));
                        output.push(')');
                        continue 'main;
                    }
                }
            }
            if (c != ' ' || (i == 0 || chars[i - 1] != ' '))
                && not_pushed
                && !(c.is_alphabetic() && c != 'x' && c != 'y' && c != 'i')
            {
                output.push(c);
            }
            i += 1;
        }
    }
    if output.is_empty()
    {
        input.to_string()
    }
    else
    {
        output
    }
}
fn is_func(word: &str) -> bool
{
    let functions: HashSet<_> = [
        "i",
        "inf",
        "sum",
        "product",
        "prod",
        "summation",
        "cofactor",
        "cofactors",
        "cof",
        "minor",
        "minors",
        "adjugate",
        "adj",
        "inv",
        "inverse",
        "transpose",
        "trans",
        "len",
        "length",
        "wid",
        "width",
        "tr",
        "trace",
        "det",
        "determinant",
        "part",
        "norm",
        "abs",
        "normalize",
        "car",
        "cartesian",
        "polar",
        "pol",
        "angle",
        "cross",
        "proj",
        "project",
        "dot",
        "rotate",
        "sin",
        "csc",
        "cos",
        "sec",
        "tan",
        "cot",
        "asin",
        "arcsin",
        "acsc",
        "arccsc",
        "acos",
        "arccos",
        "asec",
        "arcsec",
        "atan",
        "arctan",
        "atan2",
        "acot",
        "arccot",
        "sinh",
        "csch",
        "cosh",
        "sech",
        "tanh",
        "coth",
        "asinh",
        "arcsinh",
        "acsch",
        "arccsch",
        "acosh",
        "arccosh",
        "asech",
        "arcsech",
        "atanh",
        "arctanh",
        "acoth",
        "arccoth",
        "cis",
        "ln",
        "aexp",
        "ceil",
        "floor",
        "round",
        "recip",
        "exp",
        "aln",
        "log",
        "root",
        "bi",
        "binomial",
        "gamma",
        "max",
        "min",
        "sqrt",
        "asquare",
        "abs",
        "norm",
        "deg",
        "degree",
        "rad",
        "radian",
        "gradian",
        "re",
        "real",
        "im",
        "imag",
        "sgn",
        "sign",
        "arg",
        "cbrt",
        "acube",
        "frac",
        "fract",
        "int",
        "trunc",
        "square",
        "asqrt",
        "cube",
        "acbrt",
        "fact",
        "subfact",
        "sinc",
        "conj",
        "conjugate",
        "erf",
        "erfc",
        "ai",
        "digamma",
        "zeta",
        "sort",
        "Γ",
        "ζ",
        "Σ",
        "Π",
        "factor",
        "factors",
        "vec",
        "mat",
        "prime",
        "add",
        "reverse",
        "link",
        "flatten",
        "I",
        "P",
        "C",
        "split",
        "slog",
        "doublefact",
        "mean",
        "median",
        "mode",
        "quad",
    ]
    .iter()
    .cloned()
    .collect();
    functions.contains(word)
}
pub fn get_vars(options: Options) -> Vec<[String; 2]>
{
    let pi = Float::with_val(options.prec, Pi);
    let catalan = Float::with_val(options.prec, Catalan);
    let euler = Float::with_val(options.prec, Euler);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec, 1).exp();
    vec![
        ["phi".to_string(), phi.to_string()],
        ["tau".to_string(), tau.to_string()],
        ["cat".to_string(), catalan.to_string()],
        ["ec".to_string(), "1.602176634e-19".to_string()],
        ["kB".to_string(), "1.380649e-23".to_string()],
        ["me".to_string(), "9.1093837015e-31".to_string()],
        ["mn".to_string(), "1.67492749804e-27".to_string()],
        ["mp".to_string(), "1.67262192369e-27".to_string()],
        ["Na".to_string(), "6.02214076e23".to_string()],
        ["pi".to_string(), pi.to_string()],
        ["c".to_string(), "299792458".to_string()],
        ["e".to_string(), e.to_string()],
        ["G".to_string(), "6.67430e-11".to_string()],
        ["g".to_string(), "9.80665".to_string()],
        ["h".to_string(), "6.62607015e-34".to_string()],
        ["k".to_string(), "8.9875517923e9".to_string()],
        ["R".to_string(), "8.31446261815324".to_string()],
        ["φ".to_string(), phi.to_string()],
        ["π".to_string(), pi.to_string()],
        ["τ".to_string(), tau.to_string()],
        ["γ".to_string(), euler.to_string()],
    ]
    .iter()
    .map(|a| {
        if options.small_e
        {
            a.clone()
        }
        else
        {
            [a[0].clone(), a[1].replace('e', "E")]
        }
    })
    .collect()
}