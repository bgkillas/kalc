use crate::fraction::fraction;
use crate::math::{do_math, Complex};
use crate::parse::get_func;
pub fn print_answer(func:Vec<Complex>, print_options:(bool, bool, usize), color:bool)
{
    let num = match do_math(func, print_options.1)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("0");
            return;
        }
    };
    let (a, b) = num;
    if print_options.0
    {
        let c = if a != 0.0
        {
            format!("{:e}", a).replace("e0", "").replace('e', if color { "\x1b[92mE" } else { "E" })
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
        println!("{}\x1b[0m{}\x1b[0m", c, d);
    }
    else if print_options.2 != 10
    {
        match print_options.2
        {
            2 => println!("{:b}", a as i64),
            8 => println!("{:o}", a as i64),
            16 => println!("{:x}", a as i64),
            _ => println!("0"),
        }
    }
    else
    {
        let a = (a * 1e12).round() / 1e12;
        let b = (b * 1e12).round() / 1e12;
        let d = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &b.to_string() + if color { "\x1b[93mi" } else { "i" };
        println!("{}\x1b[0m{}\x1b[0m",
                 if a == 0.0 && b != 0.0 { "".to_string() } else { a.to_string() },
                 if b == 0.0 { "".to_string() } else { d });
    };
}
pub fn print_concurrent(unmodified_input:&str, input:&str, tau:bool, print_options:(bool, bool, usize), prompt:bool, color:bool) -> bool
{
    if (input.contains('x') && !input.contains("exp")) || input.contains('y') || input.contains('z')
    {
        print!("{}\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
               if color { "\x1b[96m" } else { "" },
               if prompt
               {
                   if color
                   {
                       "\x1b[94m> \x1b[0m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else
               {
                   ""
               },
               unmodified_input);
        return false;
    }
    let func = match get_func(input)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("{}\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if color { "\x1b[96m" } else { "" },
                   if prompt
                   {
                       if color
                       {
                           "\x1b[94m> \x1b[0m"
                       }
                       else
                       {
                           "> "
                       }
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
    let (a, b) = do_math(func, print_options.1).unwrap_or((0.0, 0.0));
    let fa = fraction(a, tau);
    let fb = fraction(b, tau);
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
             format!("{:e}\x1b[0m", a).replace("e0", "").replace('e', if color { "\x1b[92mE" } else { "E" })
         }
         else
         {
             "".to_owned()
         },
         if b != 0.0
         {
             format!("{}{:e}{}", if a != 0.0 && b.is_sign_positive() { "+" } else { "" }, b, if color { "\x1b[93mi" } else { "i" }).replace("e0", "")
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
             sign + d.to_string().as_str() + if color { "\x1b[93mi" } else { "i" }
         })
    };
    print!("{}{}{}{}\x1b[0m\n\x1B[2K\x1B[1G{}{}\x1b[A{}{}",
           if frac { "\x1b[0m\n\x1B[2K\x1B[1G" } else { "" },
           frac_a,
           frac_b,
           if !frac { "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A" } else { "" },
           output_a,
           output_b,
           if frac { "\x1b[A" } else { "" },
           format!("\x1B[2K\x1B[1G{}{}{}\x1b[0m",
                   if prompt
                   {
                       if color
                       {
                           "\x1b[94m> \x1b[0m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else
                   {
                       ""
                   },
                   if color { "\x1b[96m" } else { "" },
                   unmodified_input));
    frac
}