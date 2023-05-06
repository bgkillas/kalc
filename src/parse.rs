pub fn get_func(input:String) -> Vec<String>
{
    let mut count = 0;
    let mut func:Vec<String> = Vec::new();
    let mut word:String = String::new();
    let chars = input.chars().collect::<Vec<char>>();
    for (i, c) in chars.iter().enumerate()
    {
        if *c == 'x' || *c == 'y'
        {
            if i != 0 && (chars[i - 1].is_ascii_digit() || chars[i - 1] == 'x' || chars[i - 1] == 'y')
            {
                if !word.is_empty()
                {
                    func.push(word.clone());
                }
                func.push("*".to_string());
                word.clear();
            }
            func.push(c.to_string());
        }
        else if *c == 'e'
        {
            if !word.is_empty()
            {
                func.push(word.clone());
                word.clear();
            }
            func.push(std::f64::consts::E.to_string());
        }
        else if *c == 'i'
        {
            if i != 0 && chars[i - 1] == 'p'
            {
                if !word.is_empty()
                {
                    func.push(word.clone());
                    word.clear()
                }
                func.push(std::f64::consts::PI.to_string());
            }
            else
            {
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
                println!("Error: Invalid number");
                func.clear();
                func.push("0".to_string());
                return func;
            }
            word.push(*c);
        }
        else if *c == '-' && chars[i + 1] == '('
        {
            func.push((-1.0).to_string());
            func.push("*".to_string());
        }
        else if c.is_ascii_alphabetic()
        {
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
            if *c == '-' && word.is_empty()
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
            if chars[i] == ')' && chars[i - if chars[i - 2] == 'p' { 3 } else { 2 }] == '('
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
        println!("Error: Parentheses mismatch");
        func.clear();
        func.push("0".to_string());
    }
    let first = func.first().unwrap().chars().next().unwrap();
    if first == '*' || first == '/' || first == '+' || first == '-' || first == '^'
    {
        func.insert(0, "1".to_string());
    }
    let last = func.last().unwrap().chars().last().unwrap();
    if last == '*' || last == '/' || last == '+' || last == '-' || last == '^' || last.is_ascii_alphabetic()
    {
        func.push("1".to_string());
    }
    if last == 'x' || last == 'y' || last == 'i'
    {
        func.pop();
    }
    func
}