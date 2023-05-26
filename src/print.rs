use crate::fraction::fraction;
use crate::math::{do_math, Complex};
use crate::parse::get_func;
pub fn print_answer(func:Vec<Complex>, print_options:(bool, bool, usize))
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
        let c = if a != 0.0 { format!("{:e}", a) } else { "".to_owned() };
        let d = if b != 0.0
        {
            format!("{}{:e}",
                    if a != 0.0
                    {
                        if b.is_sign_positive()
                        {
                            "+"
                        }
                        else
                        {
                            "-"
                        }
                    }
                    else
                    {
                        ""
                    },
                    b)
        }
        else
        {
            "".to_owned()
        };
        println!("{}{}", c, d);
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
        let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e12).round() / 1e12).to_string() + "i";
        println!("{}{}",
                 if a == 0.0 && !(b.ends_with("0i")) { "".to_string() } else { a.to_string() },
                 if b.ends_with("0i") { "".to_string() } else { b });
    };
}
pub fn print_concurrent(input:&str, var:Vec<Vec<char>>, del:bool, tau:bool, last:&str, print_options:(bool, bool, usize)) -> bool
{
    let mut modified = input.to_string().replace('_', last);
    for i in &var
    {
        let var_name = i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>();
        let var_value = i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>();
        let var_name_len = var_name.len();
        let mut start_idx = 0;
        while start_idx < modified.len() + 1 - var_name_len
        {
            let end_idx = start_idx + var_name_len;
            if (start_idx == 0 || !modified.chars().nth(start_idx - 1).unwrap().is_ascii_alphabetic())
               && (end_idx == modified.len() || !modified.chars().nth(end_idx).unwrap().is_ascii_alphabetic())
               && modified[start_idx..end_idx] == var_name
            {
                modified.replace_range(start_idx..end_idx, &var_value);
            }
            start_idx += 1;
        }
    }
    if (modified.contains('x') && !input.contains("exp")) || modified.contains('y') || modified.contains('z')
    {
        print!("\x1b[96m\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}\x1b[0m", input);
        return false;
    }
    let func = match get_func(&modified)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
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
            (if c == 0.0 && d != 0.0 { "".to_string() } else { fa }, if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" })
        }
        (true, _) =>
        {
            frac = true;
            (if c == 0.0 && d != 0.0 { "".to_string() } else { fa }, if d == 0.0 { "".to_string() } else { sign.clone() + d.to_string().as_str() + "\x1b[93mi" })
        }
        (_, true) =>
        {
            frac = true;
            (if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() }, if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" })
        }
        _ => ("".to_string(), "".to_string()),
    };
    let (output_a, output_b) = if print_options.0
    {
        (if a != 0.0
         {
             format!("{:e}\x1b[0m", a).replace("e0", "").replace('e', "\x1b[92me")
         }
         else
         {
             "".to_owned()
         },
         if b != 0.0
         {
             format!("{}{:e}\x1b[93mi",
                     if a != 0.0
                     {
                         if b.is_sign_positive()
                         {
                             "+"
                         }
                         else
                         {
                             ""
                         }
                     }
                     else
                     {
                         ""
                     },
                     b).replace("e0", "")
                       .replace('e', "\x1b[92me")
         }
         else
         {
             "".to_owned()
         })
    }
    else if print_options.2 != 10
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
    else
    {
        (if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() }, if d == 0.0 { "".to_string() } else { sign + d.to_string().as_str() + "\x1b[93mi" })
    };
    print!("{}{}{}{}\x1b[0m\n\x1B[2K\x1B[1G{}{}\x1b[A{}{}",
           if frac { "\x1b[0m\n\x1B[2K\x1B[1G" } else { "" },
           frac_a,
           frac_b,
           if !frac { "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A" } else { "" },
           output_a,
           output_b,
           if frac { "\x1b[A" } else { "" },
           if !del { format!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input) } else { "\x1B[2K\x1B[1G".to_string() });
    frac
}