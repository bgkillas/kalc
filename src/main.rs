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
        println!("{}", (do_math(get_func(input)) * 1e9).round() / 1e9);
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
        input = line.unwrap().unwrap().replace("pi", &std::f64::consts::PI.to_string());
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
        input = input.replace('e', &std::f64::consts::E.to_string());
        println!("{}", start.elapsed().as_nanos());
        println!("{}", (do_math(get_func(input)) * 1e9).round() / 1e9);
    }
}
fn get_func(input:String) -> Vec<String>
{
    let mut func:Vec<String> = Vec::new();
    let mut word:String = String::new();
    for c in input.chars()
    {
        if c.is_ascii_alphanumeric() || c == '.'
        {
            word += &c.to_string();
        }
        else if !word.is_empty()
        {
            func.push(word.clone());
            func.push(c.to_string());
            word.clear();
        }
        else
        {
            func.push(c.to_string());
        }
    }
    if !word.is_empty()
    {
        func.push(word.clone());
    }
    func
}
fn do_math(func:Vec<String>) -> f64
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
                if func[j] == "("
                {
                    count += 1;
                }
                else if func[j] == ")"
                {
                    count -= 1;
                }
                j += 1;
            }
            func[i] = do_math(func[i + 1..j - 1].to_vec()).to_string();
            func.drain(i + 1..j);
        }
        i += 1;
    }
    i = 0;
    while i < func.len()
    {
        match func[i].as_str()
        {
            "sin" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().sin()).to_string();
                func.remove(i + 1);
            }
            "cos" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().cos()).to_string();
                func.remove(i + 1);
            }
            "tan" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().tan()).to_string();
                func.remove(i + 1);
            }
            "asin" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().asin()).to_string();
                func.remove(i + 1);
            }
            "acos" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().acos()).to_string();
                func.remove(i + 1);
            }
            "atan" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().atan()).to_string();
                func.remove(i + 1);
            }
            "sinh" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().sinh()).to_string();
                func.remove(i + 1);
            }
            "cosh" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().cosh()).to_string();
                func.remove(i + 1);
            }
            "tanh" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().tanh()).to_string();
                func.remove(i + 1);
            }
            "asinh" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().asinh()).to_string();
                func.remove(i + 1);
            }
            "acosh" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().acosh()).to_string();
                func.remove(i + 1);
            }
            "atanh" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().atanh()).to_string();
                func.remove(i + 1);
            }
            "ln" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().ln()).to_string();
                func.remove(i + 1);
            }
            "log" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().log10()).to_string();
                func.remove(i + 1);
            }
            "sqrt" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().sqrt()).to_string();
                func.remove(i + 1);
            }
            "abs" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().abs()).to_string();
                func.remove(i + 1);
            }
            "dg" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().to_degrees()).to_string();
                func.remove(i + 1);
            }
            "rd" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().to_radians()).to_string();
                func.remove(i + 1);
            }
            "cbrt" =>
            {
                func[i] = (func[i + 1].parse::<f64>().unwrap().cbrt()).to_string();
                func.remove(i + 1);
            }
            _ =>
            {}
        }
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
    func[0].parse::<f64>().unwrap()
}