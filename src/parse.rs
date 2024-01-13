use crate::{
    complex::{
        NumStr,
        NumStr::{Num, Str},
    },
    functions::functions,
    Options,
};
use rug::{
    float::Special::{Infinity, Nan},
    ops::{CompleteRound, Pow},
    Complex,
};
#[allow(clippy::too_many_arguments)]
pub fn input_var(
    input: &str,
    vars: Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>,
    sumrec: &mut Vec<(isize, String)>,
    bracket: &mut isize,
    options: Options,
    mut graph: bool,
    pwr: &mut (bool, isize, isize),
    print: bool,
) -> Result<(Vec<NumStr>, bool), &'static str>
{
    if input.starts_with("history") || input.is_empty()
    {
        return Err(" ");
    }
    let mut scientific = false;
    let mut abs = Vec::new();
    let mut neg = false;
    let n1 = Complex::with_val(options.prec, -1);
    let mut pow = String::new();
    let mut exp = (String::new(), 0);
    let mut subfact = (false, 0);
    let mut chars = input
        .replace('[', "(car{")
        .replace(']', "})")
        .chars()
        .collect::<Vec<char>>();
    let mut output: Vec<NumStr> = Vec::new();
    let mut stack_end = Vec::new();
    let mut stack_start = Vec::new();
    let mut i = 0;
    while !chars.is_empty() && chars[0].is_whitespace()
    {
        chars.remove(0);
    }
    while i < chars.len()
    {
        if chars[i].is_whitespace()
        {
            if chars.len() - 1 == i
            {
                chars.remove(i);
            }
            else if chars[i - 1].is_alphanumeric() && chars[i + 1].is_alphanumeric()
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
        i += 1;
    }
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
    chars.extend(stack_end);
    for i in stack_start
    {
        chars.insert(0, i);
    }
    i = 0;
    let mut sum = (0, String::new());
    let functions = functions();
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
            if !output.is_empty()
                && (output.last().unwrap().num().is_ok() || output.last().unwrap().str_is(")"))
            {
                output.push(Str('^'.to_string()));
            }
            output.push(Num(Complex::with_val(
                options.prec,
                match Complex::parse(pow.as_bytes())
                {
                    Ok(n) => n,
                    _ => return Err("exponent error"),
                },
            ) * Complex::with_val(options.prec, (0, 1))
                .pow(Complex::with_val(options.prec, i))));
            pow = String::new();
        }
        if c == '.' && i + 1 < chars.len() && chars[i + 1] == '.'
        {
            output.push(Str("..".to_string()));
            i += 2;
            continue;
        }
        if c.is_ascii_digit() || c == '.'
        {
            let mut num = String::new();
            let mut dot = false;
            while i < chars.len()
            {
                if chars[i].is_ascii_digit()
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
            place_multiplier(
                &mut output,
                sumrec,
                if print { Some(vars.clone()) } else { None },
            );
            if neg
            {
                if chars.len() > i + 1
                    && (chars[i] == '^' || (chars[i] == '/' && chars[i + 1] == '/'))
                {
                    output.push(Num(n1.clone()));
                    output.push(Str('*'.to_string()));
                }
                else
                {
                    num.insert(0, '-');
                }
                neg = false;
            }
            output.push(Num(Complex::parse(num.clone())
                .unwrap()
                .complete(options.prec)));
            if scientific
            {
                output.push(Str(")".to_string()));
                scientific = false;
            }
            if pwr.0 && pwr.1 == 0 && (chars.len() <= i || chars[i] != '^')
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
        if !c.is_alphabetic() && c != '@'
        {
            if !output.is_empty()
            {
                if let Str(s) = output.last_mut().unwrap()
                {
                    if functions.contains(s.as_str())
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
                        if i + 1 < chars.len()
                            && chars[i] == '^'
                            && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '-')
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
                '¼' => output.push(Num(Complex::with_val(options.prec, 0.25))),
                '½' => output.push(Num(Complex::with_val(options.prec, 0.5))),
                '¾' => output.push(Num(Complex::with_val(options.prec, 0.75))),
                '⅒' => output.push(Num(Complex::with_val(options.prec, 0.1))),
                '⅕' => output.push(Num(Complex::with_val(options.prec, 0.2))),
                '⅖' => output.push(Num(Complex::with_val(options.prec, 0.4))),
                '⅗' => output.push(Num(Complex::with_val(options.prec, 0.6))),
                '⅘' => output.push(Num(Complex::with_val(options.prec, 0.8))),
                '⅐' => output.push(Num(Complex::with_val(options.prec, 7).recip())),
                '⅑' => output.push(Num(Complex::with_val(options.prec, 9).recip())),
                '⅓' => output.push(Num(Complex::with_val(options.prec, 3).recip())),
                '⅔' => output.push(Num(Complex::with_val(options.prec, 1.5).recip())),
                '⅙' => output.push(Num(Complex::with_val(options.prec, 6).recip())),
                '⅚' => output.push(Num(Complex::with_val(options.prec, 1.2).recip())),
                '⅛' => output.push(Num(Complex::with_val(options.prec, 0.125))),
                '⅜' => output.push(Num(Complex::with_val(options.prec, 0.375))),
                '⅝' => output.push(Num(Complex::with_val(options.prec, 0.625))),
                '⅞' => output.push(Num(Complex::with_val(options.prec, 0.875))),
                '⅟' =>
                {
                    output.push(Num(Complex::with_val(options.prec, 1)));
                    output.push(Str("/".to_string()))
                }
                '↉' => output.push(Num(Complex::new(options.prec))),
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
                }
                '{' =>
                {
                    place_multiplier(
                        &mut output,
                        sumrec,
                        if print { Some(vars.clone()) } else { None },
                    );
                    if neg
                    {
                        output.push(Num(n1.clone()));
                        output.push(Str('*'.to_string()));
                        neg = false;
                    }
                    output.push(Str("{".to_string()));
                }
                '}' =>
                {
                    output.push(Str("}".to_string()));
                }
                '±' if i + 1 != chars.len() && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if output.is_empty()
                        || matches!(output.last().unwrap(),Str(s) if s==","||s=="{"||s=="(")
                    {
                        output.push(Num(Complex::new(options.prec)))
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
                    else
                    {
                        output.push(Str('/'.to_string()));
                    }
                }
                '+' if i != 0
                    && i + 1 != chars.len()
                    && (chars[i - 1].is_alphanumeric()
                        || (!output.is_empty() && output.last().unwrap().str_is(")"))
                        || matches!(chars[i - 1], '}' | ']' | ')' | '@'))
                    && chars[i - 1] != if options.small_e { 'e' } else { 'E' }
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '-'
                    {
                        if output.is_empty()
                            || matches!(output.last().unwrap(),Str(s) if s==","||s=="{"||s=="(")
                        {
                            output.push(Num(Complex::new(options.prec)))
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
                        output.push(Num(Complex::new(options.prec)))
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
                        output.push(Str("<<".to_string()));
                        i += 1;
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
                        output.push(Str(">>".to_string()));
                        i += 1;
                    }
                    else
                    {
                        output.push(Str('>'.to_string()));
                    }
                }
                '-' if i + 1 < chars.len() && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if i != 0 && chars[i - 1] == '^'
                    {
                        output.push(Str("(".to_string()));
                        output.push(Num(n1.clone()));
                        pwr.0 = true;
                        pwr.2 += 1;
                    }
                    else if i == 0
                        || !(chars[i - 1] != if options.small_e { 'e' } else { 'E' }
                            && (chars[i - 1].is_alphanumeric()
                                || (!output.is_empty() && output.last().unwrap().str_is(")"))
                                || matches!(chars[i - 1], '}' | ']' | ')' | '@')))
                    {
                        if i + 1 != chars.len() && (chars[i + 1] == '(' || chars[i + 1] == '-')
                        {
                            output.push(Num(n1.clone()));
                            output.push(Str("*".to_string()));
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
                        output.push(Str("^".to_string()))
                    }
                }
                '(' if i + 1 != chars.len() =>
                {
                    *bracket += 1;
                    if pwr.0
                    {
                        pwr.1 = *bracket;
                    }
                    if subfact.0
                    {
                        subfact.1 = *bracket;
                    }
                    place_multiplier(
                        &mut output,
                        sumrec,
                        if print { Some(vars.clone()) } else { None },
                    );
                    output.push(Str("(".to_string()))
                }
                ')' if i != 0 =>
                {
                    if pwr.1 == *bracket
                    {
                        for _ in 0..pwr.2
                        {
                            output.push(Str(')'.to_string()))
                        }
                        *pwr = (false, 0, 0);
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
                        output.push(Num(Complex::with_val(
                            options.prec,
                            match Complex::parse(exp.0.as_bytes())
                            {
                                Ok(n) => n,
                                _ => return Err("exponent error"),
                            },
                        )));
                        exp = (String::new(), 0);
                    }
                }
                '|' =>
                {
                    if !abs.is_empty() && abs[0] == *bracket && can_abs(&output)
                    {
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
                    else
                    {
                        place_multiplier(
                            &mut output,
                            sumrec,
                            if print { Some(vars.clone()) } else { None },
                        );
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
                    }
                    else if i != 0
                        && (chars[i - 1].is_alphanumeric()
                            || (!output.is_empty()
                                && (output.last().unwrap().str_is(")")
                                    || output.last().unwrap().str_is("}"))))
                    {
                        if let Num(a) = output.clone().last().unwrap()
                        {
                            if a.real().is_sign_negative()
                            {
                                output.pop();
                                output.push(Num(Complex::with_val(
                                    options.prec,
                                    (-a.real(), a.imag()),
                                )));
                                output.insert(output.len() - 1, Num(n1.clone()));
                                output.insert(output.len() - 1, Str("*".to_string()));
                            }
                        }
                        if output.clone().last().unwrap().str_is(")")
                            || output.last().unwrap().str_is("}")
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
                                            if s.chars().next().unwrap().is_alphabetic()
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
                        output.insert(output.len() - 1, Str("(".to_string()));
                        if i + 1 != chars.len() && chars[i + 1] == '!'
                        {
                            i += 1;
                            output.insert(output.len() - 1, Str("doublefact".to_string()));
                        }
                        else
                        {
                            output.insert(output.len() - 1, Str("fact".to_string()));
                        }
                        output.insert(output.len() - 1, Str("(".to_string()));
                        output.push(Str(")".to_string()));
                        output.push(Str(")".to_string()));
                    }
                    else if i != chars.len() - 1
                        && (chars[i + 1].is_alphanumeric()
                            || chars[i + 1] == '('
                            || chars[i + 1] == '{'
                            || chars[i + 1] == '|'
                            || chars[i + 1] == '-'
                            || chars[i + 1] == '!')
                    {
                        if neg
                        {
                            output.push(Num(n1.clone()));
                            output.push(Str("*".to_string()));
                            neg = false;
                        }
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
                            sumrec.remove(i);
                            break;
                        }
                    }
                    output.push(Str(','.to_string()))
                }
                '%' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    output.push(Str('%'.to_string()))
                }
                '∞' => output.push(Num(Complex::with_val(options.prec, Infinity))),
                '#' =>
                {
                    graph = true;
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
            else if c.is_alphabetic() || c == &'\'' || c == &'`'
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
        let countj = countv;
        if (word.ends_with('x') && word != "max")
            || (word.ends_with('y') && word != "any")
            || word.ends_with('z')
        {
            countv -= 1;
            word.pop();
        }
        if matches!(
            word.as_str(),
            "sum" | "summation" | "prod" | "production" | "vec" | "mat" | "Σ" | "Π"
        ) && chars.len() > i + countv + 1
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
            if place > 0
            {
                sum.0 = *bracket + 1;
                sum.1 = String::new();
                let mut count = 0;
                for c in chars[i + countv + 1..].iter()
                {
                    count += 1;
                    if c.is_alphabetic()
                    {
                        sum.1.push(*c);
                    }
                    else if c == &','
                    {
                        break;
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
                    place_multiplier(
                        &mut output,
                        sumrec,
                        if print { Some(vars.clone()) } else { None },
                    );
                    output.push(Str(word.clone()));
                    output.push(Str("(".to_string()));
                    output.push(Str(sum.1));
                    output.push(Str(",".to_string()));
                    *bracket += 1;
                    i += count + countv + 1;
                    continue;
                }
            }
        }
        if !vars.clone().iter().any(|a| {
            if a.0.contains(&'(')
            {
                a.0[..a.0.iter().position(|c| c == &'(').unwrap()]
                    .iter()
                    .collect::<String>()
                    == word
            }
            else
            {
                a.0.iter().collect::<String>() == word
            }
        }) && ((functions.contains(word.as_str())
            && i + countv < chars.len()
            && matches!(
                chars[i + countv],
                'x' | 'y' | 'z' | '(' | '|' | '[' | '{' | '0'..='9' | '^'
            ))
            || matches!(word.as_str(), "rnd" | "inf" | "nan" | "NaN"))
        {
            place_multiplier(
                &mut output,
                sumrec,
                if print { Some(vars.clone()) } else { None },
            );
            if neg
            {
                output.push(Num(n1.clone()));
                output.push(Str('*'.to_string()));
                neg = false;
            }
            i += countv;
            if matches!(word.as_str(), "inf" | "nan" | "NaN")
            {
                if matches!(word.as_str(), "nan" | "NaN")
                {
                    output.push(Num(Complex::with_val(options.prec, Nan)));
                }
                else
                {
                    output.push(Num(Complex::with_val(options.prec, Infinity)));
                }
                if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
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
                output.push(Str(word))
            }
        }
        else if sumrec.iter().any(|a| {
            if wordv.starts_with(&a.1)
            {
                word = a.1.clone();
                true
            }
            else
            {
                false
            }
        })
        {
            place_multiplier(
                &mut output,
                sumrec,
                if print { Some(vars.clone()) } else { None },
            );
            if neg
            {
                output.push(Num(n1.clone()));
                output.push(Str('*'.to_string()));
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
            output.push(Str(word));
            if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
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
            for var in vars.clone()
            {
                if var.0 != vec!['e']
                    || (!options.small_e
                        || !(i != 0
                            && i + 1 != chars.len()
                            && chars[i - 1].is_numeric()
                            && (chars[i + 1].is_numeric() || chars[i + 1] == '-')))
                {
                    let j = i;
                    if var.0.contains(&'(')
                        && wordv
                            == var
                                .0
                                .split(|c| matches!(c, '(' | '{' | '['))
                                .next()
                                .unwrap()
                                .iter()
                                .collect::<String>()
                        && i + countj < chars.len()
                        && matches!(chars[i + countj], '(' | '{' | '[')
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
                            i = chars.len() - 1
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
                            else if *c == '(' || c == &'{' || c == &'['
                            {
                                count += 1;
                            }
                            else if *c == ')' || c == &'}' || c == &']'
                            {
                                count -= 1;
                            }
                        }
                        if ccount != var.0.iter().filter(|c| c == &&',').count()
                        {
                            i = j;
                            continue;
                        }
                        if var.0.contains(&',') && chars.len() > 4
                        {
                            place_multiplier(
                                &mut output,
                                sumrec,
                                if print { Some(vars.clone()) } else { None },
                            );
                            if neg
                            {
                                output.push(Num(n1.clone()));
                                output.push(Str('*'.to_string()));
                                neg = false;
                            }
                            output.push(Str('('.to_string()));
                            let mut temp = &chars[j + countj + 1..i + 1];
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
                            for (f, c) in var.0.iter().enumerate()
                            {
                                if c == &'(' || c == &'{' || c == &'['
                                {
                                    if count == 0
                                    {
                                        start = f + 1;
                                    }
                                    count += 1;
                                }
                                else if c == &')' || c == &'}' || c == &']'
                                {
                                    count -= 1;
                                    if count == 0
                                    {
                                        func_vars.push(var.0[start..f].iter().collect());
                                    }
                                }
                                else if c == &',' && count == 1
                                {
                                    func_vars.push(var.0[start..f].iter().collect());
                                    start = f + 1;
                                }
                            }
                            let mut var = var;
                            for (varf, func_var) in split.iter().zip(func_vars)
                            {
                                let mut k = 0;
                                while k < var.1.len()
                                {
                                    if var.1[k].str_is(&func_var)
                                    {
                                        var.1.remove(k);
                                        if let Ok(n) =
                                            Complex::parse(varf.iter().collect::<String>())
                                        {
                                            var.1.insert(k, Num(n.complete(options.prec)))
                                        }
                                        else
                                        {
                                            let mut parsed;
                                            (parsed, graph) = input_var(
                                                &varf.iter().collect::<String>(),
                                                vars.clone(),
                                                sumrec,
                                                bracket,
                                                options,
                                                graph,
                                                pwr,
                                                print,
                                            )?;
                                            let j = k;
                                            if parsed.len() > 1
                                            {
                                                parsed.insert(0, Str('('.to_string()));
                                                parsed.push(Str(')'.to_string()));
                                                k += parsed.len() - 2;
                                            }
                                            else
                                            {
                                                k += parsed.len();
                                            }
                                            var.1.splice(j..j, parsed);
                                        }
                                    }
                                    k += 1;
                                }
                            }
                            output.extend(var.1);
                            if pwr.1 == *bracket + 1
                            {
                                for _ in 0..pwr.2
                                {
                                    output.push(Str(')'.to_string()))
                                }
                                *pwr = (false, 0, 0);
                            }
                            if subfact.1 == *bracket + 1
                            {
                                subfact = (false, 0);
                                output.push(Str(")".to_string()));
                                output.push(Str(")".to_string()))
                            }
                            output.push(Str(")".to_string()));
                            if !exp.0.is_empty() && exp.1 == *bracket
                            {
                                output.push(Str("^".to_string()));
                                output.push(Num(Complex::with_val(
                                    options.prec,
                                    match Complex::parse(exp.0.as_bytes())
                                    {
                                        Ok(n) => n,
                                        _ => return Err("exponent error"),
                                    },
                                )));
                                exp = (String::new(), 0);
                            }
                            i += 1;
                            continue 'main;
                        }
                        else
                        {
                            place_multiplier(
                                &mut output,
                                sumrec,
                                if print { Some(vars.clone()) } else { None },
                            );
                            if neg
                            {
                                output.push(Num(n1.clone()));
                                output.push(Str('*'.to_string()));
                                neg = false;
                            }
                            output.push(Str('('.to_string()));
                            let mut temp = &chars[j + countj + 1..i + 1];
                            if temp.ends_with(&[')'])
                            {
                                temp = &temp[..temp.len() - 1];
                            }
                            let l = var.0[var.0.iter().position(|c| c == &'(').unwrap() + 1
                                ..var.0.len() - 1]
                                .iter()
                                .collect::<String>();
                            let mut var = var;
                            let mut k = 0;
                            while k < var.1.len()
                            {
                                if var.1[k].str_is(&l)
                                {
                                    var.1.remove(k);
                                    if let Ok(n) = Complex::parse(temp.iter().collect::<String>())
                                    {
                                        var.1.insert(k, Num(n.complete(options.prec)))
                                    }
                                    else
                                    {
                                        let mut parsed;
                                        (parsed, graph) = input_var(
                                            &temp.iter().collect::<String>(),
                                            vars.clone(),
                                            sumrec,
                                            bracket,
                                            options,
                                            graph,
                                            pwr,
                                            print,
                                        )?;
                                        let j = k;
                                        if parsed.len() > 1
                                        {
                                            parsed.insert(0, Str('('.to_string()));
                                            parsed.push(Str(')'.to_string()));
                                            k += parsed.len() - 2;
                                        }
                                        else
                                        {
                                            k += parsed.len();
                                        }
                                        var.1.splice(j..j, parsed);
                                    }
                                }
                                k += 1;
                            }
                            output.extend(var.1);
                            if pwr.1 == *bracket + 1
                            {
                                for _ in 0..pwr.2
                                {
                                    output.push(Str(')'.to_string()))
                                }
                                *pwr = (false, 0, 0);
                            }
                            if subfact.1 == *bracket + 1
                            {
                                subfact = (false, 0);
                                output.push(Str(")".to_string()));
                                output.push(Str(")".to_string()))
                            }
                            output.push(Str(")".to_string()));
                            if !exp.0.is_empty() && exp.1 == *bracket
                            {
                                output.push(Str("^".to_string()));
                                output.push(Num(Complex::with_val(
                                    options.prec,
                                    match Complex::parse(exp.0.as_bytes())
                                    {
                                        Ok(n) => n,
                                        _ => return Err("exponent error"),
                                    },
                                )));
                                exp = (String::new(), 0);
                            }
                            i += 1;
                            continue 'main;
                        }
                    }
                    else if i + var.0.len() <= chars.len()
                        && (chars[i..i + var.0.len()] == var.0
                            || (wordv != chars[i..i + var.0.len()].iter().collect::<String>()
                                && wordv.starts_with(&var.0.iter().collect::<String>())))
                    {
                        i += if chars[i..i + var.0.len()].contains(&'@') && !var.0.contains(&'@')
                        {
                            let mut count = 0;
                            let mut countw = 0;
                            let mut depth = false;
                            let mut word = String::new();
                            for c in chars[i..].iter()
                            {
                                if word == var.0.iter().collect::<String>()
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
                                else if c == &var.0[countw]
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
                            var.0.len()
                        };
                        place_multiplier(
                            &mut output,
                            sumrec,
                            if print { Some(vars.clone()) } else { None },
                        );
                        if neg
                        {
                            output.push(Num(n1.clone()));
                            output.push(Str('*'.to_string()));
                            neg = false;
                        }
                        if print
                        {
                            output.push(Str(var.0.iter().collect::<String>()));
                        }
                        else
                        {
                            output.push(var.2);
                        }
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
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
                && (if options.small_e
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
                    output.push(Num(n1.clone()));
                    output.push(Str('*'.to_string()));
                    neg = false;
                }
                match c
                {
                    'ⁱ' => pow.push('i'),
                    'E' | 'e'
                        if (options.small_e && c == 'e') || (!options.small_e && c == 'E') =>
                    {
                        if let Some(last) = output.last()
                        {
                            if last.num().is_ok() || last.str_is("x") || last.str_is("y")
                            {
                                output.insert(output.len() - 1, Str("(".to_string()));
                                if i + 1 != chars.len()
                                    && matches!(
                                        chars[i + 1],
                                        '0'..='9' | '-' | '+' | 'x' | 'y' | 'i'
                                    )
                                {
                                    scientific = true;
                                }
                            }
                        }
                        place_multiplier(
                            &mut output,
                            sumrec,
                            if print { Some(vars.clone()) } else { None },
                        );
                        output.push(Num(Complex::with_val(options.prec, 10)));
                        if i + 1 != chars.len()
                            && (chars[i + 1].is_alphanumeric()
                                || matches!(chars[i + 1], '-' | '+' | '(' | '{' | '|' | '['))
                        {
                            output.push(Str('^'.to_string()));
                        }
                        if !(i + 1 != chars.len()
                            && matches!(chars[i + 1], '0'..='9' | '-' | '+' | 'x' | 'y' | 'i'))
                        {
                            output.push(Str(")".to_string()));
                        }
                    }
                    'x' | 'y' =>
                    {
                        graph = true;
                        place_multiplier(
                            &mut output,
                            sumrec,
                            if print { Some(vars.clone()) } else { None },
                        );
                        output.push(Str(c.to_string()));
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
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
                        place_multiplier(
                            &mut output,
                            sumrec,
                            if print { Some(vars.clone()) } else { None },
                        );
                        output.push(Num(Complex::with_val(options.prec, (0, 1))));
                        if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
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
                    'z' =>
                    {
                        graph = true;
                        place_multiplier(
                            &mut output,
                            sumrec,
                            if print { Some(vars.clone()) } else { None },
                        );
                        output.push(Str('('.to_string()));
                        output.push(Str('x'.to_string()));
                        output.push(Str('+'.to_string()));
                        output.push(Str('y'.to_string()));
                        output.push(Str('*'.to_string()));
                        output.push(Num(Complex::with_val(options.prec, (0, 1))));
                        output.push(Str(')'.to_string()));
                        if scientific
                        {
                            output.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
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
                    {}
                }
            }
            i += 1;
        }
    }
    if !pow.is_empty()
    {
        let i = pow.matches('i').count() % 4;
        pow = pow.replace('i', "");
        if pow.is_empty()
        {
            pow = "1".to_string();
        }
        if !output.is_empty()
            && (output.last().unwrap().num().is_ok() || output.last().unwrap().str_is(")"))
        {
            output.push(Str("^".to_string()));
        }
        output.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(pow.as_bytes())
            {
                Ok(n) => n,
                _ => return Err("exponent error"),
            },
        ) * Complex::with_val(options.prec, (0, 1))
            .pow(Complex::with_val(options.prec, i))));
    }
    if !exp.0.is_empty()
    {
        output.push(Str("^".to_string()));
        output.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(exp.0.as_bytes())
            {
                Ok(n) => n,
                _ => return Err("exponent error"),
            },
        )));
    }
    if neg
    {
        output.push(Num(n1));
    }
    for _ in abs
    {
        output.push(Str(")".to_string()));
        output.push(Str(")".to_string()));
    }
    let mut count = 0;
    i = 0;
    let mut brackets: Vec<(usize, usize)> = Vec::new();
    while i < output.len()
    {
        if let Str(s) = &output[i]
        {
            match s.as_str()
            {
                "(" =>
                {
                    if output.len() > i + 1 && output[i + 1].str_is(")")
                    {
                        output.remove(i + 1);
                        output.remove(i);
                        i = i.saturating_sub(1);
                        count -= 1;
                        continue;
                    }
                    brackets.push((i, count as usize));
                    count += 1
                }
                ")" =>
                {
                    count -= 1;
                    if let Some(k) = brackets.last()
                    {
                        if k.0 == 0 && i == output.len() - 1
                        {
                            output.pop();
                            output.remove(0);
                        }
                        brackets.pop();
                    }
                }
                _ =>
                {}
            }
        }
        i += 1;
    }
    while let Some(Str(c)) = output.last()
    {
        if print && vars.iter().any(|a| a.0.iter().collect::<String>() == *c)
        {
            break;
        }
        else if !matches!(c.as_str(), "rnd" | ")" | "}" | "x" | "y")
            && !sumrec.iter().any(|s| &s.1 == c)
        {
            output.pop();
        }
        else if output.len() > 1
            && output[output.len() - 2].str_is("norm")
            && output[output.len() - 1].str_is(")")
        {
            output.pop();
            output.pop();
            output.pop();
        }
        else
        {
            break;
        }
    }
    if output.is_empty()
    {
        return Err(" ");
    }
    Ok((
        output,
        graph
            && !input
                .replace("==", "")
                .replace("!=", "")
                .replace("<=", "")
                .replace(">=", "")
                .contains('='),
    ))
}
#[allow(clippy::type_complexity)]
fn place_multiplier(
    output: &mut Vec<NumStr>,
    sumrec: &[(isize, String)],
    vars: Option<Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>>,
)
{
    if let Some(Str(s)) = output.last()
    {
        if matches!(s.as_str(), ")" | "x" | "y" | "]" | "}" | "rnd" | "@")
            || sumrec.iter().any(|a| a.1 == *s)
            || (if let Some(n) = vars
            {
                n.iter().any(|a| a.0.iter().collect::<String>() == *s)
            }
            else
            {
                false
            })
        {
            output.push(Str('*'.to_string()))
        }
    }
    else if let Num(_) = output.last().unwrap_or(&Str("".to_string()))
    {
        output.push(Str('*'.to_string()))
    }
}
fn can_abs(output: &[NumStr]) -> bool
{
    if let Some(Str(s)) = output.last()
    {
        return !functions().contains(s.as_str());
    }
    true
}