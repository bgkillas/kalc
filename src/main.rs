use crossterm::{
    cursor::{DisableBlinking, EnableBlinking},
    execute, terminal,
};
#[cfg(feature = "gnuplot")]
use kalc_lib::graph::graph;
use kalc_lib::misc::{get_word_bank, spawn_cmd};
#[cfg(all(feature = "kalc-plot", feature = "serde"))]
use kalc_lib::units::Data;
use kalc_lib::{
    help::help_for,
    load_vars::{add_var, get_cli_vars, get_file_vars, get_vars, set_commands_or_vars},
    math::do_math,
    misc::{
        clear, clearln, convert, end_word, get_terminal_dimensions, handle_err, insert_last,
        prompt, read_single_char, to_output, write,
    },
    options::{arg_opts, commands, equal_to, file_opts, silent_commands},
    parse::input_var,
    print::{print_answer, print_concurrent},
    units::{Colors, HowGraphing, Options, Variable},
};
#[cfg(any(feature = "kalc-plot", feature = "gnuplot"))]
use std::thread::{self, JoinHandle};
use std::{
    cmp::{Ordering, min},
    env::args,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, IsTerminal, Stdout, Write, stdin, stdout},
    time::Instant,
};
#[cfg(feature = "kalc-plot")]
use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
};
fn main() -> Result<(), Error> {
    let mut colors = Colors::default();
    let mut options = Options::default();
    let mut args = args().collect::<Vec<String>>();
    let mut default = false;
    let dir = dirs::config_dir().unwrap().to_str().unwrap().to_owned() + "/kalc";
    std::fs::create_dir_all(dir.clone())?;
    let mut check = Vec::new();
    {
        let file_path = dir.clone() + "/kalc.config";
        if let Ok(s) = file_opts(
            &mut options,
            &mut colors,
            &file_path,
            &Vec::new(),
            Vec::new(),
            true,
        ) {
            check = s;
        }
        if let Ok(s) = arg_opts(&mut options, &mut colors, &mut args, &Vec::new(), true) {
            default = s;
        }
    }
    if !stdin().is_terminal() {
        let lines = stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .filter(|l| !l.is_empty() && !l.starts_with("#"));
        args.splice(0..0, lines);
    }
    options.interactive = args.is_empty();
    let mut stdout = stdout();
    if options.interactive {
        options.color.auto_set(true);
        terminal::enable_raw_mode()?;
        print!(
            "\x1b[G\x1b[K{}{}",
            prompt(options, &colors),
            if options.color.as_bool() {
                "\x1b[0m"
            } else {
                ""
            }
        );
        stdout.flush()?;
    }
    for arg in args.iter_mut() {
        let c = arg.as_bytes();
        if c[0] == c[arg.len() - 1] && matches!(c[0], b'\'' | b'"') {
            arg.remove(0);
            arg.pop();
        }
    }

    let file_path = dir.clone() + "/kalc.vars";
    let mut vars: Vec<Variable> =
        if options.allow_vars && (options.interactive || options.stay_interactive) {
            get_vars(options)
        } else {
            Default::default()
        };
    let mut err = false;
    let base = options.base;
    let mut argsj = args.join(" ");
    if !check.is_empty() {
        argsj += " ";
        let file_path = dir.clone() + "/kalc.config";
        argsj += &BufReader::new(File::open(file_path)?)
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>()
            .join(" ");
    }

    if !options.interactive && options.allow_vars && !options.stay_interactive {
        get_cli_vars(options, argsj.clone(), &mut vars)
    }

    if options.allow_vars && !default {
        options.base = (10, 10);
        if let Ok(file) = File::open(&file_path) {
            let lines = BufReader::new(file)
                .lines()
                .map(Result::unwrap)
                .filter_map(|l| (!l.starts_with('#') && !l.is_empty()).then_some(l))
                .collect::<Vec<String>>();
            let mut split;
            let mut blacklist = if options.interactive || options.stay_interactive {
                Vec::new()
            } else {
                vars.iter()
                    .map(|v| v.name.iter().collect::<String>())
                    .collect::<Vec<String>>()
            };
            'upper: for i in lines.clone() {
                split = i.splitn(2, '=');
                let l = split.next().unwrap().to_string();
                let left = if l.contains('(') {
                    l.split('(').next().unwrap().to_owned()
                } else {
                    l.clone()
                };
                if options.interactive
                    || options.stay_interactive
                    || (!blacklist.contains(&l) && {
                        let mut b = false;
                        let mut word = String::new();
                        for c in argsj.chars() {
                            if c.is_alphanumeric() || matches!(c, '\'' | '`' | '_') {
                                word.push(c)
                            } else {
                                if l.contains('(') {
                                    b = word.trim_end_matches('\'').trim_end_matches('`') == left
                                        && matches!(c, '(' | '{' | '[' | '|');
                                } else {
                                    b = word == left;
                                }
                                if b {
                                    break;
                                }
                                word.clear()
                            }
                        }
                        b
                    })
                {
                    if let Some(r) = split.next() {
                        let le = l.chars().collect::<Vec<char>>();
                        if !options.interactive && !options.stay_interactive {
                            blacklist.push(l);
                            get_file_vars(options, &mut vars, lines.clone(), r, &mut blacklist);
                        }
                        for (i, v) in vars.iter().enumerate() {
                            if v.name.split(|c| c == &'(').next() == le.split(|c| c == &'(').next()
                                && v.name.contains(&'(') == le.contains(&'(')
                                && v.name.iter().filter(|&&c| c == ',').count()
                                    == le.iter().filter(|&&c| c == ',').count()
                            {
                                if r == "null" {
                                    if let Err(s) =
                                        add_var(le, r, i, &mut vars, options, true, true, true)
                                    {
                                        err = true;
                                        println!("\x1b[G\x1b[K{s}")
                                    }
                                } else if let Err(s) =
                                    add_var(le, r, i, &mut vars, options, true, true, false)
                                {
                                    err = true;
                                    println!("\x1b[G\x1b[K{s}")
                                }
                                continue 'upper;
                            }
                        }
                        for (i, j) in vars.iter().enumerate() {
                            if j.name.len() <= le.len() {
                                if let Err(s) =
                                    add_var(le, r, i, &mut vars, options, false, false, false)
                                {
                                    err = true;
                                    println!("\x1b[G\x1b[K{s}")
                                }
                                continue 'upper;
                            }
                        }
                        if let Err(s) = add_var(le, r, 0, &mut vars, options, false, false, false) {
                            err = true;
                            println!("\x1b[G\x1b[K{s}")
                        }
                    }
                }
            }
        }
    }
    let file_path = dir.clone() + "/kalc.config";

    if !check.is_empty() {
        if let Err(s) = file_opts(&mut options, &mut colors, &file_path, &vars, check, false) {
            println!("{s}");
            std::process::exit(1);
        }
    }

    if let Err(s) = arg_opts(&mut options, &mut colors, &mut args, &vars, false) {
        println!("{s}");
        std::process::exit(1);
    }

    if options.interactive && err {
        print!(
            "\x1b[G\x1b[K{}{}",
            prompt(options, &colors),
            if options.color.as_bool() {
                "\x1b[0m"
            } else {
                ""
            }
        );
        stdout.flush()?;
    }
    options.base = base;
    let (mut file, mut unmod_lines) = if options.interactive || options.stay_interactive {
        options.color.auto_set(true);
        let file_path = &(dir.clone() + "/kalc.history");
        File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());

        (
            Some(OpenOptions::new().append(true).open(file_path)?),
            Some(
                BufReader::new(File::open(file_path)?)
                    .lines()
                    .map(Result::unwrap)
                    .collect::<Vec<String>>(),
            ),
        )
    } else {
        options.color.auto_set(false);
        (None, None)
    };
    #[cfg(any(feature = "gnuplot", feature = "kalc-plot"))]
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let mut cut: Vec<char> = Vec::new();

    'main: loop {
        let mut input = Vec::new();
        let mut graphable = HowGraphing::default();
        let mut varcheck = false;
        #[cfg(feature = "gnuplot")]
        let mut last = Vec::new();
        #[cfg(not(feature = "gnuplot"))]
        let last: Vec<char>;
        if !args.is_empty() {
            let watch = options.debug.then(Instant::now);
            input = args.remove(0).chars().collect();
            let output;
            let funcvar;
            {
                let mut options = options;
                let mut unparsed = input.clone();
                {
                    let split = input.split(|c| c == &';');
                    let count = split.clone().count();
                    if count != 1 {
                        unparsed = split.clone().next_back().unwrap().to_vec();
                        for (i, s) in split.enumerate() {
                            if i == count - 1 {
                                break;
                            }
                            silent_commands(
                                &mut options,
                                &s.iter()
                                    .copied()
                                    .filter(|&c| !c.is_whitespace())
                                    .collect::<Vec<char>>(),
                            );
                            if s.contains(&'=') {
                                if let Err(s) =
                                    set_commands_or_vars(&mut colors, &mut options, &mut vars, s)
                                {
                                    eprintln!("{s}");
                                    continue 'main;
                                }
                            }
                        }
                    }
                    let tempinput = unparsed.iter().collect::<String>();
                    if tempinput.starts_with("help ") {
                        println!("{}", help_for(tempinput.splitn(2, ' ').last().unwrap()));
                        continue;
                    } else if tempinput.ends_with('=') {
                        println!(
                            "{}",
                            equal_to(
                                options,
                                &colors,
                                &vars,
                                &tempinput[..tempinput.len().saturating_sub(1)],
                                "",
                            )
                        );
                        continue;
                    }
                }
                (output, funcvar, graphable, varcheck, _) = match input_var(
                    &unparsed.iter().map(convert).collect::<String>(),
                    &vars,
                    &mut Vec::new(),
                    &mut 0,
                    options,
                    false,
                    0,
                    Vec::new(),
                    false,
                    &mut Vec::new(),
                    None,
                    None,
                ) {
                    Ok(f) => f,
                    Err(s) => {
                        eprintln!("{}: {s}", input.iter().collect::<String>());
                        continue;
                    }
                };
                if !graphable.graph && !varcheck {
                    match do_math(output, options, funcvar) {
                        Ok(n) => print_answer(n, options, &colors),
                        Err(s) => {
                            eprintln!("{}: {s}", input.iter().collect::<String>());
                            continue;
                        }
                    }

                    println!(
                        "{}",
                        watch
                            .map(|t| " ".to_owned() + &t.elapsed().as_nanos().to_string())
                            .unwrap_or_default()
                    );
                }
            }
        } else {
            if !options.interactive {
                if options.stay_interactive {
                    setup_for_interactive(&colors, &mut options, &mut stdout)?
                } else {
                    #[cfg(any(feature = "gnuplot", feature = "kalc-plot"))]
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    break;
                }
            }
            let mut long = false;
            let mut frac = 0;
            let mut placement = 0;
            let mut current = Vec::new();
            let mut lines = unmod_lines.clone().unwrap();
            let mut i = lines.len();
            last = if i == 0 {
                "".chars()
            } else if lines[i - 1].ends_with('\t') {
                lines[i - 1][..lines[i - 1].len() - 1].chars()
            } else {
                lines[i - 1].chars()
            }
            .collect();

            let [mut start, mut end, mut xxpos] = [0; 3];
            let [mut slow, mut firstslow, mut xxbool, mut xxstart] = [false; 4];
            let mut lastd = get_terminal_dimensions();
            loop {
                let c = read_single_char();
                let watch = Instant::now();
                {
                    let d = get_terminal_dimensions();
                    if lastd != d {
                        lastd = d;
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        end = min(end, input.len());
                        placement = min(placement, end);
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if options.debug {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        } else if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                }
                match c {
                    '\n' | '\x14' | '\x09' | '\x06' => {
                        if c != '\x14' && c != '\x06' {
                            execute!(stdout, DisableBlinking)?;
                        }
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        end = min(end, input.len());
                        if (!options.real_time_output || long || (slow && !firstslow))
                            && (!input.is_empty() && !input.starts_with(&['#']))
                            && c != '\x14'
                            && c != '\x06'
                        {
                            (frac, graphable, _, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                c == '\n',
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                        }
                        if c == '\x09' {
                            clear(&input, &vars, start, end, options, &colors);
                        } else {
                            if !input.is_empty() && !input.starts_with(&['#']) && frac != 0 {
                                print!("\x1b[{frac}B");
                            }
                            if c == '\x14' || c == '\x06' {
                                print!("\x1b[G{}\x1b[J", if input.is_empty() { "\n" } else { "" });
                                terminal::disable_raw_mode()?;
                                std::process::exit(0);
                            }
                        }
                        print!("\x1b[G\n\x1b[K");
                        break;
                    }
                    '\x03' => {
                        //ctrl+backspace
                        if placement != 0 && end_word(input[placement - 1]) {
                            placement -= 1;
                            input.remove(placement);
                        } else {
                            for (i, c) in input[..placement].iter().rev().enumerate() {
                                if c.is_whitespace() || i + 1 == placement {
                                    input.drain(placement - i - 1..placement);
                                    placement -= i + 1;
                                    break;
                                }
                                if end_word(*c) {
                                    input.drain(placement - i..placement);
                                    placement -= i;
                                    break;
                                }
                            }
                        }
                        end = min(end, input.len());
                        start = min(start, end);
                        if i == lines.len() {
                            current.clone_from(&input);
                        } else {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if options.debug {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        } else if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty() {
                            slow = false;
                            clear(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x08' => {
                        //backspace
                        if placement - start == 0 && start != 0 {
                            start -= 1;
                        }
                        if placement != 0 {
                            placement -= 1;
                            input.remove(placement);
                        }
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        end = min(end, input.len());
                        if i == lines.len() {
                            current.clone_from(&input);
                        } else {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if options.debug {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        } else if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty() {
                            slow = false;
                            clear(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x7F' => {
                        //delete
                        if placement == input.len() {
                            continue;
                        }
                        if placement - start == 0 && start != 0 {
                            start -= 1;
                        }
                        if !input.is_empty() {
                            input.remove(placement);
                        }
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        end = min(end, input.len());
                        start = min(start, end);
                        if i == lines.len() {
                            current.clone_from(&input);
                        } else {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if options.debug {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        } else if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty() {
                            slow = false;
                            clear(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x11' => {
                        //end
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        } else {
                            input.len()
                                - (get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 })
                        };
                        clearln(&input, &vars, start, end, options, &colors);
                    }
                    '\x18' => {
                        //ctrl+u
                        cut = input.drain(..placement).collect();
                        end -= placement;
                        placement = 0;
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x19' => {
                        //ctrl+k
                        cut = input.drain(placement..).collect();
                        end = min(end, input.len());
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x17' => {
                        //ctrl+y
                        let mut cut = cut.clone();
                        end += cut.len();
                        cut.extend(input.drain(placement..));
                        input.extend(cut);
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x16' => {
                        //ctrl+t
                        if placement < input.len() && placement != 0 {
                            input.swap(placement - 1, placement);
                            if options.real_time_output && !slow {
                                execute!(stdout, DisableBlinking)?;
                                (frac, graphable, long, varcheck) = print_concurrent(
                                    &input,
                                    &last,
                                    &vars,
                                    options,
                                    colors.clone(),
                                    start,
                                    end,
                                    false,
                                );
                                if watch.elapsed().as_millis() > options.slowcheck {
                                    firstslow = true;
                                    slow = true;
                                }
                            } else if options.real_time_output && firstslow {
                                firstslow = false;
                                handle_err(
                                    "too slow, will print on enter",
                                    &vars,
                                    &input,
                                    options,
                                    &colors,
                                    start,
                                    end,
                                )
                            } else {
                                clearln(&input, &vars, start, end, options, &colors);
                            }
                            if end - placement != 0 {
                                print!("\x1b[{}D", end - placement)
                            }
                        }
                    }
                    '\x15' => {
                        //ctrl+l
                        print!("\x1b[H\x1b[J");
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x10' => {
                        //home
                        placement = 0;
                        start = 0;
                        end = if get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            input.len()
                        } else {
                            get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                        };
                        clearln(&input, &vars, start, end, options, &colors);
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x1D' | '\x05' => {
                        //up history
                        i -= if i > 0 { 1 } else { 0 };
                        if lines.is_empty() {
                            continue;
                        }
                        input = lines[i].clone().chars().collect::<Vec<char>>();
                        slow = input.ends_with(&['\t']);

                        if slow && options.real_time_output {
                            input.pop();
                            firstslow = false;
                            placement = input.len();
                            end = input.len();
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            } else {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            if slow {
                                input.pop();
                            }
                            placement = input.len();
                            end = input.len();
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            } else {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                            firstslow = slow
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x1E' | '\x04' => {
                        //down history
                        i += 1;
                        if i >= lines.len() {
                            i = lines.len();
                            input.clone_from(&current);
                        } else {
                            input = lines[i].clone().chars().collect::<Vec<char>>();
                        }
                        slow = input.ends_with(&['\t']);
                        if slow && options.real_time_output {
                            input.pop();
                            firstslow = false;
                            placement = input.len();
                            end = input.len();
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            } else {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            if slow {
                                input.pop();
                            }
                            placement = input.len();
                            end = input.len();
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            } else {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                            firstslow = slow;
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x1B' => {
                        //left
                        if placement - start == 0 && placement != 0 && start != 0 {
                            start -= 1;
                            placement -= 1;
                            end = start + get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 };
                            end = min(end, input.len());
                            clearln(&input, &vars, start, end, options, &colors);
                            print!("\x1b[{}D", end - placement)
                        } else if placement != 0 {
                            placement -= 1;
                            print!("\x08");
                        }
                    }
                    '\x1C' => {
                        //right
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        end = min(end, input.len());
                        if placement == end && end != input.len() {
                            start += 1;
                            placement += 1;
                            end += 1;
                            clearln(&input, &vars, start, end, options, &colors)
                        } else if placement != input.len() {
                            placement += 1;
                            print!("\x1b[C")
                        }
                    }
                    '\x12' => {
                        //ctrl+left
                        if placement != 0 {
                            let s = placement;
                            let mut hit = false;
                            for (i, j) in input[..s].iter().enumerate().rev() {
                                if !j.is_alphanumeric() {
                                    if hit {
                                        hit = false;
                                        placement = i + 1;
                                        break;
                                    }
                                } else {
                                    hit = true;
                                }
                            }
                            if hit {
                                placement = 0;
                            }
                            if placement <= start {
                                end = placement
                                    + (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 });
                                end = min(end, input.len());
                                start = placement;
                                clearln(&input, &vars, start, end, options, &colors);
                                if end - placement != 0 {
                                    print!("\x1b[{}D", end - placement)
                                }
                            } else if placement == s {
                                placement = 0;
                                print!("\x1b[{s}D");
                            } else {
                                print!("\x1b[{}D", s - placement);
                            }
                        }
                    }
                    '\x13' => {
                        //ctrl+right
                        if placement != input.len() {
                            let s = placement;
                            let mut hit = false;
                            for (i, j) in input[s + 1..].iter().enumerate() {
                                if !j.is_alphanumeric() {
                                    if hit {
                                        hit = false;
                                        placement += i + 1;
                                        break;
                                    }
                                } else {
                                    hit = true;
                                }
                            }
                            if hit {
                                placement = input.len();
                            }
                            if placement >= end {
                                start = placement.saturating_sub(
                                    get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 },
                                );
                                end = placement;
                                clearln(&input, &vars, start, end, options, &colors)
                            } else if placement == s {
                                placement = input.len();
                                print!("\x1b[{}C", input.len() - s);
                            } else {
                                print!("\x1b[{}C", placement - s);
                            }
                        }
                    }
                    '\x1F' => {
                        //tab completion
                        let mut word = String::new();
                        let mut wait = false;
                        let mut count = 0;
                        let mut start_pos = placement;
                        for (i, c) in input[..placement].iter().rev().enumerate() {
                            if !wait {
                                if c.is_alphabetic()
                                    || matches!(*c, '°' | '\'' | '`' | '_' | '∫' | '$' | '¢')
                                {
                                    word.insert(0, *c)
                                } else if i == 0 {
                                    wait = true
                                } else {
                                    break;
                                }
                            }
                            if wait {
                                if c == &'(' || c == &'{' {
                                    count -= 1;
                                } else if c == &')' || c == &'}' {
                                    count += 1;
                                }
                                if count == -1 {
                                    wait = false;
                                    start_pos -= i;
                                }
                            }
                        }
                        if !word.is_empty() {
                            let bank = get_word_bank(&word, &vars, options);
                            let mut var = false;
                            if bank.len() == 1 {
                                let mut w = bank[0].to_string();
                                if w.contains('(') {
                                    w = w.split('(').next().unwrap().to_string();
                                    if (placement == input.len() || input[placement] != '(')
                                        && input[placement - 1] != '('
                                        && start_pos == placement
                                    {
                                        w.push('(')
                                    }
                                } else {
                                    var = true
                                }
                                let w = w.chars().collect::<Vec<char>>();
                                input.splice(
                                    placement..placement,
                                    w[word.chars().count()..].iter().collect::<String>().chars(),
                                );
                                placement += w.len() - word.chars().count();
                                end = start + get_terminal_dimensions().0
                                    - if options.prompt { 3 } else { 1 }
                                    + 1;
                                if end > input.len() {
                                    end = input.len()
                                } else if placement == end {
                                    start += 1;
                                } else {
                                    end -= 1;
                                }
                                if i == lines.len() {
                                    current.clone_from(&input);
                                } else {
                                    lines[i] = input.clone().iter().collect::<String>();
                                }
                                if options.real_time_output && !slow && var {
                                    execute!(stdout, DisableBlinking)?;
                                    (frac, graphable, long, varcheck) = print_concurrent(
                                        &input,
                                        &last,
                                        &vars,
                                        options,
                                        colors.clone(),
                                        start,
                                        end,
                                        false,
                                    );
                                    if watch.elapsed().as_millis() > options.slowcheck {
                                        firstslow = true;
                                        slow = true;
                                    }
                                } else if options.real_time_output && firstslow && var {
                                    firstslow = false;
                                    handle_err(
                                        "too slow, will print on enter",
                                        &vars,
                                        &input,
                                        options,
                                        &colors,
                                        start,
                                        end,
                                    )
                                } else {
                                    clear(&input, &vars, start, end, options, &colors);
                                }
                                if end - placement != 0 {
                                    print!("\x1b[{}D", end - placement)
                                }
                            } else if !bank.is_empty() {
                                let mut k = 0;
                                let mut char = '\0';
                                'upper: for n in
                                    0..bank.iter().fold(usize::MAX, |min, str| min.min(str.len()))
                                {
                                    for b in &bank {
                                        let c = b.chars().nth(n).unwrap();
                                        if char == '\0' {
                                            char = c
                                        } else if c != char {
                                            break 'upper;
                                        }
                                    }
                                    k += 1;
                                    char = '\0'
                                }
                                input.splice(
                                    placement..placement,
                                    bank[0][word.chars().count()..k].chars(),
                                );
                                placement += k - word.chars().count();
                                end = start + get_terminal_dimensions().0
                                    - if options.prompt { 3 } else { 1 }
                                    + 1;
                                if end > input.len() {
                                    end = input.len()
                                } else if placement == end {
                                    start += 1;
                                } else {
                                    end -= 1;
                                }
                                if i == lines.len() {
                                    current.clone_from(&input);
                                } else {
                                    lines[i] = input.clone().iter().collect::<String>();
                                }
                                clear(&input, &vars, start, end, options, &colors);
                                if end - placement != 0 {
                                    print!("\x1b[{}D", end - placement)
                                }
                            }
                            if !var && !bank.is_empty() {
                                let width = get_terminal_dimensions().0;
                                let mut n = 1;
                                let tab = bank.iter().fold(0, |max, str| max.max(str.len())) + 3;
                                let mut len = 0;
                                print!("\x1b[G\n\x1b[K");
                                for b in bank {
                                    if len + tab > width {
                                        len = 0;
                                        n += 1;
                                        print!("\x1b[G\n\x1b[K")
                                    }
                                    len += tab;
                                    print!(
                                        "{}{}",
                                        to_output(
                                            &b.chars().collect::<Vec<char>>(),
                                            &vars,
                                            options.color.as_bool(),
                                            &colors
                                        ),
                                        " ".repeat(tab - b.chars().count())
                                    )
                                }
                                print!(
                                    "\x1b[G\x1b[{}A\x1b[{}C",
                                    n,
                                    placement + if options.prompt { 2 } else { 0 }
                                );
                                long = true
                            }
                        }
                    }
                    '\x0E' => {
                        //ctrl+xx
                        if xxbool {
                            (placement, xxpos) = (xxpos, placement);
                            match placement.cmp(&xxpos) {
                                Ordering::Greater => {
                                    placement = min(placement, input.len());
                                    if placement >= end {
                                        start = placement.saturating_sub(
                                            get_terminal_dimensions().0
                                                - if options.prompt { 3 } else { 1 },
                                        );
                                        end = placement;
                                        clearln(&input, &vars, start, end, options, &colors)
                                    } else if placement == xxpos {
                                        placement = input.len();
                                        print!("\x1b[{}C", input.len() - xxpos);
                                    } else {
                                        print!("\x1b[{}C", placement - xxpos);
                                    }
                                }
                                Ordering::Less => {
                                    if placement <= start {
                                        end = placement
                                            + (get_terminal_dimensions().0
                                                - if options.prompt { 3 } else { 1 });
                                        end = min(end, input.len());
                                        start = placement;
                                        clearln(&input, &vars, start, end, options, &colors);
                                        if end - placement != 0 {
                                            print!("\x1b[{}D", end - placement)
                                        }
                                    } else if placement == xxpos {
                                        placement = 0;
                                        print!("\x1b[{xxpos}D");
                                    } else {
                                        print!("\x1b[{}D", xxpos - placement);
                                    }
                                }
                                _ => (),
                            }
                            xxbool = false
                        } else {
                            xxbool = true;
                            continue;
                        }
                        if xxstart {
                            xxpos = 0;
                        }
                        xxstart = !xxstart;
                    }
                    '\x0D' => {
                        //ctrl+w
                        if placement != 0 && end_word(input[placement - 1]) {
                            placement -= 1;
                            cut = vec![input.remove(placement)];
                        } else {
                            for (i, c) in input[..placement].iter().rev().enumerate() {
                                if c.is_whitespace() || i + 1 == placement {
                                    cut = input
                                        .drain(placement - i - 1..placement)
                                        .collect::<Vec<char>>();
                                    placement -= i + 1;
                                    break;
                                }
                                if end_word(*c) {
                                    cut = input
                                        .drain(placement - i..placement)
                                        .collect::<Vec<char>>();
                                    placement -= i;
                                    break;
                                }
                            }
                        }
                        end = min(end, input.len());
                        start = min(start, end);
                        if i == lines.len() {
                            current.clone_from(&input);
                        } else {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if options.debug {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        } else if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty() {
                            slow = false;
                            clear(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x0C' => {
                        //alt+d
                        if placement < input.len() && end_word(input[placement]) {
                            cut = vec![input.remove(placement)];
                        } else {
                            let mut pos = 0;
                            for (i, c) in input[placement..].iter().enumerate() {
                                if c.is_whitespace() || placement + i + 1 == input.len() {
                                    pos = i + 1;
                                }
                                if end_word(*c) {
                                    pos = i;
                                    break;
                                }
                            }
                            cut = input.drain(placement..placement + pos).collect();
                        }
                        end = min(end, input.len());
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x0F' => {
                        //alt+t
                        let first;
                        if placement < input.len() && end_word(input[placement]) {
                            first = vec![input.remove(placement)];
                        } else {
                            let mut pos = 0;
                            for (i, c) in input[placement..].iter().enumerate() {
                                if c.is_whitespace() || placement + i + 1 == input.len() {
                                    pos = i + 1;
                                }
                                if end_word(*c) {
                                    pos = i;
                                    break;
                                }
                            }
                            first = input.drain(placement..placement + pos).collect();
                        }
                        let second;
                        if placement != 0 && end_word(input[placement - 1]) {
                            second = vec![input.remove(placement)];
                        } else {
                            let mut pos = 0;
                            for (i, c) in input[..placement].iter().rev().enumerate() {
                                if end_word(*c) {
                                    pos = i;
                                    break;
                                }
                                if c.is_whitespace() || i + 1 == placement {
                                    pos = i + 1;
                                    break;
                                }
                            }
                            second = input
                                .drain(placement - pos..placement)
                                .collect::<Vec<char>>();
                        }
                        placement -= second.len();
                        input.splice(placement..placement, first.clone());
                        placement += first.len();
                        if placement > input.len() {
                            placement = input.len() - 1
                        }
                        input.splice(placement..placement, second.clone());
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\0' => (),
                    _ => {
                        input.insert(placement, c);
                        placement += 1;
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 }
                            + 1;
                        if end > input.len() {
                            end = input.len()
                        } else if placement == end {
                            start += 1;
                        } else {
                            end -= 1;
                        }
                        if i == lines.len() {
                            current.clone_from(&input);
                        } else {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow {
                            execute!(stdout, DisableBlinking)?;
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck {
                                firstslow = true;
                                slow = true;
                            }
                        } else if options.real_time_output && firstslow {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        } else {
                            clearln(&input, &vars, start, end, options, &colors);
                        }
                        if options.debug {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        } else if end - placement != 0 {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                }
                stdout.flush()?;
            }
            commands(&mut options, &lines, &input, &mut stdout);
            if !varcheck {
                print!("{}", prompt(options, &colors));
                if options.color.as_bool() {
                    print!("\x1b[0m");
                }
            }
            stdout.flush()?;
            execute!(stdout, EnableBlinking)?;
            if input.is_empty() {
                continue;
            }
            write(
                insert_last(&input, last.iter().collect::<String>().as_str()),
                file.as_mut().unwrap(),
                unmod_lines.as_mut().unwrap(),
                slow,
                last.iter().collect::<String>(),
            );
        }
        if varcheck {
            if let Err(s) = set_commands_or_vars(&mut colors, &mut options, &mut vars, &input) {
                if !s.is_empty() {
                    print!(
                        "\x1b[G\x1b[A\x1b[K{}\x1b[G\n{}",
                        s,
                        prompt(options, &colors)
                    );
                } else {
                    print!(
                        "{}{}",
                        prompt(options, &colors),
                        if options.color.as_bool() {
                            "\x1b[0m"
                        } else {
                            ""
                        }
                    );
                }
            } else {
                print!(
                    "{}{}",
                    prompt(options, &colors),
                    if options.color.as_bool() {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                );
            }
            stdout.flush()?
        } else if graphable.graph {
            #[cfg(feature = "kalc-plot")]
            if !options.gnuplot {
                if let Some(path) = find_it("kalc-plot") {
                    #[cfg(feature = "serde")]
                    let data = Data {
                        vars: vars.clone(),
                        options,
                        colors: colors.clone(),
                    };
                    #[allow(clippy::zombie_processes)]
                    #[allow(unused_unsafe)]
                    handles.push(thread::spawn(move || unsafe {
                        let mut plot = spawn_cmd(path)
                            .arg("-d")
                            .arg(input.iter().collect::<String>())
                            .stdin(Stdio::piped())
                            .spawn()
                            .unwrap();
                        #[cfg(feature = "serde")]
                        {
                            let stdin = plot.stdin.as_mut().unwrap();
                            let data = bitcode::serialize(&data).unwrap();
                            stdin.write_all(&data.len().to_be_bytes()).unwrap();
                            stdin.write_all(&data).unwrap();
                        }
                        #[cfg(not(unix))]
                        plot.wait().unwrap();
                    }));
                    continue;
                }
            }
            #[cfg(feature = "gnuplot")]
            {
                let inputs: Vec<String> = insert_last(&input, &last.iter().collect::<String>())
                    .split('#')
                    .map(str::to_owned)
                    .collect();
                let watch = options.debug.then_some(Instant::now());
                if options.graph_cli {
                    if options.interactive {
                        terminal::disable_raw_mode()?;
                        graph(inputs, vars.clone(), options, watch, colors.clone(), true)
                            .join()
                            .unwrap();
                        terminal::enable_raw_mode()?;
                    } else {
                        graph(inputs, vars.clone(), options, watch, colors.clone(), true)
                            .join()
                            .unwrap();
                    }
                } else {
                    handles.push(graph(
                        inputs,
                        vars.clone(),
                        options,
                        watch,
                        colors.clone(),
                        false,
                    ));
                }
            }
        }
    }
    Ok(())
}
#[cfg(feature = "kalc-plot")]
fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                full_path.is_file().then_some(full_path)
            })
            .next()
    })
}

fn setup_for_interactive(
    colors: &Colors,
    options: &mut Options,
    stdout: &mut Stdout,
) -> Result<(), Error> {
    options.interactive = true;
    terminal::enable_raw_mode()?;
    print!(
        "\x1b[G\x1b[K{}{}",
        prompt(*options, colors),
        if options.color.as_bool() {
            &colors.text
        } else {
            ""
        }
    );
    stdout.flush()?;
    Ok(())
}