use std::f64::consts::{E, PI, TAU};
use crate::math::Complex;
use crate::math::Complex::{Num, Str};
pub fn get_func(input:&str) -> Result<Vec<Complex>, ()>
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
    let mut exp = 0;
    let mut func:Vec<Complex> = Vec::new();
    let mut word = String::new();
    let mut find_word = false;
    let mut abs = true;
    let mut subfact = 0;
    let mut neg = false;
    let mut i = 0;
    let chars = input.chars().collect::<Vec<char>>();
    let (mut c, mut deci);
    while i < input.len()
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
            place_multiplier(&mut func, &find_word);
            if neg
            {
                word.push('-');
                neg = false;
            }
            deci = false;
            while i < chars.len()
            {
                c = chars[i];
                if c.is_numeric()
                {
                    word.push(c);
                }
                else if c == '.' && !deci
                {
                    deci = true;
                    word.push(c);
                }
                else
                {
                    break;
                }
                i += 1;
            }
            func.push(Num((word.parse::<f64>().unwrap_or(0.0), 0.0)));
            word.clear();
            continue;
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
                        place_multiplier(&mut func, &find_word);
                        if neg
                        {
                            func.push(Num((-1.0, 0.0)));
                            func.push(Str('*'.to_string()));
                            neg = false;
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
                        place_multiplier(&mut func, &find_word);
                        if i != 0 && chars[i - 1] == 'p'
                        {
                            func.push(Num((if neg { -PI } else { PI }, 0.0)));
                            neg = false;
                        }
                        else if i + 1 != chars.len() && (chars[i + 1] == 'n' || chars[i + 1] == 'm')
                        {
                            word.push(c);
                            find_word = true;
                        }
                        else
                        {
                            func.push(Num((0.0, if neg { -1.0 } else { 1.0 })));
                            neg = false;
                        }
                    }
                    'e' =>
                    {
                        place_multiplier(&mut func, &find_word);
                        if i + 2 < chars.len() && (chars[i + 1] == 'x' || chars[i + 1] == 'y') && chars[i + 2] == 'p'
                        {
                            word.push(c);
                            find_word = true;
                        }
                        else
                        {
                            func.push(Num((if neg { -E } else { E }, 0.0)));
                            neg = false;
                        }
                    }
                    'p' =>
                    {
                        if find_word
                        {
                            word.push(c);
                        }
                    }
                    't' =>
                    {
                        if find_word || (i + 2 < chars.len() && chars[i + 2] != 'u')
                        {
                            word.push(c);
                            find_word = true;
                        }
                    }
                    'a' =>
                    {
                        if find_word || (i + 1 != chars.len() && chars[i + 1] != 'u')
                        {
                            word.push(c);
                            find_word = true;
                        }
                    }
                    'u' =>
                    {
                        func.push(Num((if neg { -TAU } else { TAU }, 0.0)));
                        neg = false;
                    }
                    _ =>
                    {
                        place_multiplier(&mut func, &find_word);
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
                if i + 4 < chars.len() && chars[i] == '^' && chars[i + 1] == '(' && chars[i + 2] == '-' && chars[i + 3] == '1' && chars[i + 4] == ')'
                {
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 5;
                    continue;
                }
                if i + 2 < chars.len() && chars[i] == '^' && chars[i + 1] == '-' && chars[i + 2] == '1'
                {
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 3;
                    continue;
                }
                if i + 1 < chars.len() && chars[i] == '^' && chars[i + 1].is_ascii_digit()
                {
                    func.push(Str(word.clone()));
                    word.clear();
                    exp = chars[i + 1].to_string().parse::<u8>().unwrap();
                    i += 2;
                    continue;
                }
                func.push(Str(word.clone()));
                word.clear();
            }
            if exp != 0 && c != '(' && c != ')'
            {
                func.push(Str("^".to_string()));
                func.push(Num((exp.into(), 0.0)));
                exp = 0;
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
                        if i + 1 != chars.len() && (chars[i + 1] == '(' || chars[i + 1] == '-')
                        {
                            func.push(Num((-1.0, 0.0)));
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
                '^' => func.push(Str('^'.to_string())),
                '(' =>
                {
                    place_multiplier(&mut func, &find_word);
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
                'π' =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Num((if neg { -PI } else { PI }, 0.0)));
                    neg = false;
                }
                'τ' =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Num((if neg { -TAU } else { TAU }, 0.0)));
                    neg = false;
                }
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
    if exp != 0
    {
        func.push(Str("^".to_string()));
        func.push(Num((exp.into(), 0.0)));
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
// noinspection RsTypeCheck
fn place_multiplier(func:&mut Vec<Complex>, find_word:&bool)
{
    if let Some(Str(s)) = func.last()
    {
        if !find_word && (s == ")" || s == "x" || s == "y")
        {
            func.push(Str('*'.to_string()))
        }
    }
    else if let Num(_) = func.last().unwrap_or(&Str("".to_string()))
    {
        func.push(Str('*'.to_string()))
    }
}