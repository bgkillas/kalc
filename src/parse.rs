use crate::{
    complex::{
        NumStr,
        NumStr::{Num, Str, Vector},
    },
    Options,
};
use rug::{float::Constant::Pi, ops::Pow, Complex, Float};
use std::collections::HashSet;
pub fn get_func(input: &str, options: Options) -> Result<Vec<NumStr>, &'static str>
{
    let mut count: i32 = 0;
    let mut exp = String::new();
    let mut func: Vec<NumStr> = Vec::new();
    let mut word = String::new();
    let mut find_word = false;
    let mut abs = true;
    let mut neg = false;
    let mut i = 0;
    let chars = input.chars().collect::<Vec<char>>();
    let (mut c, mut deci);
    let n1 = Complex::with_val(options.prec, -1);
    let mut open = false;
    let mut pow = String::new();
    'outer: while i < chars.len()
    {
        c = chars[i];
        if !(c == '⁰'
            || c == '⁹'
            || c == '⁸'
            || c == '⁷'
            || c == '⁶'
            || c == '⁵'
            || c == '⁴'
            || c == '³'
            || c == '²'
            || c == '¹'
            || c == '⁻'
            || c == 'ⁱ')
            && !pow.is_empty()
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
                    Err(_) => return Err("exponent error"),
                },
            ) * Complex::with_val(options.prec, (0, 1))
                .pow(Complex::with_val(options.prec, i))));
            pow = String::new();
        }
        if c == ' '
        {
            if !word.is_empty()
            {
                find_word = false;
                if is_func(&word)
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                }
                word.clear();
            }
            else if i != 0
                && i + 1 != chars.len()
                && chars[i - 1].is_numeric()
                && chars[i + 1].is_numeric()
            {
                func.push(Str('*'.to_string()))
            }
        }
        else if c.is_ascii_digit()
        {
            if !word.is_empty() && word != "0."
            {
                find_word = false;
                if is_func(&word)
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                }
                word.clear();
            }
            place_multiplier(&mut func, &find_word);
            deci = false;
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
                            return Err("cant have multiple '.'");
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
                if chars.len() > i && chars[i] == '^'
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
            word.clear();
            if !open
            {
                func.extend(vec![Str(")".to_string()); count as usize]);
                count = 0;
            }
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
                        place_multiplier(&mut func, &find_word);
                        func.push(Num(Complex::with_val(options.prec, 10)));
                        if i + 1 != chars.len()
                            && (chars[i + 1].is_alphanumeric()
                                || chars[i + 1] == '-'
                                || chars[i + 1] == '+'
                                || chars[i + 1] == '('
                                || chars[i + 1] == '{'
                                || chars[i + 1] == '[')
                        {
                            func.push(Str('^'.to_string()));
                            func.push(Str('('.to_string()));
                            count += 1;
                        }
                    }
                    'x' | 'y' =>
                    {
                        if !word.is_empty()
                        {
                            find_word = false;
                            if is_func(&word)
                            {
                                place_multiplier(&mut func, &find_word);
                                func.push(Str(word.clone()));
                            }
                            word.clear();
                        }
                        func.push(Str(c.to_string()));
                        if !open
                        {
                            func.extend(vec![Str(")".to_string()); count as usize]);
                            count = 0;
                        }
                    }
                    'i' =>
                    {
                        if i + 1 != chars.len() && (chars[i + 1] == 'n' || chars[i + 1] == 'm')
                        {
                            word.push(c);
                            find_word = true;
                        }
                        else
                        {
                            place_multiplier(&mut func, &find_word);
                            func.push(Num(Complex::with_val(options.prec, (0, 1))));
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
                    let pos = chars.iter().skip(i + 1).position(|&c| c == '(' || c == ')');
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
                if is_func(&word)
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                    word.clear();
                }
            }
            if !exp.is_empty() && c != '(' && c != ')'
            {
                func.push(Str("^".to_string()));
                func.push(Num(Complex::with_val(
                    options.prec,
                    match Complex::parse(exp.as_bytes())
                    {
                        Ok(n) => n,
                        Err(_) => return Err("exponent error"),
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
                '⅐' => func.push(Num(Complex::with_val(options.prec, 7.0).recip())),
                '⅑' => func.push(Num(Complex::with_val(options.prec, 9.0).recip())),
                '⅓' => func.push(Num(Complex::with_val(options.prec, 3.0).recip())),
                '⅔' => func.push(Num(Complex::with_val(options.prec, 1.5).recip())),
                '⅙' => func.push(Num(Complex::with_val(options.prec, 6.0).recip())),
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
                '⁰' => pow.push('0'),
                '⁹' => pow.push('9'),
                '⁸' => pow.push('8'),
                '⁷' => pow.push('7'),
                '⁶' => pow.push('6'),
                '⁵' => pow.push('5'),
                '⁴' => pow.push('4'),
                '³' => pow.push('3'),
                '²' => pow.push('2'),
                '¹' => pow.push('1'),
                '⁻' => pow.push('-'),
                '.' => word.push_str("0."),
                '&' if i != 0 && i + 1 < chars.len() && chars[i + 1] == '&' =>
                {
                    func.push(Str("&&".to_string()));
                }
                '*' if i != 0 && i + 1 != chars.len() =>
                {
                    if i + 1 != chars.len() && chars[i + 1] == '*'
                    {
                        func.push(Str("^".to_string()));
                        i += 1;
                    }
                    else
                    {
                        func.push(Str('*'.to_string()));
                    }
                }
                '=' if i != 0 && i + 1 < chars.len() =>
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
                    open = true;
                }
                '}' =>
                {
                    func.push(Str("}".to_string()));
                    open = false;
                }
                '/' if i != 0 && i + 1 != chars.len() => func.push(Str('/'.to_string())),
                '+' if i != 0
                    && i + 1 != chars.len()
                    && (chars[i - 1].is_alphanumeric()
                        || (!func.is_empty() && func.last().unwrap().str_is(")"))
                        || chars[i - 1] == '}'
                        || chars[i - 1] == ']')
                    && chars[i - 1] != if options.small_e { 'e' } else { 'E' } =>
                {
                    func.push(Str('+'.to_string()))
                }
                '<' if i != 0 && i + 1 < chars.len() && chars[i + 1] != '=' =>
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
                '>' if i != 0 && i + 1 < chars.len() && chars[i + 1] != '=' =>
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
                '-' =>
                {
                    if i != 0 && chars[i - 1] == '^'
                    {
                        func.push(Str("(".to_string()));
                        func.push(Num(n1.clone()));
                        count += 1;
                    }
                    else if i == 0
                        || !(chars[i - 1] != if options.small_e { 'e' } else { 'E' }
                            && (chars[i - 1].is_alphanumeric()
                                || func.last().unwrap().str_is(")")
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
                '^' if i != 0 && i + 1 != chars.len() => func.push(Str('^'.to_string())),
                '(' if i + 1 != chars.len() =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str("(".to_string()))
                }
                ')' if i != 0 => func.push(Str(")".to_string())),
                '|' =>
                {
                    if i + 1 != chars.len() && chars[i + 1] == '|' && abs
                    {
                        func.push(Str("||".to_string()));
                        i += 2;
                        continue;
                    }
                    else if abs
                    {
                        place_multiplier(&mut func, &find_word);
                        func.push(Str("norm".to_string()));
                        func.push(Str("(".to_string()));
                        abs = false;
                    }
                    else
                    {
                        func.push(Str(")".to_string()));
                        abs = true;
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
                            || (!func.is_empty() && func.last().unwrap().str_is(")")
                                || func.last().unwrap().str_is("}")))
                    {
                        if let Num(a) = func.clone().last().unwrap()
                        {
                            if a.real() < &0.0
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
                                            if s != "subfact" && s != "("
                                            {
                                                func.insert(j - 1, Str("(".to_string()));
                                                func.insert(j - 1, Str("fact".to_string()));
                                                func.push(Str(")".to_string()));
                                                i += 1;
                                                continue 'outer;
                                            }
                                        }
                                    }
                                    func.insert(j, Str("(".to_string()));
                                    func.insert(j, Str("fact".to_string()));
                                    func.push(Str(")".to_string()));
                                    i += 1;
                                    continue 'outer;
                                }
                            }
                        }
                        func.insert(func.len() - 1, Str("fact".to_string()));
                        func.insert(func.len() - 1, Str("(".to_string()));
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
                        func.push(Str("subfact".to_string()));
                        func.push(Str("(".to_string()));
                        count += 1;
                    }
                }
                ',' if i != 0 && i + 1 != chars.len() => func.push(Str(','.to_string())),
                '%' if i != 0 && i + 1 != chars.len() => func.push(Str('%'.to_string())),
                _ => (),
            }
        }
        i += 1;
    }
    func.extend(vec![Str(")".to_string()); count as usize]);
    if !pow.is_empty()
    {
        let i = pow.matches('i').count() % 4;
        pow = pow.replace('i', "");
        if pow.is_empty()
        {
            pow = "1".to_string();
        }
        func.push(Str("^".to_string()));
        func.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(pow.as_bytes())
            {
                Ok(n) => n,
                Err(_) => return Err("exponent error"),
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
                Err(_) => return Err("exponent error"),
            },
        )));
    }
    if !abs
    {
        func.push(Str(")".to_string()));
    }
    if neg
    {
        func.push(Num(n1));
    }
    if func.is_empty()
    {
        return Err("no function");
    }
    // for i in &func
    // {
    //     match i
    //     {
    //         Str(s) => println!("{}", s),
    //         Num(n) => println!("{}", n),
    //         Vector(v) => println!("{:?}", v),
    //         Matrix(m) => println!("{:?}", m),
    //     }
    // }
    Ok(func)
}
fn place_multiplier(func: &mut Vec<NumStr>, find_word: &bool)
{
    if let Some(Str(s)) = func.last()
    {
        if !find_word && (s == ")" || s == "x" || s == "y" || s == "]" || s == "}")
        {
            func.push(Str('*'.to_string()))
        }
    }
    else if let Num(_) = func.last().unwrap_or(&Str("".to_string()))
    {
        func.push(Str('*'.to_string()))
    }
    else if let Vector(_) = func.last().unwrap_or(&Str("".to_string()))
    {
        func.push(Str('*'.to_string()))
    }
}
pub fn input_var(
    input: &str,
    vars: &[[String; 2]],
    dont_do: Option<&str>,
    options: Options,
) -> String
{
    let chars = input
        .replace('[', "(car{")
        .replace(']', "})")
        .chars()
        .collect::<Vec<char>>();
    let mut output = String::new();
    let (
        mut not_pushed,
        mut start,
        mut c,
        mut k,
        mut j,
        mut v,
        mut temp,
        mut split,
        mut value,
        mut o,
    );
    let mut i = 0;
    let mut commas: Vec<usize>;
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
                        _ => (),
                    }
                }
            }
            _ => (),
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
    let mut count;
    let mut vl;
    while i < chars.len()
    {
        c = chars[i];
        not_pushed = true;
        if !c.is_alphabetic()
        {
            output.push(c);
            i += 1;
            continue;
        }
        for var in vars
        {
            vl = var[0].chars().collect::<Vec<char>>().len();
            if var[0] != "e"
                || (!options.small_e
                    || !(i != 0
                        && i + 1 != chars.len()
                        && chars[i - 1].is_numeric()
                        && (chars[i + 1].is_numeric() || chars[i + 1] == '-')))
            {
                j = i;
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
                    o = i;
                    count = 0;
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
                        if let Some(n) = dont_do
                        {
                            if n == var[0]
                            {
                                return String::new();
                            }
                        }
                        not_pushed = false;
                        output.push('(');
                        output.push_str(&input_var(&var[1], vars, Some(&var[0]), options));
                        output.push(')');
                    }
                    else if j == 0 || !chars[j - 1].is_alphabetic()
                    {
                        k = 0;
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
                        v = var[0].chars().collect::<Vec<char>>();
                        if let Some(n) = dont_do
                        {
                            if n == var[0]
                            {
                                return String::new();
                            }
                        }
                        if input.contains(',') && var[0].contains(',') && chars.len() > 4
                        {
                            not_pushed = false;
                            output.push('(');
                            temp = &chars[j + var[0].find('(').unwrap()..i + 1];
                            if temp.ends_with(&[')'])
                            {
                                temp = &temp[..temp.len() - 1];
                            }
                            commas = Vec::new();
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
                                start = 0;
                                split = Vec::new();
                                for end in commas
                                {
                                    split.push(&temp[start..end]);
                                    start = end + 1;
                                }
                                split.push(&temp[start..]);
                                value = input_var(&var[1], vars, Some(&var[0]), options).clone();
                                for i in 0..split.len()
                                {
                                    value = value.replace(
                                        v[v.len()
                                            - 2 * (i as i32 - split.len() as i32).unsigned_abs()
                                                as usize],
                                        &format!(
                                            "({})",
                                            input_var(
                                                &split[i].iter().collect::<String>(),
                                                vars,
                                                Some(&var[0]),
                                                options
                                            )
                                        ),
                                    );
                                }
                                output.push_str(&value);
                                output.push(')');
                            }
                        }
                        else
                        {
                            not_pushed = false;
                            output.push('(');
                            temp = &chars[j + var[0].split('(').next().unwrap().len() + 1..i + 1];
                            if temp.ends_with(&[')'])
                            {
                                temp = &temp[..temp.len() - 1];
                            }
                            output.push_str(
                                &input_var(&var[1], vars, Some(&var[0]), options).replace(
                                    v[v.len() - 2],
                                    &format!(
                                        "({})",
                                        input_var(
                                            &temp.iter().collect::<String>(),
                                            vars,
                                            Some(&var[0]),
                                            options
                                        )
                                    ),
                                ),
                            );
                            output.push(')');
                        }
                    }
                    else
                    {
                        i = o;
                    }
                }
                else if !(i + vl > chars.len()
                    || chars[i..i + vl].iter().collect::<String>() != var[0])
                    && (i + 1 == chars.len() || chars[i + 1] != '(')
                    && (j == 0 || !chars[j - 1].is_alphabetic())
                    && (vl - 1 + i == chars.len() - 1 || !chars[i + 1 + vl - 1].is_alphabetic())
                {
                    if let Some(n) = dont_do
                    {
                        if n == var[0]
                        {
                            return String::new();
                        }
                    }
                    i += vl - 1;
                    not_pushed = false;
                    output.push('(');
                    output.push_str(&input_var(&var[1], vars, Some(&var[0]), options));
                    output.push(')');
                }
            }
        }
        if (c != ' ' || (i == 0 || chars[i - 1] != ' ')) && not_pushed
        {
            output.push(c);
        }
        i += 1;
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
        "Γ",
        "ζ",
    ]
    .iter()
    .cloned()
    .collect();
    functions.contains(word)
}
pub fn get_vars(prec: u32) -> Vec<[String; 2]>
{
    let pi = Float::with_val(prec, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(prec, 5).sqrt()) / 2;
    vec![
        ["c".to_string(), "299792458".to_string()],
        ["g".to_string(), "9.80665".to_string()],
        ["G".to_string(), "6.67430E-11".to_string()],
        ["h".to_string(), "6.62607015E-34".to_string()],
        ["ec".to_string(), "1.602176634E-19".to_string()],
        ["me".to_string(), "9.1093837015E-31".to_string()],
        ["mp".to_string(), "1.67262192369E-27".to_string()],
        ["mn".to_string(), "1.67492749804E-27".to_string()],
        ["ev".to_string(), "1.602176634E-19".to_string()],
        ["kc".to_string(), "8.9875517923E9".to_string()],
        ["na".to_string(), "6.02214076E23".to_string()],
        ["r".to_string(), "8.31446261815324".to_string()],
        ["kb".to_string(), "1.380649E-23".to_string()],
        ["phi".to_string(), phi.to_string()],
        ["φ".to_string(), phi.to_string()],
        ["e".to_string(), Float::with_val(prec, 1).exp().to_string()],
        ["pi".to_string(), pi.to_string()],
        ["π".to_string(), pi.to_string()],
        ["tau".to_string(), tau.to_string()],
        ["τ".to_string(), tau.to_string()],
    ]
}