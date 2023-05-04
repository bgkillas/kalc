use std::env::args;
use std::io::{BufRead, stdin, stdout, Write};
use std::time::Instant;
fn main()
{
    let mut start;
    let mut line;
    let mut input;
    if args().len() > 1
    {
        input = args().nth(1).unwrap().replace("pi", &std::f64::consts::PI.to_string()).replace('e', &std::f64::consts::E.to_string());
        println!("{}", (do_math(get_func(input)).parse::<f64>().unwrap() * 1e9).round() / 1e9);
        return;
    }
    loop
    {
        line = stdin().lock().lines().next();
        start = Instant::now();
        if line.as_ref().is_none()
        {
            break;
        }
        input = line.unwrap().unwrap();
        if input == "exit"
        {
            break;
        }
        if input == "clear"
        {
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        println!("{}", start.elapsed().as_nanos());
        println!("{}", (do_math(get_func(input)).parse::<f64>().unwrap() * 1e9).round() / 1e9);
    }
}
fn get_func(input:String) -> Vec<String>
{
    let mut count = 0;
    let mut func:Vec<String> = Vec::new();
    let mut word:String = String::new();
    let chars = input.chars().collect::<Vec<char>>();
    for i in 0..chars.len()
    {
        if chars[i] == 'e'
        {
            func.push(std::f64::consts::E.to_string());
            continue;
        }
        if chars[i] == 'i' && i != 0 && chars[i - 1] == 'p'
        {
            func.push(std::f64::consts::PI.to_string());
            continue;
        }
        if chars[i].is_whitespace() || chars[i] == 'p'
        {
            continue;
        }
        if chars[i] == '.'
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
            word.push(chars[i]);
        }
        else if chars[i] == '-'
        {
            func.push((-1.0).to_string());
            func.push("*".to_string());
        }
        else if chars[i].is_ascii_alphabetic()
        {
            word.push(chars[i])
        }
        else if chars[i].is_ascii_digit()
        {
            if i != 0 && chars[i - 1].is_ascii_alphabetic()
            {
                func.push(word.clone());
                word.clear();
            }
            word.push(chars[i])
        }
        else
        {
            if chars[i] == '('
            {
                if i != 0 && chars[i - 1].is_ascii_digit()
                {
                    func.push(word.clone());
                    func.push("*".to_string());
                    word.clear();
                }
                count += 1;
            }
            if !word.is_empty()
            {
                func.push(word.clone());
                func.push(chars[i].to_string());
                word.clear();
            }
            else
            {
                func.push(chars[i].to_string());
            }
            if chars[i] == ')'
            {
                if i < chars.len() - 1 && chars[i + 1].is_ascii_digit()
                {
                    func.push("*".to_string());
                }
                count -= 1;
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
    let last = func.last().unwrap().chars().last().unwrap();
    if last == '*' || last == '/' || last == '+' || last == '-' || last == '^' || last.is_ascii_alphabetic()
    {
        func.push("0".to_string());
    }
    func
}
fn do_math(func:Vec<String>) -> String
{
    let mut func = func;
    let mut i = 0;
    while i < func.len()
    {
        if func[i] == "("
        {
            let mut j = i + 1;
            let mut count = 1;
            while count > 0
            {
                match func[j].as_str()
                {
                    "(" => count += 1,
                    ")" => count -= 1,
                    _ =>
                    {}
                }
                j += 1;
            }
            func[i] = do_math(func[i + 1..j - 1].to_vec());
            func.drain(i + 1..j);
        }
        i += 1;
    }
    i = 0;
    while i < func.len()
    {
        match func[i].as_str()
        {
            "sin" => func[i] = (func[i + 1].parse::<f64>().unwrap().sin()).to_string(),
            "cos" => func[i] = (func[i + 1].parse::<f64>().unwrap().cos()).to_string(),
            "tan" => func[i] = (func[i + 1].parse::<f64>().unwrap().tan()).to_string(),
            "asin" => func[i] = (func[i + 1].parse::<f64>().unwrap().asin()).to_string(),
            "acos" => func[i] = (func[i + 1].parse::<f64>().unwrap().acos()).to_string(),
            "atan" => func[i] = (func[i + 1].parse::<f64>().unwrap().atan()).to_string(),
            "sinh" => func[i] = (func[i + 1].parse::<f64>().unwrap().sinh()).to_string(),
            "cosh" => func[i] = (func[i + 1].parse::<f64>().unwrap().cosh()).to_string(),
            "tanh" => func[i] = (func[i + 1].parse::<f64>().unwrap().tanh()).to_string(),
            "asinh" => func[i] = (func[i + 1].parse::<f64>().unwrap().asinh()).to_string(),
            "acosh" => func[i] = (func[i + 1].parse::<f64>().unwrap().acosh()).to_string(),
            "atanh" => func[i] = (func[i + 1].parse::<f64>().unwrap().atanh()).to_string(),
            "ln" => func[i] = (func[i + 1].parse::<f64>().unwrap().ln()).to_string(),
            "log" => func[i] = (func[i + 1].parse::<f64>().unwrap().log10()).to_string(),
            "sqrt" => func[i] = (func[i + 1].parse::<f64>().unwrap().sqrt()).to_string(),
            "abs" => func[i] = (func[i + 1].parse::<f64>().unwrap().abs()).to_string(),
            "dg" => func[i] = (func[i + 1].parse::<f64>().unwrap().to_degrees()).to_string(),
            "rd" => func[i] = (func[i + 1].parse::<f64>().unwrap().to_radians()).to_string(),
            "cbrt" => func[i] = (func[i + 1].parse::<f64>().unwrap().cbrt()).to_string(),
            _ =>
            {
                i += 1;
                continue;
            }
        }
        func.remove(i + 1);
        i += 1;
    }
    i = 0;
    while i < func.len()
    {
        if func[i] == "^"
        {
            func[i] = (func[i - 1].parse::<f64>().unwrap().powf(func[i + 1].parse::<f64>().unwrap())).to_string();
            func.remove(i + 1);
            func.remove(i - 1);
            i -= 1;
        }
        i += 1;
    }
    i = 0;
    while i < func.len()
    {
        match func[i].as_str()
        {
            "*" =>
            {
                func[i] = (func[i - 1].parse::<f64>().unwrap() * func[i + 1].parse::<f64>().unwrap()).to_string();
                func.remove(i + 1);
                func.remove(i - 1);
                i -= 1;
            }
            "/" =>
            {
                func[i] = (func[i - 1].parse::<f64>().unwrap() / func[i + 1].parse::<f64>().unwrap()).to_string();
                func.remove(i + 1);
                func.remove(i - 1);
                i -= 1;
            }
            _ =>
            {}
        }
        i += 1;
    }
    i = 0;
    while i < func.len()
    {
        match func[i].as_str()
        {
            "+" =>
            {
                func[i] = (func[i - 1].parse::<f64>().unwrap() + func[i + 1].parse::<f64>().unwrap()).to_string();
                func.remove(i + 1);
                func.remove(i - 1);
                i -= 1;
            }
            "-" =>
            {
                func[i] = (func[i - 1].parse::<f64>().unwrap() - func[i + 1].parse::<f64>().unwrap()).to_string();
                func.remove(i + 1);
                func.remove(i - 1);
                i -= 1;
            }
            _ =>
            {}
        }
        i += 1;
    }
    func[0].clone()
}