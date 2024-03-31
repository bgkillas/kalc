use crate::{
    complex::{
        to_polar, NumStr,
        NumStr::{Matrix, Num, Vector},
    },
    fraction::fraction,
    help::help_for,
    load_vars::set_commands_or_vars,
    math::do_math,
    misc::{
        clear, clearln, get_terminal_dimensions, handle_err, no_col, parsed_to_string, prompt,
        to_output,
    },
    options::{equal_to, silent_commands},
    parse::input_var,
    AngleType::{Degrees, Gradians, Radians},
    Colors, Number, Options, Variable,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float, Integer};
use std::cmp::Ordering;
#[allow(clippy::too_many_arguments)]
pub fn print_concurrent(
    unmodified_input: &[char],
    last: &[char],
    mut vars: Vec<Variable>,
    mut options: Options,
    mut colors: Colors,
    start: usize,
    end: usize,
    long_output: bool,
) -> (usize, bool, bool, bool)
{
    if unmodified_input.starts_with(&['#']) || unmodified_input.is_empty()
    {
        clear(unmodified_input, start, end, options, &colors);
        return (0, false, false, false);
    }
    let mut unparsed = unmodified_input;
    {
        let split = unmodified_input.split(|c| c == &';');
        let count = split.clone().count();
        if count != 1
        {
            unparsed = split.clone().last().unwrap();
            for (i, s) in split.enumerate()
            {
                if i == count - 1
                {
                    break;
                }
                silent_commands(
                    &mut options,
                    &s.iter()
                        .copied()
                        .filter(|&c| !c.is_whitespace())
                        .collect::<Vec<char>>(),
                );
                if s.contains(&'=')
                {
                    if let Err(s) = set_commands_or_vars(&mut colors, &mut options, &mut vars, s)
                    {
                        handle_err(s, unmodified_input, options, &colors, start, end);
                        return (0, false, false, false);
                    }
                }
            }
        }
        let tempinput = unparsed.iter().collect::<String>();
        if tempinput.starts_with("help ")
        {
            let message = help_for(tempinput.splitn(2, ' ').last().unwrap());
            let mut num = message.chars().filter(|c| c == &'\n').count();
            {
                let width = get_terminal_dimensions().0;
                let mut len = 0;
                for i in no_col(&message, options.color)
                {
                    len += 1;
                    if i == '\n'
                    {
                        len = 0
                    }
                    else if len > width
                    {
                        len = 0;
                        num += 1;
                    }
                }
            }
            print!(
                "\x1b[G\x1b[J\n{}\x1b[G\x1b[{}A{}{}{}",
                message,
                num + 1,
                prompt(options, &colors),
                to_output(&unmodified_input[start..end], options.color, &colors),
                if options.color { "\x1b[0m" } else { "" },
            );
            return (num, false, false, false);
        }
        else if tempinput.ends_with('=')
        {
            let out = equal_to(
                options,
                &colors,
                &vars,
                &tempinput[..tempinput.len() - 1],
                &last.iter().collect::<String>(),
            );
            return if !out.is_empty()
            {
                let (width, height) = get_terminal_dimensions();
                let len = no_col(&out, options.color).len();
                let wrap = (len - 1) / width + 1;
                if len > width * (height - 1)
                {
                    if long_output
                    {
                        print!(
                            "\x1b[G\n\x1b[J{}{}",
                            out,
                            if options.color { "\x1b[0m" } else { "" }
                        );
                        (wrap, false, false, false)
                    }
                    else
                    {
                        print!(
                            "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                            prompt(options, &colors),
                            to_output(&unmodified_input[start..end], options.color, &colors),
                            if options.color { "\x1b[0m" } else { "" },
                        );
                        (0, false, true, false)
                    }
                }
                else
                {
                    print!(
                        "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                        out,
                        if wrap == 0
                        {
                            String::new()
                        }
                        else
                        {
                            format!("\x1b[{}A", wrap)
                        },
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    (wrap - 1, false, false, false)
                }
            }
            else
            {
                clearln(unmodified_input, start, end, options, &colors);
                (0, false, false, false)
            };
        }
    }
    let input = match input_var(
        &unparsed
            .iter()
            .collect::<String>()
            .replace('_', &format!("({})", last.iter().collect::<String>())),
        vars.to_vec(),
        &mut Vec::new(),
        &mut 0,
        options,
        false,
        false,
        0,
        Vec::new(),
    )
    {
        Ok(f) => f,
        Err(s) =>
        {
            handle_err(s, unmodified_input, options, &colors, start, end);
            return (0, false, false, false);
        }
    };
    {
        if input.3
        {
            let n = unparsed.iter().position(|c| c == &'=').unwrap_or(0) + 1;
            let mut input = unparsed[n..].iter().collect::<String>();
            let mut func = unparsed[..n].to_vec();
            let mut func_vars: Vec<(isize, String)> = Vec::new();
            if func.contains(&'(')
            {
                func.drain(0..=func.iter().position(|c| c == &'(').unwrap());
                func.pop();
                func.pop();
                for i in func.split(|c| c == &',')
                {
                    func_vars.push((-1, i.iter().collect()));
                }
            }
            if input.contains(':')
            {
                let inp = input;
                let mut split = inp.split(':').collect::<Vec<&str>>();
                input = split.pop().unwrap().to_string();
                for i in split
                {
                    if i.contains('=')
                    {
                        let mut split = i.splitn(2, '=');
                        func_vars.push((-1, split.next().unwrap().to_string()));
                    }
                }
            }
            let out = match input_var(
                &input,
                vars.to_vec(),
                &mut func_vars,
                &mut 0,
                options,
                false,
                true,
                0,
                Vec::new(),
            )
            {
                Ok(n) => parsed_to_string(n.0, n.1, &options, &colors),
                _ => String::new(),
            };
            if out.is_empty()
            {
                clear(unmodified_input, start, end, options, &colors);
                return (0, false, false, true);
            }
            let (width, height) = get_terminal_dimensions();
            let len = no_col(&out, options.color).len();
            let wrap = (len - 1) / width + 1;
            return if len > width * (height - 1)
            {
                if long_output
                {
                    print!(
                        "\x1b[G\n\x1b[J{}{}",
                        out,
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    (wrap, true, false, true)
                }
                else
                {
                    print!(
                        "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    (0, true, true, true)
                }
            }
            else
            {
                print!(
                    "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                    out,
                    if wrap == 0
                    {
                        String::new()
                    }
                    else
                    {
                        format!("\x1b[{}A", wrap)
                    },
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
                (wrap, true, false, true)
            };
        }
    }
    if input.2
    {
        return if unmodified_input.contains(&'#')
        {
            let mut out = String::new();
            {
                let split = unmodified_input.iter().collect::<String>();
                let split = split.split('#');
                if split.clone().count() > 6
                {
                    print!(
                        "\x1b[G\n\x1b[Jtoo many graphs\x1b[G\x1b[A\x1b[K{}{}{}",
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    return (1, true, false, false);
                }
                let mut vars = vars.clone();
                let mut options = options;
                let mut colors = colors.clone();
                for input in split
                {
                    if !input.is_empty()
                    {
                        let mut input = input;
                        let split = input.split(|c| c == ';');
                        let count = split.clone().count();
                        if count != 1
                        {
                            input = split.clone().last().unwrap();
                            for (i, s) in split.enumerate()
                            {
                                if i == count - 1
                                {
                                    break;
                                }
                                silent_commands(
                                    &mut options,
                                    &s.chars()
                                        .filter(|c| !c.is_whitespace())
                                        .collect::<Vec<char>>(),
                                );
                                if s.contains('=')
                                {
                                    if let Err(s) = set_commands_or_vars(
                                        &mut colors,
                                        &mut options,
                                        &mut vars,
                                        &s.chars().collect::<Vec<char>>(),
                                    )
                                    {
                                        handle_err(
                                            s,
                                            unmodified_input,
                                            options,
                                            &colors,
                                            start,
                                            end,
                                        );
                                        return (0, false, false, false);
                                    }
                                }
                            }
                        }
                        out += &equal_to(
                            options,
                            &colors,
                            &vars,
                            input,
                            &last.iter().collect::<String>(),
                        );
                        out += "\n"
                    }
                }
            }
            out.pop();
            let (width, height) = get_terminal_dimensions();
            let no_col = no_col(&out, options.color);
            let wrap = no_col
                .split(|c| c == &'\n')
                .map(|i| {
                    if i.is_empty()
                    {
                        1
                    }
                    else
                    {
                        (i.len() - 1) / width + 1
                    }
                })
                .sum();
            let len = no_col.len();
            let out = out.replace('\n', "\x1b[G\n");
            if len > width * (height - 1)
            {
                if long_output
                {
                    print!(
                        "\x1b[G\n\x1b[J{}{}",
                        out,
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    (wrap, true, false, false)
                }
                else
                {
                    print!(
                        "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    (0, true, true, false)
                }
            }
            else
            {
                print!(
                    "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                    out,
                    if wrap == 0
                    {
                        String::new()
                    }
                    else
                    {
                        format!("\x1b[{}A", wrap)
                    },
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
                (wrap, true, false, false)
            }
        }
        else
        {
            let input = unparsed.iter().collect::<String>();
            let out = equal_to(
                options,
                &colors,
                &vars,
                &input,
                &last.iter().collect::<String>(),
            );
            let (width, height) = get_terminal_dimensions();
            let len = no_col(&out, options.color).len();
            let wrap = (len - 1) / width + 1;
            if len > width * (height - 1)
            {
                if long_output
                {
                    print!(
                        "\x1b[G\n\x1b[J{}{}",
                        out,
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    (wrap, true, false, false)
                }
                else
                {
                    print!(
                        "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    (0, true, true, false)
                }
            }
            else
            {
                print!(
                    "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                    out,
                    if wrap == 0
                    {
                        String::new()
                    }
                    else
                    {
                        format!("\x1b[{}A", wrap)
                    },
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
                (wrap, true, false, false)
            }
        };
    }
    let num = match do_math(input.0, options, input.1)
    {
        Ok(n) => n,
        Err(s) =>
        {
            if s == " "
            {
                clear(unmodified_input, start, end, options, &colors)
            }
            else
            {
                handle_err(s, unmodified_input, options, &colors, start, end);
            }
            return (0, false, false, false);
        }
    };
    let mut frac = 0;
    let mut long = false;
    match num
    {
        Num(n) =>
        {
            let mut output = get_output(options, &colors, &n);
            let (mut frac_a, frac_b) = if options.frac || options.frac_iter == 0
            {
                let n = n.number;
                let fa = fraction(n.real().clone(), options);
                let fb = fraction(n.imag().clone(), options);
                let sign = if !output.0.is_empty() && n.imag().is_sign_positive()
                {
                    "+"
                }
                else
                {
                    ""
                }
                .to_string();
                match (!fa.is_empty(), !fb.is_empty())
                {
                    (true, true) =>
                    {
                        frac = 1;
                        (
                            if n.real().is_zero() && !n.imag().is_zero()
                            {
                                String::new()
                            }
                            else
                            {
                                fa
                            },
                            if n.imag().is_zero()
                            {
                                String::new()
                            }
                            else
                            {
                                sign + fb.as_str()
                                    + &if options.color
                                    {
                                        format!("{}i\x1b[0m", &colors.imag)
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
                                String::new()
                            }
                            else
                            {
                                fa
                            },
                            if n.imag().is_zero()
                            {
                                String::new()
                            }
                            else
                            {
                                output.1.clone() + if options.color { "\x1b[0m" } else { "" }
                            },
                        )
                    }
                    (_, true) =>
                    {
                        frac = 1;
                        (
                            if n.real().is_zero() && !n.imag().is_zero()
                            {
                                String::new()
                            }
                            else
                            {
                                output.0.clone()
                            },
                            if n.imag().is_zero()
                            {
                                String::new()
                            }
                            else
                            {
                                sign + fb.as_str()
                                    + &if options.color
                                    {
                                        format!("{}i\x1b[0m", &colors.imag)
                                    }
                                    else
                                    {
                                        "i".to_string()
                                    }
                            },
                        )
                    }
                    _ => (String::new(), String::new()),
                }
            }
            else
            {
                (String::new(), String::new())
            };
            let (width, height) = get_terminal_dimensions();
            if let Some(st) = output.2
            {
                output.0 += &st;
                frac_a += &st;
            }
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
                    > width
            {
                frac = 0;
            }
            if len1 + len2 > width * (height - 1)
            {
                if long_output
                {
                    let num = len1.div_ceil(width).saturating_sub(1)
                        + len2.saturating_sub(1).div_ceil(width).saturating_sub(1);
                    print!(
                        "\x1b[G\n\x1b[J{}\x1b[G{}{}{}{}",
                        if frac == 1
                        {
                            format!("{}{}\x1b[G\n", frac_a, frac_b)
                        }
                        else
                        {
                            String::new()
                        },
                        output.0,
                        if len1 != 0 && len2 != 0
                        {
                            "\x1b[G\n"
                        }
                        else
                        {
                            ""
                        },
                        &output.1.replace('+', ""),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
                }
                else
                {
                    print!(
                        "\x1b[G\x1b[J{}\x1b[G\ntoo long, will print on enter{}\x1b[G\x1b[A{}{}{}",
                        if frac == 1
                        {
                            format!("\x1b[G\n{}{}", frac_a, frac_b)
                        }
                        else
                        {
                            String::new()
                        },
                        if frac == 1 { "\x1b[A" } else { "" },
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    long = true;
                }
            }
            else if len1 + len2 > width
            {
                let num = len1.div_ceil(width).saturating_sub(1)
                    + len2.saturating_sub(1).div_ceil(width).saturating_sub(1);
                let temp = (num + frac).saturating_sub(if len1 == 0 || len2 == 0 { 1 } else { 0 });
                print!(
                    "\x1b[G\x1b[J{}\x1b[G\n{}{}{}{}\x1b[A\x1b[G\x1b[A{}{}{}",
                    if frac == 1
                    {
                        format!("\x1b[G\n{}{}", frac_a, frac_b)
                    }
                    else
                    {
                        String::new()
                    },
                    output.0,
                    if len1 != 0 && len2 != 0
                    {
                        "\x1b[G\n"
                    }
                    else
                    {
                        ""
                    },
                    &output.1.replace('+', ""),
                    if temp == 0
                    {
                        String::new()
                    }
                    else
                    {
                        format!("\x1b[{}A", temp)
                    },
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if options.color { "\x1b[0m" } else { "" },
                );
                frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
            }
            else
            {
                print!(
                    "\x1b[G{}{}\x1b[K{}\x1b[G\n{}{}\x1b[J{}\x1b[G\x1b[A\x1b[{}C{}",
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if frac == 1
                    {
                        format!("\x1b[G\n{}{}\x1b[K", frac_a, frac_b)
                    }
                    else
                    {
                        String::new()
                    },
                    output.0,
                    output.1,
                    if frac == 1 { "\x1b[A" } else { "" },
                    if options.prompt { 2 } else { 0 } + (end - start),
                    if options.color { "\x1b[0m" } else { "" }
                )
            }
        }
        Vector(mut v) =>
        {
            if options.polar
            {
                v = to_polar(
                    v,
                    match options.angle
                    {
                        Degrees => 180 / Complex::with_val(options.prec, Pi),
                        Radians => Complex::with_val(options.prec, 1),
                        Gradians => 200 / Complex::with_val(options.prec, Pi),
                    },
                );
            }
            let mut output = if options.polar { "[" } else { "{" }.to_string();
            let mut frac_out = if options.polar { "[" } else { "{" }.to_string();
            let mut out;
            let mut frac_temp;
            for (k, i) in v.iter().enumerate()
            {
                out = get_output(options, &colors, i);
                let i = &i.number;
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
                            if !out.0.is_empty()
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
                                    format!("{}i", &colors.imag)
                                }
                                else
                                {
                                    "i".to_string()
                                }
                            }
                            else
                            {
                                String::new()
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
                if let Some(st) = out.2
                {
                    output += &st;
                    frac_out += &st;
                }
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
            let (width, height) = get_terminal_dimensions();
            let length = no_col(&output, options.color).len();
            if frac_out != output
            {
                frac = 1;
            }
            if (frac == 1 && !options.frac)
                || no_col(&frac_out, options.color).len() > width
                || length > width
            {
                frac = 0;
            }
            if length > width * (height - 1)
            {
                if long_output
                {
                    let num = (length - 1) / width;
                    print!(
                        "\x1b[G\n\x1b[J{}\x1b[G{}{}",
                        if frac == 1 && options.frac
                        {
                            format!("{}\x1b[G\n", frac_out)
                        }
                        else
                        {
                            String::new()
                        },
                        output,
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    frac += num;
                }
                else
                {
                    print!(
                        "\x1b[G\x1b[J\ntoo long, will print on enter\x1b[G\x1b[A{}{}{}",
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    long = true;
                    frac = 0;
                }
            }
            else
            {
                let num = (length - 1) / width;
                print!(
                    "\x1b[G{}{}\x1b[K{}\x1b[G\n{}\x1b[J{}{}\x1b[G\x1b[A\x1b[{}C{}",
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if frac == 1
                    {
                        format!("\x1b[G\n{}\x1b[K", frac_out)
                    }
                    else
                    {
                        String::new()
                    },
                    output,
                    if num == 0
                    {
                        String::new()
                    }
                    else
                    {
                        format!("\x1b[{}A", num)
                    },
                    if frac == 1 { "\x1b[A" } else { "" },
                    if options.prompt { 2 } else { 0 } + (end - start),
                    if options.color { "\x1b[0m" } else { "" }
                );
                frac += num;
            }
        }
        Matrix(v) =>
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
                    out = get_output(options, &colors, i);
                    let i = &i.number;
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
                                if !out.0.is_empty()
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
                                        format!("{}i", &colors.imag)
                                    }
                                    else
                                    {
                                        "i".to_string()
                                    }
                                }
                                else
                                {
                                    String::new()
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
                    if let Some(st) = out.2
                    {
                        output += &st;
                        frac_out += &st;
                    }
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
                        output += "\x1b[K\x1b[G\n";
                        frac_out += "\x1b[K\x1b[G\n";
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
            let (width, height) = get_terminal_dimensions();
            let length = no_col(&output, options.color).len() - 1;
            if frac_out != output
            {
                frac = 1;
            }
            if !options.multi
            {
                num += (length - 1) / width;
                if (frac == 1 && !options.frac)
                    || no_col(&frac_out, options.color).len() > width
                    || length > width
                {
                    frac = 0;
                }
            }
            else
            {
                let mut len = 0;
                for i in no_col(&frac_out, options.color)
                {
                    len += 1;
                    if i == '\n'
                    {
                        len = 0
                    }
                    else if len > width
                    {
                        frac = 0;
                        break;
                    }
                }
                len = 0;
                for i in no_col(&output, options.color)
                {
                    len += 1;
                    if i == '\n'
                    {
                        len = 0
                    }
                    else if len > width
                    {
                        len = 0;
                        num += 1;
                    }
                }
                frac_out += "\x1b[K\x1b[G\n\x1b[K";
            }
            if length > width * (height - 1) || num > (height - 2)
            {
                if long_output
                {
                    print!(
                        "\x1b[G\n\x1b[J{}\x1b[G{}{}",
                        if frac == 1 && options.frac
                        {
                            num *= 2;
                            if options.multi
                            {
                                num += 1;
                            }
                            format!("{}\x1b[G\n", frac_out)
                        }
                        else
                        {
                            String::new()
                        },
                        output,
                        if options.color { "\x1b[0m" } else { "" }
                    );
                    frac += num;
                }
                else
                {
                    print!(
                        "\x1b[G\x1b[J\ntoo long, will print on enter\x1b[G\x1b[A{}{}{}",
                        prompt(options, &colors),
                        to_output(&unmodified_input[start..end], options.color, &colors),
                        if options.color { "\x1b[0m" } else { "" },
                    );
                    long = true;
                    frac = 0;
                }
            }
            else
            {
                if !options.frac
                {
                    frac = 0;
                }
                print!(
                    "\x1b[G{}{}\x1b[K{}\x1b[G\n{}\x1b[J{}\x1b[G\x1b[A\x1b[{}C{}",
                    prompt(options, &colors),
                    to_output(&unmodified_input[start..end], options.color, &colors),
                    if frac == 1
                    {
                        num *= 2;
                        if options.multi
                        {
                            num += 1;
                        }
                        format!("\x1b[G\n{}\x1b[K", frac_out)
                    }
                    else
                    {
                        String::new()
                    },
                    output,
                    if num + frac == 0
                    {
                        String::new()
                    }
                    else
                    {
                        format!("\x1b[{}A", num + frac)
                    },
                    if options.prompt { 2 } else { 0 } + (end - start),
                    if options.color { "\x1b[0m" } else { "" }
                );
                frac += num;
            }
        }
        _ => handle_err("str err", unmodified_input, options, &colors, start, end),
    }
    (frac, false, long, false)
}
pub fn print_answer(num: NumStr, options: Options, colors: &Colors)
{
    match num
    {
        Num(n) =>
        {
            let a = get_output(options, colors, &n);
            print!(
                "{}{}{}{}",
                a.0,
                a.2.unwrap_or_default(),
                a.1,
                if options.color { "\x1b[0m" } else { "" }
            );
        }
        Vector(mut v) =>
        {
            if options.polar
            {
                v = to_polar(
                    v,
                    match options.angle
                    {
                        Degrees => 180 / Complex::with_val(options.prec, Pi),
                        Radians => Complex::with_val(options.prec, 1),
                        Gradians => 200 / Complex::with_val(options.prec, Pi),
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
                output += &out.2.unwrap_or_default();
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
        Matrix(v) =>
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
                    output += &out.2.unwrap_or_default();
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
        _ =>
        {}
    }
}
pub fn get_output(
    options: Options,
    colors: &Colors,
    number: &Number,
) -> (String, String, Option<String>)
{
    let num = number.number.clone();
    let units = number.units;
    let dec = if options.decimal_places == 0
    {
        1
    }
    else
    {
        options.decimal_places
    };
    if options.sci
    {
        if options.base.1 != 10
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
                    let n = num
                        .real()
                        .to_string_radix(options.base.1, Some(options.decimal_places));
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
                    String::new()
                },
                if !num.imag().is_zero()
                {
                    let n = num
                        .imag()
                        .to_string_radix(options.base.1, Some(options.decimal_places));
                    sign + &if n.contains('e')
                    {
                        n
                    }
                    else
                    {
                        n.trim_end_matches('0').trim_end_matches('.').to_string()
                    } + &if options.color
                    {
                        format!("{}i", &colors.imag)
                    }
                    else
                    {
                        "i".to_string()
                    }
                }
                else
                {
                    String::new()
                },
                if options.units && num.imag().is_zero()
                {
                    units.map(|units| {
                        format!(
                            "{}{}{}",
                            if options.color
                            {
                                "\x1b[".to_owned() + &colors.units
                            }
                            else
                            {
                                String::new()
                            },
                            units.to_string(options),
                            if options.color { "\x1b[0m" } else { "" }
                        )
                    })
                }
                else
                {
                    None
                },
            )
        }
        else
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
                    String::new()
                }
                else
                {
                    if options.comma
                    {
                        add_commas(&remove_trailing_zeros(
                            &format!("{:e}", num.real()),
                            dec,
                            options.prec,
                        ))
                    }
                    else
                    {
                        remove_trailing_zeros(&format!("{:e}", num.real()), dec, options.prec)
                    }
                    .replace("e0", "")
                    .replace(
                        'e',
                        &if options.small_e
                        {
                            if options.color
                            {
                                format!("{}e", &colors.sci)
                            }
                            else
                            {
                                "e".to_string()
                            }
                        }
                        else if options.color
                        {
                            format!("{}E", &colors.sci)
                        }
                        else
                        {
                            "E".to_string()
                        },
                    ) + if options.color { "\x1b[0m" } else { "" }
                },
                if num.imag().is_zero()
                {
                    String::new()
                }
                else if num.imag() == &1
                {
                    sign + &if options.color
                    {
                        format!("{}i", &colors.imag)
                    }
                    else
                    {
                        "i".to_string()
                    }
                }
                else if num.imag() == &-1
                {
                    sign + &if options.color
                    {
                        format!("-{}i", &colors.imag)
                    }
                    else
                    {
                        "i".to_string()
                    }
                }
                else
                {
                    if options.comma
                    {
                        add_commas(
                            &(sign
                                + &remove_trailing_zeros(
                                    &format!("{:e}", num.imag()),
                                    dec,
                                    options.prec,
                                )),
                        )
                    }
                    else
                    {
                        sign + &remove_trailing_zeros(
                            &format!("{:e}", num.imag()),
                            dec,
                            options.prec,
                        )
                    }
                    .replace("e0", "")
                    .replace(
                        'e',
                        &if options.small_e
                        {
                            if options.color
                            {
                                format!("{}e", &colors.sci)
                            }
                            else
                            {
                                "e".to_string()
                            }
                        }
                        else if options.color
                        {
                            format!("{}E", &colors.sci)
                        }
                        else
                        {
                            "E".to_string()
                        },
                    ) + &if options.color
                    {
                        format!("{}i", &colors.imag)
                    }
                    else
                    {
                        "i".to_string()
                    }
                },
                if options.units && num.imag().is_zero()
                {
                    units.map(|units| {
                        format!(
                            "{}{}{}",
                            if options.color
                            {
                                "\x1b[".to_owned() + &colors.units
                            }
                            else
                            {
                                String::new()
                            },
                            units.to_string(options),
                            if options.color { "\x1b[0m" } else { "" }
                        )
                    })
                }
                else
                {
                    None
                },
            )
        }
    }
    else
    {
        let re = if options.comma
        {
            add_commas(&to_string(
                num.real(),
                options.decimal_places,
                false,
                options.base.1,
            ))
        }
        else
        {
            to_string(num.real(), options.decimal_places, false, options.base.1)
        };
        let im = if options.comma
        {
            add_commas(&to_string(
                num.imag(),
                options.decimal_places,
                true,
                options.base.1,
            ))
        }
        else
        {
            to_string(num.imag(), options.decimal_places, true, options.base.1)
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
                String::new()
            }
            else
            {
                re
            },
            if im == "0"
            {
                String::new()
            }
            else if im == "1"
            {
                sign + &if options.color
                {
                    format!("{}i", &colors.imag)
                }
                else
                {
                    "i".to_string()
                }
            }
            else if im == "-1"
            {
                sign + &if options.color
                {
                    format!("-{}i", &colors.imag)
                }
                else
                {
                    "i".to_string()
                }
            }
            else
            {
                sign + &im
                    + &if options.color
                    {
                        format!("{}i", &colors.imag)
                    }
                    else
                    {
                        "i".to_string()
                    }
            },
            if options.units && num.imag().is_zero()
            {
                units.map(|units| {
                    format!(
                        "{}{}{}",
                        if options.color
                        {
                            "\x1b[".to_owned() + &colors.units
                        }
                        else
                        {
                            String::new()
                        },
                        units.to_string(options),
                        if options.color { "\x1b[0m" } else { "" }
                    )
                })
            }
            else
            {
                None
            },
        )
    }
}
fn to_string(num: &Float, decimals: usize, imag: bool, radix: i32) -> String
{
    let (neg, mut str, exp) = num.to_sign_string_exp(radix, None);
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
    let width = get_terminal_dimensions().0;
    let decimals = if decimals == usize::MAX - 1 && (width as i32) > (2i32 + exp)
    {
        (width as i32
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
            neg.to_owned() + "0"
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
    if -exp as i128 > decimals as i128
    {
        return neg.to_owned() + "0";
    }
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
            neg.to_owned() + "0"
        }
        else
        {
            format!("{}{}", neg, l)
        };
    }
    if r.len() > decimals
    {
        if !zeros.is_empty()
        {
            r.insert(decimals.saturating_sub(1), '.');
        }
        else
        {
            r.insert(decimals, '.');
        }
    }
    let mut d = Float::parse_radix(&r, radix)
        .unwrap()
        .complete(num.prec())
        .to_integer()
        .unwrap();
    if exp > 0
    {
        zeros = "0".repeat(r.to_string().len() - r.to_string().trim_start_matches('0').len());
        if d.to_string() == (radix as f64).powi(decimals as i32 - 1).to_string()
        {
            zeros.pop();
        }
    }
    if radix == 10
        && d.to_string().trim_end_matches('0') == "1"
        && r.trim_start_matches('0')
            .trim_start_matches('.')
            .starts_with('9')
    {
        if zeros.is_empty()
        {
            let t: Float = Float::parse_radix(if l.is_empty() { "0" } else { &l }, radix)
                .unwrap()
                .complete(num.prec())
                + 1;
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
        if zeros.is_empty()
            && d.to_string_radix(radix)
                .chars()
                .next()
                .unwrap()
                .to_digit(radix as u32)
                .unwrap()
                == 1
        {
            format!(
                "{}{}",
                neg,
                Integer::from_str_radix(&l, radix).unwrap_or_default() + 1
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
            d.to_string_radix(radix)
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
            return if s.is_empty()
            {
                "0".to_string()
            }
            else if s == "-"
            {
                "-0".to_string()
            }
            else
            {
                s
            };
        }
    };
    let dec = if dec == usize::MAX - 1
    {
        get_terminal_dimensions().0
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
