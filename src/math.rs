use std::f64::consts::E;
use crate::complex::{
    div, add, mul, ln, log, abs, pow, sin, sinc, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, subfact, sgn, arg, csc, sec, cot, acsc, asec, acot, csch, sech, coth, acsch, asech, acoth, int, frac, fact
};
use crate::math::NumOrString::{Complex, Str};
#[derive(Clone)]
pub enum NumOrString
{
    Complex((f64, f64)),
    Str(String),
}
// noinspection RsBorrowChecker
pub fn do_math(func:Vec<NumOrString>, deg:bool) -> Result<(f64, f64), ()>
{
    if func.len() == 1
    {
        return if let Complex(n) = func[0] { Ok(n) } else { Err(()) };
    }
    if func.is_empty()
    {
        return Err(());
    }
    let mut function = func;
    let mut i = 0;
    let (mut j, mut count);
    let mut v;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s == "("
            {
                j = i + 1;
                count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err(());
                    }
                    match &function[j]
                    {
                        Str(s) if s == "(" => count += 1,
                        Str(s) if s == ")" => count -= 1,
                        _ =>
                        {}
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err(());
                }
                v = function[i + 1..j - 1].to_vec();
                if v.iter().any(|x| {
                               if let Str(s) = x
                               {
                                   s == ","
                               }
                               else
                               {
                                   false
                               }
                           })
                {
                    function.remove(i);
                    function.remove(j - 2);
                    i += 1;
                    continue;
                }
                function[i] = Complex(match do_math(v, deg)
                {
                    Ok(num) => num,
                    Err(e) => return Err(e),
                });
                function.drain(i + 1..j);
            }
        }
        i += 1;
    }
    i = 0;
    let mut to_parse:(f64, f64);
    let (mut arg1, mut arg2, mut a, mut b, mut c, mut d, mut base_re, mut base_im);
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s.len() > 1 && s.chars().next().unwrap().is_ascii_alphabetic()
            {
                if let Complex(n) = &function[i + 1]
                {
                    (arg1, arg2) = *n;
                    to_parse = match s.to_string().as_str()
                    {
                        "sin" =>
                        {
                            if deg
                            {
                                sin(arg1.to_radians(), 0.0)
                            }
                            else
                            {
                                sin(arg1, arg2)
                            }
                        }
                        "csc" =>
                        {
                            if deg
                            {
                                csc(arg1.to_radians(), 0.0)
                            }
                            else
                            {
                                csc(arg1, arg2)
                            }
                        }
                        "cos" =>
                        {
                            if deg
                            {
                                cos(arg1.to_radians(), 0.0)
                            }
                            else
                            {
                                cos(arg1, arg2)
                            }
                        }
                        "sec" =>
                        {
                            if deg
                            {
                                sec(arg1.to_radians(), 0.0)
                            }
                            else
                            {
                                sec(arg1, arg2)
                            }
                        }
                        "tan" =>
                        {
                            if deg
                            {
                                tan(arg1.to_radians(), 0.0)
                            }
                            else
                            {
                                tan(arg1, arg2)
                            }
                        }
                        "cot" =>
                        {
                            if deg
                            {
                                cot(arg1.to_radians(), 0.0)
                            }
                            else
                            {
                                cot(arg1, arg2)
                            }
                        }
                        "asin" | "arcsin" =>
                        {
                            if deg
                            {
                                (asin(arg1, arg2).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                asin(arg1, arg2)
                            }
                        }
                        "acsc" | "arccsc" =>
                        {
                            if deg
                            {
                                (acsc(arg1, arg2).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                acsc(arg1, arg2)
                            }
                        }
                        "acos" | "arccos" =>
                        {
                            if deg
                            {
                                (acos(arg1, arg2).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                acos(arg1, arg2)
                            }
                        }
                        "asec" | "arcsec" =>
                        {
                            if deg
                            {
                                (asec(arg1, arg2).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                asec(arg1, arg2)
                            }
                        }
                        "atan" | "arctan" =>
                        {
                            if deg
                            {
                                (atan(arg1, arg2).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                atan(arg1, arg2)
                            }
                        }
                        "acot" | "arccot" =>
                        {
                            if deg
                            {
                                (acot(arg1, arg2).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                acot(arg1, arg2)
                            }
                        }
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
                            if function.len() > i + 3
                            {
                                if let Str(s) = &function[i + 2]
                                {
                                    if s == ","
                                    {
                                        if let Complex(b) = &function[i + 3]
                                        {
                                            (base_re, base_im) = *b;
                                            log(arg1, arg2, base_re, base_im)
                                        }
                                        else
                                        {
                                            return Err(());
                                        }
                                    }
                                    else
                                    {
                                        ln(arg1, arg2)
                                    }
                                }
                                else
                                {
                                    ln(arg1, arg2)
                                }
                            }
                            else
                            {
                                ln(arg1, arg2)
                            }
                        }
                        "root" =>
                        {
                            if function.len() > i + 3
                            {
                                if let Str(s) = &function[i + 2]
                                {
                                    if s == ","
                                    {
                                        if let Complex(e) = &function[i + 3]
                                        {
                                            (base_re, base_im) = *e;
                                            match base_im == 0.0 && (base_re / 2.0).fract() != 0.0 && base_re.trunc() == base_re && arg2 == 0.0
                                            {
                                                true => (arg1 / arg1.abs() * arg1.abs().powf(base_re.recip()), 0.0),
                                                false =>
                                                {
                                                    (a, b) = div(1.0, 0.0, base_re, base_im);
                                                    pow(arg1, arg2, a, b)
                                                }
                                            }
                                        }
                                        else
                                        {
                                            return Err(());
                                        }
                                    }
                                    else
                                    {
                                        pow(arg1, arg2, 0.5, 0.0)
                                    }
                                }
                                else
                                {
                                    pow(arg1, arg2, 0.5, 0.0)
                                }
                            }
                            else
                            {
                                pow(arg1, arg2, 0.5, 0.0)
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
                        "square" => pow(arg1, arg2, 2.0, 0.0),
                        "cube" => pow(arg1, arg2, 3.0, 0.0),
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
                    function[i] = Complex(to_parse);
                    function.remove(i + 1);
                }
            }
        }
        i += 1;
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s != "%"
            {
                i += 1;
                continue;
            }
        }
        else
        {
            i += 1;
            continue;
        }
        if let Complex(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Complex(e) = &function[i + 1]
        {
            (c, d) = *e;
        }
        else
        {
            return Err(());
        }
        if b == 0.0 || d == 0.0
        {
            return Err(());
        }
        function[i] = Complex(((a % c), 0.0));
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s != "^"
            {
                i += 1;
                continue;
            }
        }
        else
        {
            i += 1;
            continue;
        }
        if let Complex(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Complex(e) = &function[i + 1]
        {
            (c, d) = *e;
        }
        else
        {
            return Err(());
        }
        function[i] = Complex(pow(a, b, c, d));
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if !(s == "*" || s == "/")
            {
                i += 1;
                continue;
            }
        }
        else
        {
            i += 1;
            continue;
        }
        if let Complex(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Complex(e) = &function[i + 1]
        {
            (c, d) = *e;
        }
        else
        {
            return Err(());
        }
        if let Str(s) = &function[i]
        {
            if s == "*"
            {
                function[i] = Complex(mul(a, b, c, d))
            }
            else if s == "/"
            {
                function[i] = Complex(div(a, b, c, d))
            }
            else
            {
                i += 1;
                continue;
            }
            function.remove(i + 1);
            function.remove(i - 1);
        }
    }
    i = 1;
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if !(s == "+" || s == "-")
            {
                i += 1;
                continue;
            }
        }
        else
        {
            i += 1;
            continue;
        }
        if let Complex(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Complex(e) = &function[i + 1]
        {
            (c, d) = *e;
        }
        else
        {
            return Err(());
        }
        if let Str(s) = &function[i]
        {
            if s == "+"
            {
                function[i] = Complex(add(a, b, c, d))
            }
            else if s == "-"
            {
                function[i] = Complex(add(a, b, -c, -d))
            }
            else
            {
                i += 1;
                continue;
            }
            function.remove(i + 1);
            function.remove(i - 1);
        }
    }
    if let Complex(n) = function[0]
    {
        Ok(n)
    }
    else
    {
        Err(())
    }
}