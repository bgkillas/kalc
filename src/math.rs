use std::f64::consts::E;
use crate::complex::{
    div, add, mul, ln, log, abs, pow, sin, sinc, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, subfact, sgn, arg, csc, sec, cot, acsc, asec, acot, csch, sech, coth, acsch, asech, acoth, int, frac, fact
};
use crate::math::Complex::{Num, Str};
#[derive(Clone)]
pub enum Complex
{
    Num((f64, f64)),
    Str(String),
}
impl Complex
{
    fn str_is(&self, s:&str) -> bool
    {
        if let Str(s2) = self
        {
            s == s2
        }
        else
        {
            false
        }
    }
}
// noinspection RsBorrowChecker
pub fn do_math(func:Vec<Complex>, deg:bool) -> Result<(f64, f64), ()>
{
    if func.len() == 1
    {
        return if let Num(n) = func[0] { Ok(n) } else { Err(()) };
    }
    if func.is_empty()
    {
        return Err(());
    }
    let mut function = func;
    let mut i = 0;
    let (mut j, mut count, mut v);
    'outer: while i < function.len() - 1
    {
        if function[i].str_is("(")
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
            for (f, n) in v.iter().enumerate()
            {
                if n.str_is(",")
                {
                    function.drain(i..j);
                    function.insert(i, Num(do_math(v[..f].to_vec(), deg)?));
                    function.insert(i + 1, Str(",".to_string()));
                    function.insert(i + 2, Num(do_math(v[f + 1..].to_vec(), deg)?));
                    i += 1;
                    continue 'outer;
                }
            }
            function[i] = Num(do_math(v, deg)?);
            function.drain(i + 1..j);
        }
        i += 1;
    }
    i = 0;
    let (mut a, mut b, mut c, mut d);
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s.len() > 1 && s.chars().next().unwrap().is_ascii_alphabetic()
            {
                if let Num(n) = function[i + 1]
                {
                    function[i] = Num(match s.to_string().as_str()
                    {
                        "sin" =>
                        {
                            if deg
                            {
                                sin(n.0.to_radians(), 0.0)
                            }
                            else
                            {
                                sin(n.0, n.1)
                            }
                        }
                        "csc" =>
                        {
                            if deg
                            {
                                csc(n.0.to_radians(), 0.0)
                            }
                            else
                            {
                                csc(n.0, n.1)
                            }
                        }
                        "cos" =>
                        {
                            if deg
                            {
                                cos(n.0.to_radians(), 0.0)
                            }
                            else
                            {
                                cos(n.0, n.1)
                            }
                        }
                        "sec" =>
                        {
                            if deg
                            {
                                sec(n.0.to_radians(), 0.0)
                            }
                            else
                            {
                                sec(n.0, n.1)
                            }
                        }
                        "tan" =>
                        {
                            if deg
                            {
                                tan(n.0.to_radians(), 0.0)
                            }
                            else
                            {
                                tan(n.0, n.1)
                            }
                        }
                        "cot" =>
                        {
                            if deg
                            {
                                cot(n.0.to_radians(), 0.0)
                            }
                            else
                            {
                                cot(n.0, n.1)
                            }
                        }
                        "asin" | "arcsin" =>
                        {
                            if deg
                            {
                                (asin(n.0, n.1).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                asin(n.0, n.1)
                            }
                        }
                        "acsc" | "arccsc" =>
                        {
                            if deg
                            {
                                (acsc(n.0, n.1).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                acsc(n.0, n.1)
                            }
                        }
                        "acos" | "arccos" =>
                        {
                            if deg
                            {
                                (acos(n.0, n.1).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                acos(n.0, n.1)
                            }
                        }
                        "asec" | "arcsec" =>
                        {
                            if deg
                            {
                                (asec(n.0, n.1).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                asec(n.0, n.1)
                            }
                        }
                        "atan" | "arctan" =>
                        {
                            if deg
                            {
                                (atan(n.0, n.1).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                atan(n.0, n.1)
                            }
                        }
                        "acot" | "arccot" =>
                        {
                            if deg
                            {
                                (acot(n.0, n.1).0.to_degrees(), 0.0)
                            }
                            else
                            {
                                acot(n.0, n.1)
                            }
                        }
                        "sinh" => sinh(n.0, n.1),
                        "csch" => csch(n.0, n.1),
                        "cosh" => cosh(n.0, n.1),
                        "sech" => sech(n.0, n.1),
                        "tanh" => tanh(n.0, n.1),
                        "coth" => coth(n.0, n.1),
                        "asinh" | "arcsinh" => asinh(n.0, n.1),
                        "acsch" | "arccsch" => acsch(n.0, n.1),
                        "acosh" | "arccosh" => acosh(n.0, n.1),
                        "asech" | "arcsech" => asech(n.0, n.1),
                        "atanh" | "arctanh" => atanh(n.0, n.1),
                        "acoth" | "arccoth" => acoth(n.0, n.1),
                        "cis" => pow(E, 0.0, -n.1, n.0),
                        "ln" | "aexp" => ln(n.0, n.1),
                        "ceil" => (n.0.ceil(), n.1.ceil()),
                        "floor" => (n.0.floor(), n.1.floor()),
                        "round" => (n.0.round(), n.1.round()),
                        "recip" => div(1.0, 0.0, n.0, n.1),
                        "exp" | "aln" => pow(E, 0.0, n.0, n.1),
                        "log" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                if let Num(b) = function[i + 3]
                                {
                                    function.remove(i + 3);
                                    function.remove(i + 2);
                                    log(n.0, n.1, b.0, b.1)
                                }
                                else
                                {
                                    return Err(());
                                }
                            }
                            else
                            {
                                ln(n.0, n.1)
                            }
                        }
                        "root" =>
                        {
                            if function.len() > i + 3 && function[i + 2].str_is(",")
                            {
                                if let Num(e) = function[i + 3]
                                {
                                    function.remove(i + 3);
                                    function.remove(i + 2);
                                    match e.1 == 0.0 && (e.0 / 2.0).fract() != 0.0 && e.0.trunc() == e.0 && n.1 == 0.0
                                    {
                                        true => (n.0 / n.0.abs() * n.0.abs().powf(e.0.recip()), 0.0),
                                        false =>
                                        {
                                            (a, b) = div(1.0, 0.0, e.0, e.1);
                                            pow(n.0, n.1, a, b)
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
                                pow(n.0, n.1, 0.5, 0.0)
                            }
                        }
                        "sqrt" | "asquare" => pow(n.0, n.1, 0.5, 0.0),
                        "abs" => (abs(n.0, n.1), 0.0),
                        "deg" | "degree" =>
                        {
                            if n.1 != 0.0
                            {
                                return Err(());
                            }
                            (n.0.to_degrees(), 0.0)
                        }
                        "rad" | "radian" =>
                        {
                            if n.1 != 0.0
                            {
                                return Err(());
                            }
                            (n.0.to_radians(), 0.0)
                        }
                        "re" | "real" => (n.0, 0.0),
                        "im" | "imag" => (n.1, 0.0),
                        "sgn" | "sign" => sgn(n.0, n.1),
                        "arg" => (arg(n.0, n.1), 0.0),
                        "cbrt" | "acube" =>
                        {
                            match n.1 == 0.0
                            {
                                true => (n.0.cbrt(), 0.0),
                                false => pow(n.0, n.1, 1.0 / 3.0, 0.0),
                            }
                        }
                        "frac" | "fract" => frac(n.0, n.1),
                        "int" | "trunc" => int(n.0, n.1),
                        "fact" =>
                        {
                            if n.1 != 0.0 || n.0 < 0.0
                            {
                                return Err(());
                            }
                            (fact(n.0), 0.0)
                        }
                        "square" | "asqrt" => pow(n.0, n.1, 2.0, 0.0),
                        "cube" | "acbrt" => pow(n.0, n.1, 3.0, 0.0),
                        "subfact" =>
                        {
                            if n.1 != 0.0 || n.0 < 0.0
                            {
                                return Err(());
                            }
                            (subfact(n.0), 0.0)
                        }
                        "sinc" => sinc(n.0, n.1),
                        _ =>
                        {
                            i += 1;
                            continue;
                        }
                    });
                    function.remove(i + 1);
                }
            }
        }
        i += 1;
    }
    i = 1;
    while i < function.len() - 1
    {
        if !function[i].str_is("%")
        {
            i += 1;
            continue;
        }
        if let Num(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Num(e) = &function[i + 1]
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
        function[i] = Num(((a % c), 0.0));
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if !function[i].str_is("^")
        {
            i += 1;
            continue;
        }
        if let Num(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Num(e) = &function[i + 1]
        {
            (c, d) = *e;
        }
        else
        {
            return Err(());
        }
        function[i] = Num(pow(a, b, c, d));
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
        if let Num(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Num(e) = &function[i + 1]
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
                function[i] = Num(mul(a, b, c, d))
            }
            else if s == "/"
            {
                function[i] = Num(div(a, b, c, d))
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
        if let Num(e) = &function[i - 1]
        {
            (a, b) = *e;
        }
        else
        {
            return Err(());
        }
        if let Num(e) = &function[i + 1]
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
                function[i] = Num(add(a, b, c, d))
            }
            else if s == "-"
            {
                function[i] = Num(add(a, b, -c, -d))
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
    if let Num(n) = function[0]
    {
        Ok(n)
    }
    else
    {
        Err(())
    }
}