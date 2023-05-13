use crate::complex::parse;
use crate::fraction::fraction;
use crate::math::do_math;
use crate::parse::get_func;
pub fn print_answer(func:Vec<String>)
{
    let num = match do_math(func)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("\x1b[91m0\x1b[0m");
            return;
        }
    };
    let (a, b) = parse(&num);
    let a = (a * 1e12).round() / 1e12;
    let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e12).round() / 1e12).to_string() + "i";
    println!("{}{}\x1b[0m",
             if a == 0.0 && !(b.ends_with("0i")) { "".to_string() } else { a.to_string() },
             if b.ends_with("0i") { "".to_string() } else { b });
}
pub fn print_concurrent(input:&String, var:Vec<Vec<char>>, del:bool, last:&mut Vec<String>, frac:bool) -> bool
{
    let mut modified = input.to_string();
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
    if modified.contains('x') || modified.contains('y') || modified.contains('z')
    {
        print!("\x1b[96m\x1b[B\x1B[2K\x1B[1G\x1b[A\x1B[2K\x1B[1G{}\x1b[0m", input);
        return false;
    }
    let func = match get_func(&modified, false)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
            return false;
        }
    };
    if func == *last
    {
        print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
        return frac;
    }
    let mut frac = false;
    *last = func.clone();
    if let Ok(num) = do_math(func.clone())
    {
        if num == "inf" || num == "-inf" || num == "nan"
        {
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}\x1b[0m\x1b[A", num);
            print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
            return false;
        }
        let (a, b) = parse(&num);
        let c = (a * 1e12).round() / 1e12;
        let sign = if c != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned();
        let d = (b * 1e12).round() / 1e12;
        let fa = fraction(a);
        let fb = fraction(b);
        let (output_a, output_b) = match (fa.contains('/'), fb.contains('/'), fa.contains('π'), fb.contains('π'), fa.contains('s'), fb.contains('s'))
        {
            (true, true, _, _, _, _) | (_, _, true, true, _, _) | (_, _, _, _, true, true) =>
            {
                frac = true;
                (if c == 0.0 && d != 0.0 { "".to_string() } else { fa }, if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" })
            }
            (true, _, _, _, _, _) | (_, _, true, _, _, _) | (_, _, _, _, true, _) if fa != func.join("") =>
            {
                frac = true;
                (if c == 0.0 && d != 0.0 { "".to_string() } else { fa }, if d == 0.0 { "".to_string() } else { sign.clone() + d.to_string().as_str() + "\x1b[93mi" })
            }
            (_, true, _, _, _, _) | (_, _, _, true, _, _) | (_, _, _, _, _, true) if fb != func.join("") =>
            {
                frac = true;
                (if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() }, if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" })
            }
            _ => ("".to_string(), "".to_string()),
        };
        print!("{}{}{}{}\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}\x1b[A",
               if frac { "\x1b[0m\x1b[B\x1B[2K\x1B[1G" } else { "" },
               output_a,
               output_b,
               if !frac { "\x1b[B\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A" } else { "" },
               if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() },
               if d == 0.0 { "".to_string() } else { sign + d.to_string().as_str() + "\x1b[93mi" });
        if frac
        {
            print!("\x1b[A");
        }
    }
    if !del
    {
        print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
    }
    else
    {
        print!("\x1B[2K\x1B[1G");
    }
    frac
}