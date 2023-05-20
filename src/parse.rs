use std::f64::consts::{E, PI, TAU};
use crate::math::NumOrString;
use crate::math::NumOrString::{Complex, Str};
pub fn get_func(input:&str) -> Result<Vec<NumOrString>, ()>
{
    let count = input.chars().filter(|&c| c == '(').count() as i32 - input.chars().filter(|&c| c == ')').count() as i32;
    let mut input = input.to_string();
    if count > 0
    {
        for _ in 0..count
        {
            input.push(')');
        }
    }
    else
    {
        for _ in 0..count.abs()
        {
            input.insert(0, '(')
        }
    }
    let mut exp = 0.0;
    let mut func:Vec<NumOrString> = Vec::new();
    let mut word = String::new();
    let mut real = String::new();
    let mut imag = String::new();
    let mut find_word = false;
    let mut abs = true;
    let mut subfact = 0;
    let mut neg = false;
    let mut i = 0;
    let (mut c, mut j, mut char, mut deci, mut f, mut imchar, mut imdeci);
    let chars = input.chars().collect::<Vec<char>>();
    while i < chars.len()
    {
        c = chars[i];
        if c.is_numeric()
        {
            if !word.is_empty()
            {
                find_word = false;
                func.push(Str(word.clone()));
                word.clear();
            }
            if i != 0 && chars[i - 1] != 'x' && chars[i - 1] != 'y' && (chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == ')')
            {
                func.push(Str('*'.to_string()))
            }
            if neg
            {
                real.push('-');
                neg = false;
            }
            j = i;
            deci = false;
            'outer: while j < chars.len()
            {
                char = chars[j];
                if char.is_numeric()
                {
                    real.push(char);
                }
                else if char == '.' && !deci
                {
                    deci = true;
                    real.push(char);
                }
                else if char == 'i'
                {
                    imag = real.clone();
                    real.clear();
                }
                else if char == '+' || char == '-'
                {
                    imag.push(char);
                    f = j + 1;
                    imdeci = false;
                    while f < chars.len()
                    {
                        imchar = chars[f];
                        if imchar.is_numeric()
                        {
                            imag.push(imchar);
                        }
                        else if imchar == '.' && !imdeci
                        {
                            imdeci = true;
                            imag.push(imchar);
                        }
                        else if imchar == 'i'
                        {
                            j = f + 1;
                            break 'outer;
                        }
                        else
                        {
                            imag.clear();
                            break 'outer;
                        }
                        f += 1;
                    }
                    imag.clear();
                    break;
                }
                else
                {
                    break;
                }
                j += 1;
            }
            func.push(Complex((real.parse::<f64>().unwrap_or(0.0), imag.parse::<f64>().unwrap_or(0.0))));
            real.clear();
            imag.clear();
            if j != chars.len() && chars[j] != 'x' && chars[j] != 'y' && (chars[j].is_ascii_alphanumeric() || chars[j] == '(')
            {
                func.push(Str('*'.to_string()))
            }
            if j != i
            {
                i = j;
                continue;
            }
        }
        else if c.is_ascii_alphabetic()
        {
            if find_word && (!(c == 'x' || c == 'y') || (chars.len() - 1 != i && chars[i + 1] == 'p'))
            {
                word.push(c);
            }
            else
            {
                match c
                {
                    'x' | 'y' =>
                    {
                        if !find_word && i != 0 && (chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == ')')
                        {
                            func.push(Str('*'.to_string()))
                        }
                        if !word.is_empty()
                        {
                            find_word = false;
                            func.push(Str(word.clone()));
                            word.clear();
                        }
                        func.push(Str(c.to_string()));
                    }
                    'i' =>
                    {
                        if i != 0 && chars[i - 1] == 'p'
                        {
                            func.push(Complex((PI, 0.0)));
                        }
                        else if i + 1 != chars.len() && (chars[i + 1] == 'n' || chars[i + 1] == 'm')
                        {
                            word.push(c);
                            find_word = true;
                            i += 1;
                            continue;
                        }
                        else
                        {
                            func.push(Complex((0.0, 1.0)));
                        }
                    }
                    'e' =>
                    {
                        if i + 2 < chars.len() && chars[i + 1] == 'x' && chars[i + 2] == 'p'
                        {
                            word.push(c);
                            find_word = true;
                            i += 1;
                            continue;
                        }
                        else
                        {
                            if !find_word && i != 0 && (chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == ')')
                            {
                                func.push(Str('*'.to_string()))
                            }
                            func.push(Complex((E, 0.0)));
                        }
                    }
                    'p' =>
                    {
                        if find_word
                        {
                            word.push(c);
                        }
                        i += 1;
                        continue;
                    }
                    't' =>
                    {
                        if find_word || (i + 2 < chars.len() && chars[i + 2] != 'u')
                        {
                            word.push(c);
                            find_word = true;
                        }
                        i += 1;
                        continue;
                    }
                    'a' =>
                    {
                        if find_word || (i + 1 != chars.len() && chars[i + 1] != 'u')
                        {
                            word.push(c);
                            find_word = true;
                        }
                        i += 1;
                        continue;
                    }
                    'u' =>
                    {
                        func.push(Complex((TAU, 0.0)));
                    }
                    _ =>
                    {
                        word.push(c);
                        find_word = true;
                        i += 1;
                        continue;
                    }
                }
                if i != chars.len() - 1 && chars[i + 1] != 'x' && chars[i + 1] != 'y' && (chars[i + 1].is_ascii_alphanumeric() || chars[i + 1] == '(')
                {
                    func.push(Str('*'.to_string()))
                }
            }
        }
        else
        {
            if !word.is_empty()
            {
                find_word = false;
                if i + 4 < chars.len() && chars[i] == '^' && chars[i + 1] == '(' && chars[i + 2] == '-' && chars[i + 3] == '1' && chars[i + 4] == ')'
                {
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 5;
                    continue;
                }
                if i + 1 < chars.len() && chars[i] == '^' && chars[i + 1].is_ascii_digit()
                {
                    func.push(Str(word.clone()));
                    word.clear();
                    exp = chars[i + 1].to_string().parse::<f64>().unwrap();
                    i += 2;
                    continue;
                }
                func.push(Str(word.clone()));
                word.clear();
            }
            if exp != 0.0 && c != '(' && c != ')'
            {
                func.push(Str("^".to_string()));
                func.push(Complex((exp, 0.0)));
                exp = 0.0;
            }
            match c
            {
                '*' => func.push(Str('*'.to_string())),
                '/' => func.push(Str('/'.to_string())),
                '+' => func.push(Str('+'.to_string())),
                '-' =>
                {
                    if i == 0 || !(chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == ')')
                    {
                        neg = true;
                    }
                    else
                    {
                        func.push(Str('-'.to_string()));
                    }
                }
                '^' => func.push(Str('^'.to_string())),
                '(' =>
                {
                    if let Some(Str(s)) = func.last()
                    {
                        if !find_word && s == ")"
                        {
                            func.push(Str('*'.to_string()))
                        }
                    }
                    func.push(Str("(".to_string()))
                }
                ')' => func.push(Str(")".to_string())),
                '|' =>
                {
                    if abs
                    {
                        func.push(Str("abs".to_string()));
                        func.push(Str("(".to_string()));
                    }
                    else
                    {
                        func.push(Str(")".to_string()));
                    }
                    abs = !abs;
                }
                '!' =>
                {
                    if i != 0 && chars[i - 1].is_ascii_digit()
                    {
                        func.insert(func.len() - 1, Str("fact".to_string()));
                        func.insert(func.len() - 1, Str("(".to_string()));
                        func.push(Str(")".to_string()));
                    }
                    else if i != chars.len() - 1 && chars[i + 1].is_ascii_digit()
                    {
                        func.push(Str("subfact".to_string()));
                        func.push(Str("(".to_string()));
                        subfact += 1;
                    }
                }
                'π' => func.push(Complex((PI, 0.0))),
                'τ' => func.push(Complex((TAU, 0.0))),
                ',' => func.push(Str(','.to_string())),
                '%' => func.push(Str('%'.to_string())),
                _ => (),
            }
        }
        i += 1;
    }
    for _ in 0..subfact
    {
        func.push(Str(")".to_string()))
    }
    if !abs
    {
        func.push(Str(")".to_string()))
    }
    if exp != 0.0
    {
        func.push(Str("^".to_string()));
        func.push(Complex((exp, 0.0)));
    }
    if !word.is_empty()
    {
        func.push(Str(word.clone()));
        word.clear();
    }
    if func.is_empty()
    {
        return Err(());
    }
    if let Str(s) = func.last().unwrap()
    {
        if s == "*" || s == "/" || s == "^" || s == "+" || s == "-" || s == "%"
        {
            func.pop();
        }
    }
    if func.is_empty()
    {
        return Err(());
    }
    Ok(func)
}