use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    fraction::fraction,
    get_terminal_width,
    math::{do_math, to_polar},
    parse::get_func,
    vars::input_var,
    AngleType::{Degrees, Gradians, Radians},
    Options,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float, Integer};
use std::{cmp::Ordering, str::FromStr};
pub fn print_answer(input: &str, func: Vec<NumStr>, options: Options)
{
    if input.contains('#')
        || input
            .replace("exp", "")
            .replace("max", "")
            .replace("}x{", "")
            .replace("]x[", "")
            .contains('x')
        || input.contains('y')
        || input
            .replace("zeta", "")
            .replace("normalize", "")
            .contains('z')
        || input
            .replace("==", "")
            .replace("!=", "")
            .replace(">=", "")
            .replace("<=", "")
            .contains('=')
    {
        return;
    }
    let num = match do_math(func, options.deg, options.prec)
    {
        Ok(num) => num,
        Err(_) => return,
    };
    if let Num(n) = num
    {
        let a = get_output(&options, &n);
        print!(
            "{}{}{}",
            a.0,
            a.1,
            if options.color { "\x1b[0m" } else { "" }
        );
    }
    else if let Vector(mut v) = num
    {
        if options.polar
        {
            v = to_polar(
                v,
                match options.deg
                {
                    Degrees =>
                    {
                        Complex::with_val(options.prec, 180) / Complex::with_val(options.prec, Pi)
                    }
                    Radians => Complex::with_val(options.prec, 1),
                    Gradians =>
                    {
                        Complex::with_val(options.prec, 200) / Complex::with_val(options.prec, Pi)
                    }
                },
            );
        }
        let mut output = if options.polar { "[" } else { "{" }.to_string();
        let mut out;
        for (k, i) in v.iter().enumerate()
        {
            out = get_output(&options, i);
            output += out.0.as_str();
            output += out.1.as_str();
            if options.color
            {
                output += "\x1b[0m";
            }
            if k == v.len() - 1
            {
                output += if options.polar { "]" } else { "}" };
            }
            else
            {
                output += ",";
            }
        }
        print!("{}{}", output, if options.color { "\x1b[0m" } else { "" });
    }
    else if let Matrix(v) = num
    {
        let mut output = if options.multi
        {
            String::new()
        }
        else
        {
            "{".to_string()
        };
        let mut out;
        for (l, j) in v.iter().enumerate()
        {
            if !options.multi
            {
                output += "{";
            }
            for (k, i) in j.iter().enumerate()
            {
                out = get_output(&options, i);
                output += out.0.as_str();
                output += out.1.as_str();
                if options.color
                {
                    output += "\x1b[0m";
                }
                if k == j.len() - 1
                {
                    if !options.multi
                    {
                        output += "}";
                    }
                }
                else if options.tabbed
                {
                    output += "\t";
                }
                else
                {
                    output += ",";
                }
            }
            if l != v.len() - 1
            {
                if options.multi
                {
                    output += "\n";
                }
                else
                {
                    output += ",";
                }
            }
        }
        if !options.multi
        {
            output += "}";
        }
        print!("{}{}", output, if options.color { "\x1b[0m" } else { "" });
    }
}
pub fn print_concurrent(
    unmodified_input: &[char],
    last: &[char],
    vars: &[[String; 2]],
    options: Options,
    start: usize,
    end: usize,
) -> usize
{
    let input = &input_var(
        &unmodified_input.iter().collect::<String>(),
        vars,
        None,
        options,
    )
    .replace('_', &format!("({})", last.iter().collect::<String>()));
    if input.contains('#')
        || input
            .replace("exp", "")
            .replace("max", "")
            .replace("}x{", "")
            .replace("]x[", "")
            .contains('x')
        || input.contains('y')
        || input
            .replace("zeta", "")
            .replace("normalize", "")
            .contains('z')
        || input
            .replace("==", "")
            .replace("!=", "")
            .replace(">=", "")
            .replace("<=", "")
            .contains('=')
    {
        print!(
            "\x1B[2K\x1B[1G\x1B[0J{}{}{}",
            if options.prompt
            {
                if options.color
                {
                    "\x1b[94m> \x1b[96m"
                }
                else
                {
                    "> "
                }
            }
            else if options.color
            {
                "\x1b[96m"
            }
            else
            {
                ""
            },
            &unmodified_input[start..end].iter().collect::<String>(),
            if options.color { "\x1b[0m" } else { "" }
        );
        return 0;
    }
    let func = match get_func(input, options)
    {
        Ok(f) => f,
        Err(s) =>
        {
            print!(
                "\x1B[0J\x1B[2K\x1B[1G\n{}\x1b[A\x1B[2K\x1B[1G{}{}{}",
                s,
                if options.prompt
                {
                    if options.color
                    {
                        "\x1b[94m> \x1b[96m"
                    }
                    else
                    {
                        "> "
                    }
                }
                else if options.color
                {
                    "\x1b[96m"
                }
                else
                {
                    ""
                },
                &unmodified_input[start..end].iter().collect::<String>(),
                if options.color { "\x1b[0m" } else { "" },
            );
            return 0;
        }
    };
    let mut frac = 0;
    let mut num = match do_math(func, options.deg, options.prec)
    {
        Ok(n) => n,
        Err(s) =>
        {
            print!(
                "\x1B[0J\x1B[2K\x1B[1G\n{}\x1b[A\x1B[2K\x1B[1G{}{}{}",
                s,
                if options.prompt
                {
                    if options.color
                    {
                        "\x1b[94m> \x1b[96m"
                    }
                    else
                    {
                        "> "
                    }
                }
                else if options.color
                {
                    "\x1b[96m"
                }
                else
                {
                    ""
                },
                &unmodified_input[start..end].iter().collect::<String>(),
                if options.color { "\x1b[0m" } else { "" },
            );
            return 0;
        }
    };
    if let Str(_) = num
    {
        num = Num(Complex::new(options.prec));
    }
    if let Num(n) = num
    {
        let sign = if n.real() != &0.0 && n.imag().is_sign_positive()
        {
            "+"
        }
        else
        {
            ""
        }
        .to_owned();
        let (frac_a, frac_b) = if options.frac || options.frac_iter == 0
        {
            let fa = fraction(n.real().clone(), options);
            let fb = fraction(n.imag().clone(), options);
            match (!fa.is_empty(), !fb.is_empty())
            {
                (true, true) =>
                {
                    frac = 1;
                    (
                        if n.real() == &0.0 && n.imag() != &0.0
                        {
                            "".to_string()
                        }
                        else
                        {
                            fa
                        },
                        if n.imag() == &0.0
                        {
                            "".to_string()
                        }
                        else
                        {
                            sign + fb.as_str()
                                + if options.color
                                {
                                    "\x1b[93mi\x1b[0m"
                                }
                                else
                                {
                                    "i"
                                }
                        },
                    )
                }
                (true, _) =>
                {
                    frac = 1;
                    (
                        if n.real() == &0.0 && n.imag() != &0.0
                        {
                            "".to_string()
                        }
                        else
                        {
                            fa
                        },
                        if n.imag() == &0.0
                        {
                            "".to_string()
                        }
                        else
                        {
                            get_output(&options, &n).1 + if options.color { "\x1b[0m" } else { "" }
                        },
                    )
                }
                (_, true) =>
                {
                    frac = 1;
                    (
                        if n.real() == &0.0 && n.imag() != &0.0
                        {
                            "".to_string()
                        }
                        else
                        {
                            get_output(&options, &n).0
                        },
                        if n.imag() == &0.0
                        {
                            "".to_string()
                        }
                        else
                        {
                            sign + fb.as_str()
                                + if options.color
                                {
                                    "\x1b[93mi\x1b[0m"
                                }
                                else
                                {
                                    "i"
                                }
                        },
                    )
                }
                _ => ("".to_string(), "".to_string()),
            }
        }
        else
        {
            ("".to_string(), "".to_string())
        };
        let output = get_output(&options, &n);
        let terlen = get_terminal_width();
        let len1 = output
            .0
            .replace("\x1b[0m", "")
            .replace("\x1b[93m", "")
            .replace("\x1b[92m", "")
            .len();
        let len2 = output
            .1
            .replace("\x1b[0m", "")
            .replace("\x1b[93m", "")
            .replace("\x1b[92m", "")
            .len();
        if (frac == 1 && !options.frac)
            || (frac_a.len() + frac_b.len()
                - if options.color && !frac_b.is_empty()
                {
                    5
                }
                else
                {
                    0
                })
                > terlen
        {
            frac = 0;
        }
        if len1 + len2 > terlen
        {
            let num = (len1 as f64 / terlen as f64).ceil() as usize
                + if len2 != 0
                {
                    ((len2 - 1) as f64 / terlen as f64).ceil() as usize - 1
                }
                else
                {
                    0
                }
                - 1;
            print!(
                "\x1B[0J{}\n\x1B[2K\x1B[1G{}{}{}{}\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}{}",
                if frac == 1
                {
                    format!("\n\x1B[2K\x1B[1G{}{}", frac_a, frac_b)
                }
                else
                {
                    "".to_string()
                },
                output.0,
                if len1 != 0 && len2 != 0 { "\n" } else { "" },
                &output.1.replace('+', ""),
                "\x1b[A".repeat(num + frac - if len1 == 0 || len2 == 0 { 1 } else { 0 }),
                if options.prompt
                {
                    if options.color
                    {
                        "\x1b[94m> \x1b[96m"
                    }
                    else
                    {
                        "> "
                    }
                }
                else if options.color
                {
                    "\x1b[96m"
                }
                else
                {
                    ""
                },
                &unmodified_input[start..end].iter().collect::<String>(),
                if options.color { "\x1b[0m" } else { "" },
            );
            frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
        }
        else
        {
            print!(
                "\x1B[0J{}\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}{}",
                if frac == 1
                {
                    format!("\n\x1B[2K\x1B[1G{}{}", frac_a, frac_b)
                }
                else
                {
                    "".to_string()
                },
                output.0,
                output.1,
                if frac == 1 { "\x1b[A" } else { "" },
                if options.prompt
                {
                    if options.color
                    {
                        "\x1b[94m> \x1b[96m"
                    }
                    else
                    {
                        "> "
                    }
                }
                else if options.color
                {
                    "\x1b[96m"
                }
                else
                {
                    ""
                },
                &unmodified_input[start..end].iter().collect::<String>(),
                if options.color { "\x1b[0m" } else { "" }
            );
        }
    }
    else if let Vector(mut v) = num
    {
        if options.polar
        {
            v = to_polar(
                v,
                match options.deg
                {
                    Degrees =>
                    {
                        Complex::with_val(options.prec, 180) / Complex::with_val(options.prec, Pi)
                    }
                    Radians => Complex::with_val(options.prec, 1),
                    Gradians =>
                    {
                        Complex::with_val(options.prec, 200) / Complex::with_val(options.prec, Pi)
                    }
                },
            );
        }
        let mut output = if options.polar { "[" } else { "{" }.to_string();
        let mut frac_out = if options.polar { "[" } else { "{" }.to_string();
        let mut out;
        let mut frac_temp;
        for (k, i) in v.iter().enumerate()
        {
            out = get_output(&options, i);
            if options.frac || options.frac_iter == 0
            {
                frac_temp = fraction(i.real().clone(), options);
                frac_out += if !frac_temp.is_empty()
                {
                    &frac_temp
                }
                else
                {
                    &out.0
                };
                frac_temp = fraction(i.imag().clone(), options);
                frac_out += &if !frac_temp.is_empty()
                {
                    format!(
                        "{}{}{}",
                        (if i.real() != &0.0 && i.imag().is_sign_positive() && i.imag() != &0.0
                        {
                            "+"
                        }
                        else
                        {
                            ""
                        }),
                        frac_temp,
                        (if i.imag() != &0.0
                        {
                            if options.color
                            {
                                "\x1b[93mi\x1b[0m"
                            }
                            else
                            {
                                "i"
                            }
                        }
                        else
                        {
                            ""
                        })
                    )
                }
                else
                {
                    out.clone().1
                };
            }
            output += &out.0;
            output += &out.1;
            if options.color
            {
                output += "\x1b[0m";
                frac_out += "\x1b[0m";
            }
            if k == v.len() - 1
            {
                output += if options.polar { "]" } else { "}" };
                frac_out += if options.polar { "]" } else { "}" };
            }
            else
            {
                output += ",";
                frac_out += ",";
            }
        }
        let terlen = get_terminal_width();
        let len = output
            .replace("\x1b[0m", "")
            .replace("\x1b[93m", "")
            .replace("\x1b[92m", "")
            .len()
            - 1;
        if frac_out != output
        {
            frac = 1;
        }
        if (frac == 1 && !options.frac)
            || frac_out
                .replace("\x1b[0m", "")
                .replace("\x1b[93m", "")
                .replace("\x1b[92m", "")
                .len()
                > terlen
            || len > terlen
        {
            frac = 0;
        }
        let num = (len as f64 / terlen as f64).floor() as usize;
        print!(
            "\x1B[0J{}\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}{}",
            if frac == 1
            {
                format!("\n\x1B[2K\x1B[1G{}", frac_out)
            }
            else
            {
                "".to_string()
            },
            output,
            "\x1b[A".repeat(num),
            if frac == 1 { "\x1b[A" } else { "" },
            if options.prompt
            {
                if options.color
                {
                    "\x1b[94m> \x1b[96m"
                }
                else
                {
                    "> "
                }
            }
            else if options.color
            {
                "\x1b[96m"
            }
            else
            {
                ""
            },
            &unmodified_input[start..end].iter().collect::<String>(),
            if options.color { "\x1b[0m" } else { "" }
        );
        frac += num;
    }
    else if let Matrix(v) = num
    {
        let mut output = if !options.multi { "{" } else { "" }.to_string();
        let mut frac_out = if !options.multi { "{" } else { "" }.to_string();
        let mut out;
        let mut frac_temp;
        let mut num = 0;
        for (l, j) in v.iter().enumerate()
        {
            if !options.multi
            {
                output += "{";
                frac_out += "{";
            }
            for (k, i) in j.iter().enumerate()
            {
                out = get_output(&options, i);
                if options.frac || options.frac_iter == 0
                {
                    frac_temp = fraction(i.real().clone(), options);
                    frac_out += if !frac_temp.is_empty()
                    {
                        &frac_temp
                    }
                    else
                    {
                        &out.0
                    };
                    frac_temp = fraction(i.imag().clone(), options);
                    frac_out += &if !frac_temp.is_empty()
                    {
                        format!(
                            "{}{}{}",
                            (if i.real() != &0.0 && i.imag().is_sign_positive() && i.imag() != &0.0
                            {
                                "+"
                            }
                            else
                            {
                                ""
                            }),
                            frac_temp,
                            (if i.imag() != &0.0
                            {
                                if options.color
                                {
                                    "\x1b[93mi\x1b[0m"
                                }
                                else
                                {
                                    "i"
                                }
                            }
                            else
                            {
                                ""
                            })
                        )
                    }
                    else
                    {
                        out.clone().1
                    };
                }
                output += &out.0;
                output += &out.1;
                if options.color
                {
                    output += "\x1b[0m";
                    frac_out += "\x1b[0m";
                }
                if k == j.len() - 1
                {
                    if !options.multi
                    {
                        output += "}";
                        frac_out += "}";
                    }
                }
                else if options.tabbed
                {
                    output += "\t";
                    frac_out += "\t";
                }
                else
                {
                    output += ",";
                    frac_out += ",";
                }
            }
            if l != v.len() - 1
            {
                if options.multi
                {
                    num += 1;
                    output += "\n";
                    frac_out += "\n";
                }
                else
                {
                    output += ",";
                    frac_out += ",";
                }
            }
        }
        if !options.multi
        {
            output += "}";
            frac_out += "}";
        }
        let terlen = get_terminal_width();
        let len = output
            .replace("\x1b[0m", "")
            .replace("\x1b[93m", "")
            .replace("\x1b[92m", "")
            .len()
            - 1;
        if frac_out != output
        {
            frac = 1;
        }
        if (frac == 1 && !options.frac)
            || frac_out
                .replace("\x1b[0m", "")
                .replace("\x1b[93m", "")
                .replace("\x1b[92m", "")
                .len()
                > terlen
            || len > terlen
        {
            frac = 0;
        }
        if !options.multi
        {
            num += (len as f64 / terlen as f64).floor() as usize;
        }
        else
        {
            let mut len = 0;
            for i in output
                .replace("\x1b[0m", "")
                .replace("\x1b[93m", "")
                .replace("\x1b[92m", "")
                .chars()
            {
                len += 1;
                if i == '\n'
                {
                    len = 0
                }
                else if len > terlen
                {
                    len = 0;
                    num += 1;
                }
            }
            frac_out += "\n";
            num += frac * 2;
        }
        print!(
            "\x1B[0J{}\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}{}",
            if frac == 1
            {
                format!("\n\x1B[2K\x1B[1G{}", frac_out)
            }
            else
            {
                "".to_string()
            },
            output,
            "\x1b[A".repeat(num),
            if frac == 1 { "\x1b[A" } else { "" },
            if options.prompt
            {
                if options.color
                {
                    "\x1b[94m> \x1b[96m"
                }
                else
                {
                    "> "
                }
            }
            else if options.color
            {
                "\x1b[96m"
            }
            else
            {
                ""
            },
            &unmodified_input[start..end].iter().collect::<String>(),
            if options.color { "\x1b[0m" } else { "" }
        );
        frac += num;
    }
    frac
}
pub fn get_output(options: &Options, num: &Complex) -> (String, String)
{
    let sign = if num.real() != &0.0 && num.imag().is_sign_positive()
    {
        "+"
    }
    else
    {
        ""
    }
    .to_owned();
    let mut n;
    let dec = if options.decimal_places == 0
    {
        1
    }
    else
    {
        options.decimal_places
    };
    if options.base != 10
    {
        (
            if num.real() != &0.0
            {
                n = num.real().to_string_radix(options.base as i32, None);
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
                n = num.imag().to_string_radix(options.base as i32, None);
                sign + &if n.contains('e')
                {
                    n
                }
                else
                {
                    n.trim_end_matches('0').trim_end_matches('.').to_owned()
                } + if options.color { "\x1b[93mi" } else { "i" }
            }
            else
            {
                "".to_string()
            },
        )
    }
    else if options.sci
    {
        (
            if num.real() != &0.0
            {
                add_commas(
                    &remove_trailing_zeros(&format!("{:e}", num.real()), dec, num.real().prec()),
                    options.comma,
                )
                .replace("e0", "")
                .replace(
                    'e',
                    if options.small_e
                    {
                        if options.color
                        {
                            "\x1b[92me"
                        }
                        else
                        {
                            "e"
                        }
                    }
                    else if options.color
                    {
                        "\x1b[92mE"
                    }
                    else
                    {
                        "E"
                    },
                ) + if options.color { "\x1b[0m" } else { "" }
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
                add_commas(
                    &(sign.as_str().to_owned()
                        + &remove_trailing_zeros(
                            &format!(
                                "{:e}{}",
                                num.imag(),
                                if options.color
                                {
                                    "\x1b[93mi\x1b[0m"
                                }
                                else
                                {
                                    "i"
                                }
                            ),
                            dec,
                            num.real().prec(),
                        )),
                    options.comma,
                )
                .replace("e0", "")
                .replace(
                    'e',
                    if options.small_e
                    {
                        if options.color
                        {
                            "\x1b[92me"
                        }
                        else
                        {
                            "e"
                        }
                    }
                    else if options.color
                    {
                        "\x1b[92mE"
                    }
                    else
                    {
                        "E"
                    },
                ) + if options.color { "\x1b[0m" } else { "" }
            }
            else
            {
                "".to_owned()
            },
        )
    }
    else
    {
        n = add_commas(
            &to_string(num.real(), options.decimal_places, false),
            options.comma,
        );
        let sign = if n == "0" { "".to_string() } else { sign };
        let im = add_commas(
            &to_string(num.imag(), options.decimal_places, true),
            options.comma,
        );
        (
            if n == "0" && im != "0"
            {
                "".to_string()
            }
            else
            {
                n
            },
            if im == "0"
            {
                "".to_string()
            }
            else
            {
                sign + &im + if options.color { "\x1b[93mi" } else { "i" }
            },
        )
    }
}
fn to_string(num: &Float, decimals: usize, imag: bool) -> String
{
    let (neg, mut str, exp) = num.to_sign_string_exp(10, None);
    let mut neg = if neg { "-" } else { "" };
    if exp.is_none()
    {
        return if str == "0"
        {
            "0".to_string()
        }
        else
        {
            format!("{}{}", neg, str)
        };
    }
    let exp = exp.unwrap();
    let decimals = if decimals == usize::MAX - 1 && (get_terminal_width() as i32) > (2i32 + exp)
    {
        (get_terminal_width() as i32
            - match exp.cmp(&0)
            {
                Ordering::Equal => 2i32,
                Ordering::Less => 3i32,
                Ordering::Greater => 1i32 + exp,
            }
            - if imag { 1 } else { 0 }
            - if !neg.is_empty() { 1 } else { 0 }) as usize
    }
    else
    {
        decimals
    };
    if str.len() as i32 == exp
    {
        return if str == "0"
        {
            "0".to_string()
        }
        else
        {
            format!("{}{}", neg, str)
        };
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
        return if str == "0"
        {
            "0".to_string()
        }
        else
        {
            format!("{}{}", neg, l)
        };
    }
    if r.len() > decimals
    {
        r.insert(decimals, '.');
    }
    let mut d = Float::with_val(num.prec(), Float::parse(&r).unwrap())
        .to_integer()
        .unwrap();
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
        let t: Float = Float::with_val(
            num.prec(),
            Float::parse(if l.is_empty() { "0" } else { &l }).unwrap(),
        ) + 1;
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
            format!(
                "{}{}",
                neg,
                Integer::from_str(&l).unwrap_or(Integer::new()) + 1
            )
        }
        else
        {
            format!("{}{}", neg, if l.is_empty() { "0" } else { &l })
        }
    }
    else
    {
        format!(
            "{}{}.{}{}",
            neg,
            if l.is_empty() { "0" } else { &l },
            zeros,
            d
        )
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
    }
}
fn add_commas(input: &str, commas: bool) -> String
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
fn remove_trailing_zeros(input: &str, dec: usize, prec: u32) -> String
{
    let pos = match input.find('e')
    {
        Some(n) => n,
        None =>
        {
            return input
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        }
    };
    let dec = if dec == usize::MAX - 1
    {
        get_terminal_width()
            - (if &input[pos..] == "e0"
            {
                1
            }
            else if &input[pos..] == "e0\x1b[93mi"
            {
                2
            }
            else
            {
                (input.len() - (pos - 1)) - if input.ends_with("\x1b[93mi") { 5 } else { 0 }
            })
            - if input.starts_with('-') { 1 } else { 0 }
    }
    else
    {
        dec
    };
    if dec > pos
    {
        input[..pos]
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
            + &input[pos..]
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
        num = Float::parse(num)
            .unwrap()
            .complete(prec)
            .to_integer()
            .unwrap()
            .to_string();
        num.insert(1, '.');
        sign + num.trim_end_matches('0').trim_end_matches('.') + &input[pos..]
    }
}