use std::f64::consts::E;
use crate::complex::{
    parse, div, add, mul, ln, log, abs, pow, sin, sinc, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, to_string, subfact, sgn, arg, csc, sec, cot, acsc, asec, acot, csch, sech, coth, acsch, asech, acoth, int, frac, fact
};
pub fn do_math(func:Vec<String>) -> Result<String, ()>
{
    if func.len() == 1
    {
        return if let Ok(n) = func[0].parse::<f64>() { Ok(n.to_string()) } else { Err(()) };
    }
    if func.is_empty()
    {
        return Err(());
    }
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
                if j >= func.len()
                {
                    return Err(());
                }
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
                "csc" => csc(arg1, arg2),
                "cos" => cos(arg1, arg2),
                "sec" => sec(arg1, arg2),
                "tan" => tan(arg1, arg2),
                "cot" => cot(arg1, arg2),
                "asin" | "arcsin" => asin(arg1, arg2),
                "acsc" | "arccsc" => acsc(arg1, arg2),
                "acos" | "arccos" => acos(arg1, arg2),
                "asec" | "arcsec" => asec(arg1, arg2),
                "atan" | "arctan" => atan(arg1, arg2),
                "acot" | "arccot" => acot(arg1, arg2),
                "sinh" => sinh(arg1, arg2),
                "csch" => csch(arg1, arg2),
                "cosh" => cosh(arg1, arg2),
                "sech" => sech(arg1, arg2),
                "tanh" => tanh(arg1, arg2),
                "coth" => coth(arg1, arg2),
                "asinh" | "arcsinh" => asinh(arg1, arg2),
                "acsch" | "arccsch" => acsch(arg1, arg2),
                "acosh" | "arccosh" => acosh(arg1, arg2),
                "asech" | "arcsech" => asech(arg1, arg2),
                "atanh" | "arctanh" => atanh(arg1, arg2),
                "acoth" | "arccoth" => acoth(arg1, arg2),
                "ln" => ln(arg1, arg2),
                "ceil" => (arg1.ceil(), arg2.ceil()),
                "floor" => (arg1.floor(), arg2.floor()),
                "round" => (arg1.round(), arg2.round()),
                "recip" => div(1.0, 0.0, arg1, arg2),
                "exp" => pow(E, 0.0, arg1, arg2),
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
                            match arg2 == 0.0 && (arg1 / 2.0).fract() != 0.0 && arg1.trunc() == arg1 && base_im == 0.0
                            {
                                true => (base_re / base_re.abs() * base_re.abs().powf(arg1.recip()), 0.0),
                                false =>
                                {
                                    let (a, b) = div(1.0, 0.0, arg1, arg2);
                                    pow(base_re, base_im, a, b)
                                }
                            }
                        }
                        false => pow(arg1, arg2, 0.5, 0.0),
                    }
                }
                "sqrt" => pow(arg1, arg2, 0.5, 0.0),
                "abs" => (abs(arg1, arg2), 0.0),
                "deg" | "degree" =>
                {
                    if arg2 != 0.0
                    {
                        return Err(());
                    }
                    (arg1.to_degrees(), 0.0)
                }
                "rad" | "radian" =>
                {
                    if arg2 != 0.0
                    {
                        return Err(());
                    }
                    (arg1.to_radians(), 0.0)
                }
                "re" | "real" => (arg1, 0.0),
                "im" | "imag" => (arg2, 0.0),
                "sgn" | "sign" => sgn(arg1, arg2),
                "arg" => (arg(arg1, arg2), 0.0),
                "cbrt" =>
                {
                    match arg2 == 0.0
                    {
                        true => (arg1.cbrt(), 0.0),
                        false => pow(arg1, arg2, 1.0 / 3.0, 0.0),
                    }
                }
                "frac" | "fract" => frac(arg1, arg2),
                "int" | "trunc" => int(arg1, arg2),
                "fact" =>
                {
                    if arg2 != 0.0 || arg1 < 0.0
                    {
                        return Err(());
                    }
                    (fact(arg1), 0.0)
                }
                "subfact" =>
                {
                    if arg2 != 0.0 || arg1 < 0.0
                    {
                        return Err(());
                    }
                    (subfact(arg1), 0.0)
                }
                "sinc" => sinc(arg1, arg2),
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
        if func[i] != "%"
        {
            i += 1;
            continue;
        }
        let (a, b) = parse(&func[i - 1]);
        let (c, d) = parse(&func[i + 1]);
        if b == 0.0 || d == 0.0
        {
            return Err(());
        }
        func[i] = (a % c).to_string();
        func.remove(i + 1);
        func.remove(i - 1);
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