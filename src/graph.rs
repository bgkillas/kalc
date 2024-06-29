#[cfg(not(unix))]
use crate::misc::get_terminal_dimensions;
#[cfg(unix)]
use crate::misc::get_terminal_dimensions_pixel;
use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    load_vars::set_commands_or_vars,
    math::do_math,
    misc::{place_funcvar, place_funcvarxy, place_var, place_varxy, prompt},
    options::silent_commands,
    parse::{input_var, simplify},
    Auto, Colors, GraphType,
    GraphType::{Depth, Flat, Normal},
    HowGraphing, Number, Options, Variable,
};
use rug::Complex;
use std::{
    fs,
    fs::File,
    io::{stdout, Write},
    process::{Command, Stdio},
    thread,
    thread::JoinHandle,
    time::Instant,
};
//TODO domain coloring
#[allow(clippy::type_complexity)]
pub fn graph(
    mut input: Vec<String>,
    mut vars: Vec<Variable>,
    mut options: Options,
    watch: Option<Instant>,
    mut colors: Colors,
    cli: bool,
) -> JoinHandle<()>
{
    thread::spawn(move || {
        let mut func = Vec::new();
        for (i, s) in input.clone().iter().enumerate()
        {
            if s.is_empty()
            {
                continue;
            }
            {
                options.prec = options.graph_prec;
                let split = s.split(';');
                let count = split.clone().count();
                if count != 1
                {
                    input[i] = split.clone().last().unwrap().to_string();
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
                                print!(
                                    "\x1b[G\x1b[A\x1b[K{}\x1b[G\n{}",
                                    s,
                                    prompt(options, &colors)
                                );
                                stdout().flush().unwrap()
                            }
                        }
                    }
                }
            }
            if options.graphtype == GraphType::None
            {
                return;
            }
            options.prec = options.graph_prec;
            func.push(
                match input_var(
                    &input[i],
                    &vars,
                    &mut Vec::new(),
                    &mut 0,
                    options,
                    false,
                    0,
                    Vec::new(),
                    true,
                    &mut Vec::new(),
                    None,
                )
                {
                    Ok(f) => (f.0, f.1, options, f.2),
                    Err(s) =>
                    {
                        print!("\x1b[G\x1b[K{}\x1b[G\n{}", s, prompt(options, &colors));
                        stdout().flush().unwrap();
                        return;
                    }
                },
            );
        }
        if input.iter().all(|i| i.is_empty())
        {
            return;
        }
        let mut gnuplot = if cfg!(not(unix))
        {
            if cli
            {
                Command::new("gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .stderr(Stdio::null())
                .spawn().unwrap_or(
                    Command::new("C:/Program Files/gnuplot/bin/gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH."))
            }
            else
            {
                Command::new("gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn().unwrap_or(
                    Command::new("C:/Program Files/gnuplot/bin/gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH."))
            }
        }
        else if cli
        {
            Command::new("gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH.")
        }
        else if cfg!(debug_assertions)
        {
            Command::new("gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .spawn()
                .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH.")
        }
        else
        {
            Command::new("gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH.")
        };
        let stdin = gnuplot.stdin.as_mut().expect("Failed to open stdin");
        if cli
        {
            options.onaxis = false;
            options.scale_graph = false;
            writeln!(stdin, "set terminal dumb size 125,60 aspect 1,1").unwrap();
        }
        let mut cap: Vec<String> = Vec::new();
        let mut d2_or_d3 = (false, false);
        let mut handles = Vec::new();
        let base_dir = if cfg!(unix)
        {
            "/tmp"
        }
        else
        {
            &dirs::cache_dir().unwrap().to_str().unwrap().to_owned()
        };
        let data_dir = &(base_dir.to_owned() + "/kalc" + &fastrand::u32(..).to_string());
        if fs::read_dir(data_dir).is_ok()
        {
            fs::remove_dir_all(data_dir).unwrap();
        }
        fs::create_dir_all(data_dir).unwrap();
        for (i, (func, input)) in func.iter().zip(input.iter()).enumerate()
        {
            handles.push(get_data(
                colors.clone(),
                func.clone(),
                input.clone(),
                i,
                data_dir.clone(),
            ));
        }
        let mut i = 0;
        let mut lines = Vec::new();
        let mut records = Vec::new();
        #[allow(clippy::explicit_counter_loop)]
        for handle in handles
        {
            let re_or_im;
            let failed;
            let dimen;
            let line;
            let rec;
            (dimen, re_or_im, line, failed, rec) = handle.join().unwrap();
            records.push(rec);
            if failed
            {
                return;
            }
            lines.push(
                if func[i].2.lines == Auto::Auto
                {
                    line
                }
                else
                {
                    func[i].2.lines == Auto::True
                },
            );
            if re_or_im.0 || !re_or_im.1
            {
                if re_or_im.1
                {
                    cap.push(format!("re({})", input[i].clone().replace('`', "\\`")));
                }
                else
                {
                    cap.push(input[i].clone().replace('`', "\\`"))
                }
            }
            if re_or_im.1
            {
                cap.push(format!("im({})", input[i].clone().replace('`', "\\`")));
            }
            if dimen.0
            {
                d2_or_d3.0 = true;
            }
            if dimen.1
            {
                d2_or_d3.1 = true;
            }
            i += 1;
        }
        if d2_or_d3.0 == d2_or_d3.1
        {
            print!(
                "\x1b[G\x1b[Kcant graph 2d and 3d\x1b[G\n{}",
                prompt(options, &colors)
            );
            stdout().flush().unwrap();
            return;
        }
        {
            writeln!(stdin, "set encoding utf8").unwrap();
            writeln!(stdin, "set termoption noenhanced").unwrap();
            if d2_or_d3.0
            {
                if options.ticks.1 == -1.0
                {
                    writeln!(stdin, "set ytics 1 axis scale 0.5,0.5").unwrap();
                }
                else if options.ticks.1 == 0.0
                {
                    writeln!(stdin, "unset ytics").unwrap();
                }
                else if options.ticks.1 > 0.0
                {
                    let n = (options.yr.1 - options.yr.0) / options.ticks.1;
                    writeln!(stdin, "set ytics {:e} axis scale 0.5,0.5", n).unwrap();
                }
            }
            if d2_or_d3.1
            {
                if options.ticks.2 == -1.0
                {
                    writeln!(stdin, "set ztics 1 axis scale 0.5,0.5").unwrap();
                }
                else if options.ticks.2 == 0.0
                {
                    writeln!(stdin, "unset ztics").unwrap();
                }
                else if options.ticks.2 > 0.0
                {
                    let n = (options.zr.1 - options.zr.0) / options.ticks.2;
                    writeln!(stdin, "set ztics {:e} axis scale 0.5,0.5", n).unwrap();
                }
            }
            if options.vxr.0 != 0.0 || options.vxr.1 != 0.0
            {
                options.xr = options.vxr;
            }
            if options.vyr.0 != 0.0 || options.vyr.1 != 0.0
            {
                options.yr = options.vyr;
            }
            if d2_or_d3.1 && (options.vzr.0 != 0.0 || options.vzr.1 != 0.0)
            {
                options.zr = options.vzr;
            }
            if options.scale_graph
            {
                if d2_or_d3.0
                {
                    #[cfg(not(unix))]
                    let (x, y) = if options.window_size.0 != 0
                    {
                        options.window_size
                    }
                    else
                    {
                        get_terminal_dimensions()
                    };
                    #[cfg(unix)]
                    let (x, y) = if options.window_size.0 != 0
                    {
                        options.window_size
                    }
                    else
                    {
                        get_terminal_dimensions_pixel()
                    };
                    let rt = y as f64 / x as f64;
                    let w = rt * (options.xr.1 - options.xr.0) / (options.yr.1 - options.yr.0);
                    options.yr = (w * options.yr.0, w * options.yr.1);
                    writeln!(stdin, "set size ratio {}", rt).unwrap();
                }
                else
                {
                    writeln!(stdin, "set view equal").unwrap();
                }
            }
            writeln!(stdin, "set xrange [{:e}:{:e}]", options.xr.0, options.xr.1).unwrap();
            writeln!(stdin, "set yrange [{:e}:{:e}]", options.yr.0, options.yr.1).unwrap();
            if d2_or_d3.1
            {
                writeln!(stdin, "set zrange [{:e}:{:e}]", options.zr.0, options.zr.1).unwrap();
            }
            writeln!(stdin, "set xlabel \"{}\"", colors.label.0).unwrap();
            writeln!(stdin, "set ylabel \"{}\"", colors.label.1).unwrap();
            if d2_or_d3.1
            {
                writeln!(stdin, "set zlabel \"{}\"", colors.label.2).unwrap();
            }
            if !options.onaxis
            {
                writeln!(stdin, "unset xzeroaxis").unwrap();
                writeln!(stdin, "unset yzeroaxis").unwrap();
                if d2_or_d3.1
                {
                    writeln!(stdin, "unset zzeroaxis").unwrap();
                }
            }
            if options.ticks.0 == -1.0
            {
                writeln!(stdin, "set xtics 1 axis scale 0.5,0.5").unwrap();
            }
            else if options.ticks.0 == 0.0
            {
                writeln!(stdin, "unset xtics").unwrap();
            }
            else if options.ticks.0 > 0.0
            {
                let n = (options.xr.1 - options.xr.0) / options.ticks.0;
                writeln!(stdin, "set xtics {:e} axis scale 0.5,0.5", n).unwrap();
            }
            if d2_or_d3.1
            {
                if options.ticks.1 == -1.0
                {
                    writeln!(stdin, "set ytics 1 axis scale 0.5,0.5").unwrap();
                }
                else if options.ticks.1 == 0.0
                {
                    writeln!(stdin, "unset ytics").unwrap();
                }
                else if options.ticks.1 > 0.0
                {
                    let n = (options.yr.1 - options.yr.0) / options.ticks.1;
                    writeln!(stdin, "set ytics {:e} axis scale 0.5,0.5", n).unwrap();
                }
            }
            writeln!(stdin, "set grid").unwrap();
        }
        if !colors.graphtofile.is_empty()
        {
            writeln!(
                stdin,
                "set terminal pngcairo size {}, {}",
                options.window_size.0, options.window_size.1
            )
            .unwrap();
            if colors.graphtofile == *"-"
            {
                writeln!(stdin, "set output '{base_dir}/kalc-temp.png'").unwrap();
            }
            else
            {
                writeln!(stdin, "set output '{}'", colors.graphtofile).unwrap();
            }
        }
        {
            let mut paths: Vec<_> = fs::read_dir(data_dir)
                .unwrap()
                .map(|p| p.unwrap().path().display().to_string())
                .collect();
            paths.sort_by_key(|dir| {
                let st = dir.split('/').last().unwrap();
                st[2..].to_string()
            });
            if paths.is_empty()
            {
                println!("\x1b[G\x1b[Knothing to graph for {}\x1b[G", input.join("#"));
                return;
            }
            writeln!(
                stdin,
                "{}{}",
                if d2_or_d3.0 { "plot" } else { "splot" },
                paths
                    .iter()
                    .enumerate()
                    .map(|(j, f)| {
                        let n;
                        let col = if f.contains("re")
                        {
                            n = f.split("re").last().unwrap().parse::<usize>().unwrap();
                            colors.recol[n % colors.recol.len()].clone()
                        }
                        else
                        {
                            n = f.split("im").last().unwrap().parse::<usize>().unwrap();
                            colors.imcol[n % colors.recol.len()].clone()
                        };
                        if func[n].2.surface&&d2_or_d3.1
                        {
                                format!(
                            " NaN with lines lc\"{}\"t\"{}\",'{}'binary endian=little array=({},{}) format=\"%float64\"origin=({:e},{:e},0) dx={:e} dy={:e} with pm3d lc\"{}\"t\"\"",
                            col, cap[j], f,options.samples_3d.0+1,options.samples_3d.1+1,
                                    options.xr.0,options.yr.0,(options.xr.1-options.xr.0)/options.samples_3d.0 as f64,(options.yr.1-options.yr.0)/options.samples_3d.1 as f64
                                ,col)
                        }else if lines[n]
                        {
                            if records[n]==0
                            {
                            format!(
                            " NaN with lines lc\"{}\"t\"{}\",'{}'binary endian=little array={} format=\"%float64\"origin={} with linespoints pt {} lc\"{}\"t\"\"",
                            col, cap[j], f,
                                if d2_or_d3.1
                            {
                              format!("({},{})" ,options.samples_3d.0+1,options.samples_3d.1+1)}else{(options.samples_2d+1).to_string()},
                                   if d2_or_d3.1{format!("({:e},{:e},0) dx={:e} dy={:e}",options.xr.0,options.yr.0,(options.xr.1-options.xr.0)/options.samples_3d.0 as f64,(options.yr.1-options.yr.0)/options.samples_3d.1 as f64)}else{
                                       format!("({:e},0) dx={:e}",options.xr.0,(options.xr.1-options.xr.0)/options.samples_2d as f64)
                                   }
                                ,func[n].2.point_style, col)
                            }else if func[n].2.point_style == 0
                            {
                                format!("'{}'binary endian=little record={} format=\"%float64\"with lines lc\"{}\"t\"{}\"", f,records[j], col, cap[j])
                            }
                            else
                            {
                                format!(
                                    " NaN with lines lc\"{}\"t\"{}\",'{}'binary endian=little record={} format=\"%float64\"with linespoints pt {} lc\"{}\"t\"\"",
                                     col, cap[j],f,records[n], func[n].2.point_style, col
                                )
                            }
                        }
                        else  if records[n]==0
                            {
                            format!(
                            " NaN with lines lc\"{}\"t\"{}\",'{}'binary endian=little array={} format=\"%float64\"origin={} with points pt {} lc\"{}\"t\"\"",
                            col, cap[j], f,
                                if d2_or_d3.1
                            {
                              format!("({},{})" ,options.samples_3d.0+1,options.samples_3d.1+1)}else{(options.samples_2d+1).to_string()},
                                   if d2_or_d3.1{format!("({:e},{:e},0) dx={:e} dy={:e}",options.xr.0,options.yr.0,(options.xr.1-options.xr.0)/options.samples_3d.0 as f64,(options.yr.1-options.yr.0)/options.samples_3d.1 as f64)}else{
                                       format!("({:e},0) dx={:e}",options.xr.0,(options.xr.1-options.xr.0)/options.samples_2d as f64)
                                   }
                                ,func[n].2.point_style, col)
                            }
                        else
                        {
                            format!(
                            " NaN with lines lc\"{}\"t\"{}\",'{}'binary endian=little record={} format=\"%float64\"with points pt {} lc\"{}\"t\"\"",
                            col, cap[j], f,records[n],func[n].2.point_style, col
                        )
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .unwrap();
        }
        writeln!(stdin, "pause mouse close").unwrap();
        stdin.flush().unwrap();
        if colors.graphtofile != "-"
        {
            if let Some(time) = watch
            {
                println!("\x1b[G\x1b[K{}ms\x1b[G", time.elapsed().as_millis());
            }
        }
        gnuplot.wait().unwrap();
        if colors.graphtofile == "-"
        {
            if let Ok(n) = fs::read(base_dir.to_owned() + "/kalc-temp.png")
            {
                stdout().lock().write_all(&n).unwrap();
                fs::remove_file(base_dir.to_owned() + "/kalc-temp.png").unwrap()
            }
        }
        if fs::read_dir(data_dir).is_ok()
        {
            fs::remove_dir_all(data_dir).unwrap();
        }
    })
}
#[allow(clippy::type_complexity)]
pub fn get_list_2d(
    func: (
        Vec<NumStr>,
        Vec<(String, Vec<NumStr>)>,
        Options,
        HowGraphing,
    ),
    i: usize,
    data_dir: &str,
    has_x: bool,
) -> ((bool, bool), bool, u64)
{
    let mut rec = 0;
    let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
    let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
    let mut d3 = false;
    let mut nan = true;
    let den_range = (func.2.xr.1 - func.2.xr.0) / func.2.samples_2d as f64;
    let mut zero = (false, false);
    let list = func.0.iter().any(|c| {
        if let Str(s) = c
        {
            matches!(
                s.as_str(),
                "±" | "cubic" | "quadratic" | "quad" | "quartic" | "unity" | "solve"
            )
        }
        else
        {
            false
        }
    }) || func.1.iter().any(|c| {
        c.1.iter().any(|c| {
            if let Str(s) = c
            {
                matches!(
                    s.as_str(),
                    "±" | "cubic" | "quadratic" | "quad" | "quartic" | "unity" | "solve"
                )
            }
            else
            {
                false
            }
        })
    });
    for i in 0..=func.2.samples_2d
    {
        let n = func.2.xr.0 + i as f64 * den_range;
        let num = Num(Number::from(Complex::with_val(func.2.prec, n), None));
        match do_math(
            place_varxy(func.0.clone(), num.clone()),
            func.2,
            place_funcvarxy(func.1.clone(), num),
        )
        {
            Ok(Num(num)) =>
            {
                let num = num.number;
                if num.real().is_nan() || num.imag().is_nan()
                {
                    continue;
                }
                let mut r = 0.0;
                let mut i = 0.0;
                let re = !num.real().is_infinite();
                let ri = !num.imag().is_infinite();
                if re
                {
                    nan = false;
                    r = num.real().to_f64();
                    if !zero.0 && ((r * 1e8).round() / 1e8 != 0.0)
                    {
                        zero.0 = true
                    }
                    if func.2.graphtype == Normal
                    {
                        if has_x
                        {
                            real.write_all(&r.to_le_bytes()).unwrap();
                        }
                        else
                        {
                            rec += 1;
                            real.write_all(&r.to_le_bytes()).unwrap();
                            real.write_all(&n.to_le_bytes()).unwrap();
                        }
                    }
                }
                else if has_x
                {
                    real.write_all(&f64::INFINITY.to_le_bytes()).unwrap();
                }
                if ri
                {
                    nan = false;
                    i = num.imag().to_f64();
                    if !zero.1 && ((i * 1e8).round() / 1e8 != 0.0)
                    {
                        zero.1 = true
                    }
                    if func.2.graphtype == Normal
                    {
                        if has_x
                        {
                            imag.write_all(&i.to_le_bytes()).unwrap();
                        }
                        else
                        {
                            rec += 1;
                            imag.write_all(&i.to_le_bytes()).unwrap();
                            imag.write_all(&n.to_le_bytes()).unwrap();
                        }
                    }
                }
                else if has_x
                {
                    imag.write_all(&f64::INFINITY.to_le_bytes()).unwrap();
                }
                if re && ri
                {
                    if func.2.graphtype == Flat
                    {
                        rec += 1;
                        zero.1 = false;
                        real.write_all(&r.to_le_bytes()).unwrap();
                        real.write_all(&i.to_le_bytes()).unwrap();
                    }
                    else if func.2.graphtype == Depth
                    {
                        rec += 1;
                        d3 = true;
                        zero.1 = false;
                        real.write_all(&n.to_le_bytes()).unwrap();
                        real.write_all(&r.to_le_bytes()).unwrap();
                        real.write_all(&i.to_le_bytes()).unwrap();
                    }
                }
            }
            Ok(Vector(v)) =>
            {
                if list || v.len() == 1 || v.len() > 3
                {
                    for num in v
                    {
                        let num = num.number;
                        rec += 1;
                        if num.real().is_nan() || num.imag().is_nan()
                        {
                            continue;
                        }
                        let mut r = 0.0;
                        let mut i = 0.0;
                        let re = !num.real().is_infinite();
                        let ri = !num.imag().is_infinite();
                        if re
                        {
                            nan = false;
                            r = num.real().to_f64();
                            if !zero.0 && ((r * 1e8).round() / 1e8 != 0.0)
                            {
                                zero.0 = true
                            }
                            if func.2.graphtype == Normal
                            {
                                if has_x
                                {
                                    real.write_all(&n.to_le_bytes()).unwrap();
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                }
                                else
                                {
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                    real.write_all(&n.to_le_bytes()).unwrap();
                                }
                            }
                        }
                        if ri
                        {
                            nan = false;
                            i = num.imag().to_f64();
                            if !zero.1 && ((i * 1e8).round() / 1e8 != 0.0)
                            {
                                zero.1 = true
                            }
                            if func.2.graphtype == Normal
                            {
                                if has_x
                                {
                                    imag.write_all(&n.to_le_bytes()).unwrap();
                                    imag.write_all(&i.to_le_bytes()).unwrap();
                                }
                                else
                                {
                                    imag.write_all(&i.to_le_bytes()).unwrap();
                                    imag.write_all(&n.to_le_bytes()).unwrap();
                                }
                            }
                        }
                        if re && ri
                        {
                            if func.2.graphtype == Flat
                            {
                                zero.1 = false;
                                real.write_all(&r.to_le_bytes()).unwrap();
                                real.write_all(&i.to_le_bytes()).unwrap();
                            }
                            else if func.2.graphtype == Depth
                            {
                                d3 = true;
                                zero.1 = false;
                                real.write_all(&n.to_le_bytes()).unwrap();
                                real.write_all(&r.to_le_bytes()).unwrap();
                                real.write_all(&i.to_le_bytes()).unwrap();
                            }
                        }
                    }
                }
                else if v.len() == 3
                {
                    rec += 1;
                    d3 = true;
                    nan = false;
                    let xr = v[0].number.real().to_f64();
                    let yr = v[1].number.real().to_f64();
                    let zr = v[2].number.real().to_f64();
                    let xi = v[0].number.imag().to_f64();
                    let yi = v[1].number.imag().to_f64();
                    let zi = v[2].number.imag().to_f64();
                    if !zero.0
                        && ((xr * 1e8).round() / 1e8 != 0.0
                            || (yr * 1e8).round() / 1e8 != 0.0
                            || (zr * 1e8).round() / 1e8 != 0.0)
                    {
                        zero.0 = true;
                    }
                    if !zero.1
                        && ((xi * 1e8).round() / 1e8 != 0.0
                            || (yi * 1e8).round() / 1e8 != 0.0
                            || (zi * 1e8).round() / 1e8 != 0.0)
                    {
                        zero.1 = true;
                    }
                    real.write_all(&xr.to_le_bytes()).unwrap();
                    real.write_all(&yr.to_le_bytes()).unwrap();
                    real.write_all(&zr.to_le_bytes()).unwrap();
                    imag.write_all(&xi.to_le_bytes()).unwrap();
                    imag.write_all(&yi.to_le_bytes()).unwrap();
                    imag.write_all(&zi.to_le_bytes()).unwrap();
                }
                else if v.len() == 2
                {
                    rec += 1;
                    nan = false;
                    let xr = v[0].number.real().to_f64();
                    let yr = v[1].number.real().to_f64();
                    let xi = v[0].number.imag().to_f64();
                    let yi = v[1].number.imag().to_f64();
                    if !zero.0
                        && ((xr * 1e8).round() / 1e8 != 0.0 || (yr * 1e8).round() / 1e8 != 0.0)
                    {
                        zero.0 = true;
                    }
                    if !zero.1
                        && ((xi * 1e8).round() / 1e8 != 0.0 || (yi * 1e8).round() / 1e8 != 0.0)
                    {
                        zero.1 = true;
                    }
                    real.write_all(&xr.to_le_bytes()).unwrap();
                    real.write_all(&yr.to_le_bytes()).unwrap();
                    imag.write_all(&xi.to_le_bytes()).unwrap();
                    imag.write_all(&yi.to_le_bytes()).unwrap();
                }
            }
            Ok(Matrix(m)) =>
            {
                for v in m
                {
                    for num in v
                    {
                        let num = num.number;
                        rec += 1;
                        if num.real().is_nan() || num.imag().is_nan()
                        {
                            continue;
                        }
                        let mut r = 0.0;
                        let mut i = 0.0;
                        let re = !num.real().is_infinite();
                        let ri = !num.imag().is_infinite();
                        if re
                        {
                            nan = false;
                            r = num.real().to_f64();
                            if !zero.0 && ((r * 1e8).round() / 1e8 != 0.0)
                            {
                                zero.0 = true
                            }
                            if func.2.graphtype == Normal
                            {
                                if has_x
                                {
                                    real.write_all(&n.to_le_bytes()).unwrap();
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                }
                                else
                                {
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                    real.write_all(&n.to_le_bytes()).unwrap();
                                }
                            }
                        }
                        if ri
                        {
                            nan = false;
                            i = num.imag().to_f64();
                            if !zero.1 && ((i * 1e8).round() / 1e8 != 0.0)
                            {
                                zero.1 = true
                            }
                            if func.2.graphtype == Normal
                            {
                                if has_x
                                {
                                    imag.write_all(&n.to_le_bytes()).unwrap();
                                    imag.write_all(&i.to_le_bytes()).unwrap();
                                }
                                else
                                {
                                    imag.write_all(&i.to_le_bytes()).unwrap();
                                    imag.write_all(&n.to_le_bytes()).unwrap();
                                }
                            }
                        }
                        if re && ri
                        {
                            if func.2.graphtype == Flat
                            {
                                zero.1 = false;
                                real.write_all(&r.to_le_bytes()).unwrap();
                                real.write_all(&i.to_le_bytes()).unwrap();
                            }
                            else if func.2.graphtype == Depth
                            {
                                d3 = true;
                                zero.1 = false;
                                real.write_all(&n.to_le_bytes()).unwrap();
                                real.write_all(&r.to_le_bytes()).unwrap();
                                real.write_all(&i.to_le_bytes()).unwrap();
                            }
                        }
                    }
                }
            }
            Err(s) =>
            {
                println!("{}", s);
                return Default::default();
            }
            _ =>
            {}
        }
    }
    if nan
    {
        fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
        fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
    }
    else
    {
        if !zero.0 && zero.1
        {
            fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
        }
        if !zero.1
        {
            fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
        }
    }
    (zero, d3, rec)
}
#[allow(clippy::type_complexity)]
pub fn get_list_3d(
    func: (
        Vec<NumStr>,
        Vec<(String, Vec<NumStr>)>,
        Options,
        HowGraphing,
    ),
    i: usize,
    data_dir: &str,
) -> ((bool, bool), bool, u64)
{
    let mut rec = 0;
    let mut d2 = false;
    let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
    let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
    let den_x_range = (func.2.xr.1 - func.2.xr.0) / func.2.samples_3d.0 as f64;
    let den_y_range = (func.2.yr.1 - func.2.yr.0) / func.2.samples_3d.1 as f64;
    let mut modified: Vec<NumStr>;
    let mut modifiedvars: Vec<(String, Vec<NumStr>)>;
    let mut zero = (false, false);
    let mut nan = true;
    let list = func.0.iter().any(|c| {
        if let Str(s) = c
        {
            matches!(
                s.as_str(),
                "±" | "cubic" | "quadratic" | "quad" | "quartic" | "unity" | "solve"
            )
        }
        else
        {
            false
        }
    }) || func.1.iter().any(|c| {
        c.1.iter().any(|c| {
            if let Str(s) = c
            {
                matches!(
                    s.as_str(),
                    "±" | "cubic" | "quadratic" | "quad" | "quartic" | "unity" | "solve"
                )
            }
            else
            {
                false
            }
        })
    });
    for i in 0..=func.2.samples_3d.0
    {
        let n = func.2.xr.0 + i as f64 * den_x_range;
        let num = Num(Number::from(Complex::with_val(func.2.prec, n), None));
        modified = place_var(func.0.clone(), "x", num.clone());
        modifiedvars = place_funcvar(func.1.clone(), "x", num);
        simplify(&mut modified, &mut modifiedvars, func.2);
        for g in 0..=func.2.samples_3d.1
        {
            let f = func.2.yr.0 + g as f64 * den_y_range;
            let num = Num(Number::from(Complex::with_val(func.2.prec, f), None));
            match do_math(
                place_var(modified.clone(), "y", num.clone()),
                func.2,
                place_funcvar(modifiedvars.clone(), "y", num),
            )
            {
                Ok(Num(num)) =>
                {
                    let num = num.number;
                    if num.real().is_nan() || num.imag().is_nan()
                    {
                        continue;
                    }
                    let mut r = 0.0;
                    let mut i = 0.0;
                    let re = !num.real().is_infinite();
                    let ri = !num.imag().is_infinite();
                    if re
                    {
                        nan = false;
                        r = num.real().to_f64();
                        if !zero.0 && ((r * 1e8).round() / 1e8 != 0.0)
                        {
                            zero.0 = true
                        }
                        if func.2.graphtype == Normal
                        {
                            real.write_all(&r.to_le_bytes()).unwrap();
                        }
                    }
                    else
                    {
                        real.write_all(&f64::INFINITY.to_le_bytes()).unwrap();
                    }
                    if ri
                    {
                        nan = false;
                        i = num.imag().to_f64();
                        if !zero.1 && ((i * 1e8).round() / 1e8 != 0.0)
                        {
                            zero.1 = true
                        }
                        if func.2.graphtype == Normal
                        {
                            imag.write_all(&i.to_le_bytes()).unwrap();
                        }
                    }
                    else
                    {
                        imag.write_all(&f64::INFINITY.to_le_bytes()).unwrap();
                    }
                    if re && ri
                    {
                        if func.2.graphtype == Flat
                        {
                            rec += 1;
                            zero.1 = false;
                            real.write_all(&n.to_le_bytes()).unwrap();
                            real.write_all(&r.to_le_bytes()).unwrap();
                            real.write_all(&i.to_le_bytes()).unwrap();
                        }
                        else if func.2.graphtype == Depth
                        {
                            rec += 1;
                            zero.1 = false;
                            real.write_all(&f.to_le_bytes()).unwrap();
                            real.write_all(&r.to_le_bytes()).unwrap();
                            real.write_all(&i.to_le_bytes()).unwrap();
                        }
                    }
                }
                Ok(Vector(v)) =>
                {
                    if list || v.len() == 1 || v.len() > 3
                    {
                        for num in v
                        {
                            let num = num.number;
                            rec += 1;
                            if num.real().is_nan() || num.imag().is_nan()
                            {
                                continue;
                            }
                            let mut r = 0.0;
                            let mut i = 0.0;
                            let re = !num.real().is_infinite();
                            let ri = !num.imag().is_infinite();
                            if re
                            {
                                nan = false;
                                r = num.real().to_f64();
                                if !zero.0 && ((r * 1e8).round() / 1e8 != 0.0)
                                {
                                    zero.0 = true
                                }
                                if func.2.graphtype == Normal
                                {
                                    if !func.2.surface
                                    {
                                        real.write_all(&n.to_le_bytes()).unwrap();
                                        real.write_all(&f.to_le_bytes()).unwrap();
                                    }
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                }
                            }
                            if ri
                            {
                                nan = false;
                                i = num.imag().to_f64();
                                if !zero.1 && ((i * 1e8).round() / 1e8 != 0.0)
                                {
                                    zero.1 = true
                                }
                                if func.2.graphtype == Normal
                                {
                                    if !func.2.surface
                                    {
                                        imag.write_all(&n.to_le_bytes()).unwrap();
                                        imag.write_all(&f.to_le_bytes()).unwrap();
                                    }
                                    imag.write_all(&i.to_le_bytes()).unwrap();
                                }
                            }
                            if re && ri
                            {
                                if func.2.graphtype == Flat
                                {
                                    zero.1 = false;
                                    real.write_all(&n.to_le_bytes()).unwrap();
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                    real.write_all(&i.to_le_bytes()).unwrap();
                                }
                                else if func.2.graphtype == Depth
                                {
                                    zero.1 = false;
                                    real.write_all(&f.to_le_bytes()).unwrap();
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                    real.write_all(&i.to_le_bytes()).unwrap();
                                }
                            }
                        }
                    }
                    else if v.len() == 3
                    {
                        rec += 1;
                        nan = false;
                        let xr = v[0].number.real().to_f64();
                        let yr = v[1].number.real().to_f64();
                        let zr = v[2].number.real().to_f64();
                        let xi = v[0].number.imag().to_f64();
                        let yi = v[1].number.imag().to_f64();
                        let zi = v[2].number.imag().to_f64();
                        if !zero.0
                            && ((xr * 1e8).round() / 1e8 != 0.0
                                || (yr * 1e8).round() / 1e8 != 0.0
                                || (zr * 1e8).round() / 1e8 != 0.0)
                        {
                            zero.0 = true;
                        }
                        if !zero.1
                            && ((xi * 1e8).round() / 1e8 != 0.0
                                || (yi * 1e8).round() / 1e8 != 0.0
                                || (zi * 1e8).round() / 1e8 != 0.0)
                        {
                            zero.1 = true;
                        }
                        real.write_all(&xr.to_le_bytes()).unwrap();
                        real.write_all(&yr.to_le_bytes()).unwrap();
                        real.write_all(&zr.to_le_bytes()).unwrap();
                        imag.write_all(&xi.to_le_bytes()).unwrap();
                        imag.write_all(&yi.to_le_bytes()).unwrap();
                        imag.write_all(&zi.to_le_bytes()).unwrap();
                    }
                    else if v.len() == 2
                    {
                        rec += 1;
                        d2 = true;
                        nan = false;
                        let xr = v[0].number.real().to_f64();
                        let yr = v[1].number.real().to_f64();
                        let xi = v[0].number.imag().to_f64();
                        let yi = v[1].number.imag().to_f64();
                        if !zero.0
                            && ((xr * 1e8).round() / 1e8 != 0.0 || (yr * 1e8).round() / 1e8 != 0.0)
                        {
                            zero.0 = true;
                        }
                        if !zero.1
                            && ((xi * 1e8).round() / 1e8 != 0.0 || (yi * 1e8).round() / 1e8 != 0.0)
                        {
                            zero.1 = true;
                        }
                        real.write_all(&xr.to_le_bytes()).unwrap();
                        real.write_all(&yr.to_le_bytes()).unwrap();
                        imag.write_all(&xi.to_le_bytes()).unwrap();
                        imag.write_all(&yi.to_le_bytes()).unwrap();
                    }
                }
                Ok(Matrix(m)) =>
                {
                    for v in m
                    {
                        for num in v
                        {
                            let num = num.number;
                            rec += 1;
                            if num.real().is_nan() || num.imag().is_nan()
                            {
                                continue;
                            }
                            let mut r = 0.0;
                            let mut i = 0.0;
                            let re = !num.real().is_infinite();
                            let ri = !num.imag().is_infinite();
                            if re
                            {
                                nan = false;
                                r = num.real().to_f64();
                                if !zero.0 && ((r * 1e8).round() / 1e8 != 0.0)
                                {
                                    zero.0 = true
                                }
                                if func.2.graphtype == Normal
                                {
                                    if !func.2.surface
                                    {
                                        real.write_all(&n.to_le_bytes()).unwrap();
                                        real.write_all(&f.to_le_bytes()).unwrap();
                                    }
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                }
                            }
                            if ri
                            {
                                nan = false;
                                i = num.imag().to_f64();
                                if !zero.1 && ((i * 1e8).round() / 1e8 != 0.0)
                                {
                                    zero.1 = true
                                }
                                if func.2.graphtype == Normal
                                {
                                    if !func.2.surface
                                    {
                                        imag.write_all(&n.to_le_bytes()).unwrap();
                                        imag.write_all(&f.to_le_bytes()).unwrap();
                                    }
                                    imag.write_all(&i.to_le_bytes()).unwrap();
                                }
                            }
                            if re && ri
                            {
                                if func.2.graphtype == Flat
                                {
                                    zero.1 = false;
                                    real.write_all(&n.to_le_bytes()).unwrap();
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                    real.write_all(&i.to_le_bytes()).unwrap();
                                }
                                else if func.2.graphtype == Depth
                                {
                                    zero.1 = false;
                                    real.write_all(&f.to_le_bytes()).unwrap();
                                    real.write_all(&r.to_le_bytes()).unwrap();
                                    real.write_all(&i.to_le_bytes()).unwrap();
                                }
                            }
                        }
                    }
                }
                Err(s) =>
                {
                    println!("{}", s);
                    return Default::default();
                }
                _ =>
                {}
            }
        }
    }
    if nan
    {
        fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
        fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
    }
    else
    {
        if !zero.0 && zero.1
        {
            fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
        }
        if !zero.1
        {
            fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
        }
    }
    (zero, d2, rec)
}
fn fail(options: Options, colors: &Colors, input: String)
{
    print!(
        "\x1b[G\x1b[KNo data to plot for {}\x1b[G\n{}",
        input,
        prompt(options, colors)
    );
    stdout().flush().unwrap();
}
#[allow(clippy::type_complexity)]
fn get_data(
    colors: Colors,
    func: (
        Vec<NumStr>,
        Vec<(String, Vec<NumStr>)>,
        Options,
        HowGraphing,
    ),
    input: String,
    i: usize,
    data_dir: String,
) -> JoinHandle<((bool, bool), (bool, bool), bool, bool, u64)>
{
    thread::spawn(move || {
        let mut rec = 0;
        let mut lines = false;
        let mut d2_or_d3: (bool, bool) = (false, false);
        let mut re_or_im = (false, false);
        let (has_x, has_y) = (func.3.x, func.3.y);
        if !has_y && !has_x
        {
            match match do_math(func.0.clone(), func.2, func.1)
            {
                Ok(n) => n,
                _ =>
                {
                    fail(func.2, &colors, input);
                    return ((false, false), (false, false), false, true, 0);
                }
            }
            {
                Num(n) =>
                {
                    let n = n.number;
                    d2_or_d3.0 = true;
                    let im = n.imag().to_f64();
                    let re = n.real().to_f64();
                    let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                    let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                    for _ in 0..func.2.samples_2d
                    {
                        if re != 0.0 || im == 0.0
                        {
                            real.write_all(&re.to_le_bytes()).unwrap();
                        }
                        if im != 0.0
                        {
                            imag.write_all(&im.to_le_bytes()).unwrap();
                        }
                    }
                    if re == 0.0 && im != 0.0
                    {
                        fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
                    }
                    if im == 0.0
                    {
                        fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
                    }
                    re_or_im = (re != 0.0 || im == 0.0, im != 0.0);
                }
                Vector(v) =>
                {
                    lines = true;
                    match v.len()
                    {
                        3 =>
                        {
                            rec += 2;
                            d2_or_d3.1 = true;
                            let xr = v[0].number.real().to_f64();
                            let yr = v[1].number.real().to_f64();
                            let zr = v[2].number.real().to_f64();
                            let xi = v[0].number.imag().to_f64();
                            let yi = v[1].number.imag().to_f64();
                            let zi = v[2].number.imag().to_f64();
                            if (xr * 1e8).round() / 1e8 != 0.0
                                || (yr * 1e8).round() / 1e8 != 0.0
                                || (zr * 1e8).round() / 1e8 != 0.0
                            {
                                re_or_im.0 = true;
                                let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                                real.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                real.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                real.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                real.write_all(&xr.to_le_bytes()).unwrap();
                                real.write_all(&yr.to_le_bytes()).unwrap();
                                real.write_all(&zr.to_le_bytes()).unwrap();
                            }
                            if (xi * 1e8).round() / 1e8 != 0.0
                                || (yi * 1e8).round() / 1e8 != 0.0
                                || (zi * 1e8).round() / 1e8 != 0.0
                            {
                                re_or_im.1 = true;
                                let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                                imag.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                imag.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                imag.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                imag.write_all(&xi.to_le_bytes()).unwrap();
                                imag.write_all(&yi.to_le_bytes()).unwrap();
                                imag.write_all(&zi.to_le_bytes()).unwrap();
                            }
                        }
                        2 if func.0.iter().any(|c| c.str_is("±")) =>
                        {
                            lines = false;
                            let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                            let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                            {
                                let n = v[0].number.clone();
                                d2_or_d3.0 = true;
                                let change = (func.2.xr.1 - func.2.xr.0) / func.2.samples_2d as f64;
                                let im = n.imag().to_f64();
                                let re = n.real().to_f64();
                                for i in 0..func.2.samples_2d
                                {
                                    rec += 1;
                                    if re != 0.0 || im == 0.0
                                    {
                                        real.write_all(
                                            &(func.2.xr.0 + change * i as f64).to_le_bytes(),
                                        )
                                        .unwrap();
                                        real.write_all(&re.to_le_bytes()).unwrap();
                                    }
                                    if im != 0.0
                                    {
                                        imag.write_all(
                                            &(func.2.xr.0 + change * i as f64).to_le_bytes(),
                                        )
                                        .unwrap();
                                        imag.write_all(&im.to_le_bytes()).unwrap();
                                    }
                                }
                                re_or_im = (re != 0.0 || im == 0.0, im != 0.0);
                            }
                            {
                                let n = v[1].number.clone();
                                d2_or_d3.0 = true;
                                let change = (func.2.xr.1 - func.2.xr.0) / func.2.samples_2d as f64;
                                let im = n.imag().to_f64();
                                let re = n.real().to_f64();
                                for i in 0..func.2.samples_2d
                                {
                                    rec += 1;
                                    if re != 0.0 || im == 0.0
                                    {
                                        real.write_all(
                                            &(func.2.xr.0 + change * i as f64).to_le_bytes(),
                                        )
                                        .unwrap();
                                        real.write_all(&re.to_le_bytes()).unwrap();
                                    }
                                    if im != 0.0
                                    {
                                        imag.write_all(
                                            &(func.2.xr.0 + change * i as f64).to_le_bytes(),
                                        )
                                        .unwrap();
                                        imag.write_all(&im.to_le_bytes()).unwrap();
                                    }
                                }
                                if !re_or_im.0
                                {
                                    re_or_im.0 = re != 0.0 || im == 0.0
                                }
                                if !re_or_im.1
                                {
                                    re_or_im.1 = im != 0.0
                                }
                            }
                            if !re_or_im.0 && re_or_im.1
                            {
                                fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
                            }
                            if !re_or_im.1
                            {
                                fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
                            }
                        }
                        2 =>
                        {
                            rec += 2;
                            d2_or_d3.0 = true;
                            let xr = v[0].number.real().to_f64();
                            let yr = v[1].number.real().to_f64();
                            let xi = v[0].number.imag().to_f64();
                            let yi = v[1].number.imag().to_f64();
                            if (xr * 1e8).round() / 1e8 != 0.0 || (yr * 1e8).round() / 1e8 != 0.0
                            {
                                re_or_im.0 = true;
                                let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                                real.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                real.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                real.write_all(&xr.to_le_bytes()).unwrap();
                                real.write_all(&yr.to_le_bytes()).unwrap();
                            }
                            if (xi * 1e8).round() / 1e8 != 0.0 || (yi * 1e8).round() / 1e8 != 0.0
                            {
                                re_or_im.1 = true;
                                let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                                imag.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                imag.write_all(&0.0_f64.to_le_bytes()).unwrap();
                                imag.write_all(&xi.to_le_bytes()).unwrap();
                                imag.write_all(&yi.to_le_bytes()).unwrap();
                            }
                        }
                        _ =>
                        {
                            d2_or_d3.0 = true;
                            let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                            let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                            let mut zero = (false, false);
                            for (i, p) in v.iter().enumerate()
                            {
                                rec += 1;
                                let pr = p.number.real().to_f64();
                                if (pr * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.0 = true
                                }
                                let pi = p.number.imag().to_f64();
                                if (pi * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.1 = true
                                }
                                real.write_all(&((i + 1) as f64).to_le_bytes()).unwrap();
                                real.write_all(&pr.to_le_bytes()).unwrap();
                                imag.write_all(&((i + 1) as f64).to_le_bytes()).unwrap();
                                imag.write_all(&pi.to_le_bytes()).unwrap();
                            }
                            if !zero.0 && zero.1
                            {
                                fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
                            }
                            else
                            {
                                re_or_im.0 = true;
                            }
                            if !zero.1
                            {
                                fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
                            }
                            else
                            {
                                re_or_im.1 = true;
                            }
                        }
                    }
                }
                Matrix(m) =>
                {
                    if !m.iter().all(|a| a.len() == m[0].len())
                    {
                        print!(
                            "\x1b[G\x1b[Kbad matrix data in {}\x1b[G\n{}",
                            input,
                            prompt(func.2, &colors)
                        );
                        stdout().flush().unwrap();
                        return ((false, false), (false, false), false, true, 0);
                    }
                    lines = m.len() != 1;
                    match m[0].len()
                    {
                        3 =>
                        {
                            d2_or_d3.1 = true;
                            let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                            let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                            for v in m
                            {
                                rec += 1;
                                let xr = v[0].number.real().to_f64();
                                let yr = v[1].number.real().to_f64();
                                let zr = v[2].number.real().to_f64();
                                let xi = v[0].number.imag().to_f64();
                                let yi = v[1].number.imag().to_f64();
                                let zi = v[2].number.imag().to_f64();
                                if !re_or_im.0
                                    && ((xr * 1e8).round() / 1e8 != 0.0
                                        || (yr * 1e8).round() / 1e8 != 0.0
                                        || (zr * 1e8).round() / 1e8 != 0.0)
                                {
                                    re_or_im.0 = true;
                                }
                                if !re_or_im.1
                                    && ((xi * 1e8).round() / 1e8 != 0.0
                                        || (yi * 1e8).round() / 1e8 != 0.0
                                        || (zi * 1e8).round() / 1e8 != 0.0)
                                {
                                    re_or_im.1 = true;
                                }
                                real.write_all(&xr.to_le_bytes()).unwrap();
                                real.write_all(&yr.to_le_bytes()).unwrap();
                                real.write_all(&zr.to_le_bytes()).unwrap();
                                imag.write_all(&xi.to_le_bytes()).unwrap();
                                imag.write_all(&yi.to_le_bytes()).unwrap();
                                imag.write_all(&zi.to_le_bytes()).unwrap();
                            }
                            if !re_or_im.0
                            {
                                fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
                            }
                            if !re_or_im.1
                            {
                                fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
                            }
                        }
                        2 =>
                        {
                            d2_or_d3.0 = true;
                            let mut real = File::create(format!("{data_dir}/re{i}")).unwrap();
                            let mut imag = File::create(format!("{data_dir}/im{i}")).unwrap();
                            for v in m
                            {
                                rec += 1;
                                let xr = v[0].number.real().to_f64();
                                let yr = v[1].number.real().to_f64();
                                let xi = v[0].number.imag().to_f64();
                                let yi = v[1].number.imag().to_f64();
                                if !re_or_im.0
                                    && ((xr * 1e8).round() / 1e8 != 0.0
                                        || (yr * 1e8).round() / 1e8 != 0.0)
                                {
                                    re_or_im.0 = true;
                                }
                                if !re_or_im.1
                                    && ((xi * 1e8).round() / 1e8 != 0.0
                                        || (yi * 1e8).round() / 1e8 != 0.0)
                                {
                                    re_or_im.1 = true;
                                }
                                real.write_all(&xr.to_le_bytes()).unwrap();
                                real.write_all(&yr.to_le_bytes()).unwrap();
                                imag.write_all(&xi.to_le_bytes()).unwrap();
                                imag.write_all(&yi.to_le_bytes()).unwrap();
                            }
                            if !re_or_im.0
                            {
                                fs::remove_file(format!("{data_dir}/re{i}")).unwrap();
                            }
                            if !re_or_im.1
                            {
                                fs::remove_file(format!("{data_dir}/im{i}")).unwrap();
                            }
                        }
                        _ =>
                        {}
                    }
                }
                _ =>
                {}
            }
        }
        else if !has_y || !has_x
        {
            let d3;
            (re_or_im, d3, rec) = get_list_2d(func, i, &data_dir, has_x);
            if d3
            {
                d2_or_d3.1 = true;
            }
            else
            {
                d2_or_d3.0 = true;
            }
        }
        else
        {
            let d2;
            (re_or_im, d2, rec) = get_list_3d(func, i, &data_dir);
            if d2
            {
                d2_or_d3.0 = true;
            }
            else
            {
                d2_or_d3.1 = true;
            }
        }
        (d2_or_d3, re_or_im, lines, false, rec)
    })
}
