use std::f64::consts::{E, PI};
pub fn get_func(input:&str, done:bool) -> Result<Vec<String>, ()>
{
    let mut count:i32 = 0;
    let mut func:Vec<String> = Vec::new();
    let mut word:String = String::new();
    let chars = input.chars().collect::<Vec<char>>();
    let mut abs = true;
    for (i, c) in chars.iter().enumerate()
    {
        if (*c == 'x' || *c == 'y') && !(chars.len() > i + 1 && chars[i + 1] == 'p')
        {
            if !word.is_empty()
            {
                func.push(word.clone());
            }
            if i != 0 && (word.parse::<f64>().is_ok() || word.ends_with('i') || chars[i - 1] == 'x' || chars[i - 1] == 'y')
            {
                func.push("*".to_string());
            }
            word.clear();
            word = c.to_string();
        }
        else if *c == '|'
        {
            if !word.is_empty()
            {
                func.push(word.clone());
                word.clear();
            }
            if abs
            {
                func.push("abs".to_string());
                func.push("(".to_string());
                count += 1;
            }
            else
            {
                func.push(")".to_string());
                count -= 1;
            }
            abs = !abs;
        }
        else if *c == '!'
        {
            if i != 0 && chars[i - 1].is_ascii_digit()
            {
                func.push("fact".to_string());
                func.push("(".to_string());
                func.push(word.clone());
                word.clear();
                func.push(")".to_string());
            }
            else if i != chars.len() - 1 && chars[i + 1].is_ascii_digit()
            {
                func.push(word.clone());
                word.clear();
                func.push("subfact".to_string());
                func.push("(".to_string());
                count += 1;
            }
        }
        else if *c == 'e' && (i == 0 || !chars[i - 1].is_ascii_alphabetic() || chars[i - 1] == 'x' || chars[i - 1] == 'i') && !(chars.len() > i + 2 && chars[i + 2] == 'p')
        {
            if word == "-"
            {
                word = "-1".to_string();
            }
            if !word.is_empty()
            {
                func.push(word.clone());
                func.push("*".to_string());
                word.clear();
            }
            word = E.to_string();
        }
        else if *c == 'π'
        {
            if word == "-"
            {
                word = "-1".to_string();
            }
            if !word.is_empty()
            {
                func.push(word.clone());
                func.push("*".to_string());
                word.clear();
            }
            word = PI.to_string();
        }
        else if *c == 'i'
        {
            if i != 0 && chars[i - 1] == 'p'
            {
                if word == "-"
                {
                    word = "-1".to_string();
                }
                if !word.is_empty()
                {
                    func.push(word.clone());
                    func.push("*".to_string());
                    word.clear()
                }
                word = PI.to_string();
            }
            else if i == chars.len() - 1 || !chars[i + 1].is_ascii_alphabetic()
            {
                if i != 0 && (!word.is_empty() && word != "(")
                {
                    func.push(word.clone());
                    func.push("*".to_string());
                    word.clear();
                }
                if word.is_empty()
                {
                    word = "1".to_string();
                }
                word.push(*c);
            }
            else if chars[i + 1].is_ascii_alphabetic()
            {
                word.push(*c);
            }
        }
        else if *c == 'p'
        {
            if i == chars.len() - 1 || chars[i + 1] != 'i'
            {
                word.push(*c);
            }
            continue;
        }
        else if *c == '.'
        {
            if word.is_empty()
            {
                word = "0".to_string();
            }
            if word.contains('.')
            {
                if done
                {
                    println!("Error: Invalid number");
                }
                func.clear();
                func.push("0".to_string());
                return Ok(func);
            }
            word.push(*c);
        }
        else if chars.len() > i + 1 && *c == '-' && chars[i + 1] == '(' && (i == 0 || chars[i - 1] == '(')
        {
            func.push((-1.0).to_string());
            func.push("*".to_string());
        }
        else if c.is_ascii_alphabetic()
        {
            if !word.is_empty() && (((word.chars().next().unwrap().is_ascii_digit() || word.starts_with('-')) && word.ends_with('i')) || word.chars().last().unwrap().is_ascii_digit())
            {
                func.push(word.clone());
                func.push("*".to_string());
                word.clear();
            }
            if word == "-"
            {
                word = "-1".to_string();
                func.push(word.clone());
                func.push("*".to_string());
                word.clear();
            }
            word.push(*c);
        }
        else if c.is_ascii_digit()
        {
            if i != 0 && chars[i - 1].is_ascii_alphabetic()
            {
                func.push(word.clone());
                if word == "1i" || word == "x" || word == "y"
                {
                    func.push("*".to_string());
                }
                word.clear();
            }
            word.push(*c);
        }
        else
        {
            if *c == '('
            {
                count += 1;
            }
            else if *c == ')'
            {
                count -= 1;
            }
            if *c == '-' && word.is_empty() && i != 0 && chars[i - 1] != ')' && chars[i - 1] != 'x' && chars[i - 1] != 'y'
            {
                word.push(*c);
                continue;
            }
            if *c == '('
               && i != 0
               && (chars[i - 1].is_ascii_digit()
                   || (chars[i - 1] == 'e' && (i == 1 || chars[i - 2] != 'r'))
                   || chars[i - 1] == 'i'
                   || chars[i - 1] == ')'
                   || chars[i - 1] == 'x'
                   || chars[i - 1] == 'y')
            {
                if !word.is_empty()
                {
                    func.push(word.clone());
                }
                func.push("*".to_string());
                word.clear();
            }
            if i != 0 && chars[i - 1] == '(' && *c == '+'
            {
                continue;
            }
            if !word.is_empty()
            {
                func.push(word.clone());
            }
            func.push(c.to_string());
            word.clear();
            if chars[i] == ')' && i < chars.len() - 1 && chars[i + 1].is_ascii_alphanumeric()
            {
                func.push("*".to_string());
            }
        }
    }
    if !word.is_empty()
    {
        func.push(word);
    }
    if count != 0
    {
        if count > 0
        {
            for _ in 0..count
            {
                func.push(")".to_string());
            }
        }
        else
        {
            for _ in 0..count.abs()
            {
                func.insert(0, "(".to_string());
            }
        }
    }
    let mut i = 0;
    let mut double = false;
    let mut location = 0;
    loop
    {
        if func.is_empty() || i >= func.len() - 1
        {
            break;
        }
        if func[i] == "(" && func[i + 1] == "("
        {
            double = true;
            location = i;
            i += 1;
            continue;
        }
        if double && func[i] == ")" && func[i + 1] == ")"
        {
            double = false;
            func.remove(location);
            func.remove(i);
            continue;
        }
        if func[i] == "(" && func[i + 1] == ")"
        {
            func.remove(i);
            func.remove(i);
            continue;
        }
        if i < func.len() - 2 && func[i] == "(" && func[i + 2] == ")"
        {
            func.remove(i);
            func.remove(i + 1);
            continue;
        }
        i += 1;
    }
    if func.is_empty()
    {
        return Err(());
    }
    let first = func.first().unwrap().to_string();
    if first == "*" || first == "/" || first == "^" || first == "-"
    {
        func.insert(0, "0".to_string());
    }
    if first == "+"
    {
        func.remove(0);
        if func.is_empty()
        {
            return Err(());
        }
    }
    let last = func.last().unwrap().chars().last().unwrap();
    if last == '*' || last == '/' || last == '^' || last == '+' || last == '-'
    {
        func.pop();
    }
    Ok(func)
}