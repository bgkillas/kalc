use std::env::args;
use std::io::{BufRead, BufReader, stdin, stdout, Write};
use std::f64::consts::PI;
use std::f64::consts::E;
use console::{Key, Term};
#[cfg(target_os = "linux")]
use libc::{isatty, STDIN_FILENO};
use std::fs::{File, OpenOptions};
fn print_answer(func:Vec<String>)
{
    let num = do_math(func);
    let (a, b) = parse(&num);
    let a = (a * 1e9).round() / 1e9;
    let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "i";
    println!("{}{}",
             if a == 0.0 && !(b == "-0i" || b == "+0i" || b == "0i") { "".to_string() } else { a.to_string() },
             if b == "-0i" || b == "+0i" || b == "0i" { "".to_string() } else { b });
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) => c,
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        _ => read_single_char(),
    }
}
fn main()
{
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
                let num = do_math(modified);
                let (a, b) = parse(&num);
                let a = (a * 1e9).round() / 1e9;
                let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "i";
                println!("{} {}{}",
                         n as f64 / 10000.0,
                         if a == 0.0 { "".to_string() } else { a.to_string() },
                         if b == "-0i" || b == "+0i" { "".to_string() } else { b });
            }
            return;
        }
        print_answer(func);
        return;
    }
    let line;
    let mut input = String::new();
    #[cfg(target_os = "linux")]
    if !unsafe { isatty(STDIN_FILENO) != 0 }
    {
        line = stdin().lock().lines().next();
        if line.as_ref().is_none()
        {
            return;
        }
        input = line.unwrap().unwrap();
        if input.is_empty()
        {
            return;
        }
        print_answer(get_func(input));
        return;
    }
    #[cfg(target_os = "linux")]
    let file_path:&str = &(std::env::var_os("HOME").unwrap().to_str().unwrap().to_owned() + "/.config/calc.history");
    #[cfg(target_os = "windows")]
    let file_path = "C:\\Users\\%USERNAME%\\AppData\\Roaming\\calc.history";
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut var:Vec<Vec<char>> = Vec::new();
    loop
    {
        input.clear();
        print!("> ");
        stdout().flush().unwrap();
        let mut i = BufReader::new(File::open(file_path).unwrap()).lines().count() as i32;
        let max = i;
        loop
        {
            let c = read_single_char();
            match c
            {
                '\n' =>
                {
                    println!();
                    break;
                }
                '\x08' | '\x1B' =>
                {
                    input.pop();
                    print!("\x08 \x08");
                }
                '\x1D' =>
                {
                    i -= 1;
                    input.clear();
                    if i == -1
                    {
                        i = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    print!("\x1B[2K\x1B[1G> {}", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    input.clear();
                    if i >= max
                    {
                        print!("\x1B[2K\x1B[1G> ");
                        stdout().flush().unwrap();
                        i -= 1;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    print!("\x1B[2K\x1B[1G> {}", input);
                }
                '\x1C' => print!("\x1b[1C"),
                _ =>
                {
                    input.push(c);
                    print!("{}", c);
                }
            }
            stdout().flush().unwrap();
        }
        if input.contains('=')
        {
            for i in 0..var.len()
            {
                if var[i][0] == input.chars().next().unwrap()
                {
                    var.remove(i);
                    break;
                }
            }
            var.push(input.chars().collect());
            let mut file = OpenOptions::new().append(true).open(file_path).expect("Failed to open file");
            file.write_all(input.as_bytes()).expect("Failed to write to file");
            file.write_all(b"\n").expect("Failed to write to file");
            continue;
        }
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
        if input == "help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        let unmodified = input.clone();
        for i in &var
        {
            input = input.replace(&i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>(),
                                  &i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>());
        }
        if input.contains('x') || input.contains('y')
        {
            println!("{}", input);
            let mut file = OpenOptions::new().append(true).open(file_path).expect("Failed to open file");
            file.write_all(input.as_bytes()).expect("Failed to write to file");
            file.write_all(b"\n").expect("Failed to write to file");
            continue;
        }
        print_answer(get_func(input.clone()));
        let mut file = OpenOptions::new().append(true).open(file_path).expect("Failed to open file");
        file.write_all(unmodified.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\n").expect("Failed to write to file");
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
            func.push(E.to_string());
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
                func.push(PI.to_string());
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
                func.remove(func.len() - if n == "x" || n == &PI.to_string() || n == &E.to_string() { 2 } else { 1 });
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
        if func[i].len() > 1 && func[i].chars().next().unwrap().is_ascii_alphabetic()
        {
            let (arg1, arg2) = parse(&func[i + 1][if func[i + 1].contains(',') { func[i + 1].find(',').unwrap() + 1 } else { 0 }..].to_string());
            match func[i].as_str()
            {
                "sin" => func[i] = sin(arg1, arg2).to_string(),
                "cos" => func[i] = cos(arg1, arg2).to_string(),
                "tan" => func[i] = tan(arg1, arg2).to_string(),
                "asin" => func[i] = asin(arg1, arg2).to_string(),
                "acos" => func[i] = acos(arg1, arg2).to_string(),
                "atan" => func[i] = atan(arg1, arg2).to_string(),
                "sinh" => func[i] = sinh(arg1, arg2).to_string(),
                "cosh" => func[i] = cosh(arg1, arg2).to_string(),
                "tanh" => func[i] = tanh(arg1, arg2).to_string(),
                "asinh" => func[i] = asinh(arg1, arg2).to_string(),
                "acosh" => func[i] = acosh(arg1, arg2).to_string(),
                "atanh" => func[i] = atanh(arg1, arg2).to_string(),
                "ln" => func[i] = ln(arg1, arg2).to_string(),
                "log" =>
                {
                    let (base_re, base_im) = if func[i + 1].contains(',')
                    {
                        parse(&func[i + 1][..func[i + 1].find(',').unwrap()].to_string())
                    }
                    else
                    {
                        (10.0, 0.0)
                    };
                    func[i] = log(base_re, base_im, arg1, arg2).to_string()
                }
                "sqrt" => func[i] = pow(arg1, arg2, 0.5, 0.0).to_string(),
                "abs" => func[i] = abs(arg1, arg2).to_string(),
                "dg" => func[i] = arg1.to_degrees().to_string(),
                "rd" => func[i] = arg1.to_radians().to_string(),
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
    i = 0;
    while i < func.len() - 1
    {
        if (func[i].contains('i') || func[i + 1].contains('i')) && (func[i] != "+" && func[i] != "-") && (func[i + 1] != "+" && func[i + 1] != "-")
        {
            let (a, b) = parse(&func[i]);
            let (c, d) = parse(&func[i + 1]);
            func[i] = add(a, b, c, d);
            func.remove(i + 1);
            i += 1;
            continue;
        }
        if i == 0
        {
            i += 1;
            continue;
        }
        if (func[i - 1].contains('i') || func[i + 1].contains('i')) && func[i] == "+"
        {
            let (a, b) = parse(&func[i - 1]);
            let (c, d) = parse(&func[i + 1]);
            func[i] = add(a, b, c, d);
            func.remove(i + 1);
            func.remove(i - 1);
            continue;
        }
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
fn add(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)+(c+di)=(a+c)+(b+d)i
    let im = (b + d).to_string();
    let sign = if im.contains('-') { "" } else { "+" };
    (a + c).to_string() + sign + im.as_str() + "i"
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
    if b == 0.0 && d == 0.0 && c.fract() == 0.0
    {
        return (a.powf(c)).to_string();
    }
    let r = c * (b.atan2(a)) + d * (0.5 * (a * a + b * b).ln());
    let m = E.powf(c * (0.5 * (a * a + b * b).ln()) - d * (b.atan2(a)));
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
fn log(c:f64, d:f64, a:f64, b:f64) -> String
{
    // log(c,a+bi)=ln(a+bi)/ln(c)
    let (a, b) = parse(&ln(a, b));
    let (c, d) = parse(&ln(c, d));
    div(a, b, c, d)
}
fn asin(a:f64, b:f64) -> String
{
    // asin(a+bi)=i*asinh(-i(a+bi))
    let (a, b) = parse(&asinh(b, -a));
    let sign = if a.to_string().contains('-') { "" } else { "+" };
    ((b * 1e15).round() / 1e15).to_string() + sign + ((a * 1e15).round() / 1e15).to_string().as_str() + "i"
}
fn acos(a:f64, b:f64) -> String
{
    // acos(a+bi)=pi/2+i*asinh(i(a+bi))
    let (a, b) = parse(&asinh(-b, a));
    let sign = if a.to_string().contains('-') { "" } else { "+" };
    (((-b + PI / 2.0) * 1e15).round() / 1e15).to_string() + sign + ((a * 1e15).round() / 1e15).to_string().as_str() + "i"
}
fn atan(a:f64, b:f64) -> String
{
    // atan(a+bi)=i*atanh(-i(a+bi))
    let (a, b) = parse(&atanh(b, -a));
    let sign = if a.to_string().contains('-') { "" } else { "+" };
    ((-b * 1e15).round() / 1e15).to_string() + sign + ((a * 1e15).round() / 1e15).to_string().as_str() + "i"
}
fn sinh(a:f64, b:f64) -> String
{
    // sinh(a+bi)=sinh(a)cos(b)+i*cosh(a)sin(b)
    let im = a.cosh() * b.sin();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.sinh() * b.cos()).to_string() + sign + im.to_string().as_str() + "i"
}
fn cosh(a:f64, b:f64) -> String
{
    // cosh(a+bi)=cosh(a)cos(b)+i*sinh(a)sin(b)
    let im = a.sinh() * b.sin();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.cosh() * b.cos()).to_string() + sign + im.to_string().as_str() + "i"
}
fn tanh(a:f64, b:f64) -> String
{
    // tanh(a+bi)=sinh(a+bi)/cosh(a+bi)
    div(a.sinh() * b.cos(), a.cosh() * b.sin(), a.cosh() * b.cos(), a.sinh() * b.sin())
}
fn asinh(a:f64, b:f64) -> String
{
    // asinh(a+bi)=ln(sqrt(a^2+b^2)+a+bi)
    let (c, d) = parse(&pow(a * a - b * b + 1.0, 2.0 * a * b, 0.5, 0.0));
    let (a, b) = parse(&add(c, d, a, b));
    let (re, im) = parse(&ln(a, b));
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    ((re * 1e15).round() / 1e15).to_string() + sign + ((im * 1e15).round() / 1e15).to_string().as_str() + "i"
}
fn acosh(a:f64, b:f64) -> String
{
    // acosh(a+bi)=ln(sqrt(a+ib-1)*sqrt(a+ib+1)+a+ib)
    let (e, f) = parse(&pow(a - 1.0, b, 0.5, 0.0));
    let (g, h) = parse(&pow(a + 1.0, b, 0.5, 0.0));
    let (c, d) = parse(&mul(e, f, g, h));
    let (a, b) = parse(&add(c, d, a, b));
    let (re, im) = parse(&ln(a, b));
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    ((re * 1e15).round() / 1e15).to_string() + sign + ((im * 1e15).round() / 1e15).to_string().as_str() + "i"
}
fn atanh(a:f64, b:f64) -> String
{
    // atanh(a+bi)=ln(a+bi+1)/2-ln(-a-bi+1)/2
    let (c, d) = parse(&add(a, b, 1.0, 0.0));
    let (e, f) = parse(&add(-a, -b, 1.0, 0.0));
    let (g, h) = parse(&ln(c, d));
    let (i, j) = parse(&ln(e, f));
    let (k, l) = parse(&div(g, h, 2.0, 0.0));
    let (m, n) = parse(&div(i, j, 2.0, 0.0));
    let (o, p) = parse(&add(k, l, -m, -n));
    let sign = if p.to_string().contains('-') { "" } else { "+" };
    ((o * 1e15).round() / 1e15).to_string() + sign + ((p * 1e15).round() / 1e15).to_string().as_str() + "i"
}