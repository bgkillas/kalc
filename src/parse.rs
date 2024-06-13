use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    functions::functions,
    math::do_math,
    units::{is_unit, prefixes, to_unit},
    GraphType, HowGraphing,
    Notation::SmallEngineering,
    Number, Options, Variable,
};
use rug::{
    float::Special::{Infinity, Nan},
    ops::{CompleteRound, Pow},
    Complex,
};
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn input_var(
    input: &str,
    vars: &[Variable],
    sumrec: &mut Vec<(isize, String)>,
    bracket: &mut isize,
    options: Options,
    print: bool,
    depth: usize,
    blacklist: Vec<char>,
    isgraphing: bool,
    collectvars: &mut Vec<(isize, usize)>,
) -> Result<
    (
        Vec<NumStr>,
        Vec<(String, Vec<NumStr>)>,
        HowGraphing,
        bool,
        Option<String>,
    ),
    &'static str,
>
{
    let mut sumvar = None;
    let mut graph = HowGraphing::default();
    let prec = (options.prec, options.prec);
    let mut funcvars = Vec::new();
    if input.starts_with("history")
        || input.starts_with("his")
        || input.starts_with("onaxis")
        || input.starts_with("exit")
        || input.starts_with("quit")
        || input.starts_with("break")
        || input.is_empty()
    {
        return Err(" ");
    }
    let mut undf = false;
    let mut funcfailed = false;
    let mut absfailed = false;
    let mut scientific = false;
    let mut abs = Vec::new();
    let mut neg = false;
    let n1 = Complex::with_val(options.prec, -1);
    let mut pow = String::new();
    let mut exp = (String::new(), 0);
    let mut subfact = (false, 0);
    let mut err = "";
    let mut chars = input
        .replace('[', "(car{")
        .replace(']', "})")
        .chars()
        .collect::<Vec<char>>();
    if chars.ends_with(&['^'])
    {
        chars.pop();
    }
    let mut sarea = 0;
    let mut output: Vec<NumStr> = Vec::new();
    let mut stack_end = Vec::new();
    let mut stack_start = Vec::new();
    let mut i = 0;
    let mut piecewise = 0;
    let mut pwr: (bool, isize, isize) = (false, 0, 0);
    while !chars.is_empty() && chars[0].is_whitespace()
    {
        chars.remove(0);
    }
    while i < chars.len()
    {
        if chars[i].is_whitespace()
        {
            if chars.len().saturating_sub(1) == i
            {
                chars.remove(i);
            }
            else if (chars[i - 1].is_alphanumeric() || matches!(chars[i - 1], ')' | '}' | '|'))
                && (chars[i + 1].is_alphanumeric() || matches!(chars[i + 1], '(' | '{' | '|'))
            {
                chars[i] = '*'
            }
            else if chars[i - 1] == '+' && chars[i + 1] == '-'
            {
                chars.drain(i - 1..=i);
            }
            else
            {
                chars.remove(i);
            }
        }
        else
        {
            i += 1;
        }
    }
    for c in &chars
    {
        match c
        {
            '(' => stack_end.insert(0, ')'),
            '{' => stack_end.insert(0, '}'),
            ')' | '}' =>
            {
                if !stack_end.is_empty() && stack_end[0] == *c
                {
                    stack_end.remove(0);
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
    chars.extend(stack_end);
    for i in stack_start
    {
        chars.insert(0, i);
    }
    i = 0;
    let mut sum = (0, String::new());
    let functions = functions();
    let mut ceilfoor = 0;
    'main: while i < chars.len()
    {
        let c = chars[i];
        if !matches!(
            c,
            '⁰' | '₀'
                | '⁹'
                | '₉'
                | '⁸'
                | '₈'
                | '⁷'
                | '₇'
                | '⁶'
                | '₆'
                | '⁵'
                | '₅'
                | '⁴'
                | '₄'
                | '³'
                | '₃'
                | '²'
                | '₂'
                | '¹'
                | '₁'
                | '⁻'
                | 'ⁱ'
        ) && !pow.is_empty()
        {
            let i = pow.matches('i').count() % 4;
            pow = pow.replace('i', "");
            if pow.is_empty()
            {
                pow = '1'.to_string();
            }
            match output.last()
            {
                Some(Num(_)) | Some(Vector(_)) | Some(Matrix(_)) =>
                {
                    output.push(Str('^'.to_string()))
                }
                Some(Str(s))
                    if matches!(s.as_str(), "x" | "y" | "rnd" | "rand" | "epoch")
                        || sumrec.iter().any(|v| &v.1 == s) =>
                {
                    output.push(Str('^'.to_string()))
                }
                _ =>
                {}
            }
            output.push(Num(Number::from(
                match Complex::parse_radix(pow.as_bytes(), options.base.0)
                {
                    Ok(n) => n.complete(prec),
                    _ => return Err("exponent error"),
                } * Complex::with_val(options.prec, (0, 1)).pow(Complex::with_val(options.prec, i)),
                None,
            )));
            pow = String::new();
        }
        if c == '.' && i + 1 < chars.len() && chars[i + 1] == '.'
        {
            output.push(Str("..".to_string()));
            i += 2;
            continue;
        }
        if c.is_ascii_digit()
            || c == '.'
            || (options.base.0 > 10
                && (97..=97 + (options.base.0 as u8 - 11)).contains(&(chars[i] as u8)))
        {
            let mut num = String::new();
            let mut dot = false;
            while i < chars.len()
            {
                if is_digit(chars[i], options.base.0)
                {
                    num.push(chars[i]);
                }
                else if chars[i] == '.'
                {
                    if i + 1 < chars.len() && chars[i + 1] == '.'
                    {
                        break;
                    }
                    else if dot
                    {
                        return Err("invalid digit");
                    }
                    else
                    {
                        dot = true;
                        num.push('.')
                    }
                }
                else
                {
                    break;
                }
                i += 1;
            }
            if num.starts_with('.')
            {
                num.insert(0, '0')
            }
            place_multiplier(&mut output, sumrec);
            if neg
            {
                if chars.len() > i
                    && (chars[i] == '^'
                        || chars[i] == '!'
                        || (chars.len() > i + 1 && chars[i] == '/' && chars[i + 1] == '/'))
                {
                    output.push(Num(Number::from(n1.clone(), None)));
                    output.push(Str('×'.to_string()));
                }
                else
                {
                    num.insert(0, '-');
                }
                neg = false;
            }
            if num == "0" && i != chars.len() && chars[i] == '⁻'
            {
                num.insert(0, '-');
                i += 1;
            }
            output.push(Num(Number::from(
                match Complex::parse_radix(num.clone(), options.base.0)
                {
                    Ok(n) => n.complete(prec),
                    Err(_) => return Err("probably radix error"),
                },
                None,
            )));
            if scientific
            {
                output.push(Str(")".to_string()));
                scientific = false;
            }
            if pwr.0 && pwr.1 == *bracket && (chars.len() <= i || chars[i] != '^')
            {
                for _ in 0..pwr.2
                {
                    output.push(Str(')'.to_string()))
                }
                pwr.0 = false;
                pwr.2 = 0
            }
            if subfact.0 && subfact.1 == 0
            {
                output.push(Str(')'.to_string()));
                output.push(Str(')'.to_string()));
                subfact.0 = false;
            }
            continue;
        }
        if !c.is_alphabetic() && !matches!(c, '°' | '@' | '∫' | '$' | '¢')
        {
            if !output.is_empty()
            {
                if let Str(s) = output.last_mut().unwrap()
                {
                    if functions.contains(s.as_str()) && !sumrec.iter().any(|a| a.1 == *s)
                    {
                        if i + 4 < chars.len()
                            && chars[i] == '^'
                            && chars[i + 1] == '('
                            && chars[i + 2] == '-'
                            && chars[i + 3] == '1'
                            && chars[i + 4] == ')'
                        {
                            s.insert(0, 'a');
                            i += 5;
                            continue;
                        }
                        if i + 2 < chars.len()
                            && chars[i] == '^'
                            && chars[i + 1] == '-'
                            && chars[i + 2] == '1'
                        {
                            s.insert(0, 'a');
                            i += 3;
                            continue;
                        }
                        if i + 1 < chars.len() && chars[i] == '⁻' && chars[i + 1] == '¹'
                        {
                            s.insert(0, 'a');
                            i += 2;
                            continue;
                        }
                        if i + 1 < chars.len()
                            && chars[i] == '^'
                            && (is_digit(chars[i + 1], options.base.0) || chars[i + 1] == '-')
                        {
                            let pos = chars
                                .iter()
                                .skip(i + 1)
                                .position(|&c| c == '(' || c == ')' || c == ',');
                            if pos.is_none()
                            {
                                return Err("bad exponent");
                            }
                            exp = (
                                chars[i + 1..i + 1 + pos.unwrap()].iter().collect(),
                                *bracket,
                            );
                            i += pos.unwrap() + 1;
                            continue;
                        }
                    }
                }
            }
            match c
            {
                '√' => output.push(Str("sqrt".to_string())),
                '∛' => output.push(Str("cbrt".to_string())),
                '¼' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.25),
                    None,
                ))),
                '½' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.5),
                    None,
                ))),
                '¾' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.75),
                    None,
                ))),
                '⅒' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 1) / 10,
                    None,
                ))),
                '⅕' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 1) / 5,
                    None,
                ))),
                '⅖' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 2) / 5,
                    None,
                ))),
                '⅗' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 3) / 5,
                    None,
                ))),
                '⅘' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 4) / 5,
                    None,
                ))),
                '⅐' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 7).recip(),
                    None,
                ))),
                '⅑' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 9).recip(),
                    None,
                ))),
                '⅓' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 3).recip(),
                    None,
                ))),
                '⅔' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 1.5).recip(),
                    None,
                ))),
                '⅙' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 6).recip(),
                    None,
                ))),
                '⅚' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 5) / 6,
                    None,
                ))),
                '⅛' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.125),
                    None,
                ))),
                '⅜' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.375),
                    None,
                ))),
                '⅝' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.625),
                    None,
                ))),
                '⅞' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, 0.875),
                    None,
                ))),
                '⅟' =>
                {
                    output.push(Num(Number::from(Complex::with_val(options.prec, 1), None)));
                    output.push(Str("/".to_string()))
                }
                '↉' => output.push(Num(Number::from(Complex::new(options.prec), None))),
                '⁰' | '₀' => pow.push('0'),
                '⁹' | '₉' => pow.push('9'),
                '⁸' | '₈' => pow.push('8'),
                '⁷' | '₇' => pow.push('7'),
                '⁶' | '₆' => pow.push('6'),
                '⁵' | '₅' => pow.push('5'),
                '⁴' | '₄' => pow.push('4'),
                '³' | '₃' => pow.push('3'),
                '²' | '₂' => pow.push('2'),
                '¹' | '₁' => pow.push('1'),
                '⁻' => pow.push('-'),
                '&' if i != 0
                    && i + 1 < chars.len()
                    && chars[i + 1] == '&'
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    output.push(Str("&&".to_string()));
                }
                '=' if i != 0
                    && i + 1 < chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '='
                    {
                        output.push(Str("==".to_string()));
                        i += 1;
                    }
                    else if chars[i - 1] == '>'
                    {
                        output.push(Str(">=".to_string()));
                    }
                    else if chars[i - 1] == '<'
                    {
                        output.push(Str("<=".to_string()));
                    }
                    else
                    {
                        return Ok((Vec::new(), Vec::new(), HowGraphing::default(), true, None));
                    }
                }
                '{' =>
                {
                    *bracket += 1;
                    place_multiplier(&mut output, sumrec);
                    output.push(Str("{".to_string()));
                }
                '}' =>
                {
                    *bracket -= 1;
                    output.push(Str("}".to_string()));
                    if pwr.0 && pwr.1 == *bracket && (chars.len() <= i + 1 || chars[i + 1] != '^')
                    {
                        for _ in 0..pwr.2
                        {
                            output.push(Str(')'.to_string()))
                        }
                        pwr.0 = false;
                        pwr.2 = 0
                    }
                }
                '≈' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    output.push(Str("≈".to_string()))
                }
                '±' if i + 1 != chars.len() && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if output.is_empty()
                        || matches!(output.last().unwrap(),Str(s) if s==","||s=="{"||s=="(")
                    {
                        output.push(Num(Number::from(Complex::new(options.prec), None)))
                    }
                    output.push(Str("±".to_string()))
                }
                '*' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '*'
                    {
                        if chars.len() > i + 2
                        {
                            output.push(Str("^".to_string()));
                        }
                        i += 1;
                    }
                    else
                    {
                        output.push(Str('*'.to_string()));
                    }
                }
                '/' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '/'
                    {
                        output.push(Str("//".to_string()));
                        i += 1;
                    }
                    else if chars[i + 1] == '-'
                    {
                        place_multiplier(&mut output, sumrec);
                        output.push(Num(Number::from(n1.clone(), None)));
                        output.push(Str('/'.to_string()));
                        i += 1;
                    }
                    else
                    {
                        output.push(Str('/'.to_string()));
                    }
                }
                '+' if i != 0
                    && i + 1 != chars.len()
                    && (chars[i - 1].is_alphanumeric()
                        || (!output.is_empty() && output.last().unwrap().str_is(")"))
                        || matches!(
                            chars[i - 1],
                            '}' | ']' | ')' | '@' | '°' | '$' | '¢' | '%'
                        ))
                    && chars[i - 1]
                        != if options.notation == SmallEngineering
                        {
                            'e'
                        }
                        else
                        {
                            'E'
                        }
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '-'
                    {
                        if output.is_empty()
                            || matches!(output.last().unwrap(),Str(s) if s==","||s=="{"||s=="(")
                        {
                            output.push(Num(Number::from(Complex::new(options.prec), None)))
                        }
                        i += 1;
                        output.push(Str('±'.to_string()))
                    }
                    else
                    {
                        output.push(Str('+'.to_string()))
                    }
                }
                '+' if i + 1 < chars.len()
                    && chars[i + 1] == '-'
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if output.is_empty()
                        || matches!(output.last().unwrap(),Str(s) if s==","||s=="{"||s=="(")
                    {
                        output.push(Num(Number::from(Complex::new(options.prec), None)))
                    }
                    i += 1;
                    output.push(Str('±'.to_string()))
                }
                '<' if i != 0
                    && i + 1 < chars.len()
                    && chars[i + 1] != '='
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '<'
                    {
                        if i + 2 < chars.len()
                        {
                            output.push(Str("<<".to_string()));
                            i += 1;
                        }
                    }
                    else
                    {
                        output.push(Str('<'.to_string()));
                    }
                }
                '>' if i != 0
                    && i + 1 < chars.len()
                    && chars[i + 1] != '='
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '>'
                    {
                        if i + 2 < chars.len()
                        {
                            output.push(Str(">>".to_string()));
                            i += 1;
                        }
                    }
                    else
                    {
                        output.push(Str('>'.to_string()));
                    }
                }
                '-' if i + 1 < chars.len() && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if options.units && chars[i + 1] == '>'
                    {
                        output.push(Str("->".to_string()));
                        i += 1;
                    }
                    else if (i != 0 && chars[i - 1] == '^')
                        || (i > 1 && chars[i - 1] == '/' && chars[i - 2] == '/')
                    {
                        output.push(Str("(".to_string()));
                        output.push(Num(Number::from(n1.clone(), None)));
                        output.push(Str("×".to_string()));
                        pwr.0 = true;
                        pwr.1 = *bracket;
                        pwr.2 += 1;
                    }
                    else if i == 0
                        || !(chars[i - 1]
                            != if options.notation == SmallEngineering
                            {
                                'e'
                            }
                            else
                            {
                                'E'
                            }
                            && (chars[i - 1].is_alphanumeric()
                                || (!output.is_empty() && output.last().unwrap().str_is(")"))
                                || matches!(
                                    chars[i - 1],
                                    '}' | ']' | ')' | '@' | '°' | '$' | '¢' | '%'
                                )))
                    {
                        if i + 1 != chars.len()
                            && matches!(chars[i + 1], '(' | '{' | '[' | '|' | '-' | '!')
                        {
                            output.push(Num(Number::from(n1.clone(), None)));
                            output.push(Str("×".to_string()));
                        }
                        else
                        {
                            neg = true;
                        }
                    }
                    else
                    {
                        output.push(Str('-'.to_string()));
                    }
                }
                '^' if !output.is_empty()
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '^'
                    {
                        if chars.len() > i + 2
                        {
                            output.push(Str("^^".to_string()))
                        }
                        i += 1;
                    }
                    else
                    {
                        output.push(Str("^".to_string()));
                    }
                }
                '⌈' if i + 1 != chars.len() =>
                {
                    *bracket += 1;
                    ceilfoor += 2;
                    output.push(Str("(".to_string()));
                    output.push(Str("ceil".to_string()));
                    output.push(Str("(".to_string()));
                }
                '⌊' if i + 1 != chars.len() =>
                {
                    *bracket += 1;
                    ceilfoor += 2;
                    output.push(Str("(".to_string()));
                    output.push(Str("floor".to_string()));
                    output.push(Str("(".to_string()));
                }
                '⌉' if i != 0 =>
                {
                    *bracket -= 1;
                    ceilfoor -= 2;
                    output.push(Str(")".to_string()));
                    output.push(Str(")".to_string()));
                    if pwr.0 && pwr.1 == *bracket && (chars.len() <= i + 1 || chars[i + 1] != '^')
                    {
                        for _ in 0..pwr.2
                        {
                            output.push(Str(')'.to_string()))
                        }
                        pwr.0 = false;
                        pwr.2 = 0
                    }
                }
                '⌋' if i != 0 =>
                {
                    *bracket -= 1;
                    ceilfoor -= 2;
                    output.push(Str(")".to_string()));
                    output.push(Str(")".to_string()));
                    if pwr.0 && pwr.1 == *bracket && (chars.len() <= i + 1 || chars[i + 1] != '^')
                    {
                        for _ in 0..pwr.2
                        {
                            output.push(Str(')'.to_string()))
                        }
                        pwr.0 = false;
                        pwr.2 = 0
                    }
                }
                '(' if i + 1 != chars.len() =>
                {
                    *bracket += 1;
                    if subfact.0
                    {
                        subfact.1 = *bracket;
                    }
                    place_multiplier(&mut output, sumrec);
                    output.push(Str("(".to_string()))
                }
                ')' if i != 0 =>
                {
                    if !collectvars.is_empty() && collectvars[0].0 == *bracket
                    {
                        output.insert(collectvars[0].1, Str(",".to_string()));
                        collectvars.remove(0);
                    }
                    if piecewise == *bracket as usize
                    {
                        piecewise = 0;
                    }
                    if subfact.1 == *bracket
                    {
                        subfact = (false, 0);
                        output.push(Str(")".to_string()));
                        output.push(Str(")".to_string()))
                    }
                    *bracket -= 1;
                    output.push(Str(")".to_string()));
                    if !exp.0.is_empty() && exp.1 == *bracket
                    {
                        output.push(Str("^".to_string()));
                        output.push(Num(Number::from(
                            match Complex::parse_radix(exp.0.as_bytes(), options.base.0)
                            {
                                Ok(n) => n.complete(prec),
                                _ => return Err("exponent error"),
                            },
                            None,
                        )));
                        exp = (String::new(), 0);
                    }
                    if pwr.0 && pwr.1 == *bracket && (chars.len() <= i + 1 || chars[i + 1] != '^')
                    {
                        for _ in 0..pwr.2
                        {
                            output.push(Str(')'.to_string()))
                        }
                        pwr.0 = false;
                        pwr.2 = 0
                    }
                }
                '|' =>
                {
                    if !abs.is_empty() && abs[0] == *bracket && can_abs(&output, vars)
                    {
                        *bracket -= 1;
                        if (i + 2 >= chars.len() || chars[i + 1] != '^') && pwr.1 == *bracket
                        {
                            for _ in 0..pwr.2
                            {
                                output.push(Str(')'.to_string()))
                            }
                            pwr = (false, 0, 0);
                        }
                        output.push(Str(")".to_string()));
                        output.push(Str(")".to_string()));
                        abs.remove(0);
                    }
                    else if i + 1 != chars.len() && chars[i + 1] == '|'
                    {
                        output.push(Str("||".to_string()));
                        i += 2;
                        continue;
                    }
                    else if i + 1 != chars.len()
                    {
                        *bracket += 1;
                        if subfact.0
                        {
                            subfact.1 = *bracket;
                        }
                        place_multiplier(&mut output, sumrec);
                        output.push(Str("(".to_string()));
                        output.push(Str("norm".to_string()));
                        output.push(Str("(".to_string()));
                        abs.insert(0, *bracket);
                    }
                }
                '!' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '='
                    {
                        output.push(Str("!=".to_string()));
                        i += 1;
                    }
                    else if i != 0
                        && (chars[i - 1].is_alphanumeric()
                            || (!output.is_empty()
                                && (output.last().unwrap().str_is(")")
                                    || output.last().unwrap().str_is("}"))))
                    {
                        if !output.is_empty()
                            && (output.last().unwrap().str_is(")")
                                || output.last().unwrap().str_is("}"))
                        {
                            let mut count = 0;
                            for (j, c) in output.iter().enumerate().rev()
                            {
                                if let Str(s) = c
                                {
                                    if s == "(" || s == "{"
                                    {
                                        count -= 1;
                                    }
                                    else if s == ")" || s == "}"
                                    {
                                        count += 1;
                                    }
                                }
                                if count == 0
                                {
                                    if j != 0
                                    {
                                        if let Str(s) = &output[j - 1]
                                        {
                                            if !s.is_empty()
                                                && s.chars().next().unwrap().is_alphabetic()
                                            {
                                                output.insert(j - 1, Str("(".to_string()));
                                                if i + 1 != chars.len() && chars[i + 1] == '!'
                                                {
                                                    i += 1;
                                                    output.insert(j, Str("(".to_string()));
                                                    output.insert(j, Str("doublefact".to_string()));
                                                }
                                                else
                                                {
                                                    output.insert(j, Str("(".to_string()));
                                                    output.insert(j, Str("fact".to_string()));
                                                }
                                                output.push(Str(")".to_string()));
                                                output.push(Str(")".to_string()));
                                                i += 1;
                                                continue 'main;
                                            }
                                        }
                                    }
                                    output.insert(j, Str("(".to_string()));
                                    if i + 1 != chars.len() && chars[i + 1] == '!'
                                    {
                                        i += 1;
                                        output.insert(j, Str("doublefact".to_string()));
                                    }
                                    else
                                    {
                                        output.insert(j, Str("fact".to_string()));
                                    }
                                    output.push(Str(")".to_string()));
                                    i += 1;
                                    continue 'main;
                                }
                            }
                        }
                        output.insert(output.len().saturating_sub(1), Str("(".to_string()));
                        if i + 1 != chars.len() && chars[i + 1] == '!'
                        {
                            i += 1;
                            output.insert(
                                output.len().saturating_sub(1),
                                Str("doublefact".to_string()),
                            );
                        }
                        else
                        {
                            output.insert(output.len().saturating_sub(1), Str("fact".to_string()));
                        }
                        output.insert(output.len().saturating_sub(1), Str("(".to_string()));
                        output.push(Str(")".to_string()));
                        output.push(Str(")".to_string()));
                    }
                    else if i != chars.len().saturating_sub(1)
                        && (chars[i + 1].is_alphanumeric()
                            || matches!(chars[i + 1], '(' | '{' | '|' | '-' | '!'))
                    {
                        output.push(Str("(".to_string()));
                        output.push(Str("subfact".to_string()));
                        output.push(Str("(".to_string()));
                        subfact.0 = true;
                    }
                }
                ',' if i != 0 && i + 1 != chars.len() && chars[i + 1] != ')' =>
                {
                    for (i, sum) in sumrec.clone().iter().enumerate()
                    {
                        if &sum.0 == bracket
                        {
                            if sarea > 0
                            {
                                sarea -= 1;
                            }
                            else
                            {
                                sumrec.remove(i);
                            }
                            break;
                        }
                    }
                    if scientific
                    {
                        output.push(Str(")".to_string()));
                        scientific = false;
                    }
                    if pwr.0 && pwr.1 == *bracket && (chars.len() <= i + 1 || chars[i + 1] != '^')
                    {
                        for _ in 0..pwr.2
                        {
                            output.push(Str(')'.to_string()))
                        }
                        pwr.0 = false;
                        pwr.2 = 0
                    }
                    if subfact.0 && subfact.1 == 0
                    {
                        subfact.0 = false;
                        output.push(Str(")".to_string()));
                        output.push(Str(")".to_string()))
                    }
                    output.push(Str(','.to_string()))
                }
                '%' if i != 0 =>
                {
                    if i + 1 == chars.len()
                        || matches!(chars[i + 1], '+' | '-' | '*' | '/' | '^' | '}' | ')')
                    {
                        let mut j: isize = -1;
                        let mut bracket = 0;
                        for (k, n) in output.iter().rev().enumerate()
                        {
                            if let Str(s) = n
                            {
                                if s == ")"
                                {
                                    bracket += 1;
                                }
                                else if s == "("
                                {
                                    bracket -= 1;
                                }
                                else if bracket == 0 && matches!(s.as_str(), "+" | "-" | "±")
                                {
                                    j = ((output.len() - k) - 1) as isize;
                                    break;
                                }
                            }
                        }
                        if j != -1
                        {
                            output.insert(
                                j as usize,
                                Num(Number::from(Complex::with_val(options.prec, 1), None)),
                            );
                            output.insert(j as usize, Str('('.to_string()));
                            output.insert(j as usize, Str('*'.to_string()));
                        }
                        if let Some(Num(_)) = output.last()
                        {
                        }
                        else if let Some(Str(s)) = output.last()
                        {
                            if !matches!(s.as_str(), ")" | "}")
                            {
                                output.push(Num(Number::from(
                                    Complex::with_val(options.prec, 1),
                                    None,
                                )))
                            }
                        }
                        else
                        {
                            output.push(Num(Number::from(Complex::with_val(options.prec, 1), None)))
                        }
                        output.push(Str('*'.to_string()));
                        output.push(Num(Number::from(
                            Complex::with_val(options.prec, 1) / 100,
                            None,
                        )));
                        if j != -1
                        {
                            output.push(Str(')'.to_string()));
                        }
                    }
                    else if !matches!(chars[i + 1], ')' | '}' | ']')
                    {
                        output.push(Str('%'.to_string()))
                    }
                }
                '∞' => output.push(Num(Number::from(
                    Complex::with_val(options.prec, Infinity),
                    None,
                ))),
                '#' =>
                {
                    graph.graph = true;
                    break 'main;
                }
                _ =>
                {}
            }
            i += 1;
            continue;
        }
        let mut depthcheck = false;
        let mut word = String::new();
        let mut countv = 0;
        for c in chars[i..].iter()
        {
            if c == &'@'
            {
                depthcheck = !depthcheck;
            }
            else if c.is_alphabetic()
                || matches!(*c, '°' | '\'' | '`' | '_' | '∫' | '$' | '¢')
                || (c == &'2' && word == "atan")
            {
                word.push(*c);
            }
            else if !depthcheck
            {
                break;
            }
            countv += 1;
        }
        let wordv = word.clone();
        let var_overrule = if i + countv < chars.len()
            && (matches!(chars[i + countv], '(' | '{' | '[')
                || (!funcfailed && chars[i + countv] == '|'))
        {
            !vars.iter().any(|a| {
                if a.name.contains(&'(')
                {
                    a.name[..a.name.iter().position(|c| c == &'(').unwrap()]
                        .iter()
                        .collect::<String>()
                        == word
                }
                else
                {
                    a.name.iter().collect::<String>() == word
                }
            })
        }
        else
        {
            !vars
                .iter()
                .any(|a| a.name.iter().collect::<String>() == word)
        };
        if var_overrule
            && ((word.ends_with('x')
                && word != "max"
                && !word.ends_with("lx")
                && !word.ends_with("lux"))
                || (word.ends_with('y')
                    && word != "any"
                    && word != "unity"
                    && !word.ends_with("day")
                    && !word.ends_with("ly")
                    && !word.ends_with("henry")
                    && !word.ends_with("Gy")
                    && !word.ends_with("gray"))
                || (word.ends_with('z')
                    && !word.ends_with("Hz")
                    && !word.ends_with("hertz")
                    && !word.ends_with("oz")))
        {
            countv -= 1;
            word.pop();
        }
        if (word == "piecewise" || word == "pw") && piecewise == 0
        {
            piecewise = *bracket as usize + 1;
        }
        else if matches!(
            word.as_str(),
            "∫" | "area"
                | "surfacearea"
                | "sarea"
                | "solve"
                | "length"
                | "slope"
                | "sum"
                | "iter"
                | "extrema"
                | "summation"
                | "prod"
                | "production"
                | "vec"
                | "mat"
                | "Σ"
                | "Π"
                | "D"
                | "integrate"
                | "arclength"
                | "lim"
                | "limit"
        ) && chars.len() > i + countv + 1
            && var_overrule
        {
            let mut place = 0;
            let mut count2 = 0;
            for c in &chars[i + countv + 1..]
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
            if match word.as_str()
            {
                "integrate" | "vec" | "arclength" | "mat" | "prod" | "production" | "iter"
                | "length" | "∫" | "area" | "sum" | "Σ" | "summation" | "Π" => place >= 3,
                "sarea" | "surfacearea" => place >= 6,
                "solve" | "extrema" => place >= 1,
                "D" | "slope" | "lim" | "limit" => place >= 2,
                _ => place > 0,
            }
            {
                sum.0 = *bracket + 1;
                sum.1 = String::new();
                let mut count = 0;
                for c in chars[i + countv + 1..].iter()
                {
                    count += 1;
                    if c.is_alphabetic() || matches!(c, '\'' | '`' | '_')
                    {
                        sum.1.push(*c);
                    }
                    else if c == &','
                    {
                        break;
                    }
                    else
                    {
                        *bracket += 1;
                        place_multiplier(&mut output, sumrec);
                        output.push(Str(word.clone()));
                        output.push(Str("(".to_string()));
                        collectvars.insert(0, (*bracket, output.len()));
                        i += countv + 1;
                        continue 'main;
                    }
                }
                if !sum.1.is_empty()
                {
                    if sumrec.is_empty()
                    {
                        sumrec.push(sum.clone())
                    }
                    else
                    {
                        for (i, j) in sumrec.iter().enumerate()
                        {
                            if j.1.chars().count() <= sum.1.len()
                            {
                                sumrec.insert(i, sum.clone());
                                break;
                            }
                        }
                    }
                    place_multiplier(&mut output, sumrec);
                    output.push(Str(word.clone()));
                    output.push(Str("(".to_string()));
                    if sumrec.iter().any(|c| c.0 == -1)
                    {
                        output.push(Str("@".to_owned() + &sum.1));
                    }
                    else
                    {
                        output.push(Str(sum.1));
                    }
                    output.push(Str(",".to_string()));
                    if matches!(word.as_str(), "surfacearea" | "sarea")
                    {
                        sarea += 1;
                        sum.0 = *bracket + 1;
                        sum.1 = String::new();
                        for c in chars[i + countv + count + 1..].iter()
                        {
                            count += 1;
                            if c.is_alphabetic() || matches!(c, '\'' | '`' | '_')
                            {
                                sum.1.push(*c);
                            }
                            else if c == &','
                            {
                                break;
                            }
                        }
                        for (i, j) in sumrec.iter().enumerate()
                        {
                            if j.1.chars().count() <= sum.1.len()
                            {
                                sumrec.insert(i, sum.clone());
                                break;
                            }
                        }
                        if sumrec.iter().any(|c| c.0 == -1)
                        {
                            output.push(Str("@".to_owned() + &sum.1));
                        }
                        else
                        {
                            output.push(Str(sum.1));
                        }
                        output.push(Str(",".to_string()));
                    }
                    *bracket += 1;
                    i += count + countv + 1;
                    continue;
                }
            }
            else if place == 0
            {
                if matches!(word.as_str(), "extrema" | "solve")
                {
                    *bracket += 1;
                    place_multiplier(&mut output, sumrec);
                    output.push(Str(word.clone()));
                    output.push(Str("(".to_string()));
                    collectvars.insert(0, (*bracket, output.len()));
                    i += countv + 1;
                    continue 'main;
                }
            }
            else
            {
                *bracket += 1;
                place_multiplier(&mut output, sumrec);
                output.push(Str(word.clone()));
                output.push(Str("(".to_string()));
                collectvars.insert(0, (*bracket, output.len()));
                i += countv + 1;
                continue 'main;
            }
        }
        let (mut unit, mul);
        let mut num = 0;
        let var_overrule = if i + countv < chars.len()
            && (matches!(chars[i + countv], '(' | '{' | '[')
                || (!funcfailed && chars[i + countv] == '|'))
        {
            !vars.iter().any(|a| {
                if a.name.contains(&'(')
                {
                    a.name[..a.name.iter().position(|c| c == &'(').unwrap()]
                        .iter()
                        .collect::<String>()
                        == word
                }
                else
                {
                    a.name.iter().collect::<String>() == word
                }
            })
        }
        else
        {
            !vars.iter().any(|a| {
                a.name.iter().collect::<String>().starts_with(&word)
                    && (a.name.len() == word.chars().count()
                        || (i + (a.name.len().saturating_sub(1)) < chars.len()
                            && chars[i..i + a.name.len()] == a.name))
            })
        };
        if sumrec.iter().any(|a| {
            if wordv == a.1
            {
                num = a.0;
                word.clone_from(&a.1);
                true
            }
            else
            {
                false
            }
        }) || (!vars
            .iter()
            .any(|c| c.name.iter().collect::<String>().split('(').next().unwrap() == wordv)
            && !functions.contains(wordv.as_str())
            && sumrec.iter().any(|a| {
                if wordv.starts_with(&a.1) && !a.1.is_empty()
                {
                    num = a.0;
                    word.clone_from(&a.1);
                    true
                }
                else
                {
                    false
                }
            }))
        {
            place_multiplier(&mut output, sumrec);
            if neg
            {
                output.push(Num(Number::from(n1.clone(), None)));
                output.push(Str('×'.to_string()));
                neg = false;
            }
            i += if c == '@'
            {
                chars[i + 1..].iter().position(|a| a == &'@').unwrap_or(0) + 2
            }
            else
            {
                word.chars().count()
            };
            if num > 0 && sumrec.iter().any(|c| c.0 == -1)
            {
                output.push(Str("@".to_owned() + &word));
            }
            else
            {
                output.push(Str(word));
            }
            if pwr.0 && pwr.1 == *bracket && chars[i] != '^'
            {
                for _ in 0..pwr.2
                {
                    output.push(Str(')'.to_string()))
                }
                pwr.0 = false;
                pwr.2 = 0
            }
            if scientific
            {
                output.push(Str(")".to_string()));
                scientific = false;
            }
            if subfact.0 && subfact.1 == 0
            {
                subfact.0 = false;
                output.push(Str(")".to_string()));
                output.push(Str(")".to_string()))
            }
        }
        else if var_overrule
            && ((functions.contains(word.as_str())
                && i + countv < chars.len()
                && (matches!(
                    chars[i + countv],
                    'x' | 'y' | 'z' | '(' | '|' | '{' | '0'..='9' | '⁻' | '*'
                ) || (chars[i + countv] == '^' && chars[i] != 'C' && countv != 1)))
                || matches!(
                    word.as_str(),
                    "rnd" | "rand" | "epoch" | "inf" | "true" | "false" | "nan" | "NaN"
                ))
        {
            place_multiplier(&mut output, sumrec);
            if neg
            {
                output.push(Num(Number::from(n1.clone(), None)));
                output.push(Str('×'.to_string()));
                neg = false;
            }
            i += countv;
            if matches!(
                word.as_str(),
                "rnd" | "rand" | "epoch" | "inf" | "true" | "false" | "nan" | "NaN"
            )
            {
                if matches!(word.as_str(), "nan" | "NaN")
                {
                    output.push(Num(Number::from(
                        Complex::with_val(options.prec, Nan),
                        None,
                    )));
                }
                else if word == "true"
                {
                    output.push(Num(Number::from(Complex::with_val(options.prec, 1), None)));
                }
                else if word == "false"
                {
                    output.push(Num(Number::from(Complex::new(options.prec), None)));
                }
                else if word == "inf"
                {
                    output.push(Num(Number::from(
                        Complex::with_val(options.prec, Infinity),
                        None,
                    )));
                }
                else
                {
                    output.push(Str(word))
                }
                if pwr.0 && pwr.1 == *bracket && (chars.len() <= i + 1 || chars[i + 1] != '^')
                {
                    for _ in 0..pwr.2
                    {
                        output.push(Str(')'.to_string()))
                    }
                    pwr.0 = false;
                    pwr.2 = 0
                }
                if scientific
                {
                    output.push(Str(")".to_string()));
                    scientific = false;
                }
                if subfact.0 && subfact.1 == 0
                {
                    subfact.0 = false;
                    output.push(Str(")".to_string()));
                    output.push(Str(")".to_string()))
                }
            }
            else
            {
                if chars[i] == '*'
                {
                    chars.remove(i);
                }
                output.push(Str(word))
            }
        }
        else if options.units
            && (collectvars.is_empty() || word.len() > 1)
            && {
                (unit, mul) = prefixes(word.clone(), prec);
                is_unit(&mut unit)
            }
            && var_overrule
        {
            place_multiplier(&mut output, sumrec);
            if neg
            {
                output.push(Num(Number::from(n1.clone(), None)));
                output.push(Str('×'.to_string()));
                neg = false;
            }
            if let Some(Str(s)) = output.last_mut()
            {
                if s == "*"
                {
                    *s = '×'.to_string()
                }
            }
            else if !output.is_empty()
            {
                output.push(Str('×'.to_string()))
            }
            let (num, add) = to_unit(unit, mul, options);
            output.push(Num(num));
            if let Some(num) = add
            {
                if i > 2
                    && ((chars[i - 2] == 't' && chars[i - 1] == 'o')
                        || (chars[i - 2] == '-' && chars[i - 1] == '>'))
                {
                    if chars[i - 3] == 'K'
                    {
                        output.insert(output.len().saturating_sub(2), Str('-'.to_string()));
                        output.insert(output.len().saturating_sub(2), Num(num));
                    }
                    else
                    {
                        output.insert(output.len().saturating_sub(3), Str('-'.to_string()));
                        output.insert(output.len().saturating_sub(3), Num(num));
                    }
                }
                else
                {
                    output.insert(
                        if i != 0 && chars[i - 1].is_alphanumeric()
                        {
                            output.len().saturating_sub(3)
                        }
                        else if i != 0 && chars[i - 1] == ')'
                        {
                            output.len().saturating_sub(
                                output.iter().rev().position(|c| c.str_is("(")).unwrap(),
                            )
                        }
                        else
                        {
                            output.len().saturating_sub(1)
                        },
                        Str('('.to_string()),
                    );
                    output.push(Str('+'.to_string()));
                    output.push(Num(num));
                    output.push(Str(')'.to_string()));
                }
            }
            i += countv;
        }
        else if options.units && word.starts_with("to") && !output.is_empty() && var_overrule
        {
            if i != 0 && chars[i - 1] == '*'
            {
                chars.remove(i - 1);
            }
            if let Some(Str(s)) = output.last_mut()
            {
                if s == "*"
                {
                    *s = "->".to_string()
                }
                else
                {
                    i += 1;
                    output.push(Str("->".to_string()));
                }
            }
            else
            {
                i += 1;
                output.push(Str("->".to_string()));
            }
            i += 1;
            if chars.len() > i && (chars[i] == '*' || chars[i] == '×')
            {
                chars.remove(i);
            }
        }
        else
        {
            for var in vars
            {
                if var.name != vec!['e']
                    || (options.notation != SmallEngineering
                        || !(i != 0
                            && i + 1 != chars.len()
                            && chars[i - 1].is_numeric()
                            && (chars[i + 1].is_numeric() || chars[i + 1] == '-')))
                {
                    let j = i;
                    let vn = var
                        .name
                        .split(|c| matches!(c, '(' | '{' | '[' | '|'))
                        .next()
                        .unwrap();
                    if var.name.contains(&'(')
                        && i + vn.len() < chars.len()
                        && chars[i..i + vn.len()] == *vn
                        && matches!(chars[i + vn.len()], '(' | '{' | '[' | '|')
                        && !absfailed
                    {
                        let abs = chars[i + vn.len()] == '|';
                        let countj = vn.len();
                        let mut count = 0;
                        let mut abstest = false;
                        for (f, c) in chars[i..].iter().enumerate()
                        {
                            if abs && *c == '|' && (count == 0 || (abstest && count == 1))
                            {
                                if abstest
                                {
                                    abstest = false;
                                    i += f;
                                    break;
                                }
                                count += 1;
                                abstest = true
                            }
                            else if matches!(c, '(' | '{' | '[')
                            {
                                count += 1;
                            }
                            else if matches!(c, ')' | '}' | ']')
                            {
                                count -= 1;
                                if count == 0
                                {
                                    i += f;
                                    break;
                                }
                            }
                        }
                        if abstest
                        {
                            i = j;
                            funcfailed = true;
                            absfailed = true;
                            continue 'main;
                        }
                        if i == j
                        {
                            i = chars.len().saturating_sub(1)
                        }
                        if blacklist == var.name && piecewise == 0
                        {
                            return Err("recursive");
                        }
                        count = 0;
                        let mut ccount = 0;
                        for c in &chars[j..i]
                        {
                            if *c == ','
                                && count
                                    == if matches!(chars[j + 1], '{' | '[')
                                    {
                                        0
                                    }
                                    else
                                    {
                                        1
                                    }
                            {
                                ccount += 1;
                            }
                            else if abs && *c == '|' && (count == 0 || (abstest && count == 1))
                            {
                                if abstest
                                {
                                    abstest = false;
                                    count -= 1;
                                    continue;
                                }
                                count += 1;
                                abstest = true
                            }
                            else if matches!(c, '(' | '{' | '[')
                            {
                                count += 1;
                            }
                            else if matches!(c, ')' | '}' | ']')
                            {
                                count -= 1;
                            }
                        }
                        if ccount != var.name.iter().filter(|c| c == &&',').count()
                        {
                            i = j;
                            continue;
                        }
                        if blacklist == var.name && piecewise != 0
                        {
                            output.push(Str("@".to_owned() + &var.name.iter().collect::<String>()));
                            i = j + var.name.split(|c| c == &'(').next().unwrap().len();
                            continue 'main;
                        }
                        if var.name.contains(&',') && chars.len() > 4
                        {
                            place_multiplier(&mut output, sumrec);
                            if neg
                            {
                                output.push(Num(Number::from(n1.clone(), None)));
                                output.push(Str('×'.to_string()));
                                neg = false;
                            }
                            let nobrackets = i + 1 != chars.len()
                                && j != 0
                                && chars[j - 1] == ','
                                && chars[i + 1] == ',';
                            if !nobrackets
                            {
                                output.push(Str('('.to_string()));
                            }
                            let mut temp = &chars[j + countj + 1..i + 1];
                            if temp.ends_with(&[')'])
                            {
                                temp = &temp[..temp.len().saturating_sub(1)];
                            }
                            let mut commas = Vec::new();
                            count = 0;
                            for (f, c) in temp.iter().enumerate()
                            {
                                if matches!(c, '(' | '{' | '[')
                                {
                                    count += 1;
                                }
                                else if matches!(c, ')' | '}' | ']')
                                {
                                    count -= 1;
                                }
                                else if c == &',' && count == 0
                                {
                                    commas.push(f);
                                }
                            }
                            let mut start = 0;
                            let mut split = Vec::new();
                            for end in commas
                            {
                                split.push(&temp[start..end]);
                                start = end + 1;
                            }
                            split.push(&temp[start..]);
                            let mut func_vars: Vec<String> = Vec::new();
                            start = 0;
                            for (f, c) in var.name.iter().enumerate()
                            {
                                if matches!(c, '(' | '{' | '[')
                                {
                                    if count == 0
                                    {
                                        start = f + 1;
                                    }
                                    count += 1;
                                }
                                else if matches!(c, ')' | '}' | ']')
                                {
                                    count -= 1;
                                    if count == 0
                                    {
                                        func_vars.push(var.name[start..f].iter().collect());
                                    }
                                }
                                else if c == &',' && count == 1
                                {
                                    func_vars.push(var.name[start..f].iter().collect());
                                    start = f + 1;
                                }
                            }
                            let mut parsed = var.parsed.clone();
                            let mut fvs = var.funcvars.clone();
                            for (varf, func_var) in split.iter().zip(func_vars)
                            {
                                let mut num = if let Ok(n) = Complex::parse_radix(
                                    varf.iter().collect::<String>(),
                                    options.base.0,
                                )
                                {
                                    vec![Num(Number::from(n.complete(prec), None))]
                                }
                                else
                                {
                                    let parsed;
                                    let exit;
                                    let func;
                                    let tempgraph;
                                    let sum_var;
                                    let mut cv = collectvars.clone();
                                    if !cv.is_empty()
                                    {
                                        if cv[0].0 < 0
                                        {
                                            cv[0].0 -= 1;
                                        }
                                        else
                                        {
                                            cv[0].0 = -1
                                        }
                                    }
                                    (parsed, func, tempgraph, exit, sum_var) = match input_var(
                                        &varf.iter().collect::<String>(),
                                        vars,
                                        sumrec,
                                        bracket,
                                        options,
                                        print,
                                        depth + 1,
                                        blacklist.clone(),
                                        false,
                                        &mut cv,
                                    )
                                    {
                                        Ok(f) => f,
                                        Err(s) =>
                                        {
                                            err = s;
                                            continue;
                                        }
                                    };
                                    if tempgraph.graph
                                    {
                                        graph.graph = true
                                    }
                                    if tempgraph.x
                                    {
                                        graph.x = true
                                    }
                                    if tempgraph.y
                                    {
                                        graph.y = true
                                    }
                                    if exit
                                    {
                                        return Ok((
                                            Vec::new(),
                                            Vec::new(),
                                            HowGraphing::default(),
                                            true,
                                            None,
                                        ));
                                    }
                                    if let Some(s) = sum_var
                                    {
                                        if collectvars[0].0 < 0
                                        {
                                            sumvar = Some(s)
                                        }
                                        else
                                        {
                                            output.insert(collectvars[0].1, Str(",".to_string()));
                                            output.insert(collectvars[0].1, Str(s.clone()));
                                            collectvars.remove(0);
                                        }
                                    }
                                    if (tempgraph.graph && parsed.len() > 1)
                                        || print
                                        || sumrec.iter().any(|c| c.0 == -1)
                                        || parsed.iter().any(|c| {
                                            if let Str(s) = c
                                            {
                                                sumrec.iter().any(|r| &r.1 == s)
                                            }
                                            else
                                            {
                                                false
                                            }
                                        })
                                        || func.iter().any(|c| {
                                            c.1.iter().any(|c| {
                                                if let Str(s) = c
                                                {
                                                    sumrec.iter().any(|r| &r.1 == s)
                                                }
                                                else
                                                {
                                                    false
                                                }
                                            })
                                        })
                                    {
                                        let iden =
                                            format!("@{}{}{}{}@", i, func_var, depth, vars.len());
                                        if parsed.len() == 1
                                        {
                                            parsed
                                        }
                                        else
                                        {
                                            funcvars.extend(func);
                                            funcvars.push((iden.clone(), parsed));
                                            vec![Str(iden)]
                                        }
                                    }
                                    else
                                    {
                                        vec![match do_math(parsed, options, func)
                                        {
                                            Ok(f) => f,
                                            Err(s) =>
                                            {
                                                err = s;
                                                continue;
                                            }
                                        }]
                                    }
                                };
                                if print
                                    && num.len() == 1
                                    && if let Num(n) = num[0].clone()
                                    {
                                        n.number.real().is_sign_negative()
                                    }
                                    else
                                    {
                                        false
                                    }
                                {
                                    num.insert(0, Str("(".to_string()));
                                    num.push(Str(")".to_string()));
                                }
                                let mut k = 0;
                                for (x, fv) in fvs.clone().iter().enumerate()
                                {
                                    if !fv.0.ends_with(')')
                                    {
                                        k = fv.1.len();
                                        while k != 0
                                        {
                                            k -= 1;
                                            if fv.1[k].str_is(&func_var)
                                            {
                                                fvs[x].1.remove(k);
                                                fvs[x].1.splice(k..k, num.clone());
                                            }
                                        }
                                    }
                                }
                                while k < parsed.len()
                                {
                                    if parsed[k].str_is(&func_var)
                                    {
                                        parsed.remove(k);
                                        if num.len() == 1
                                        {
                                            parsed.insert(k, num[0].clone());
                                        }
                                        else
                                        {
                                            parsed.splice(k..k, num.clone());
                                            k += num.len();
                                            continue;
                                        }
                                    }
                                    k += 1;
                                }
                            }
                            let mut k = 0;
                            while k < parsed.len()
                            {
                                for fv in &fvs
                                {
                                    if parsed[k].str_is(&fv.0)
                                    {
                                        if !fv.0.ends_with(')')
                                        {
                                            parsed[k] = Str(format!("@{}@{}{}", i, depth, fv.0));
                                        }
                                        else if !fv.0.starts_with('@')
                                        {
                                            parsed[k] = Str(format!("@{}", fv.0));
                                        }
                                    }
                                }
                                k += 1;
                            }
                            for (x, fv) in fvs.clone().iter().enumerate()
                            {
                                k = fv.1.len();
                                while k != 0
                                {
                                    k -= 1;
                                    for fc in fvs.clone()
                                    {
                                        if let Str(s) = &fv.1[k]
                                        {
                                            if s == &fc.0 && s != &fv.0
                                            {
                                                if !fc.0.contains('(')
                                                {
                                                    fvs[x].1[k] =
                                                        Str(format!("@{}@{}{}", i, depth, fc.0))
                                                }
                                                else if !fc.0.starts_with('@')
                                                {
                                                    fvs[x].1[k] = Str(format!("@{}", fc.0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            for (x, fv) in fvs.clone().iter().enumerate()
                            {
                                if !fv.0.ends_with(')')
                                {
                                    fvs[x].0 = format!("@{}@{}{}", i, depth, fv.0);
                                }
                                else if !fv.0.starts_with('@')
                                {
                                    fvs[x].0 = format!("@{}", fv.0);
                                }
                            }
                            funcvars.extend(fvs);
                            output.extend(parsed);
                            if pwr.1 == *bracket + 1
                            {
                                for _ in 0..pwr.2
                                {
                                    output.push(Str(')'.to_string()))
                                }
                                pwr = (false, 0, 0);
                            }
                            if subfact.1 == *bracket + 1
                            {
                                subfact = (false, 0);
                                output.push(Str(")".to_string()));
                                output.push(Str(")".to_string()))
                            }
                            if !nobrackets
                            {
                                output.push(Str(")".to_string()));
                            }
                            if !exp.0.is_empty() && exp.1 == *bracket
                            {
                                output.push(Str("^".to_string()));
                                output.push(Num(Number::from(
                                    match Complex::parse_radix(exp.0.as_bytes(), options.base.0)
                                    {
                                        Ok(n) => n.complete(prec),
                                        _ => return Err("exponent error"),
                                    },
                                    None,
                                )));
                                exp = (String::new(), 0);
                            }
                            i += 1;
                            continue 'main;
                        }
                        else
                        {
                            place_multiplier(&mut output, sumrec);
                            if neg
                            {
                                output.push(Num(Number::from(n1.clone(), None)));
                                output.push(Str('×'.to_string()));
                                neg = false;
                            }
                            let nobrackets = j != 0
                                && chars[j - 1] == ','
                                && i + 1 != chars.len()
                                && chars[i + 1] == ',';
                            if !nobrackets
                            {
                                output.push(Str('('.to_string()));
                            }
                            let mut temp = &chars[j + countj + 1..i + 1];
                            if temp.ends_with(&[')'])
                            {
                                temp = &temp[..temp.len().saturating_sub(1)];
                            }
                            let l = var.name[var.name.iter().position(|c| c == &'(').unwrap() + 1
                                ..var.name.len().saturating_sub(1)]
                                .iter()
                                .collect::<String>();
                            let mut parsed = var.parsed.clone();
                            let mut fvs = var.funcvars.clone();
                            let mut k = 0;
                            let mut num = if let Ok(n) = Complex::parse_radix(
                                temp.iter().collect::<String>(),
                                options.base.0,
                            )
                            {
                                vec![Num(Number::from(n.complete(prec), None))]
                            }
                            else
                            {
                                let parsed;
                                let exit;
                                let func;
                                let tempgraph;
                                let sum_var;
                                let mut cv = collectvars.clone();
                                if !cv.is_empty()
                                {
                                    if cv[0].0 < 0
                                    {
                                        cv[0].0 -= 1;
                                    }
                                    else
                                    {
                                        cv[0].0 = -1
                                    }
                                }
                                (parsed, func, tempgraph, exit, sum_var) = match input_var(
                                    &temp.iter().collect::<String>(),
                                    vars,
                                    sumrec,
                                    bracket,
                                    options,
                                    print,
                                    depth + 1,
                                    blacklist.clone(),
                                    false,
                                    &mut cv,
                                )
                                {
                                    Ok(f) => f,
                                    Err(s) =>
                                    {
                                        err = s;
                                        continue;
                                    }
                                };
                                if tempgraph.graph
                                {
                                    graph.graph = true
                                }
                                if tempgraph.x
                                {
                                    graph.x = true
                                }
                                if tempgraph.y
                                {
                                    graph.y = true
                                }
                                if exit
                                {
                                    return Ok((
                                        Vec::new(),
                                        Vec::new(),
                                        HowGraphing::default(),
                                        true,
                                        None,
                                    ));
                                }
                                if let Some(s) = sum_var
                                {
                                    if collectvars[0].0 < 0
                                    {
                                        sumvar = Some(s)
                                    }
                                    else
                                    {
                                        output.insert(collectvars[0].1, Str(",".to_string()));
                                        output.insert(collectvars[0].1, Str(s.clone()));
                                        collectvars.remove(0);
                                    }
                                }
                                if (tempgraph.graph && parsed.len() > 1)
                                    || print
                                    || sumrec.iter().any(|c| c.0 == -1)
                                    || parsed.iter().any(|c| {
                                        if let Str(s) = c
                                        {
                                            sumrec.iter().any(|r| &r.1 == s)
                                        }
                                        else
                                        {
                                            false
                                        }
                                    })
                                    || func.iter().any(|c| {
                                        c.1.iter().any(|c| {
                                            if let Str(s) = c
                                            {
                                                sumrec.iter().any(|r| &r.1 == s)
                                            }
                                            else
                                            {
                                                false
                                            }
                                        })
                                    })
                                {
                                    let iden = format!("@{}{}{}{}@", i, l, depth, vars.len());
                                    if parsed.len() == 1
                                    {
                                        parsed
                                    }
                                    else
                                    {
                                        funcvars.extend(func);
                                        funcvars.push((iden.clone(), parsed));
                                        vec![Str(iden)]
                                    }
                                }
                                else
                                {
                                    vec![match do_math(parsed, options, func)
                                    {
                                        Ok(f) => f,
                                        Err(s) =>
                                        {
                                            err = s;
                                            continue;
                                        }
                                    }]
                                }
                            };
                            if abs
                            {
                                num.insert(0, Str("(".to_string()));
                                num.insert(0, Str("norm".to_string()));
                                num.insert(0, Str("(".to_string()));
                                num.push(Str(")".to_string()));
                                num.push(Str(")".to_string()))
                            }
                            if print
                                && num.len() == 1
                                && if let Num(n) = num[0].clone()
                                {
                                    n.number.real().is_sign_negative()
                                }
                                else
                                {
                                    false
                                }
                            {
                                num.insert(0, Str("(".to_string()));
                                num.push(Str(")".to_string()));
                            }
                            while k < parsed.len()
                            {
                                if parsed[k].str_is(&l)
                                {
                                    parsed.remove(k);
                                    if num.len() == 1
                                    {
                                        parsed.insert(k, num[0].clone());
                                    }
                                    else
                                    {
                                        parsed.splice(k..k, num.clone());
                                        k += num.len();
                                        continue;
                                    }
                                }
                                k += 1;
                            }
                            for fv in fvs.iter_mut()
                            {
                                if !fv.0.ends_with(')')
                                {
                                    k = fv.1.len();
                                    while k != 0
                                    {
                                        k -= 1;
                                        if fv.1[k].str_is(&l)
                                        {
                                            fv.1.remove(k);
                                            if num.len() == 1
                                            {
                                                fv.1.insert(k, num[0].clone());
                                            }
                                            else
                                            {
                                                fv.1.splice(k..k, num.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            let mut k = 0;
                            while k < parsed.len()
                            {
                                for fv in &fvs
                                {
                                    if parsed[k].str_is(&fv.0)
                                    {
                                        if !fv.0.ends_with(')')
                                        {
                                            parsed[k] = Str(format!("@{}@{}{}", i, depth, fv.0));
                                        }
                                        else if !fv.0.starts_with('@')
                                        {
                                            parsed[k] = Str(format!("@{}", fv.0));
                                        }
                                    }
                                }
                                k += 1;
                            }
                            for (x, fv) in fvs.clone().iter().enumerate()
                            {
                                k = fv.1.len();
                                while k != 0
                                {
                                    k -= 1;
                                    for fc in fvs.clone()
                                    {
                                        if let Str(s) = &fv.1[k]
                                        {
                                            if s == &fc.0 && s != &fv.0
                                            {
                                                if !fc.0.contains('(')
                                                {
                                                    fvs[x].1[k] =
                                                        Str(format!("@{}@{}{}", i, depth, fc.0))
                                                }
                                                else if !fc.0.starts_with('@')
                                                {
                                                    fvs[x].1[k] = Str(format!("@{}", fc.0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            for (x, fv) in fvs.clone().iter().enumerate()
                            {
                                if !fv.0.ends_with(')')
                                {
                                    fvs[x].0 = format!("@{}@{}{}", i, depth, fv.0);
                                }
                                else if !fv.0.starts_with('@')
                                {
                                    fvs[x].0 = format!("@{}", fv.0);
                                }
                            }
                            funcvars.extend(fvs);
                            output.extend(parsed);
                            if pwr.1 == *bracket + 1
                            {
                                for _ in 0..pwr.2
                                {
                                    output.push(Str(')'.to_string()))
                                }
                                pwr = (false, 0, 0);
                            }
                            if subfact.1 == *bracket + 1
                            {
                                subfact = (false, 0);
                                output.push(Str(")".to_string()));
                                output.push(Str(")".to_string()))
                            }
                            if !nobrackets
                            {
                                output.push(Str(")".to_string()));
                            }
                            if !exp.0.is_empty() && exp.1 == *bracket
                            {
                                output.push(Str("^".to_string()));
                                output.push(Num(Number::from(
                                    match Complex::parse_radix(exp.0.as_bytes(), options.base.0)
                                    {
                                        Ok(n) => n.complete(prec),
                                        _ => return Err("exponent error"),
                                    },
                                    None,
                                )));
                                exp = (String::new(), 0);
                            }
                            i += 1;
                            continue 'main;
                        }
                    }
                    else if i + var.name.len() <= chars.len()
                        && (chars[i..i + var.name.len()] == var.name
                            || (wordv != chars[i..i + var.name.len()].iter().collect::<String>()
                                && wordv.starts_with(&var.name.iter().collect::<String>())))
                    {
                        if blacklist == var.name
                        {
                            return Err("recursive");
                        }
                        i += if chars[i..i + var.name.len()].contains(&'@')
                            && !var.name.contains(&'@')
                        {
                            let mut count = 0;
                            let mut countw = 0;
                            let mut depth = false;
                            let mut word = String::new();
                            for c in chars[i..].iter()
                            {
                                if word == var.name.iter().collect::<String>()
                                {
                                    if depth
                                    {
                                        count += chars[i + count..]
                                            .iter()
                                            .position(|a| a == &'@')
                                            .unwrap()
                                            + 1;
                                    }
                                    break;
                                }
                                if c == &'@'
                                {
                                    depth = !depth;
                                }
                                else if c == &var.name[countw]
                                {
                                    word.push(*c);
                                    countw += 1;
                                }
                                else if !depth
                                {
                                    i += 1;
                                    continue 'main;
                                }
                                count += 1;
                            }
                            count
                        }
                        else
                        {
                            var.name.len()
                        };
                        place_multiplier(&mut output, sumrec);
                        if neg
                        {
                            output.push(Num(Number::from(n1.clone(), None)));
                            output.push(Str('×'.to_string()));
                            neg = false;
                        }
                        let print = print
                            && if let Num(n) = var.parsed[0].clone()
                            {
                                n.number.real().is_sign_negative()
                            }
                            else
                            {
                                false
                            };
                        if print
                        {
                            output.push(Str("(".to_string()));
                        }
                        if !var.parsed.is_empty()
                        {
                            output.push(var.parsed[0].clone());
                        }
                        else
                        {
                            return Err("bad input2");
                        }
                        if print
                        {
                            output.push(Str(")".to_string()));
                        }
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0
                            && pwr.1 == *bracket
                            && (chars.len() <= i + 1 || chars[i + 1] != '^')
                        {
                            for _ in 0..pwr.2
                            {
                                output.push(Str(')'.to_string()))
                            }
                            pwr.0 = false;
                            pwr.2 = 0
                        }
                        if subfact.0 && subfact.1 == 0
                        {
                            subfact.0 = false;
                            output.push(Str(")".to_string()));
                            output.push(Str(")".to_string()))
                        }
                        continue 'main;
                    }
                }
            }
            if (i == 0 || chars[i - 1] != ' ' || c != ' ')
                && (if options.notation == SmallEngineering
                {
                    matches!(c, 'x' | 'y' | 'z' | 'i' | 'e')
                }
                else
                {
                    matches!(c, 'x' | 'y' | 'z' | 'i' | 'E')
                } || !c.is_alphabetic())
            {
                if neg
                {
                    output.push(Num(Number::from(n1.clone(), None)));
                    output.push(Str('×'.to_string()));
                    neg = false;
                }
                match c
                {
                    'ⁱ' => pow.push('i'),
                    'E' | 'e'
                        if (options.notation == SmallEngineering && c == 'e')
                            || (options.notation != SmallEngineering && c == 'E') =>
                    {
                        if let Some(last) = output.last()
                        {
                            if last.num().is_ok() || last.str_is("x") || last.str_is("y")
                            {
                                output.insert(output.len().saturating_sub(1), Str("(".to_string()));
                                if i + 1 != chars.len()
                                    && (matches!(chars[i + 1], '-' | '+' | 'x' | 'y' | 'z' | 'i')
                                        || is_digit(chars[i + 1], options.base.0))
                                {
                                    scientific = true;
                                }
                            }
                        }
                        place_multiplier(&mut output, sumrec);
                        output.push(Num(Number::from(
                            Complex::with_val(options.prec, options.base.0),
                            None,
                        )));
                        if i + 1 != chars.len()
                            && (chars[i + 1].is_alphanumeric()
                                || is_digit(chars[i + 1], options.base.0)
                                || matches!(chars[i + 1], '-' | '+' | '(' | '{' | '|'))
                        {
                            output.push(Str('^'.to_string()));
                        }
                        if !(i + 1 != chars.len()
                            && (matches!(chars[i + 1], '-' | '+' | 'x' | 'y' | 'z' | 'i')
                                || is_digit(chars[i + 1], options.base.0)))
                        {
                            output.push(Str(")".to_string()));
                        }
                    }
                    'x' | 'y' if collectvars.is_empty() && options.graphtype != GraphType::None =>
                    {
                        graph.graph = true;
                        if c == 'x'
                        {
                            graph.x = true
                        }
                        if c == 'y'
                        {
                            graph.y = true
                        }
                        place_multiplier(&mut output, sumrec);
                        output.push(Str(c.to_string()));
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0
                            && pwr.1 == *bracket
                            && (chars.len() <= i + 1 || chars[i + 1] != '^')
                        {
                            for _ in 0..pwr.2
                            {
                                output.push(Str(')'.to_string()))
                            }
                            pwr.0 = false;
                            pwr.2 = 0
                        }
                        if subfact.0 && subfact.1 == 0
                        {
                            subfact.0 = false;
                            output.push(Str(")".to_string()));
                            output.push(Str(")".to_string()))
                        }
                    }
                    'i' =>
                    {
                        place_multiplier(&mut output, sumrec);
                        output.push(Num(Number::from(
                            Complex::with_val(options.prec, (0, 1)),
                            None,
                        )));
                        if pwr.0
                            && pwr.1 == *bracket
                            && (chars.len() <= i + 1 || chars[i + 1] != '^')
                        {
                            for _ in 0..pwr.2
                            {
                                output.push(Str(')'.to_string()))
                            }
                            pwr.0 = false;
                            pwr.2 = 0
                        }
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if subfact.0 && subfact.1 == 0
                        {
                            subfact.0 = false;
                            output.push(Str(")".to_string()));
                            output.push(Str(")".to_string()))
                        }
                    }
                    'z' if collectvars.is_empty() && options.graphtype != GraphType::None =>
                    {
                        graph.graph = true;
                        graph.x = true;
                        graph.y = true;
                        place_multiplier(&mut output, sumrec);
                        output.push(Str('('.to_string()));
                        output.push(Str('x'.to_string()));
                        output.push(Str('+'.to_string()));
                        output.push(Str('y'.to_string()));
                        output.push(Str('*'.to_string()));
                        output.push(Num(Number::from(
                            Complex::with_val(options.prec, (0, 1)),
                            None,
                        )));
                        output.push(Str(')'.to_string()));
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0
                            && pwr.1 == *bracket
                            && (chars.len() <= i + 1 || chars[i + 1] != '^')
                        {
                            for _ in 0..pwr.2
                            {
                                output.push(Str(')'.to_string()))
                            }
                            pwr.0 = false;
                            pwr.2 = 0
                        }
                        if subfact.0 && subfact.1 == 0
                        {
                            subfact.0 = false;
                            output.push(Str(")".to_string()));
                            output.push(Str(")".to_string()))
                        }
                    }
                    _ =>
                    {
                        if !collectvars.is_empty()
                        {
                            if neg
                            {
                                output.push(Num(Number::from(n1.clone(), None)));
                                output.push(Str('×'.to_string()));
                                neg = false;
                            }
                            sumrec.push((collectvars[0].0, wordv.clone()));
                            if collectvars[0].0 < 0
                            {
                                sumvar = Some(wordv.clone());
                            }
                            else
                            {
                                output.insert(collectvars[0].1, Str(",".to_string()));
                                output.insert(collectvars[0].1, Str(wordv.clone()));
                            }
                            place_multiplier(&mut output, sumrec);
                            output.push(Str(wordv));
                            collectvars.remove(0);
                        }
                    }
                }
            }
            else if !collectvars.is_empty()
            {
                if neg
                {
                    output.push(Num(Number::from(n1.clone(), None)));
                    output.push(Str('×'.to_string()));
                    neg = false;
                }
                sumrec.push((collectvars[0].0, word.clone()));
                if collectvars[0].0 < 0
                {
                    sumvar = Some(word.clone());
                }
                else
                {
                    output.insert(collectvars[0].1, Str(",".to_string()));
                    output.insert(collectvars[0].1, Str(word.clone()));
                }
                place_multiplier(&mut output, sumrec);
                output.push(Str(word));
                collectvars.remove(0);
            }
            else
            {
                undf = true
            }
            funcfailed = false;
            absfailed = false;
            i += 1;
        }
    }
    for _ in 0..pwr.2 + ceilfoor
    {
        output.push(Str(')'.to_string()))
    }
    if !pow.is_empty()
    {
        let i = pow.matches('i').count() % 4;
        pow = pow.replace('i', "");
        if pow.is_empty()
        {
            pow = "1".to_string();
        }
        match output.last()
        {
            Some(Num(_)) | Some(Vector(_)) | Some(Matrix(_)) => output.push(Str('^'.to_string())),
            Some(Str(s))
                if matches!(s.as_str(), "x" | "y" | "rnd" | "rand" | "epoch")
                    || sumrec.iter().any(|v| &v.1 == s) =>
            {
                output.push(Str('^'.to_string()))
            }
            _ =>
            {}
        }
        output.push(Num(Number::from(
            match Complex::parse_radix(pow.as_bytes(), options.base.0)
            {
                Ok(n) => n.complete(prec),
                _ => return Err("exponent error"),
            } * Complex::with_val(options.prec, (0, 1)).pow(Complex::with_val(options.prec, i)),
            None,
        )));
    }
    if !exp.0.is_empty()
    {
        output.push(Str("^".to_string()));
        output.push(Num(Number::from(
            match Complex::parse_radix(exp.0.as_bytes(), options.base.0)
            {
                Ok(n) => n.complete(prec),
                _ => return Err("exponent error"),
            },
            None,
        )));
    }
    if neg
    {
        output.push(Num(Number::from(n1, None)));
    }
    for _ in abs
    {
        output.push(Str(")".to_string()));
        output.push(Str(")".to_string()));
    }
    let mut count = 0;
    i = 0;
    let mut double = Vec::new();
    let mut brackets = Vec::new();
    while i < output.len()
    {
        if let Str(s) = &output[i]
        {
            match s.as_str()
            {
                "(" if i + 1 < output.len() =>
                {
                    count += 1;
                    if output[i + 1].str_is(")")
                    {
                        output.remove(i + 1);
                        output.remove(i);
                        i = i.saturating_sub(1);
                        count -= 1;
                        continue;
                    }
                    else if output[i + 1].str_is("(")
                    {
                        double.push((i, count));
                    }
                    if i == 0 || output[i - 1].str_is(",")
                    {
                        brackets.push((i, count));
                    }
                }
                ")" =>
                {
                    count -= 1;
                    if let Some(d) = double.last()
                    {
                        if d.1 == count
                        {
                            if output.len() > i + 1 && output[i + 1].str_is(")")
                            {
                                output.remove(i);
                                output.remove(d.0);
                                i -= 2;
                            }
                            double.pop();
                        }
                    }
                    if let Some(d) = brackets.last()
                    {
                        if d.1 == count + 1
                        {
                            if i == output.len() - 1
                            {
                                output.remove(i);
                                output.remove(d.0);
                                i -= 2;
                            }
                            if i + 1 == output.len() || !output[i + 1].str_is(",")
                            {
                                brackets.pop();
                            }
                        }
                    }
                }
                "," =>
                {
                    if let Some(d) = brackets.last()
                    {
                        if d.1 == count + 1
                        {
                            output.remove(i - 1);
                            output.remove(d.0);
                            i -= 2;
                            brackets.pop();
                        }
                    }
                }
                _ if (print
                    || (i + 1 < output.len()
                        && (output[i + 1].str_is("rnd")
                            || output[i + 1].str_is("rand")
                            || output[i + 1].str_is("epoch"))))
                    && functions.contains(s.as_str())
                    && !sumrec.iter().any(|a| a.1 == *s)
                    && i + 1 < output.len() =>
                {
                    if let Str(s) = &output[i + 1]
                    {
                        if !matches!(s.as_str(), "(" | "{" | "[" | "|")
                        {
                            output.insert(i + 2, Str(")".to_string()));
                            output.insert(i + 1, Str("(".to_string()));
                            i += 1;
                        }
                    }
                }
                _ =>
                {}
            }
        }
        i += 1;
    }
    if !err.is_empty()
    {
        return Err(err);
    }
    if isgraphing && (graph.x || graph.y)
    {
        i = output.len();
        count = 0;
        let mut to = 0;
        while i != 0
        {
            i -= 1;
            if let Str(s) = &output[i]
            {
                match s.as_str()
                {
                    "x" | "y" | "roll" | "rnd" | "rand" | "epoch" =>
                    {
                        to = 0;
                    }
                    "(" =>
                    {
                        count -= 1;
                        if count == 0 && to != 0
                        {
                            if let Ok(n) =
                                do_math(output[i + 1..to].to_vec(), options, funcvars.clone())
                            {
                                output.drain(i..=to);
                                output.insert(i, n);
                            }
                            to = 0;
                        }
                    }
                    ")" =>
                    {
                        if count == 0
                        {
                            to = i;
                        }
                        count += 1;
                    }
                    _ =>
                    {}
                }
            }
        }
        i = 0;
        while i < funcvars.len()
        {
            let v = funcvars[i].clone();
            if (v.1.len() != 1
                || (if let Str(s) = &v.1[0]
                {
                    matches!(s.as_str(), "rnd" | "rand" | "epoch")
                }
                else
                {
                    false
                }))
                && !v.0.ends_with(')')
            {
                if let Ok(n) = do_math(v.1.clone(), options, funcvars[..i].to_vec())
                {
                    for f in output.iter_mut()
                    {
                        if let Str(s) = &f
                        {
                            if *s == v.0
                            {
                                *f = n.clone();
                            }
                        }
                    }
                    if i + 1 < funcvars.len()
                    {
                        for fv in funcvars[i + 1..].iter_mut()
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
                    funcvars.remove(i);
                    continue;
                }
            }
            i += 1;
        }
    }
    while let Some(Str(s)) = output.last()
    {
        if matches!(
            s.as_str(),
            "*" | "^" | "^^" | "/" | "//" | "+" | "-" | "±" | "×"
        ) || functions.contains(s.as_str())
        {
            output.pop();
        }
        else
        {
            break;
        }
    }
    if undf
    {
        return Err("undefined var");
    }
    if options.graphtype == GraphType::None
    {
        graph = HowGraphing::default()
    }
    Ok((output, funcvars, graph, false, sumvar))
}
fn place_multiplier(output: &mut Vec<NumStr>, sumrec: &[(isize, String)])
{
    match output.last()
    {
        Some(Str(s))
            if matches!(
                s.as_str(),
                ")" | "x" | "y" | "]" | "}" | "rnd" | "rand" | "epoch" | "@"
            ) || sumrec
                .iter()
                .any(|a| a.1 == *s || "@".to_owned() + &a.1 == *s) =>
        {
            output.push(Str('*'.to_string()))
        }
        Some(Num(_)) | Some(Vector(_)) | Some(Matrix(_)) => output.push(Str('*'.to_string())),
        _ =>
        {}
    }
}
fn can_abs(output: &[NumStr], vars: &[Variable]) -> bool
{
    if let Some(Str(s)) = output.last()
    {
        !(functions().contains(s.as_str())
            || vars.iter().any(|c| c.name.iter().collect::<String>() == *s))
    }
    else
    {
        true
    }
}
fn is_digit(char: char, base: i32) -> bool
{
    char.is_ascii_digit() || (base > 10 && (97..=97 + (base as u8 - 11)).contains(&(char as u8)))
}
