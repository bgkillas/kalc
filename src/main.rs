use std::env::args;
use std::io::{BufRead, stdin, stdout, Write};
use std::time::Instant;
fn main()
{
    let mut start;
    if args().len() > 1
    {
        let func = get_func(args().nth(1).unwrap());
        if func.contains(&"x".to_string())
        {
            let mut modified;
            for n in -100000..=100000
            {
                modified = func.clone();
                for i in &mut modified
                {
                    if i == "x"
                    {
                        *i = (n as f64 / 10000.0).to_string();
                    }
                }
                let mut num = do_math(modified);
                if num.parse::<f64>().is_ok()
                {
                    num = ((num.parse::<f64>().unwrap() * 1e9).round() / 1e9).to_string();
                }
                println!("{}:{}", n as f64 / 10000.0, num);
            }
            return;
        }
        start = Instant::now();
        println!("{}", (do_math(func).parse::<f64>().unwrap() * 1e9).round() / 1e9);
        println!("{}", start.elapsed().as_nanos());
        return;
    }
    let mut line;
    let mut input;
    loop
    {
        line = stdin().lock().lines().next();
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
        start = Instant::now();
        let mut num = do_math(get_func(input));
        if num.parse::<f64>().is_ok()
        {
            num = ((num.parse::<f64>().unwrap() * 1e9).round() / 1e9).to_string();
        }
        println!("{}", num);
        println!("{}", start.elapsed().as_nanos());
    }
}
fn get_func(input:String) -> Vec<String>
{
    let mut count = 0;
    let mut func:Vec<String> = Vec::new();
    let mut word:String = String::new();
    let chars = input.chars().collect::<Vec<char>>();
    for (i, c) in chars.iter().enumerate()
    {
        if *c == 'x'
        {
            if i != 0 && chars[i - 1].is_ascii_digit()
            {
                func.push(word.clone());
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
                            - if n == "x" || n == &std::f64::consts::PI.to_string() || n == &std::f64::consts::E.to_string()
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
    let last = func.last().unwrap().chars().last().unwrap();
    if last == '*' || last == '/' || last == '+' || last == '-' || last == '^' || last.is_ascii_alphabetic()
    {
        func.push("0".to_string());
    }
    if last == 'x' || last == 'i'
    {
        func.pop();
    }
    func
}
fn do_math(func:Vec<String>) -> String
{
    let mut func = func;
    let mut i = 0;
    while i < func.len() - 1
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
    for i in 0..func.len() - 1
    {
        if func[i].len() > 1
        {
            let (arg1, arg2) = parse(&func[i + 1]);
            match func[i].as_str()
            {
                "sin" => func[i] = sin(arg1, arg2).to_string(),
                "cos" => func[i] = cos(arg1, arg2).to_string(),
                "tan" => func[i] = tan(arg1, arg2).to_string(),
                "asin" => func[i] = (func[i + 1].parse::<f64>().unwrap().asin()).to_string(),
                "acos" => func[i] = (func[i + 1].parse::<f64>().unwrap().acos()).to_string(),
                "atan" => func[i] = (func[i + 1].parse::<f64>().unwrap().atan()).to_string(),
                "sinh" => func[i] = (func[i + 1].parse::<f64>().unwrap().sinh()).to_string(),
                "cosh" => func[i] = (func[i + 1].parse::<f64>().unwrap().cosh()).to_string(),
                "tanh" => func[i] = (func[i + 1].parse::<f64>().unwrap().tanh()).to_string(),
                "asinh" => func[i] = (func[i + 1].parse::<f64>().unwrap().asinh()).to_string(),
                "acosh" => func[i] = (func[i + 1].parse::<f64>().unwrap().acosh()).to_string(),
                "atanh" => func[i] = (func[i + 1].parse::<f64>().unwrap().atanh()).to_string(),
                "ln" => func[i] = ln(arg1, arg2).to_string(),
                "log" => func[i] = (func[i + 1].parse::<f64>().unwrap().log10()).to_string(),
                "sqrt" => func[i] = pow(arg1, arg2, 0.5, 0.0).to_string(),
                "abs" => func[i] = abs(arg1, arg2).to_string(),
                "dg" => func[i] = (func[i + 1].parse::<f64>().unwrap().to_degrees()).to_string(),
                "rd" => func[i] = (func[i + 1].parse::<f64>().unwrap().to_radians()).to_string(),
                "cbrt" => func[i] = pow(arg1, arg2, 1.0 / 3.0, 0.0).to_string(),
                _ =>
                {
                    continue;
                }
            }
            func.remove(i + 1);
        }
    }
    i = 1;
    while i < func.len() - 1
    {
        if func[i] != "^"
        {
            i += 1;
            continue;
        }
        if func[i - 1] == "0"
        {
            func[i] = "0".to_string();
            func.remove(i + 1);
            func.remove(i - 1);
            continue;
        }
        let (a, b) = parse(&func[i - 1]);
        let (c, d) = parse(&func[i + 1]);
        func[i] = pow(a, b, c, d);
        func.remove(i + 1);
        func.remove(i - 1);
    }
    i = 1;
    while i < func.len() - 1
    {
        if !(func[i] == "*" || func[i] == "/")
        {
            i += 1;
            continue;
        }
        if func[i + 1] == "0" && func[i] == "/"
        {
            func[i] = "0".to_string();
            func.remove(i + 1);
            func.remove(i - 1);
            continue;
        }
        let first_im = func[i - 1].contains('i');
        let second_im = func[i + 1].contains('i');
        if first_im || second_im
        {
            let (a, b) = parse(&func[i - 1]);
            let (c, d) = parse(&func[i + 1]);
            match func[i].as_str()
            {
                "*" => func[i] = mul(a, b, c, d),
                "/" => func[i] = div(a, b, c, d),
                _ =>
                {
                    i += 1;
                    continue;
                }
            }
            func.remove(i + 1);
            func.remove(i - 1);
            continue;
        }
        match func[i].as_str()
        {
            "*" => func[i] = (func[i - 1].parse::<f64>().unwrap() * func[i + 1].parse::<f64>().unwrap()).to_string(),
            "/" => func[i] = (func[i - 1].parse::<f64>().unwrap() / func[i + 1].parse::<f64>().unwrap()).to_string(),
            _ =>
            {
                i += 1;
                continue;
            }
        }
        func.remove(i + 1);
        func.remove(i - 1);
    }
    i = 1;
    while i < func.len() - 1
    {
        if func[i + 1].contains('i') || func[i - 1].contains('i')
        {
            i += 1;
            continue;
        }
        match func[i].as_str()
        {
            "+" => func[i] = (func[i - 1].parse::<f64>().unwrap() + func[i + 1].parse::<f64>().unwrap()).to_string(),
            "-" => func[i] = (func[i - 1].parse::<f64>().unwrap() - func[i + 1].parse::<f64>().unwrap()).to_string(),
            _ =>
            {
                i += 1;
                continue;
            }
        }
        func.remove(i + 1);
        func.remove(i - 1);
    }
    func.join("")
}
fn parse(num:&String) -> (f64, f64)
{
    let im = num.contains('i');
    let mut index = None;
    if let Some(i) = num.find('+')
    {
        index = Some(i);
    }
    else if let Some(i) = num.rfind('-')
    {
        if i != 0
        {
            index = Some(i);
        }
    }
    if let Some(i) = index
    {
        (num[..i].parse::<f64>().unwrap(), num[i..].replace('i', "").parse::<f64>().unwrap())
    }
    else if im
    {
        (0.0, num[..num.len() - 1].parse::<f64>().unwrap())
    }
    else
    {
        (num.parse::<f64>().unwrap(), 0.0)
    }
}
fn mul(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)(c+di)=(ac-bd)+i(ad+bc)
    let im = (a * d + b * c).to_string();
    let sign = if im.contains('-') { "" } else { "+" };
    (a * c - b * d).to_string() + sign + im.as_str() + "i"
}
fn div(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2)+i(bc-ad)/(c^2+d^2)
    let im = b * c - a * d;
    let den = c * c + d * d;
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    ((a * c + b * d) / den).to_string() + sign + (im / den).to_string().as_str() + "i"
}
fn pow(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)^(c+di)=e^((c+di)(ln(a^2+b^2)/2+i*atan2(b,a)))
    // re=e^(c*ln(a^2+b^2)/2-d*atan2(b,a))*cos(d*ln(a^2+b^2)/2+c*atan2(b,a))
    // im=e^(c*ln(a^2+b^2)/2-d*atan2(b,a))*sin(d*ln(a^2+b^2)/2+c*atan2(b,a))
    let e = std::f64::consts::E;
    let r = c * (b.atan2(a)) + d * (0.5 * (a * a + b * b).ln());
    let m = e.powf(c * (0.5 * (a * a + b * b).ln()) - d * (b.atan2(a)));
    let im = m * r.sin();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    ((m * r.cos() * 1e15).round() / 1e15).to_string() + sign + ((im * 1e15).round() / 1e15).to_string().as_str() + "i"
}
fn abs(a:f64, b:f64) -> String
{
    // abs(a+bi)=sqrt(a^2+b^2)
    (a * a + b * b).sqrt().to_string()
}
fn ln(a:f64, b:f64) -> String
{
    // ln(a+bi)=ln(a^2+b^2)/2+i*atan2(b,a)
    let i = b.atan2(a);
    let sign = if i.to_string().contains('-') { "" } else { "+" };
    (0.5 * (a * a + b * b).ln()).to_string() + sign + i.to_string().as_str() + "i"
}
fn sin(a:f64, b:f64) -> String
{
    // sin(a+bi)=sin(a)cosh(b)+i*cos(a)sinh(b)
    let im = a.cos() * b.sinh();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.sin() * b.cosh()).to_string() + sign + im.to_string().as_str() + "i"
}
fn cos(a:f64, b:f64) -> String
{
    // cos(a+bi)=cos(a)cosh(b)-i*sin(a)sinh(b)
    let im = -a.sin() * b.sinh();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.cos() * b.cosh()).to_string() + sign + im.to_string().as_str() + "i"
}
fn tan(a:f64, b:f64) -> String
{
    // tan(a+bi)=sin(a+bi)/cos(a+bi)
    div(a.sin() * b.cosh(), a.cos() * b.sinh(), a.cos() * b.cosh(), -a.sin() * b.sinh())
}