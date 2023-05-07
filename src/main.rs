mod complex;
mod math;
mod parse;
use parse::get_func;
use complex::parse;
use math::do_math;
use std::env::{args, var};
use std::io::{BufRead, BufReader, stdout, Write};
use console::{Key, Term};
#[cfg(target_os = "linux")]
use std::io::stdin;
#[cfg(target_os = "linux")]
use libc::{isatty, STDIN_FILENO};
use std::fs::{File, OpenOptions};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Point, Rect};
use sdl2::mouse::MouseButton;
fn main()
{
    if args().len() > 1
    {
        if args().nth(1).unwrap() == "--help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            return;
        }
        let func = get_func(&args().nth(1).unwrap(), true);
        let mut data = Vec::new();
        if func.contains(&"x".to_string())
        {
            let mut modified:Vec<String>;
            if func.contains(&"y".to_string())
            {
                for n in -100..=100
                {
                    modified = func.iter().map(|i| i.replace('x', &(n as f64 / 10.0).to_string())).collect();
                    for g in -100..=100
                    {
                        let num = match do_math(modified.iter().map(|j| j.replace('y', &(g as f64 / 10.0).to_string())).collect())
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("0");
                                continue;
                            }
                        };
                        let (a, b) = parse(&num);
                        let a = (a * 1e9).round() / 1e9;
                        let b = ((b * 1e9).round() / 1e9).to_string() + "i";
                        println!("{} {} {} {}", n as f64 / 10.0, g as f64 / 10.0, a, b);
                    }
                }
                return;
            }
            for n in -100000..=100000
            {
                modified = func.iter().map(|i| i.replace('x', &(n as f64 / 10000.0).to_string())).collect();
                let num = match do_math(modified)
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("0");
                        continue;
                    }
                };
                let (a, b) = parse(&num);
                data.push([n as f64 / 10000.0, a, b]);
                // println!("{} {} {}", n as f64 / 10000.0, a, b);
            }
            graph2d(data);
            return;
        }
        print_answer(func);
        return;
    }
    let mut input = String::new();
    #[cfg(target_os = "linux")]
    if !unsafe { isatty(STDIN_FILENO) != 0 }
    {
        let line = stdin().lock().lines().next();
        if line.as_ref().is_none()
        {
            return;
        }
        input = line.unwrap().unwrap();
        if input.is_empty()
        {
            return;
        }
        print_answer(get_func(&input, true));
        return;
    }
    #[cfg(target_os = "linux")]
    let file_path = &(var("HOME").unwrap() + "/.config/calc.history");
    #[cfg(target_os = "windows")]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\calc.history", var("USERNAME").unwrap());
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut var:Vec<Vec<char>> = Vec::new();
    loop
    {
        input.clear();
        let fg = "\x1b[96m";
        print!("{fg}");
        stdout().flush().unwrap();
        let mut i = BufReader::new(File::open(file_path).unwrap()).lines().count() as i32;
        let max = i;
        let mut cursor = 0;
        loop
        {
            let c = read_single_char();
            match c
            {
                '\n' =>
                {
                    println!("\x1b[0m");
                    break;
                }
                '\x08' =>
                {
                    if input.is_empty()
                    {
                        continue;
                    }
                    cursor -= 1;
                    input.remove(cursor);
                    print!("\x1B[2K\x1B[1G{}", input);
                    if input.is_empty()
                    {
                        print_concurrent(&"0".to_string(), var.clone(), true);
                    }
                    else
                    {
                        print_concurrent(&input, var.clone(), false);
                    }
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
                '\x1D' =>
                {
                    i -= 1;
                    input.clear();
                    if i == -1
                    {
                        i = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    print_concurrent(&input, var.clone(), false);
                    print!("\x1B[2K\x1B[1G{fg}{}", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    input.clear();
                    if i >= max
                    {
                        print!("\x1B[2K\x1B[1G{fg}");
                        stdout().flush().unwrap();
                        i = max;
                        cursor = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    print_concurrent(&input, var.clone(), false);
                    print!("\x1B[2K\x1B[1G{fg}{}", input);
                }
                '\x1B' =>
                {
                    if cursor == 0
                    {
                        continue;
                    }
                    cursor -= 1;
                    print!("\x08");
                }
                '\x1C' =>
                {
                    if cursor == input.len()
                    {
                        continue;
                    }
                    cursor += 1;
                    print!("\x1b[1C")
                }
                _ =>
                {
                    //"\x1b[B"
                    input.insert(cursor, c);
                    cursor += 1;
                    print_concurrent(&input, var.clone(), false);
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
            }
            stdout().flush().unwrap();
        }
        if input.contains('=')
        {
            print!("\x1B[2K\x1B[1G");
            for i in 0..var.len()
            {
                if var[i][0] == input.chars().next().unwrap()
                {
                    var.remove(i);
                    break;
                }
            }
            var.push(input.chars().collect());
            write_history(&input, file_path);
            continue;
        }
        if input == "exit"
        {
            break;
        }
        if input == "clear"
        {
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();
            continue;
        }
        if input == "help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        let unmodified = input.clone();
        for i in &var
        {
            input = input.replace(&i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>(),
                                  &i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>());
        }
        if input.contains('x') || input.contains('y')
        {
            println!("{}", input);
            write_history(&input, file_path);
            continue;
        }
        write_history(&unmodified, file_path);
        println!();
    }
}
fn graph2d(data:Vec<[f64; 3]>)
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("graph", 1920, 1080).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_zooming = false;
    let mut zoom_rect:Option<Rect> = None;
    let mut zoom_factor = 1.0;
    let mut offset = (0.0, 0.0);
    'running: loop
    {
        canvas.clear();
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, x, y, .. } =>
                {
                    is_zooming = true;
                    zoom_rect = Some(Rect::new(x, y, 0, 0));
                }
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } =>
                {
                    is_zooming = false;
                    if zoom_rect.is_none()
                    {
                        continue;
                    }
                    let rect = zoom_rect.unwrap();
                    let x = rect.x() as f64 * zoom_factor - 1920.0 / 2.0;
                    let y = rect.y() as f64 * zoom_factor - 1080.0 / 2.0;
                    let w = rect.width() as f64 * zoom_factor;
                    let h = rect.height() as f64 * zoom_factor;
                    let new_center = (x + w / 2.0, y + h / 2.0);
                    zoom_factor *= 1920.0 / w;
                    let new_offset = (1920.0 / 2.0 - new_center.0, 1080.0 / 2.0 - new_center.1);
                    offset = (offset.0 + new_offset.0, offset.1 + new_offset.1);
                    zoom_rect = None;
                }
                Event::MouseMotion { x, y, .. } if is_zooming =>
                {
                    if let Some(rect) = &mut zoom_rect
                    {
                        rect.w = x - rect.x();
                        rect.h = y - rect.y();
                    }
                }

                // Handle key events
                Event::KeyDown { keycode: Some(Keycode::U), .. } =>
                {
                    zoom_factor = 1.0;
                    offset = (0.0, 0.0);
                }
                _ =>
                {}
            }
        }
        if let Some(rect) = &zoom_rect
        {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(*rect).unwrap();
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let lines = compute_lines(zoom_factor, offset);
        for line in lines
        {
            canvas.draw_line(line.0, line.1).unwrap();
        }
        for i in &data
        {
            canvas.set_draw_color(Color::RGB(148, 0, 211)); // re
            let x = ((i[0] + 10.0) * (1920.0 / 20.0) * zoom_factor + offset.0).round() as i32;
            let y = ((-i[1] + 10.0) * (1080.0 / 20.0) * zoom_factor + offset.1).round() as i32;
            canvas.draw_point(Point::new(x, y)).unwrap();
            canvas.set_draw_color(Color::RGB(0, 158, 115)); // im
            let y = ((-i[2] + 10.0) * (1080.0 / 20.0) * zoom_factor + offset.1).round() as i32;
            canvas.draw_point(Point::new(x, y)).unwrap();
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}
fn compute_lines(zoom_factor:f64, offset:(f64, f64)) -> Vec<((i32, i32), (i32, i32))>
{
    let width = 1920_f64 / zoom_factor;
    let height = 1080_f64 / zoom_factor;
    let x_offset = offset.0;
    let y_offset = offset.1;
    let mut lines = Vec::new();
    for i in 1..20
    {
        let x1 = ((i as f64) / 20.0 * width + x_offset) as i32;
        let y1 = (height + y_offset) as i32;
        let x2 = ((i as f64) / 20.0 * width + x_offset) as i32;
        let y2 = y_offset as i32;
        lines.push(((x1, y1), (x2, y2)));
    }
    for i in 1..20
    {
        let x1 = (width + x_offset) as i32;
        let y1 = ((i as f64) / 20.0 * height + y_offset) as i32;
        let x2 = x_offset as i32;
        let y2 = ((i as f64) / 20.0 * height + y_offset) as i32;
        lines.push(((x1, y1), (x2, y2)));
    }
    lines
}
fn print_answer(func:Vec<String>)
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
    let a = (a * 1e9).round() / 1e9;
    let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "\x1b[93mi";
    println!("{}{}\x1b[0",
             if a == 0.0 && !(b.ends_with("0\x1b[93mi")) { "".to_string() } else { a.to_string() },
             if b.ends_with("0\x1b[93mi") { "".to_string() } else { b });
}
fn print_concurrent(input:&String, var:Vec<Vec<char>>, del:bool)
{
    let mut modified = input.to_string();
    for i in &var
    {
        modified = input.replace(&i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>(),
                                 &i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>());
    }
    if let Ok(num) = do_math(get_func(&modified, false))
    {
        let (a, b) = parse(&num);
        let a = (a * 1e9).round() / 1e9;
        let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "\x1b[93mi";
        print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}\x1b[A",
               if a == 0.0 && !(b.ends_with("0\x1b[93mi")) { "".to_string() } else { a.to_string() },
               if b.ends_with("0\x1b[93mi") { "".to_string() } else { b });
    }
    if !del
    {
        print!("\x1b[96m\x1B[2K\x1B[1G{}", input);
    }
    else
    {
        print!("\x1b[96m\x1B[2K\x1B[1G");
    }
}
fn write_history(input:&str, file_path:&str)
{
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    file.write_all(input.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) =>
        {
            if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' || c == '.' || c == '=' || c == ','
            {
                c
            }
            else
            {
                read_single_char()
            }
        }
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        _ => read_single_char(),
    }
}