use crate::{
    complex::{
        NumStr,
        NumStr::{Num, Str},
    },
    vars::functions,
    Options,
};
use rug::{float::Special::Infinity, ops::Pow, Complex};
pub fn get_func(input: &str, options: Options) -> Result<Vec<NumStr>, &'static str>
{
    if input.is_empty()
    {
        return Ok(Vec::new());
    }
    let mut count: i32 = 0;
    let mut exp = String::new();
    let mut func: Vec<NumStr> = Vec::new();
    let mut word = String::new();
    let mut find_word = false;
    let mut abs = Vec::new();
    let mut neg = false;
    let mut i = 1;
    let mut scientific = false;
    let mut chars = input.chars().collect::<Vec<char>>();
    while i < chars.len()
    {
        if chars[i].is_whitespace()
        {
            if chars.len() - 1 == i
            {
                chars.remove(i);
            }
            else if chars[i - 1].is_numeric() && chars[i + 1].is_numeric()
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
    i = 0;
    let n1 = Complex::with_val(options.prec, -1);
    let mut pow = String::new();
    let mut pwr = (false, 0, 0);
    let mut subfact = (false, 0);
    'outer: while i < chars.len()
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
            if !func.is_empty()
                && (func.last().unwrap().num().is_ok() || func.last().unwrap().str_is(")"))
            {
                func.push(Str('^'.to_string()));
            }
            func.push(Num(Complex::with_val(
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
        if c.is_ascii_digit()
        {
            if !word.is_empty() && word != "0."
            {
                if c == '2' && word == "atan"
                {
                    word.push(c);
                    i += 1;
                    continue;
                }
                find_word = false;
                place_multiplier(&mut func, &find_word);
                func.push(Str(word.clone()));
                word.clear();
            }
            place_multiplier(&mut func, &find_word);
            let mut iter = false;
            let mut deci = false;
            for c in chars[i..].iter()
            {
                match c
                {
                    '0'..='9' =>
                    {
                        word.push(*c);
                    }
                    '.' =>
                    {
                        if deci
                        {
                            if word.ends_with('.')
                            {
                                word.pop();
                                iter = true;
                                i += 1;
                                break;
                            }
                            else
                            {
                                return Err("cant have multiple '.'");
                            }
                        }
                        deci = true;
                        word.push(*c);
                    }
                    _ => break,
                }
                i += 1;
            }
            if neg
            {
                if chars.len() > i + 1
                    && (chars[i] == '^' || (chars[i] == '/' && chars[i + 1] == '/'))
                {
                    func.push(Num(n1.clone()));
                    func.push(Str('*'.to_string()));
                }
                else
                {
                    word.insert(0, '-');
                }
                neg = false;
            }
            func.push(Num(Complex::with_val(
                options.prec,
                Complex::parse(word.as_bytes()).unwrap(),
            )));
            if iter
            {
                func.push(Str("..".to_string()));
            }
            if scientific
            {
                func.push(Str(")".to_string()));
                scientific = false;
            }
            if pwr.0 && pwr.1 == 0 && (chars.len() <= i || chars[i] != '^')
            {
                for _ in 0..pwr.2
                {
                    func.push(Str(')'.to_string()))
                }
                pwr.0 = false;
                pwr.2 = 0
            }
            if subfact.0 && subfact.1 == 0
            {
                func.push(Str(')'.to_string()));
                func.push(Str(')'.to_string()));
                subfact.0 = false;
            }
            word.clear();
            continue;
        }
        else if c.is_alphabetic()
        {
            if find_word
                && (!(c == 'x' || c == 'y')
                    || (chars.len() - 1 != i && chars[i + 1] == 'p' && word == "e")
                    || word == "ma")
            {
                word.push(c);
            }
            else
            {
                if neg
                {
                    func.push(Num(n1.clone()));
                    func.push(Str('*'.to_string()));
                    neg = false;
                }
                match c
                {
                    'ⁱ' => pow.push('i'),
                    'E' | 'e'
                        if (options.small_e && c == 'e') || (!options.small_e && c == 'E') =>
                    {
                        if let Some(last) = func.last()
                        {
                            if last.num().is_ok() || last.str_is("x") || last.str_is("y")
                            {
                                func.insert(func.len() - 1, Str("(".to_string()));
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
                        place_multiplier(&mut func, &find_word);
                        func.push(Num(Complex::with_val(options.prec, 10)));
                        if i + 1 != chars.len()
                            && (chars[i + 1].is_alphanumeric()
                                || matches!(chars[i + 1], '-' | '+' | '(' | '{'))
                        {
                            func.push(Str('^'.to_string()));
                        }
                        if !(i + 1 != chars.len()
                            && matches!(chars[i + 1], '0'..='9' | '-' | '+' | 'x' | 'y' | 'i'))
                        {
                            func.push(Str(")".to_string()));
                        }
                    }
                    'x' | 'y' =>
                    {
                        if c == 'y' && word == "an"
                        {
                            word.push('y');
                            i += 1;
                            continue;
                        }
                        if !word.is_empty()
                        {
                            find_word = false;
                            place_multiplier(&mut func, &find_word);
                            func.push(Str(word.clone()));
                            word.clear();
                        }
                        place_multiplier(&mut func, &find_word);
                        func.push(Str(c.to_string()));
                        if scientific
                        {
                            func.push(Str(")".to_string()));
                            scientific = false;
                        }
                        if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
                        {
                            for _ in 0..pwr.2
                            {
                                func.push(Str(')'.to_string()))
                            }
                            pwr.0 = false;
                            pwr.2 = 0
                        }
                        if subfact.0 && subfact.1 == 0
                        {
                            subfact.0 = false;
                            func.push(Str(")".to_string()));
                            func.push(Str(")".to_string()))
                        }
                    }
                    'i' =>
                    {
                        if i + 1 != chars.len() && matches!(chars[i + 1], 'n' | 'm' | 'd')
                        {
                            if chars[i + 1] == 'n' && i + 2 != chars.len() && chars[i + 2] == 'f'
                            {
                                i += 2;
                                place_multiplier(&mut func, &find_word);
                                func.push(Num(Complex::with_val(options.prec, Infinity)));
                                if pwr.0
                                    && pwr.1 == 0
                                    && (chars.len() <= i + 1 || chars[i + 1] != '^')
                                {
                                    for _ in 0..pwr.2
                                    {
                                        func.push(Str(')'.to_string()))
                                    }
                                    pwr.0 = false;
                                    pwr.2 = 0
                                }
                                if scientific
                                {
                                    func.push(Str(")".to_string()));
                                    scientific = false;
                                }
                                if subfact.0 && subfact.1 == 0
                                {
                                    subfact.0 = false;
                                    func.push(Str(")".to_string()));
                                    func.push(Str(")".to_string()))
                                }
                            }
                            else
                            {
                                word.push(c);
                                find_word = true;
                            }
                        }
                        else
                        {
                            place_multiplier(&mut func, &find_word);
                            func.push(Num(Complex::with_val(options.prec, (0, 1))));
                            if pwr.0 && pwr.1 == 0 && (chars.len() <= i + 1 || chars[i + 1] != '^')
                            {
                                for _ in 0..pwr.2
                                {
                                    func.push(Str(')'.to_string()))
                                }
                                pwr.0 = false;
                                pwr.2 = 0
                            }
                            if scientific
                            {
                                func.push(Str(")".to_string()));
                                scientific = false;
                            }
                            if subfact.0 && subfact.1 == 0
                            {
                                subfact.0 = false;
                                func.push(Str(")".to_string()));
                                func.push(Str(")".to_string()))
                            }
                        }
                    }
                    _ =>
                    {
                        word.push(c);
                        find_word = true;
                    }
                }
            }
        }
        else
        {
            if !word.is_empty()
            {
                find_word = false;
                if i + 4 < chars.len()
                    && chars[i] == '^'
                    && chars[i + 1] == '('
                    && chars[i + 2] == '-'
                    && chars[i + 3] == '1'
                    && chars[i + 4] == ')'
                {
                    place_multiplier(&mut func, &find_word);
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 5;
                    continue;
                }
                if i + 2 < chars.len()
                    && chars[i] == '^'
                    && chars[i + 1] == '-'
                    && chars[i + 2] == '1'
                {
                    place_multiplier(&mut func, &find_word);
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 3;
                    continue;
                }
                if i + 1 < chars.len()
                    && chars[i] == '^'
                    && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '-')
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                    word.clear();
                    let pos = chars
                        .iter()
                        .skip(i + 1)
                        .position(|&c| c == '(' || c == ')' || c == ',');
                    if pos.is_none()
                    {
                        continue;
                    }
                    exp = chars[i + 1..i + 1 + pos.unwrap()].iter().collect();
                    if exp == "-"
                    {
                        exp = "-1".to_string();
                    }
                    i += pos.unwrap() + 1;
                    continue;
                }
                place_multiplier(&mut func, &find_word);
                func.push(Str(word.clone()));
                word.clear();
            }
            if !exp.is_empty() && c != '(' && c != ')'
            {
                func.push(Str("^".to_string()));
                func.push(Num(Complex::with_val(
                    options.prec,
                    match Complex::parse(exp.as_bytes())
                    {
                        Ok(n) => n,
                        _ => return Err("exponent error"),
                    },
                )));
                exp = String::new();
            }
            match c
            {
                '√' => func.push(Str("sqrt".to_string())),
                '∛' => func.push(Str("cbrt".to_string())),
                '¼' => func.push(Num(Complex::with_val(options.prec, 0.25))),
                '½' => func.push(Num(Complex::with_val(options.prec, 0.5))),
                '¾' => func.push(Num(Complex::with_val(options.prec, 0.75))),
                '⅒' => func.push(Num(Complex::with_val(options.prec, 0.1))),
                '⅕' => func.push(Num(Complex::with_val(options.prec, 0.2))),
                '⅖' => func.push(Num(Complex::with_val(options.prec, 0.4))),
                '⅗' => func.push(Num(Complex::with_val(options.prec, 0.6))),
                '⅘' => func.push(Num(Complex::with_val(options.prec, 0.8))),
                '⅐' => func.push(Num(Complex::with_val(options.prec, 7).recip())),
                '⅑' => func.push(Num(Complex::with_val(options.prec, 9).recip())),
                '⅓' => func.push(Num(Complex::with_val(options.prec, 3).recip())),
                '⅔' => func.push(Num(Complex::with_val(options.prec, 1.5).recip())),
                '⅙' => func.push(Num(Complex::with_val(options.prec, 6).recip())),
                '⅚' => func.push(Num(Complex::with_val(options.prec, 1.2).recip())),
                '⅛' => func.push(Num(Complex::with_val(options.prec, 0.125))),
                '⅜' => func.push(Num(Complex::with_val(options.prec, 0.375))),
                '⅝' => func.push(Num(Complex::with_val(options.prec, 0.625))),
                '⅞' => func.push(Num(Complex::with_val(options.prec, 0.875))),
                '⅟' =>
                {
                    func.push(Num(Complex::with_val(options.prec, 1)));
                    func.push(Str("/".to_string()))
                }
                '↉' => func.push(Num(Complex::new(options.prec))),
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
                '.' =>
                {
                    if i + 1 != chars.len() && chars[i + 1] == '.'
                    {
                        i += 1;
                        func.push(Str("..".to_string()));
                    }
                    else
                    {
                        word.push_str("0.")
                    }
                }
                '&' if i != 0
                    && i + 1 < chars.len()
                    && chars[i + 1] == '&'
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    func.push(Str("&&".to_string()));
                }
                '=' if i != 0
                    && i + 1 < chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '='
                    {
                        func.push(Str("==".to_string()));
                        i += 1;
                    }
                    else if chars[i - 1] == '>'
                    {
                        func.push(Str(">=".to_string()));
                    }
                    else if chars[i - 1] == '<'
                    {
                        func.push(Str("<=".to_string()));
                    }
                }
                '{' =>
                {
                    place_multiplier(&mut func, &find_word);
                    if neg
                    {
                        func.push(Num(n1.clone()));
                        func.push(Str('*'.to_string()));
                        neg = false;
                    }
                    func.push(Str("{".to_string()));
                }
                '}' =>
                {
                    func.push(Str("}".to_string()));
                }
                '±' if i + 1 != chars.len() && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if func.is_empty()
                        || matches!(func.last().unwrap(),Str(s) if (s==","||s=="{"||s=="("))
                    {
                        func.push(Num(Complex::new(options.prec)))
                    }
                    func.push(Str("±".to_string()))
                }
                '*' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '*'
                    {
                        if chars.len() > i + 2
                        {
                            func.push(Str("^".to_string()));
                        }
                        i += 1;
                    }
                    else
                    {
                        func.push(Str('*'.to_string()));
                    }
                }
                '/' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '/'
                    {
                        func.push(Str("//".to_string()));
                        i += 1;
                    }
                    else
                    {
                        func.push(Str('/'.to_string()));
                    }
                }
                '+' if i != 0
                    && i + 1 != chars.len()
                    && (chars[i - 1].is_alphanumeric()
                        || (!func.is_empty() && func.last().unwrap().str_is(")"))
                        || chars[i - 1] == '}'
                        || chars[i - 1] == ']')
                    && chars[i - 1] != if options.small_e { 'e' } else { 'E' }
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '-'
                    {
                        if func.is_empty()
                            || matches!(func.last().unwrap(),Str(s) if (s==","||s=="{"||s=="("))
                        {
                            func.push(Num(Complex::new(options.prec)))
                        }
                        i += 1;
                        func.push(Str('±'.to_string()))
                    }
                    else
                    {
                        func.push(Str('+'.to_string()))
                    }
                }
                '+' if i + 1 < chars.len()
                    && chars[i + 1] == '-'
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if func.is_empty()
                        || matches!(func.last().unwrap(),Str(s) if (s==","||s=="{"||s=="("))
                    {
                        func.push(Num(Complex::new(options.prec)))
                    }
                    i += 1;
                    func.push(Str('±'.to_string()))
                }
                '<' if i != 0
                    && i + 1 < chars.len()
                    && chars[i + 1] != '='
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '<'
                    {
                        func.push(Str("<<".to_string()));
                        i += 1;
                    }
                    else
                    {
                        func.push(Str('<'.to_string()));
                    }
                }
                '>' if i != 0
                    && i + 1 < chars.len()
                    && chars[i + 1] != '='
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '>'
                    {
                        func.push(Str(">>".to_string()));
                        i += 1;
                    }
                    else
                    {
                        func.push(Str('>'.to_string()));
                    }
                }
                '-' if i + 1 < chars.len() && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if i != 0 && chars[i - 1] == '^'
                    {
                        func.push(Str("(".to_string()));
                        func.push(Num(n1.clone()));
                        pwr.0 = true;
                        pwr.2 += 1;
                    }
                    else if i == 0
                        || !(chars[i - 1] != if options.small_e { 'e' } else { 'E' }
                            && (chars[i - 1].is_alphanumeric()
                                || (!func.is_empty() && func.last().unwrap().str_is(")"))
                                || chars[i - 1] == '}'
                                || chars[i - 1] == ']'))
                    {
                        if i + 1 != chars.len() && (chars[i + 1] == '(' || chars[i + 1] == '-')
                        {
                            func.push(Num(n1.clone()));
                            func.push(Str("*".to_string()));
                        }
                        else
                        {
                            neg = true;
                        }
                    }
                    else
                    {
                        func.push(Str('-'.to_string()));
                    }
                }
                '^' if !func.is_empty()
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    if chars[i + 1] == '^'
                    {
                        if chars.len() > i + 2
                        {
                            func.push(Str("^^".to_string()))
                        }
                        i += 1;
                    }
                    else
                    {
                        func.push(Str("^".to_string()))
                    }
                }
                '(' if i + 1 != chars.len() =>
                {
                    count += 1;
                    if pwr.0
                    {
                        pwr.1 = count;
                    }
                    if subfact.0
                    {
                        subfact.1 = count;
                    }
                    place_multiplier(&mut func, &find_word);
                    func.push(Str("(".to_string()))
                }
                ')' if i != 0 =>
                {
                    if pwr.1 == count
                    {
                        for _ in 0..pwr.2
                        {
                            func.push(Str(')'.to_string()))
                        }
                        pwr = (false, 0, 0);
                    }
                    if subfact.1 == count
                    {
                        subfact = (false, 0);
                        func.push(Str(")".to_string()));
                        func.push(Str(")".to_string()))
                    }
                    count -= 1;
                    func.push(Str(")".to_string()))
                }
                '|' =>
                {
                    if !abs.is_empty() && abs[0] == count && can_abs(&func)
                    {
                        func.push(Str(")".to_string()));
                        func.push(Str(")".to_string()));
                        abs.remove(0);
                    }
                    else if i + 1 != chars.len() && chars[i + 1] == '|'
                    {
                        func.push(Str("||".to_string()));
                        i += 2;
                        continue;
                    }
                    else
                    {
                        place_multiplier(&mut func, &find_word);
                        func.push(Str("(".to_string()));
                        func.push(Str("norm".to_string()));
                        func.push(Str("(".to_string()));
                        abs.insert(0, count);
                    }
                }
                '!' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '='
                    {
                        func.push(Str("!=".to_string()));
                    }
                    else if i != 0
                        && (chars[i - 1].is_alphanumeric()
                            || (!func.is_empty()
                                && (func.last().unwrap().str_is(")")
                                    || func.last().unwrap().str_is("}"))))
                    {
                        if let Num(a) = func.clone().last().unwrap()
                        {
                            if a.real().is_sign_negative()
                            {
                                func.pop();
                                func.push(Num(Complex::with_val(
                                    options.prec,
                                    (-a.real(), a.imag()),
                                )));
                                func.insert(func.len() - 1, Num(n1.clone()));
                                func.insert(func.len() - 1, Str("*".to_string()));
                            }
                        }
                        if func.clone().last().unwrap().str_is(")")
                            || func.last().unwrap().str_is("}")
                        {
                            let mut count = 0;
                            for (j, c) in func.iter().enumerate().rev()
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
                                        if let Str(s) = &func[j - 1]
                                        {
                                            if s.chars().next().unwrap().is_alphabetic()
                                            {
                                                func.insert(j - 1, Str("(".to_string()));
                                                if i + 1 != chars.len() && chars[i + 1] == '!'
                                                {
                                                    i += 1;
                                                    func.insert(j, Str("(".to_string()));
                                                    func.insert(j, Str("doublefact".to_string()));
                                                }
                                                else
                                                {
                                                    func.insert(j, Str("(".to_string()));
                                                    func.insert(j, Str("fact".to_string()));
                                                }
                                                func.push(Str(")".to_string()));
                                                func.push(Str(")".to_string()));
                                                i += 1;
                                                continue 'outer;
                                            }
                                        }
                                    }
                                    func.insert(j, Str("(".to_string()));
                                    if i + 1 != chars.len() && chars[i + 1] == '!'
                                    {
                                        i += 1;
                                        func.insert(j, Str("doublefact".to_string()));
                                    }
                                    else
                                    {
                                        func.insert(j, Str("fact".to_string()));
                                    }
                                    func.push(Str(")".to_string()));
                                    i += 1;
                                    continue 'outer;
                                }
                            }
                        }
                        func.insert(func.len() - 1, Str("(".to_string()));
                        if i + 1 != chars.len() && chars[i + 1] == '!'
                        {
                            i += 1;
                            func.insert(func.len() - 1, Str("doublefact".to_string()));
                        }
                        else
                        {
                            func.insert(func.len() - 1, Str("fact".to_string()));
                        }
                        func.insert(func.len() - 1, Str("(".to_string()));
                        func.push(Str(")".to_string()));
                        func.push(Str(")".to_string()));
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
                            func.push(Num(n1.clone()));
                            func.push(Str("*".to_string()));
                            neg = false;
                        }
                        func.push(Str("(".to_string()));
                        func.push(Str("subfact".to_string()));
                        func.push(Str("(".to_string()));
                        subfact.0 = true;
                    }
                }
                ',' if i != 0 && i + 1 != chars.len() && chars[i + 1] != ')' =>
                {
                    func.push(Str(','.to_string()))
                }
                '%' if i != 0
                    && i + 1 != chars.len()
                    && !matches!(chars[i + 1], ')' | '}' | ']') =>
                {
                    func.push(Str('%'.to_string()))
                }
                '∞' => func.push(Num(Complex::with_val(options.prec, Infinity))),
                _ =>
                {}
            }
        }
        i += 1;
    }
    if !pow.is_empty()
    {
        let i = pow.matches('i').count() % 4;
        pow = pow.replace('i', "");
        if pow.is_empty()
        {
            pow = "1".to_string();
        }
        if !func.is_empty()
            && (func.last().unwrap().num().is_ok() || func.last().unwrap().str_is(")"))
        {
            func.push(Str("^".to_string()));
        }
        func.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(pow.as_bytes())
            {
                Ok(n) => n,
                _ => return Err("exponent error"),
            },
        ) * Complex::with_val(options.prec, (0, 1))
            .pow(Complex::with_val(options.prec, i))));
    }
    if !exp.is_empty()
    {
        func.push(Str("^".to_string()));
        func.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(exp.as_bytes())
            {
                Ok(n) => n,
                _ => return Err("exponent error"),
            },
        )));
    }
    if neg
    {
        func.push(Num(n1));
    }
    if word == "rnd"
    {
        func.push(Str("rnd".to_string()))
    }
    for _ in abs
    {
        func.push(Str(")".to_string()));
        func.push(Str(")".to_string()));
    }
    count = 0;
    i = 0;
    let mut brackets: Vec<(usize, usize)> = Vec::new();
    while i < func.len()
    {
        if let Str(s) = &func[i]
        {
            match s.as_str()
            {
                "(" =>
                {
                    if func.len() > i + 2 && func[i + 2].str_is(")")
                    {
                        func.remove(i + 2);
                        func.remove(i);
                        i = i.saturating_sub(1);
                        count -= 1;
                        continue;
                    }
                    else if func.len() > i + 1 && func[i + 1].str_is(")")
                    {
                        func.remove(i + 1);
                        func.remove(i);
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
                        if k.0 == 0
                        {
                            if i == func.len() - 1
                            {
                                func.pop();
                                func.remove(0);
                            }
                        }
                        else if i != func.len() - 1
                            && func[k.0 - 1].str_is("(")
                            && func[i + 1].str_is(")")
                        {
                            func.remove(i + 1);
                            func.remove(k.0 - 1);
                            i = k.0.saturating_sub(2);
                            brackets.pop();
                            continue;
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
    while let Some(Str(c)) = func.last()
    {
        if !matches!(c.as_str(), "rnd" | ")" | "}" | "x" | "y")
        {
            func.pop();
        }
        else if func.len() > 1
            && func[func.len() - 2].str_is("norm")
            && func[func.len() - 1].str_is(")")
        {
            func.pop();
            func.pop();
            func.pop();
        }
        else
        {
            break;
        }
    }
    if func.is_empty()
    {
        return Err("no function");
    }
    Ok(func)
}
fn can_abs(func: &[NumStr]) -> bool
{
    if let Some(Str(s)) = func.last()
    {
        return !functions().contains(s.as_str());
    }
    true
}
fn place_multiplier(func: &mut Vec<NumStr>, find_word: &bool)
{
    if let Some(Str(s)) = func.last()
    {
        if !find_word && matches!(s.as_str(), ")" | "x" | "y" | "]" | "}" | "rnd")
        {
            func.push(Str('*'.to_string()))
        }
    }
    else if let Num(_) = func.last().unwrap_or(&Str("".to_string()))
    {
        func.push(Str('*'.to_string()))
    }
}