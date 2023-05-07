use crate::complex::{parse, div, add, mul, ln, log, abs, pow, sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh};
pub fn do_math(func:Vec<String>) -> String
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
    i = 0;
    while i < func.len() - 1
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
                    i += 1;
                    continue;
                }
            }
            func.remove(i + 1);
        }
        i += 1;
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
        if (func[i].contains('i') || func[i + 1].contains('i')) && (func[i] != "+" && func[i] != "-") && (func[i + 1] != "+" && func[i + 1] != "-") && func[i] != "," && func[i + 1] != ","
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