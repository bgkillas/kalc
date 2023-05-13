use crate::complex::{parse, div, add, mul, ln, log, abs, pow, sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, to_string, sgn, arg};
pub fn do_math(func:Vec<String>) -> Result<String, ()>
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
            if i + 1 == j - 1
            {
                return Err(());
            }
            func[i] = match do_math(func[i + 1..j - 1].to_vec())
                      {
                          Ok(num) => num,
                          Err(e) => return Err(e),
                      }.to_string();
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
            let to_parse = match func[i].as_str()
            {
                "sin" => sin(arg1, arg2),
                "cos" => cos(arg1, arg2),
                "tan" => tan(arg1, arg2),
                "asin" => asin(arg1, arg2),
                "acos" => acos(arg1, arg2),
                "atan" => atan(arg1, arg2),
                "sinh" => sinh(arg1, arg2),
                "cosh" => cosh(arg1, arg2),
                "tanh" => tanh(arg1, arg2),
                "asinh" => asinh(arg1, arg2),
                "acosh" => acosh(arg1, arg2),
                "atanh" => atanh(arg1, arg2),
                "ln" => ln(arg1, arg2),
                "log" =>
                {
                    match func[i + 1].contains(',')
                    {
                        true =>
                        {
                            let (base_re, base_im) = parse(&func[i + 1][..func[i + 1].find(',').unwrap()].to_string());
                            log(base_re, base_im, arg1, arg2)
                        }
                        false => ln(arg1, arg2),
                    }
                }
                "root" =>
                {
                    match func[i + 1].contains(',')
                    {
                        true =>
                        {
                            let (base_re, base_im) = parse(&func[i + 1][..func[i + 1].find(',').unwrap()].to_string());
                            let (a, b) = div(1.0, 0.0, arg1, arg2);
                            pow(base_re, base_im, a, b)
                        }
                        false => pow(arg1, arg2, 0.5, 0.0),
                    }
                }
                "sqrt" => pow(arg1, arg2, 0.5, 0.0),
                "abs" => (abs(arg1, arg2), 0.0),
                "dg" => (arg1.to_degrees(), 0.0),
                "rd" => (arg1.to_radians(), 0.0),
                "re" => (arg1, 0.0),
                "im" => (arg2, 0.0),
                "sgn" => sgn(arg1, arg2),
                "arg" => (arg(arg1, arg2), 0.0),
                "cbrt" => pow(arg1, arg2, 1.0 / 3.0, 0.0),
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            func[i] = to_string(to_parse);
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
        let (a, b) = parse(&func[i - 1]);
        let (c, d) = parse(&func[i + 1]);
        func[i] = to_string(pow(a, b, c, d));
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
        let (a, b) = parse(&func[i - 1]);
        let (c, d) = parse(&func[i + 1]);
        match func[i].as_str()
        {
            "*" => func[i] = to_string(mul(a, b, c, d)),
            "/" => func[i] = to_string(div(a, b, c, d)),
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
        if !(func[i] == "+" || func[i] == "-")
        {
            i += 1;
            continue;
        }
        let (a, b) = parse(&func[i - 1]);
        let (c, d) = parse(&func[i + 1]);
        match func[i].as_str()
        {
            "+" => func[i] = to_string(add(a, b, c, d)),
            "-" => func[i] = to_string(add(a, b, -c, -d)),
            _ =>
            {
                i += 1;
                continue;
            }
        }
        func.remove(i + 1);
        func.remove(i - 1);
    }
    Ok(func.join(""))
}