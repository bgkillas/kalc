pub fn get_func(input:&str, done:bool) -> Vec<String>
{
    let mut count:i32 = 0;
    let mut func:Vec<String> = Vec::new();
    let mut word:String = String::new();
    let chars = input.chars().collect::<Vec<char>>();
    for (i, c) in chars.iter().enumerate()
    {
        if *c == 'x' || *c == 'y'
        {
            if !word.is_empty()
            {
                func.push(word.clone());
            }
            word.clear();
            if i != 0 && (chars[i - 1].is_ascii_digit() || chars[i - 1] == 'x' || chars[i - 1] == 'y')
            {
                func.push("*".to_string());
            }
            func.push(c.to_string());
        }
        else if *c == 'e'
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
            func.push(std::f64::consts::E.to_string());
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
            func.push(std::f64::consts::PI.to_string());
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
                func.push(std::f64::consts::PI.to_string());
            }
            else
            {
                if !func.is_empty() && (func.last().unwrap() == "x" || func.last().unwrap() == "y")
                {
                    func.push("*".to_string());
                }
                if word.is_empty()
                {
                    word = "1".to_string();
                }
                word.push(*c);
            }
        }
        else if c.is_whitespace() || *c == 'p'
        {
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
                return func;
            }
            word.push(*c);
        }
        else if chars.len() > i + 1 && *c == '-' && chars[i + 1] == '('
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
            if *c == '-' && word.is_empty() && i == 0
            {
                word.push(*c);
                continue;
            }
            if *c == '(' && i != 0 && (chars[i - 1].is_ascii_digit() || chars[i - 1] == ')')
            {
                if !word.is_empty()
                {
                    func.push(word.clone());
                }
                func.push("*".to_string());
                word.clear();
            }
            if chars.len() > 2 && chars[i] == ')' && chars[i - if chars[i - 2] == 'p' { 3 } else { 2 }] == '('
            {
                let n = func.last().unwrap();
                func.remove(func.len()
                            - if n == "x" || n == "y" || n == &std::f64::consts::PI.to_string() || n == &std::f64::consts::E.to_string()
                            {
                                2
                            }
                            else
                            {
                                1
                            });
                continue;
            }
            if !word.is_empty()
            {
                func.push(word.clone());
            }
            func.push(c.to_string());
            word.clear();
            if chars[i] == ')' && i < chars.len() - 1 && chars[i + 1].is_ascii_digit()
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
    if func.is_empty()
    {
        return vec!["0".to_string()];
    }
    let first = func.first().unwrap().to_string();
    if first == "*" || first == "/" || first == "^" || first == "-"
    {
        func.insert(0, "0".to_string());
    }
    if first == "+"
    {
        func.remove(0);
    }
    if func.is_empty()
    {
        func.push("0".to_string());
    }
    let last = func.last().unwrap().chars().last().unwrap();
    if last == '*' || last == '/' || last == '+' || last == '-' || last == '^' || last.is_ascii_alphabetic()
    {
        func.push("0".to_string());
    }
    if last == 'x' || last == 'y' || last == 'i'
    {
        func.pop();
    }
    func
}