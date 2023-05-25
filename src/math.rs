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
        match self
        {
            Str(s2) => s == s2,
            _ => false,
        }
    }
    fn str_is_any(&self, arr:&[&str]) -> bool
    {
        match self
        {
            Str(s) => arr.iter().any(|&x| x == s),
            _ => false,
        }
    }
    fn num(&self) -> Result<(f64, f64), ()>
    {
        match self
        {
            Num(n) => Ok(*n),
            _ => Err(()),
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
    let (mut a, mut b);
    while i < function.len() - 1
    {
        if let Str(s) = &function[i]
        {
            if s.len() > 1 && s.chars().next().unwrap().is_ascii_alphabetic()
            {
                a = function[i + 1].num()?;
                function[i] = Num(match s.to_string().as_str()
                {
                    "sin" =>
                    {
                        if deg
                        {
                            sin(a.0.to_radians(), 0.0)
                        }
                        else
                        {
                            sin(a.0, a.1)
                        }
                    }
                    "csc" =>
                    {
                        if deg
                        {
                            csc(a.0.to_radians(), 0.0)
                        }
                        else
                        {
                            csc(a.0, a.1)
                        }
                    }
                    "cos" =>
                    {
                        if deg
                        {
                            cos(a.0.to_radians(), 0.0)
                        }
                        else
                        {
                            cos(a.0, a.1)
                        }
                    }
                    "sec" =>
                    {
                        if deg
                        {
                            sec(a.0.to_radians(), 0.0)
                        }
                        else
                        {
                            sec(a.0, a.1)
                        }
                    }
                    "tan" =>
                    {
                        if deg
                        {
                            tan(a.0.to_radians(), 0.0)
                        }
                        else
                        {
                            tan(a.0, a.1)
                        }
                    }
                    "cot" =>
                    {
                        if deg
                        {
                            cot(a.0.to_radians(), 0.0)
                        }
                        else
                        {
                            cot(a.0, a.1)
                        }
                    }
                    "asin" | "arcsin" =>
                    {
                        if deg
                        {
                            (asin(a.0, a.1).0.to_degrees(), 0.0)
                        }
                        else
                        {
                            asin(a.0, a.1)
                        }
                    }
                    "acsc" | "arccsc" =>
                    {
                        if deg
                        {
                            (acsc(a.0, a.1).0.to_degrees(), 0.0)
                        }
                        else
                        {
                            acsc(a.0, a.1)
                        }
                    }
                    "acos" | "arccos" =>
                    {
                        if deg
                        {
                            (acos(a.0, a.1).0.to_degrees(), 0.0)
                        }
                        else
                        {
                            acos(a.0, a.1)
                        }
                    }
                    "asec" | "arcsec" =>
                    {
                        if deg
                        {
                            (asec(a.0, a.1).0.to_degrees(), 0.0)
                        }
                        else
                        {
                            asec(a.0, a.1)
                        }
                    }
                    "atan" | "arctan" =>
                    {
                        if deg
                        {
                            (atan(a.0, a.1).0.to_degrees(), 0.0)
                        }
                        else
                        {
                            atan(a.0, a.1)
                        }
                    }
                    "acot" | "arccot" =>
                    {
                        if deg
                        {
                            (acot(a.0, a.1).0.to_degrees(), 0.0)
                        }
                        else
                        {
                            acot(a.0, a.1)
                        }
                    }
                    "sinh" => sinh(a.0, a.1),
                    "csch" => csch(a.0, a.1),
                    "cosh" => cosh(a.0, a.1),
                    "sech" => sech(a.0, a.1),
                    "tanh" => tanh(a.0, a.1),
                    "coth" => coth(a.0, a.1),
                    "asinh" | "arcsinh" => asinh(a.0, a.1),
                    "acsch" | "arccsch" => acsch(a.0, a.1),
                    "acosh" | "arccosh" => acosh(a.0, a.1),
                    "asech" | "arcsech" => asech(a.0, a.1),
                    "atanh" | "arctanh" => atanh(a.0, a.1),
                    "acoth" | "arccoth" => acoth(a.0, a.1),
                    "cis" => pow(E, 0.0, -a.1, a.0),
                    "ln" | "aexp" => ln(a.0, a.1),
                    "ceil" => (a.0.ceil(), a.1.ceil()),
                    "floor" => (a.0.floor(), a.1.floor()),
                    "round" => (a.0.round(), a.1.round()),
                    "recip" => div(1.0, 0.0, a.0, a.1),
                    "exp" | "aln" => pow(E, 0.0, a.0, a.1),
                    "log" =>
                    {
                        if function.len() > i + 3 && function[i + 2].str_is(",")
                        {
                            b = function[i + 3].num()?;
                            function.remove(i + 3);
                            function.remove(i + 2);
                            log(a.0, a.1, b.0, b.1)
                        }
                        else
                        {
                            ln(a.0, a.1)
                        }
                    }
                    "root" =>
                    {
                        if function.len() > i + 3 && function[i + 2].str_is(",")
                        {
                            b = function[i + 3].num()?;
                            function.remove(i + 3);
                            function.remove(i + 2);
                            match b.1 == 0.0 && (b.0 / 2.0).fract() != 0.0 && b.0.trunc() == b.0 && a.1 == 0.0
                            {
                                true => (a.0 / a.0.abs() * a.0.abs().powf(b.0.recip()), 0.0),
                                false =>
                                {
                                    b = div(1.0, 0.0, b.0, b.1);
                                    pow(a.0, a.1, b.0, b.1)
                                }
                            }
                        }
                        else
                        {
                            pow(a.0, a.1, 0.5, 0.0)
                        }
                    }
                    "sqrt" | "asquare" => pow(a.0, a.1, 0.5, 0.0),
                    "abs" => (abs(a.0, a.1), 0.0),
                    "deg" | "degree" =>
                    {
                        if a.1 != 0.0
                        {
                            return Err(());
                        }
                        (a.0.to_degrees(), 0.0)
                    }
                    "rad" | "radian" =>
                    {
                        if a.1 != 0.0
                        {
                            return Err(());
                        }
                        (a.0.to_radians(), 0.0)
                    }
                    "re" | "real" => (a.0, 0.0),
                    "im" | "imag" => (a.1, 0.0),
                    "sgn" | "sign" => sgn(a.0, a.1),
                    "arg" => (arg(a.0, a.1), 0.0),
                    "cbrt" | "acube" =>
                    {
                        match a.1 == 0.0
                        {
                            true => (a.0.cbrt(), 0.0),
                            false => pow(a.0, a.1, 1.0 / 3.0, 0.0),
                        }
                    }
                    "frac" | "fract" => frac(a.0, a.1),
                    "int" | "trunc" => int(a.0, a.1),
                    "fact" =>
                    {
                        if a.1 != 0.0 || a.0 < 0.0
                        {
                            return Err(());
                        }
                        (fact(a.0), 0.0)
                    }
                    "square" | "asqrt" => pow(a.0, a.1, 2.0, 0.0),
                    "cube" | "acbrt" => pow(a.0, a.1, 3.0, 0.0),
                    "subfact" =>
                    {
                        if a.1 != 0.0 || a.0 < 0.0
                        {
                            return Err(());
                        }
                        (subfact(a.0), 0.0)
                    }
                    "sinc" => sinc(a.0, a.1),
                    _ =>
                    {
                        i += 1;
                        continue;
                    }
                });
                function.remove(i + 1);
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
        a = function[i - 1].num()?;
        b = function[i + 1].num()?;
        if a.1 == 0.0 || b.1 == 0.0
        {
            return Err(());
        }
        function[i] = Num(((a.0 % b.0), 0.0));
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
        a = function[i - 1].num()?;
        b = function[i + 1].num()?;
        function[i] = Num(pow(a.0, a.1, b.0, b.1));
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if !function[i].str_is_any(&["*", "/"])
        {
            i += 1;
            continue;
        }
        a = function[i - 1].num()?;
        b = function[i + 1].num()?;
        match &function[i]
        {
            Str(s) if s == "*" => function[i] = Num(mul(a.0, a.1, b.0, b.1)),
            Str(s) if s == "/" => function[i] = Num(div(a.0, a.1, b.0, b.1)),
            _ => return Err(()),
        }
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len() - 1
    {
        if !function[i].str_is_any(&["+", "-"])
        {
            i += 1;
            continue;
        }
        a = function[i - 1].num()?;
        b = function[i + 1].num()?;
        match &function[i]
        {
            Str(s) if s == "+" => function[i] = Num(add(a.0, a.1, b.0, b.1)),
            Str(s) if s == "-" => function[i] = Num(add(a.0, a.1, -b.0, -b.1)),
            _ => return Err(()),
        }
        function.remove(i + 1);
        function.remove(i - 1);
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