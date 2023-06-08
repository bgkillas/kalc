use rug::{Complex, Float};
use rug::float::Constant::Pi;
use crate::math::NumStr;
use crate::math::NumStr::{Num, Str};
pub fn get_func(input:&str, prec:u32) -> Result<Vec<NumStr>, ()>
{
    let mut count:i32 = 0;
    let mut exp = 0;
    let mut func:Vec<NumStr> = Vec::new();
    let mut word = String::new();
    let mut find_word = false;
    let mut abs = true;
    let mut neg = false;
    let mut i = 0;
    let chars = input.chars().collect::<Vec<char>>();
    let (mut c, mut deci);
    let n1 = Complex::with_val(prec, -1.0);
    let pi = Complex::with_val(prec, Pi);
    'outer: while i < input.len()
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
            deci = false;
            for c in chars[i..].iter()
            {
                match c
                {
                    '0'..='9' =>
                    {
                        word.push(*c);
                    }
                    '.' if !deci =>
                    {
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
            func.push(Num(Complex::with_val(prec, Complex::parse(word.as_bytes()).unwrap())));
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
                place_multiplier(&mut func, &find_word);
                if neg
                {
                    func.push(Num(n1.clone()));
                    func.push(Str('*'.to_string()));
                    neg = false;
                }
                match c
                {
                    'E' =>
                    {
                        func.push(Num(Complex::with_val(prec, 10.0)));
                        func.push(Str('^'.to_string()));
                    }
                    'x' | 'y' =>
                    {
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
                        if i + 1 != chars.len() && (chars[i + 1] == 'n' || chars[i + 1] == 'm')
                        {
                            word.push(c);
                            find_word = true;
                        }
                        else
                        {
                            func.push(Num(Complex::with_val(prec, (0.0, 1.0))));
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
                func.push(Num(Complex::with_val(prec, exp as f64)));
                exp = 0;
            }
            match c
            {
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
                '/' if i != 0 && i + 1 != chars.len() => func.push(Str('/'.to_string())),
                '+' if i != 0 && i + 1 != chars.len() => func.push(Str('+'.to_string())),
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
                    if i == 0 || !(chars[i - 1] != 'E' && (chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == ')'))
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
                    count += 1;
                    place_multiplier(&mut func, &find_word);
                    func.push(Str("(".to_string()))
                }
                ')' if i != 0 =>
                {
                    count -= 1;
                    func.push(Str(")".to_string()))
                }
                '|' =>
                {
                    if i + 1 != chars.len() && chars[i + 1] == '|'
                    {
                        func.push(Str("||".to_string()));
                        i += 2;
                        continue;
                    }
                    else if abs
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
                    if i + 1 < chars.len() && chars[i + 1] == '='
                    {
                        func.push(Str("!=".to_string()));
                    }
                    else if i != 0 && (chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == '(' || chars[i - 1] == ')')
                    {
                        if let Num(a) = func.clone().last().unwrap()
                        {
                            if a.real() < &0.0
                            {
                                func.pop();
                                func.push(Num(Complex::with_val(prec, (-a.real(), a.imag()))));
                                func.insert(func.len() - 1, Num(n1.clone()));
                                func.insert(func.len() - 1, Str("*".to_string()));
                            }
                        }
                        if chars[i - 1] == ')'
                        {
                            let mut count = 0;
                            for (j, c) in chars[..i].iter().enumerate().rev()
                            {
                                if c == &'('
                                {
                                    count -= 1;
                                }
                                else if c == &')'
                                {
                                    count += 1;
                                }
                                if count == 0
                                {
                                    func.insert(j + 1, Str("(".to_string()));
                                    func.insert(j + 1, Str("fact".to_string()));
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
                    else if i != chars.len() - 1 && (chars[i + 1].is_ascii_alphanumeric() || chars[i + 1] == '(' || chars[i + 1] == ')' || chars[i + 1] == '-')
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
                'π' =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Num(if neg { -1 * pi.clone() } else { pi.clone() }));
                    neg = false;
                }
                'τ' =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Num(if neg { -2 * pi.clone() } else { 2 * pi.clone() }));
                    neg = false;
                }
                ',' if i != 0 && i + 1 != chars.len() => func.push(Str(','.to_string())),
                '%' if i != 0 && i + 1 != chars.len() => func.push(Str('%'.to_string())),
                _ => (),
            }
        }
        i += 1;
    }
    let bracket = Str(if count > 0 { ")".to_string() } else { "(".to_string() });
    for _ in 0..count.abs()
    {
        func.push(bracket.to_owned());
    }
    if !abs
    {
        func.push(Str(")".to_string()))
    }
    if exp != 0
    {
        func.push(Str("^".to_string()));
        func.push(Num(Complex::with_val(prec, exp as f64)));
    }
    if neg
    {
        func.push(Num(n1));
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
    // for i in &func
    // {
    //     match i
    //     {
    //         Str(s) => println!("{}", s),
    //         Num(n) => println!("{}", n),
    //     }
    // }
    Ok(func)
}
fn place_multiplier(func:&mut Vec<NumStr>, find_word:&bool)
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
pub fn input_var(input:&str, vars:&[[String; 2]]) -> String
{
    let chars = input.chars().collect::<Vec<char>>();
    let mut output = String::new();
    let (mut not_pushed, mut c, mut k, mut j, mut v, mut temp, mut count, mut split, mut value);
    let mut i = 0;
    while i < chars.len()
    {
        c = chars[i];
        not_pushed = true;
        for var in vars
        {
            j = i;
            if var[0].contains('(') && input.contains('(') && i + var[0].len() - 1 <= input.len() && input[i..i + var[0].len() - 1].split('(').next() == var[0].split('(').next()
            {
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
                if input[j..i + 1] == var[0]
                {
                    not_pushed = false;
                    output.push('(');
                    output.push_str(&input_var(&var[1], vars));
                    output.push(')');
                }
                else if j == 0 || !chars[j - 1].is_ascii_alphabetic()
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
                    if input.contains(',') && var[0].contains(',') && chars.len() > 4
                    {
                        not_pushed = false;
                        output.push('(');
                        temp = &input[j + var[0].split('(').next().unwrap().len() + 1..i + 1];
                        if temp.ends_with(')')
                        {
                            temp = &temp[..temp.len() - 1];
                        }
                        split = temp.split(',');
                        value = input_var(&var[1], vars).clone();
                        for i in (0..split.clone().count()).rev()
                        {
                            value = value.replace(v[v.len() - 2 * (i + 1)], &format!("({})", input_var(split.next().unwrap(), vars)));
                        }
                        output.push_str(&value);
                        output.push(')');
                    }
                    else
                    {
                        not_pushed = false;
                        output.push('(');
                        temp = &input[j + var[0].split('(').next().unwrap().len() + 1..i + 1];
                        if temp.ends_with(')')
                        {
                            temp = &temp[..temp.len() - 1];
                        }
                        output.push_str(&input_var(&var[1], vars).replace(v[v.len() - 2], &format!("({})", input_var(temp, vars))));
                        output.push(')');
                    }
                }
            }
            else if !(i + var[0].len() > input.len() || input[i..i + var[0].len()] != var[0])
                      && (i + 1 == chars.len() || chars[i + 1] != '(')
                      && (j == 0 || !chars[j - 1].is_ascii_alphabetic())
                      && (var[0].len() - 1 + i == chars.len() - 1 || !chars[i + 1 + var[0].len() - 1].is_ascii_alphabetic())
            {
                i += var[0].len() - 1;
                not_pushed = false;
                output.push('(');
                output.push_str(&input_var(&var[1], vars));
                output.push(')');
            }
        }
        if not_pushed
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
pub fn get_vars(allow_vars:bool, prec:u32) -> Vec<[String; 2]>
{
    if allow_vars
    {
        let pi = Float::with_val(prec, Pi);
        let tau:Float = pi.clone() * 2;
        let phi:Float = (1 + Float::with_val(prec, 5).sqrt()) / 2;
        vec![["c".to_string(), "299792458".to_string()],
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
             ["e".to_string(), Float::with_val(prec, 1).exp().to_string()],
             ["pi".to_string(), pi.to_string()],
             ["tau".to_string(), tau.to_string()]]
    }
    else
    {
        vec![]
    }
}