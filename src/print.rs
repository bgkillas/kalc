use rug::Complex;
use crate::fraction::fraction;
use crate::math::{do_math, NumStr};
use crate::parse::get_func;
pub fn print_answer(input:&str, func:Vec<NumStr>, print_options:(bool, bool, usize, bool, bool, usize), color:bool, prec:u32)
{
    if (input.contains('x') && !input.contains("exp")) || input.contains('y') || input.contains('z') || input.contains('=')
    {
        return;
    }
    let num = match do_math(func, print_options.1, prec)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("0");
            return;
        }
    };
    let (a, b) = (num.real().to_f64(), num.imag().to_f64());
    if print_options.0
    {
        let c = if a != 0.0
        {
            format!("{:e}", num.real()).replace("e0", "").replace('e', if color { "\x1b[92mE" } else { "E" })
        }
        else if b == 0.0
        {
            "0".to_owned()
        }
        else
        {
            "".to_owned()
        };
        let d = if b != 0.0
        {
            format!("{}{:e}{}", if a != 0.0 && b.is_sign_positive() { "+" } else { "" }, b, if color { "\x1b[93mi" } else { "i" }).replace("e0", "")
                                                                                                                                  .replace('e', if color { "\x1b[92mE" } else { "E" })
        }
        else
        {
            "".to_owned()
        };
        print!("{}\x1b[0m{}\x1b[0m", c, d);
    }
    else if print_options.2 != 10
    {
        match print_options.2
        {
            2 => print!("{:b}", a as i64),
            8 => print!("{:o}", a as i64),
            16 => print!("{:x}", a as i64),
            _ => print!("0"),
        }
    }
    else
    {
        let a = (a * 1e12).round() / 1e12;
        let b = (b * 1e12).round() / 1e12;
        let d = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &b.to_string() + if color { "\x1b[93mi" } else { "i" };
        print!("{}\x1b[0m{}\x1b[0m",
               if a == 0.0 && b != 0.0 { "".to_string() } else { a.to_string() },
               if b == 0.0 { "".to_string() } else { d });
    };
}
pub fn print_concurrent(unmodified_input:&str, input:&str, print_options:(bool, bool, usize, bool, bool, usize), prompt:bool, color:bool, prec:u32) -> bool
{
    if (input.contains('x') && !input.contains("exp")) || input.contains('y') || input.contains('z') || input.contains('=')
    {
        print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
               if prompt
               {
                   if color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               unmodified_input);
        return false;
    }
    let func = match get_func(input, prec)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if prompt
                   {
                       if color
                       {
                           "\x1b[94m> \x1b[96m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else if color
                   {
                       "\x1b[96m"
                   }
                   else
                   {
                       ""
                   },
                   input);
            return false;
        }
    };
    let mut frac = false;
    let num = do_math(func, print_options.1, prec).unwrap_or(Complex::with_val(256, 0.0));
    let a = num.real().to_f64();
    let b = num.imag().to_f64();
    let fa = fraction(num.real().clone(), print_options.3, prec, print_options.5);
    let fb = fraction(num.imag().clone(), print_options.3, prec, print_options.5);
    let c = (a * 1e12).round() / 1e12;
    let d = (b * 1e12).round() / 1e12;
    let sign = if c != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned();
    let (frac_a, frac_b) = match (!fa.is_empty(), !fb.is_empty())
    {
        (true, true) =>
        {
            frac = true;
            (if c == 0.0 && d != 0.0 { "".to_string() } else { fa },
             if d == 0.0
             {
                 "".to_string()
             }
             else
             {
                 sign.clone() + fb.as_str() + if color { "\x1b[93mi" } else { "i" }
             })
        }
        (true, _) =>
        {
            frac = true;
            (if c == 0.0 && d != 0.0 { "".to_string() } else { fa },
             if d == 0.0
             {
                 "".to_string()
             }
             else
             {
                 sign.clone() + d.to_string().as_str() + if color { "\x1b[93mi" } else { "i" }
             })
        }
        (_, true) =>
        {
            frac = true;
            (if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() },
             if d == 0.0
             {
                 "".to_string()
             }
             else
             {
                 sign.clone() + fb.as_str() + if color { "\x1b[93mi" } else { "i" }
             })
        }
        _ => ("".to_string(), "".to_string()),
    };
    let (output_a, output_b) = if print_options.2 != 10
    {
        (match print_options.2
         {
             2 => format!("{:b}", a as i64),
             8 => format!("{:o}", a as i64),
             16 => format!("{:x}", a as i64),
             _ => "0".to_string(),
         },
         "".to_owned())
    }
    else if print_options.0
    {
        (if a != 0.0
         {
             remove_trailing_zeros(&format!("{:.dec$e}\x1b[0m", num.real(), dec = print_options.5)).replace("e0", "")
                                                                                                   .replace('e', if color { "\x1b[92mE" } else { "E" })
         }
         else if b == 0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if b != 0.0
         {
             remove_trailing_zeros(&format!("{}{:.dec$e}{}",
                                            if a != 0.0 && b.is_sign_positive() { "+" } else { "" },
                                            num.imag(),
                                            if color { "\x1b[93mi" } else { "i" },
                                            dec = print_options.5)).replace("e0", "")
                                                                   .replace('e', if color { "\x1b[92mE" } else { "E" })
         }
         else
         {
             "".to_owned()
         })
    }
    else
    {
        (if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() },
         if d == 0.0
         {
             "".to_string()
         }
         else
         {
             sign + &d.to_string() + if color { "\x1b[93mi" } else { "i" }
         })
    };
    print!("{}{}{}{}\x1b[0m\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}\x1b[0m",
           if frac { "\x1b[0m\n\x1B[2K\x1B[1G" } else { "" },
           frac_a,
           frac_b,
           if !frac { "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A" } else { "" },
           output_a,
           output_b,
           if frac { "\x1b[A" } else { "" },
           if prompt
           {
               if color
               {
                   "\x1b[94m> \x1b[96m"
               }
               else
               {
                   "> "
               }
           }
           else if color
           {
               "\x1b[96m"
           }
           else
           {
               ""
           },
           unmodified_input);
    frac
}
fn remove_trailing_zeros(input:&str) -> String
{
    let chars = input.chars();
    let mut result = Vec::new();
    let mut found = false;
    let mut non_zero = false;
    for c in chars.rev()
    {
        if !non_zero && found && (c == '0' || c == '.')
        {
            continue;
        }
        else
        {
            non_zero = true;
        }
        if c == 'e'
        {
            found = true;
            non_zero = false;
        }
        result.push(c);
    }
    result.iter().rev().collect::<String>()
}