use crate::{
    complex::{
        to_polar, NumStr,
        NumStr::{Matrix, Num, Vector},
    },
    fraction::fraction,
    get_terminal_width,
    graph::can_graph,
    math::do_math,
    misc::{clear, get_terminal_height, handle_err, no_col, prompt, to_output},
    options::equal_to,
    parse::get_func,
    vars::input_var,
    AngleType::{Degrees, Gradians, Radians},
    Colors, Options,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float, Integer};
use std::{cmp::Ordering, str::FromStr};
pub fn print_answer(input: &str, func: Vec<NumStr>, options: Options, colors: &Colors)
{
    if can_graph(input)
    {
        return;
    }
    let num = match do_math(func, options)
    {
        Ok(num) => num,
        _ => return,
    };
    if let Num(n) = num
    {
        let a = get_output(options, colors, &n);
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
            out = get_output(options, colors, i);
            output += out.0.as_str();
            output += out.1.as_str();
            if options.color
            {
                output += "\x1b[0m";
            }
            output += if k == v.len() - 1
            {
                if options.polar
                {
                    "]"
                }
                else
                {
                    "}"
                }
            }
            else
            {
                ","
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
                out = get_output(options, colors, i);
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
    colors: &Colors,
    start: usize,
    end: usize,
) -> usize
{
    {
        let input = unmodified_input.iter().collect::<String>();
        if input.ends_with('=')
        {
            let out = equal_to(
                options,
                colors,
                vars,
                &input[..input.len() - 1],
                &last.iter().collect::<String>(),
            );
            return if !out.is_empty()
            {
                let wrap = (no_col(&out, options.color).len() - 1) / get_terminal_width() + 1;
                print!(
                    "\n\x1b[G\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                    out,
                    "\x1b[A".repeat(wrap),
                    prompt(options, colors),
                    to_output(&unmodified_input[start..end], options.color, colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
                wrap
            }
            else
            {
                clear(unmodified_input, start, end, options, colors);
                0
            };
        }
        else if input
            .replace("==", "")
            .replace("!=", "")
            .replace(">=", "")
            .replace("<=", "")
            .contains('=')
        {
            clear(unmodified_input, start, end, options, colors);
            return 0;
        }
    }
    let input = &input_var(
        &unmodified_input
            .iter()
            .collect::<String>()
            .replace('_', &format!("({})", last.iter().collect::<String>())),
        vars,
        &Vec::new(),
        None,
        &mut Vec::new(),
        &mut 0,
        options,
    );
    if can_graph(input)
    {
        return if input.contains('#')
        {
            let mut out = String::new();
            {
                let split = unmodified_input.iter().collect::<String>();
                let split = split.split('#');
                if split.clone().count() > 6
                {
                    print!(
                        "\n\x1b[G\x1b[Jtoo many graphs\x1b[A\x1b[G\x1b[K{}{}{}",
                        prompt(options, colors),
                        to_output(&unmodified_input[start..end], options.color, colors),
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    return 1;
                }
                for input in split
                {
                    if !input.is_empty()
                    {
                        out += &equal_to(
                            options,
                            colors,
                            vars,
                            input,
                            &last.iter().collect::<String>(),
                        );
                        out += "\n"
                    }
                }
            }
            out.pop();
            let wrap = no_col(&out, options.color)
                .split('\n')
                .map(|i| {
                    if i.is_empty()
                    {
                        1
                    }
                    else
                    {
                        (i.len() - 1) / get_terminal_width() + 1
                    }
                })
                .sum();
            print!(
                "\n\x1b[G\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                out.replace('\n', "\n\x1b[G"),
                "\x1b[A".repeat(wrap),
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" }
            );
            wrap
        }
        else
        {
            let input = unmodified_input.iter().collect::<String>();
            let out = equal_to(
                options,
                colors,
                vars,
                &input,
                &last.iter().collect::<String>(),
            );
            let wrap = (no_col(&out, options.color).len() - 1) / get_terminal_width() + 1;
            print!(
                "\n\x1b[G\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                out,
                "\x1b[A".repeat(wrap),
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" }
            );
            wrap
        };
    }
    let num = match do_math(
        match get_func(input, options)
        {
            Ok(f) => f,
            Err(s) =>
            {
                handle_err(s, unmodified_input, options, colors, start, end);
                return 0;
            }
        },
        options,
    )
    {
        Ok(n) => n,
        Err(s) =>
        {
            handle_err(s, unmodified_input, options, colors, start, end);
            return 0;
        }
    };
    let mut frac = 0;
    if let Num(n) = num
    {
        let sign = if !n.real().is_zero() && n.imag().is_sign_positive()
        {
            "+"
        }
        else
        {
            ""
        }
        .to_string();
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
                        if n.real().is_zero() && !n.imag().is_zero()
                        {
                            "".to_string()
                        }
                        else
                        {
                            fa
                        },
                        if n.imag().is_zero()
                        {
                            "".to_string()
                        }
                        else
                        {
                            sign + fb.as_str()
                                + &if options.color
                                {
                                    format!("{}i\x1b[0m", colors.imag)
                                }
                                else
                                {
                                    "i".to_string()
                                }
                        },
                    )
                }
                (true, _) =>
                {
                    frac = 1;
                    (
                        if n.real().is_zero() && !n.imag().is_zero()
                        {
                            "".to_string()
                        }
                        else
                        {
                            fa
                        },
                        if n.imag().is_zero()
                        {
                            "".to_string()
                        }
                        else
                        {
                            get_output(options, colors, &n).1
                                + if options.color { "\x1b[0m" } else { "" }
                        },
                    )
                }
                (_, true) =>
                {
                    frac = 1;
                    (
                        if n.real().is_zero() && !n.imag().is_zero()
                        {
                            "".to_string()
                        }
                        else
                        {
                            get_output(options, colors, &n).0
                        },
                        if n.imag().is_zero()
                        {
                            "".to_string()
                        }
                        else
                        {
                            sign + fb.as_str()
                                + &if options.color
                                {
                                    format!("{}i\x1b[0m", colors.imag)
                                }
                                else
                                {
                                    "i".to_string()
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
        let output = get_output(options, colors, &n);
        let terlen = get_terminal_width();
        let len1 = no_col(&output.0, options.color).len();
        let len2 = no_col(&output.1, options.color).len();
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
        if len1 + len2 > terlen * (get_terminal_height() - 1)
        {
            print!(
                "\x1b[J\n\x1b[Gtoo long, run from cli\x1b[A\x1b[G\x1b[K{}{}{}",
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" },
            );
            frac = 0;
        }
        else if len1 + len2 > terlen
        {
            let num = len1.div_ceil(terlen)
                + if len2 != 0
                {
                    (len2 - 1).div_ceil(terlen) - 1
                }
                else
                {
                    0
                }
                - 1;
            print!(
                "\x1b[J{}\n\x1b[G\x1b[K{}{}{}{}\x1b[A\x1b[A\x1b[G\x1b[K{}{}{}",
                if frac == 1
                {
                    format!("\n\x1b[G\x1b[K{}{}", frac_a, frac_b)
                }
                else
                {
                    "".to_string()
                },
                output.0,
                if len1 != 0 && len2 != 0
                {
                    "\n\x1b[G"
                }
                else
                {
                    ""
                },
                &output.1.replace('+', ""),
                "\x1b[A".repeat(num + frac - if len1 == 0 || len2 == 0 { 1 } else { 0 }),
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" },
            );
            frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
        }
        else
        {
            print!(
                "\x1b[J{}\n\x1b[G\x1b[K{}{}\x1b[A{}\x1b[G\x1b[K{}{}{}",
                if frac == 1
                {
                    format!("\n\x1b[G\x1b[K{}{}", frac_a, frac_b)
                }
                else
                {
                    "".to_string()
                },
                output.0,
                output.1,
                if frac == 1 { "\x1b[A" } else { "" },
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
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
            out = get_output(options, colors, i);
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
                        if !i.real().is_zero() && !i.imag().is_zero() && i.imag().is_sign_positive()
                        {
                            "+"
                        }
                        else
                        {
                            ""
                        },
                        frac_temp,
                        if !i.imag().is_zero()
                        {
                            if options.color
                            {
                                format!("{}i", colors.imag)
                            }
                            else
                            {
                                "i".to_string()
                            }
                        }
                        else
                        {
                            "".to_string()
                        }
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
        let length = no_col(&output, options.color).len();
        if frac_out != output
        {
            frac = 1;
        }
        if (frac == 1 && !options.frac)
            || no_col(&frac_out, options.color).len() > terlen
            || length > terlen
        {
            frac = 0;
        }
        if length > terlen * (get_terminal_height() - 1)
        {
            print!(
                "\x1b[J\n\x1b[Gtoo long, run from cli\x1b[A\x1b[G\x1b[K{}{}{}",
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" },
            );
            frac = 0;
        }
        else
        {
            let num = (length - 1) / terlen;
            print!(
                "\x1b[J{}\n\x1b[G\x1b[K{}{}\x1b[A{}\x1b[G\x1b[K{}{}{}",
                if frac == 1
                {
                    format!("\n\x1b[G\x1b[K{}", frac_out)
                }
                else
                {
                    "".to_string()
                },
                output,
                "\x1b[A".repeat(num),
                if frac == 1 { "\x1b[A" } else { "" },
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" }
            );
            frac += num;
        }
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
                out = get_output(options, colors, i);
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
                            if !i.real().is_zero()
                                && !i.imag().is_zero()
                                && i.imag().is_sign_positive()
                            {
                                "+"
                            }
                            else
                            {
                                ""
                            },
                            frac_temp,
                            if !i.imag().is_zero()
                            {
                                if options.color
                                {
                                    format!("{}i", colors.imag)
                                }
                                else
                                {
                                    "i".to_string()
                                }
                            }
                            else
                            {
                                "".to_string()
                            }
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
                    output += "\n\x1b[G";
                    frac_out += "\n\x1b[G";
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
        let length = no_col(&output, options.color).len() - 1;
        if frac_out != output
        {
            frac = 1;
        }
        if !options.multi
        {
            num += (length - 1) / terlen;
            if (frac == 1 && !options.frac)
                || no_col(&frac_out, options.color).len() > terlen
                || length > terlen
            {
                frac = 0;
            }
        }
        else
        {
            let mut len = 0;
            for i in no_col(&frac_out, options.color).chars()
            {
                len += 1;
                if i == '\n'
                {
                    len = 0
                }
                else if len > terlen
                {
                    frac = 0;
                    break;
                }
            }
            len = 0;
            for i in no_col(&output, options.color).chars()
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
        }
        if length > terlen * (get_terminal_height() - 1) || num > (get_terminal_height() - 2)
        {
            print!(
                "\x1b[J\n\x1b[Gtoo long, run from cli\x1b[A\x1b[G\x1b[K{}{}{}",
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" },
            );
            frac = 0;
        }
        else
        {
            print!(
                "\x1b[J{}\n\x1b[G\x1b[K{}{}\x1b[A{}\x1b[G\x1b[K{}{}{}",
                if frac == 1
                {
                    num *= 2;
                    num += 1;
                    format!("\n\x1b[G\x1b[K{}", frac_out)
                }
                else
                {
                    "".to_string()
                },
                output,
                "\x1b[A".repeat(num),
                if frac == 1 { "\x1b[A" } else { "" },
                prompt(options, colors),
                to_output(&unmodified_input[start..end], options.color, colors),
                if options.color { "\x1b[0m" } else { "" }
            );
            frac += num;
        }
    }
    else
    {
        handle_err("str err", unmodified_input, options, colors, start, end);
    }
    frac
}
pub fn get_output(options: Options, colors: &Colors, num: &Complex) -> (String, String)
{
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
        let sign = if num.imag().is_sign_positive() && !num.real().is_zero()
        {
            "+"
        }
        else
        {
            ""
        }
        .to_string();
        (
            if !num.real().is_zero()
            {
                let n = num.real().to_string_radix(options.base as i32, None);
                if n.contains('e')
                {
                    n
                }
                else
                {
                    n.trim_end_matches('0').trim_end_matches('.').to_string()
                }
            }
            else if num.imag().is_zero()
            {
                "0".to_string()
            }
            else
            {
                "".to_string()
            },
            if !num.imag().is_zero()
            {
                let n = num.imag().to_string_radix(options.base as i32, None);
                sign + &if n.contains('e')
                {
                    n
                }
                else
                {
                    n.trim_end_matches('0').trim_end_matches('.').to_string()
                } + &if options.color
                {
                    format!("{}i", colors.imag)
                }
                else
                {
                    "i".to_string()
                }
            }
            else
            {
                "".to_string()
            },
        )
    }
    else if options.sci
    {
        let sign = if num.imag().is_sign_positive() && !num.real().is_zero()
        {
            "+"
        }
        else
        {
            ""
        }
        .to_string();
        (
            if num.real().is_zero() && !num.imag().is_zero()
            {
                "".to_string()
            }
            else
            {
                if options.comma
                {
                    add_commas(&remove_trailing_zeros(
                        &format!("{:e}", num.real()),
                        dec,
                        num.real().prec(),
                    ))
                }
                else
                {
                    remove_trailing_zeros(&format!("{:e}", num.real()), dec, num.real().prec())
                }
                .replace("e0", "")
                .replace(
                    'e',
                    &if options.small_e
                    {
                        if options.color
                        {
                            format!("{}e", colors.sci)
                        }
                        else
                        {
                            "e".to_string()
                        }
                    }
                    else if options.color
                    {
                        format!("{}E", colors.sci)
                    }
                    else
                    {
                        "E".to_string()
                    },
                ) + if options.color { "\x1b[0m" } else { "" }
            },
            if num.imag().is_zero()
            {
                "".to_string()
            }
            else
            {
                if options.comma
                {
                    add_commas(
                        &(sign
                            + &remove_trailing_zeros(
                                &format!("{:e}", num.imag(),),
                                dec,
                                num.real().prec(),
                            )),
                    )
                }
                else
                {
                    sign + &remove_trailing_zeros(
                        &format!("{:e}", num.imag(),),
                        dec,
                        num.real().prec(),
                    )
                }
                .replace("e0", "")
                .replace(
                    'e',
                    &if options.small_e
                    {
                        if options.color
                        {
                            format!("{}e", colors.sci)
                        }
                        else
                        {
                            "e".to_string()
                        }
                    }
                    else if options.color
                    {
                        format!("{}E", colors.sci)
                    }
                    else
                    {
                        "E".to_string()
                    },
                ) + &if options.color
                {
                    format!("{}i", colors.imag)
                }
                else
                {
                    "i".to_string()
                }
            },
        )
    }
    else
    {
        let re = if options.comma
        {
            add_commas(&to_string(num.real(), options.decimal_places, false))
        }
        else
        {
            to_string(num.real(), options.decimal_places, false)
        };
        let im = if options.comma
        {
            add_commas(&to_string(num.imag(), options.decimal_places, true))
        }
        else
        {
            to_string(num.imag(), options.decimal_places, true)
        };
        let sign = if num.imag().is_sign_positive() && re != "0"
        {
            "+"
        }
        else
        {
            ""
        }
        .to_string();
        (
            if re == "0" && im != "0"
            {
                "".to_string()
            }
            else
            {
                re
            },
            if im == "0"
            {
                "".to_string()
            }
            else
            {
                sign + &im
                    + &if options.color
                    {
                        format!("{}i", colors.imag)
                    }
                    else
                    {
                        "i".to_string()
                    }
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
    if d.to_string().trim_end_matches('0') == "1"
        && r.trim_start_matches('0')
            .trim_start_matches('.')
            .starts_with('9')
    {
        if zeros.is_empty()
        {
            let t: Float = Float::with_val(
                num.prec(),
                Float::parse(if l.is_empty() { "0" } else { &l }).unwrap(),
            ) + 1;
            l = t.to_integer().unwrap().to_string();
            d = Integer::new();
        }
        else
        {
            zeros.pop();
        }
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
fn add_commas(input: &str) -> String
{
    let mut split = input.split('.');
    let mut result = String::new();
    let mut count = 0;
    let mut exp = false;
    for c in split.next().unwrap().chars().rev()
    {
        if c == '-'
        {
            count -= 1;
        }
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
            if c == '-'
            {
                count -= 1;
            }
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
            let s = input
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();
            return if s.is_empty() { "0".to_string() } else { s };
        }
    };
    let dec = if dec == usize::MAX - 1
    {
        get_terminal_width()
            - (if &input[pos..] == "e0"
            {
                2
            }
            else
            {
                input.len() - pos + 2
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
        if dec >= num.len()
        {
            input[..pos]
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
                + &input[pos..]
        }
        else
        {
            num.insert(dec, '.');
            num = Float::parse(num)
                .unwrap()
                .complete(prec)
                .to_integer()
                .unwrap()
                .to_string();
            num.insert(1, '.');
            sign + num.trim_end_matches('0').trim_end_matches('.')
                + "e"
                + &(input[pos + 1..].parse::<isize>().unwrap()
                    + if num.len() - 1 > dec { 1 } else { 0 })
                .to_string()
        }
    }
}