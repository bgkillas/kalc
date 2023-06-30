use std::str::FromStr;
use rug::{Complex, Float, Integer};
use rug::float::Constant::Pi;
use rug::ops::CompleteRound;
use crate::fraction::fraction;
use crate::math::{do_math, NumStr, to_polar};
use crate::math::NumStr::{Num, Str, Vector};
use crate::parse::get_func;
use crate::{get_terminal_width, PrintOptions};
pub fn print_answer(input:&str, func:Vec<NumStr>, print_options:PrintOptions, prec:u32)
{
    if input.contains('#')
       || (input.contains('x') && !input.contains("exp") && !input.contains("}x{") && !input.contains("]x["))
       || input.contains('y')
       || (input.contains('z') && !input.contains("zeta"))
       || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<=")))
    {
        return;
    }
    let num = match do_math(func, print_options.deg, prec)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("0");
            return;
        }
    };
    if let Num(n) = num
    {
        let a = get_output(&print_options, &n);
        print!("{}{}{}", a.0, a.1, if print_options.color { "\x1b[0m" } else { "" });
    }
    else if let Vector(mut v) = num
    {
        if print_options.polar
        {
            v = to_polar(&v,
                         if print_options.deg == 0
                         {
                             Complex::with_val(prec, 1.0)
                         }
                         else if print_options.deg == 1
                         {
                             Complex::with_val(prec, 180.0) / Complex::with_val(prec, Pi)
                         }
                         else
                         {
                             Complex::with_val(prec, 200.0) / Complex::with_val(prec, Pi)
                         });
        }
        let mut output = if print_options.polar { "[" } else { "{" }.to_string();
        let mut out;
        for (k, i) in v.iter().enumerate()
        {
            out = get_output(&print_options, i);
            output += out.0.as_str();
            output += out.1.as_str();
            if print_options.color
            {
                output += "\x1b[0m";
            }
            if k == v.len() - 1
            {
                output += if print_options.polar { "]" } else { "}" };
            }
            else
            {
                output += ",";
            }
        }
        print!("{}{}", output, if print_options.color { "\x1b[0m" } else { "" });
    }
}
pub fn print_concurrent(unmodified_input:&str, input:&str, print_options:PrintOptions, prec:u32, start:usize, end:usize) -> usize
{
    if input.contains('#')
       || (input.contains('x') && !input.contains("exp") && !input.contains("}x{") && !input.contains("]x["))
       || input.contains('y')
       || (input.contains('z') && !input.contains("zeta"))
       || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<=")))
    {
        print!("\x1B[0J\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
               if print_options.prompt
               {
                   if print_options.color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if print_options.color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               &unmodified_input[start..end]);
        return 0;
    }
    let func = match get_func(input, prec, print_options.deg)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\x1B[0J\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if print_options.prompt
                   {
                       if print_options.color
                       {
                           "\x1b[94m> \x1b[96m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else if print_options.color
                   {
                       "\x1b[96m"
                   }
                   else
                   {
                       ""
                   },
                   &unmodified_input[start..end]);
            return 0;
        }
    };
    let mut frac = 0;
    let mut num = do_math(func, print_options.deg, prec).unwrap_or(Num(Complex::with_val(256, 0.0)));
    if let Str(_) = num
    {
        num = Num(Complex::with_val(256, 0.0));
    }
    if let Num(n) = num
    {
        let fa = fraction(n.real().clone(), print_options.tau);
        let fb = fraction(n.imag().clone(), print_options.tau);
        let sign = if n.real() != &0.0 && n.imag().is_sign_positive() { "+" } else { "" }.to_owned();
        let (frac_a, frac_b) = match (!fa.is_empty(), !fb.is_empty())
        {
            (true, true) =>
            {
                frac = 1;
                (if n.real() == &0.0 && n.imag() != &0.0 { "".to_string() } else { fa },
                 if n.imag() == &0.0
                 {
                     "".to_string()
                 }
                 else
                 {
                     sign + fb.as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
                 })
            }
            (true, _) =>
            {
                frac = 1;
                (if n.real() == &0.0 && n.imag() != &0.0 { "".to_string() } else { fa }, if n.imag() == &0.0 { "".to_string() } else { get_output(&print_options, &n).1 })
            }
            (_, true) =>
            {
                frac = 1;
                (if n.real() == &0.0 && n.imag() != &0.0 { "".to_string() } else { get_output(&print_options, &n).0 },
                 if n.imag() == &0.0
                 {
                     "".to_string()
                 }
                 else
                 {
                     sign + fb.as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
                 })
            }
            _ => ("".to_string(), "".to_string()),
        };
        let output = get_output(&print_options, &n);
        let terlen = get_terminal_width();
        if (frac == 1 && !print_options.frac) || (frac_a.len() + frac_b.len() - if print_options.color && !frac_b.is_empty() { 5 } else { 0 }) > terlen
        {
            frac = 0;
        }
        let len1 = output.0.replace("\x1b[0m", "").replace("\x1b[93m", "").replace("\x1b[92m", "").len();
        let len2 = output.1.replace("\x1b[0m", "").replace("\x1b[93m", "").replace("\x1b[92m", "").len();
        if len1 + len2 > terlen
        {
            let num = (len1 as f64 / terlen as f64).ceil() as usize + if len2 != 0 { ((len2 - 1) as f64 / terlen as f64).ceil() as usize - 1 } else { 0 } - 1;
            print!("\x1B[0J{}\x1b[0m\n\x1B[2K\x1B[1G{}{}{}{}\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if frac == 1
                   {
                       format!("\x1b[0m\n\x1B[2K\x1B[1G{}{}", frac_a, frac_b)
                   }
                   else
                   {
                       "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A".to_string()
                   },
                   output.0,
                   if len1 != 0 && len2 != 0 { "\n" } else { "" },
                   &output.1.replace('+', ""),
                   "\x1b[A".repeat(num + frac - if len1 == 0 || len2 == 0 { 1 } else { 0 }),
                   if print_options.prompt
                   {
                       if print_options.color
                       {
                           "\x1b[94m> \x1b[96m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else if print_options.color
                   {
                       "\x1b[96m"
                   }
                   else
                   {
                       ""
                   },
                   &unmodified_input[start..end]);
            frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
        }
        else
        {
            print!("\x1B[0J{}\x1b[0m\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if frac == 1
                   {
                       format!("\x1b[0m\n\x1B[2K\x1B[1G{}{}", frac_a, frac_b)
                   }
                   else
                   {
                       "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A".to_string()
                   },
                   output.0,
                   output.1,
                   if frac == 1 { "\x1b[A" } else { "" },
                   if print_options.prompt
                   {
                       if print_options.color
                       {
                           "\x1b[94m> \x1b[96m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else if print_options.color
                   {
                       "\x1b[96m"
                   }
                   else
                   {
                       ""
                   },
                   &unmodified_input[start..end]);
        }
    }
    else if let Vector(mut v) = num
    {
        if print_options.polar
        {
            v = to_polar(&v,
                         if print_options.deg == 0
                         {
                             Complex::with_val(prec, 1.0)
                         }
                         else if print_options.deg == 1
                         {
                             Complex::with_val(prec, 180.0) / Complex::with_val(prec, Pi)
                         }
                         else
                         {
                             Complex::with_val(prec, 200.0) / Complex::with_val(prec, Pi)
                         });
        }
        let mut output = if print_options.polar { "[" } else { "{" }.to_string();
        let mut frac_out = if print_options.polar { "[" } else { "{" }.to_string();
        let mut out;
        let mut frac_temp;
        for (k, i) in v.iter().enumerate()
        {
            out = get_output(&print_options, i);
            frac_temp = fraction(i.real().clone(), print_options.tau);
            frac_out += if !frac_temp.is_empty() { &frac_temp } else { &out.0 };
            frac_temp = fraction(i.imag().clone(), print_options.tau);
            frac_out += &if !frac_temp.is_empty()
            {
                format!("{}{}{}",
                        (if i.real() != &0.0 && i.imag().is_sign_positive() && i.imag() != &0.0 { "+" } else { "" }),
                        frac_temp,
                        (if i.imag() != &0.0
                        {
                            if print_options.color
                            {
                                "\x1b[93mi"
                            }
                            else
                            {
                                "i"
                            }
                        }
                        else
                        {
                            ""
                        }))
            }
            else
            {
                out.clone().1
            };
            output += &out.0;
            output += &out.1;
            if print_options.color
            {
                output += "\x1b[0m";
                frac_out += "\x1b[0m";
            }
            if k == v.len() - 1
            {
                output += if print_options.polar { "]" } else { "}" };
                frac_out += if print_options.polar { "]" } else { "}" };
            }
            else
            {
                output += ",";
                frac_out += ",";
            }
        }
        if frac_out != output
        {
            frac = 1;
        }
        if (frac == 1 && !print_options.frac) || frac_out.replace("\x1b[0m", "").replace("\x1b[93m", "").replace("\x1b[92m", "").len() > get_terminal_width()
        {
            frac = 0;
        }
        print!("\x1B[0J{}\x1b[0m\n\x1B[2K\x1B[1G{}\x1b[A{}\x1B[2K\x1B[1G{}{}\x1b[0m",
               if frac == 1
               {
                   format!("\x1b[0m\n\x1B[2K\x1B[1G{}", frac_out)
               }
               else
               {
                   "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A".to_string()
               },
               output,
               if frac == 1 { "\x1b[A" } else { "" },
               if print_options.prompt
               {
                   if print_options.color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if print_options.color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               &unmodified_input[start..end]);
    }
    frac
}
pub fn get_output(print_options:&PrintOptions, num:&Complex) -> (String, String)
{
    let sign = if num.real() != &0.0 && num.imag().is_sign_positive() { "+" } else { "" }.to_owned();
    let mut n;
    let dec = if print_options.decimal_places == 0 { 1 } else { print_options.decimal_places };
    if print_options.base != 10
    {
        (if num.real() != &0.0
         {
             n = remove_trailing_zeros(&num.real().to_string_radix(print_options.base as i32, None), dec, num.real().prec());
             if n.contains('e')
             {
                 n
             }
             else
             {
                 n.trim_end_matches('0').trim_end_matches('.').to_owned()
             }
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if num.imag() != &0.0
         {
             n = remove_trailing_zeros(&num.imag().to_string_radix(print_options.base as i32, None), dec, num.real().prec());
             sign + &if n.contains('e') { n } else { n.trim_end_matches('0').trim_end_matches('.').to_owned() } + if print_options.color { "\x1b[93mi" } else { "i" }
         }
         else
         {
             "".to_string()
         })
    }
    else if print_options.sci
    {
        (if num.real() != &0.0
         {
             add_commas(&remove_trailing_zeros(&format!("{:e}", num.real()), dec, num.real().prec()), print_options.comma).replace("e0", "")
                                                                                                                          .replace('e', if print_options.color { "\x1b[92mE" } else { "E" })
             + if print_options.color { "\x1b[0m" } else { "" }
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if num.imag() != &0.0
         {
             add_commas(&(sign.as_str().to_owned() + &remove_trailing_zeros(&format!("{:e}{}", num.imag(), if print_options.color { "\x1b[93mi" } else { "i" }), dec, num.real().prec())),
                        print_options.comma).replace("e0", "")
                                            .replace('e', if print_options.color { "\x1b[92mE" } else { "E" })
         }
         else
         {
             "".to_owned()
         })
    }
    else
    {
        n = add_commas(&to_string(num.real(), print_options.decimal_places), print_options.comma);
        let sign = if n == "0" { "".to_string() } else { sign };
        let im = add_commas(&to_string(num.imag(), print_options.decimal_places), print_options.comma);
        (if n == "0" && im != "0" { "".to_string() } else { n },
         if im == "0"
         {
             "".to_string()
         }
         else
         {
             sign + &im + if print_options.color { "\x1b[93mi" } else { "i" }
         })
    }
}
fn to_string(num:&Float, decimals:usize) -> String
{
    let (neg, mut str, exp) = num.to_sign_string_exp(10, None);
    let mut neg = if neg { "-" } else { "" };
    if exp.is_none()
    {
        return if str == "0" { "0".to_string() } else { format!("{}{}", neg, str) };
    }
    let exp = exp.unwrap();
    let decimals = if decimals != usize::MAX - 1 && (get_terminal_width() as i32) < (2i32 + exp)
    {
        decimals
    }
    else if exp == 0
    {
        get_terminal_width() - 2
    }
    else if exp < 0
    {
        get_terminal_width() - 3
    }
    else
    {
        (get_terminal_width() as i32 - 1i32 - exp) as usize
    };
    if str.len() as i32 == exp
    {
        return if str == "0" { "0".to_string() } else { format!("{}{}", neg, str) };
    }
    if exp > str.len() as i32
    {
        str.push_str(&"0".repeat(exp as usize - str.len()));
    }
    let mut zeros = String::new();
    if exp < 0
    {
        zeros = "0".repeat(-exp as usize);
        str.insert_str(0, &zeros);
        str.insert(1, '.');
    }
    else
    {
        str.insert(exp as usize, '.');
    }
    let mut split = str.split('.');
    let mut l = split.next().unwrap().to_string();
    let mut r = split.next().unwrap().to_string();
    if r.is_empty()
    {
        return if str == "0" { "0".to_string() } else { format!("{}{}", neg, l) };
    }
    if r.len() > decimals
    {
        r.insert(decimals, '.');
    }
    let mut d = Float::with_val(num.prec(), Float::parse(&r).unwrap()).to_integer().unwrap();
    if exp > 0
    {
        zeros = "0".repeat(r.to_string().len() - r.to_string().trim_start_matches('0').len());
        if d.to_string() == 10.0f64.powi(decimals as i32 - 1).to_string()
        {
            zeros.pop();
        }
    }
    if zeros.is_empty() && d.to_string().trim_end_matches('0') == "1" && r.starts_with('9')
    {
        let t:Float = Float::with_val(num.prec(), Float::parse(if l.is_empty() { "0" } else { &l }).unwrap()) + 1;
        l = t.to_integer().unwrap().to_string();
        d = Integer::new();
    }
    if d.to_string() == "0" && (l.is_empty() || l == "0")
    {
        neg = ""
    }
    if decimals == 0
    {
        if zeros.is_empty() && d.to_string().chars().next().unwrap().to_digit(10).unwrap() == 1
        {
            format!("{}{}", neg, Integer::from_str(&l).unwrap_or(Integer::new()) + 1)
        }
        else
        {
            format!("{}{}", neg, if l.is_empty() { "0" } else { &l })
        }
    }
    else
    {
        format!("{}{}.{}{}", neg, if l.is_empty() { "0" } else { &l }, zeros, d).trim_end_matches('0')
                                                                                .trim_end_matches('.')
                                                                                .to_string()
    }
}
fn add_commas(input:&str, commas:bool) -> String
{
    if !commas
    {
        return input.to_owned();
    }
    let mut split = input.split('.');
    let mut result = String::new();
    let mut count = 0;
    let mut exp = false;
    for c in split.next().unwrap().chars().rev()
    {
        if c == 'e'
        {
            exp = true;
        }
        if count == 3 && !exp
        {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    if split.clone().count() == 1
    {
        let mut result = result.chars().rev().collect::<String>();
        result.push('.');
        count = 0;
        for c in split.next().unwrap().chars()
        {
            if c == 'e'
            {
                exp = true;
            }
            if count == 3 && !exp
            {
                result.push(',');
                count = 0;
            }
            result.push(c);
            count += 1;
        }
        return result;
    }
    result.chars().rev().collect::<String>()
}
fn remove_trailing_zeros(input:&str, dec:usize, prec:u32) -> String
{
    let pos = match input.find('e')
    {
        Some(n) => n,
        None => return input.trim_end_matches('0').trim_end_matches('.').to_string(),
    };
    let dec = if dec == usize::MAX - 1
    {
        (if &input[pos..] == "e0" || &input[pos..] == "e0\x1b[93mi"
        {
            get_terminal_width() - 1
        }
        else
        {
            get_terminal_width() - (input.len() - pos) - 1
        })
        + if input.ends_with("\x1b[93mi") { 5 } else { 0 }
    }
    else
    {
        dec
    };
    if dec > pos
    {
        input[..pos].trim_end_matches('0').trim_end_matches('.').to_string() + &input[pos..]
    }
    else
    {
        let mut sign = String::new();
        let mut num = if input.starts_with('-')
        {
            sign = "-".to_string();
            input[1..pos].to_string()
        }
        else
        {
            input[0..pos].to_string()
        };
        num.remove(1);
        num.insert(dec, '.');
        num = Float::parse(num).unwrap().complete(prec).to_integer().unwrap().to_string();
        num.insert(1, '.');
        sign + num.trim_end_matches('0').trim_end_matches('.') + &input[pos..]
    }
}