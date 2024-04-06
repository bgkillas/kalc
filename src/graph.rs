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
    parse::input_var,
    Colors, Number, Options, Variable,
};
use gnuplot::{Auto, AxesCommon, Caption, Color, Figure, Fix, PointSymbol, TickOption};
use rug::Complex;
use std::{
    io::{stdout, Write},
    thread,
    thread::JoinHandle,
    time::Instant,
};
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
                let split = s.split(|c| c == ';');
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
            if !options.graph
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
                    false,
                    0,
                    Vec::new(),
                )
                {
                    Ok(f) => (f.0, f.1, options),
                    _ =>
                    {
                        print!("\x1b[G\x1b[Kbad input\x1b[G\n{}", prompt(options, &colors));
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
        let mut options = func[0].2;
        let mut fg = Figure::new();
        if cli
        {
            options.onaxis = false;
            options.scale_graph = false;
            fg.set_terminal("dumb size 125,60 aspect 1,1", "");
        }
        fg.set_enhanced_text(false);
        let mut re_cap: [String; 6] = Default::default();
        let mut im_cap: [String; 6] = Default::default();
        let mut points2d: [[[Vec<f64>; 2]; 2]; 6] = Default::default();
        let mut points3d: [[[Vec<f64>; 3]; 2]; 6] = Default::default();
        let mut d2_or_d3 = (false, false);
        let mut lines = false;
        let mut handles = Vec::new();
        for (func, input) in func.iter().zip(input.iter())
        {
            handles.push(get_data(colors.clone(), func.clone(), input.clone()));
        }
        let mut i = 0;
        #[allow(clippy::explicit_counter_loop)]
        for handle in handles
        {
            let re_or_im;
            let failed;
            let dimen;
            let line;
            (dimen, re_or_im, line, failed, points2d[i], points3d[i]) = handle.join().unwrap();
            if failed
            {
                return;
            }
            if re_or_im.0 || !re_or_im.1
            {
                re_cap[i] = input[i].clone() + if re_or_im.1 { ":re" } else { "" }
            }
            if re_or_im.1
            {
                im_cap[i] = input[i].clone() + ":im"
            }
            if dimen.0
            {
                d2_or_d3.0 = true;
            }
            if dimen.1
            {
                d2_or_d3.1 = true;
            }
            if line
            {
                lines = true
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
        if d2_or_d3.0
        {
            // if lines
            // {
            //     if Options::default().xr == options.xr
            //     {
            //         options.xr = (
            //             points2d.iter().fold(f64::MAX, |min, x| {
            //                 min.min(
            //                     x[0][0]
            //                         .iter()
            //                         .chain(&x[1][0])
            //                         .fold(f64::MAX, |min, x| min.min(*x)),
            //                 )
            //             }),
            //             points2d.iter().fold(f64::MIN, |max, x| {
            //                 max.max(
            //                     x[0][0]
            //                         .iter()
            //                         .chain(&x[1][0])
            //                         .fold(f64::MIN, |max, x| max.max(*x)),
            //                 )
            //             }),
            //         )
            //     }
            //     if Options::default().yr == options.yr
            //     {
            //         options.yr = (
            //             points2d.iter().fold(f64::MAX, |min, x| {
            //                 min.min(
            //                     x[0][1]
            //                         .iter()
            //                         .chain(&x[1][1])
            //                         .fold(f64::MAX, |min, x| min.min(*x)),
            //                 )
            //             }),
            //             points2d.iter().fold(f64::MIN, |max, x| {
            //                 max.max(
            //                     x[0][1]
            //                         .iter()
            //                         .chain(&x[1][1])
            //                         .fold(f64::MIN, |max, x| max.max(*x)),
            //                 )
            //             }),
            //         )
            //     }
            // }
            if options.vxr.0 != 0.0 || options.vxr.1 != 0.0
            {
                options.xr = options.vxr;
            }
            if options.vyr.0 != 0.0 || options.vyr.1 != 0.0
            {
                options.yr = options.vyr;
            }
            let (xticks, yticks) = if options.ticks == -2.0
            {
                (Some((Auto, 0)), Some((Auto, 0)))
            }
            else if options.ticks == -1.0
            {
                (Some((Fix(1.0), 0)), Some((Fix(1.0), 0)))
            }
            else if options.ticks == 0.0
            {
                (None, None)
            }
            else
            {
                (
                    Some((Fix((options.xr.1 - options.xr.0) / options.ticks), 0)),
                    Some((Fix((options.yr.1 - options.yr.0) / options.ticks), 0)),
                )
            };
            let ratio = if options.scale_graph
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
                Fix(rt)
            }
            else
            {
                Auto
            };
            if (options.lines || lines) && lines ^ options.lines
            {
                fg.axes2d()
                    .set_x_grid(!cli)
                    .set_y_grid(!cli)
                    .set_aspect_ratio(ratio)
                    .set_x_ticks(xticks, &[TickOption::OnAxis(options.onaxis)], &[])
                    .set_y_ticks(yticks, &[TickOption::OnAxis(options.onaxis)], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_x_label(if options.depth { "re" } else { &colors.label.0 }, &[])
                    .set_y_label(if options.depth { "im" } else { &colors.label.1 }, &[])
                    .lines([0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .lines_points(
                        &points2d[0][0][0],
                        &points2d[0][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .lines_points(
                        if points2d[0][1][0].is_empty()
                        {
                            &points2d[0][0][0]
                        }
                        else
                        {
                            &points2d[0][1][0]
                        },
                        &points2d[0][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .lines_points(
                        &points2d[1][0][0],
                        &points2d[1][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .lines_points(
                        if points2d[1][1][0].is_empty()
                        {
                            &points2d[1][0][0]
                        }
                        else
                        {
                            &points2d[1][1][0]
                        },
                        &points2d[1][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .lines_points(
                        &points2d[2][0][0],
                        &points2d[2][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .lines_points(
                        if points2d[2][1][0].is_empty()
                        {
                            &points2d[2][0][0]
                        }
                        else
                        {
                            &points2d[2][1][0]
                        },
                        &points2d[2][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .lines_points(
                        &points2d[3][0][0],
                        &points2d[3][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .lines_points(
                        if points2d[3][1][0].is_empty()
                        {
                            &points2d[3][0][0]
                        }
                        else
                        {
                            &points2d[3][1][0]
                        },
                        &points2d[3][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .lines_points(
                        &points2d[4][0][0],
                        &points2d[4][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .lines_points(
                        if points2d[4][1][0].is_empty()
                        {
                            &points2d[4][0][0]
                        }
                        else
                        {
                            &points2d[4][1][0]
                        },
                        &points2d[4][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .lines_points(
                        &points2d[5][0][0],
                        &points2d[5][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .lines_points(
                        if points2d[5][1][0].is_empty()
                        {
                            &points2d[5][0][0]
                        }
                        else
                        {
                            &points2d[5][1][0]
                        },
                        &points2d[5][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
            else
            {
                fg.axes2d()
                    .set_x_grid(!cli)
                    .set_y_grid(!cli)
                    .set_aspect_ratio(ratio)
                    .set_x_ticks(xticks, &[TickOption::OnAxis(options.onaxis)], &[])
                    .set_y_ticks(yticks, &[TickOption::OnAxis(options.onaxis)], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_x_label(if options.depth { "re" } else { &colors.label.0 }, &[])
                    .set_y_label(if options.depth { "im" } else { &colors.label.1 }, &[])
                    .lines([0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .points(
                        &points2d[0][0][0],
                        &points2d[0][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .points(
                        if points2d[0][1][0].is_empty()
                        {
                            &points2d[0][0][0]
                        }
                        else
                        {
                            &points2d[0][1][0]
                        },
                        &points2d[0][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .points(
                        &points2d[1][0][0],
                        &points2d[1][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .points(
                        if points2d[1][1][0].is_empty()
                        {
                            &points2d[1][0][0]
                        }
                        else
                        {
                            &points2d[1][1][0]
                        },
                        &points2d[1][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .points(
                        &points2d[2][0][0],
                        &points2d[2][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .points(
                        if points2d[2][1][0].is_empty()
                        {
                            &points2d[2][0][0]
                        }
                        else
                        {
                            &points2d[2][1][0]
                        },
                        &points2d[2][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .points(
                        &points2d[3][0][0],
                        &points2d[3][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .points(
                        if points2d[3][1][0].is_empty()
                        {
                            &points2d[3][0][0]
                        }
                        else
                        {
                            &points2d[3][1][0]
                        },
                        &points2d[3][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .points(
                        &points2d[4][0][0],
                        &points2d[4][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .points(
                        if points2d[4][1][0].is_empty()
                        {
                            &points2d[4][0][0]
                        }
                        else
                        {
                            &points2d[4][1][0]
                        },
                        &points2d[4][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .points(
                        &points2d[5][0][0],
                        &points2d[5][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .points(
                        if points2d[5][1][0].is_empty()
                        {
                            &points2d[5][0][0]
                        }
                        else
                        {
                            &points2d[5][1][0]
                        },
                        &points2d[5][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
        }
        if d2_or_d3.1
        {
            // if lines
            // {
            //     if Options::default().xr == options.xr
            //     {
            //         options.xr = (
            //             points3d.iter().fold(f64::MAX, |min, x| {
            //                 min.min(
            //                     x[0][0]
            //                         .iter()
            //                         .chain(&x[1][0])
            //                         .fold(f64::MAX, |min, x| min.min(*x)),
            //                 )
            //             }),
            //             points3d.iter().fold(f64::MIN, |max, x| {
            //                 max.max(
            //                     x[0][0]
            //                         .iter()
            //                         .chain(&x[1][0])
            //                         .fold(f64::MIN, |max, x| max.max(*x)),
            //                 )
            //             }),
            //         )
            //     }
            //     if Options::default().yr == options.yr
            //     {
            //         options.yr = (
            //             points3d.iter().fold(f64::MAX, |min, x| {
            //                 min.min(
            //                     x[0][1]
            //                         .iter()
            //                         .chain(&x[1][1])
            //                         .fold(f64::MAX, |min, x| min.min(*x)),
            //                 )
            //             }),
            //             points3d.iter().fold(f64::MIN, |max, x| {
            //                 max.max(
            //                     x[0][1]
            //                         .iter()
            //                         .chain(&x[1][1])
            //                         .fold(f64::MIN, |max, x| max.max(*x)),
            //                 )
            //             }),
            //         )
            //     }
            //     if Options::default().zr == options.zr
            //     {
            //         options.zr = (
            //             points3d.iter().fold(f64::MAX, |min, x| {
            //                 min.min(
            //                     x[0][2]
            //                         .iter()
            //                         .chain(&x[1][2])
            //                         .fold(f64::MAX, |min, x| min.min(*x)),
            //                 )
            //             }),
            //             points3d.iter().fold(f64::MIN, |max, x| {
            //                 max.max(
            //                     x[0][2]
            //                         .iter()
            //                         .chain(&x[1][2])
            //                         .fold(f64::MIN, |max, x| max.max(*x)),
            //                 )
            //             }),
            //         )
            //     }
            // }
            if options.vxr.0 != 0.0 || options.vxr.1 != 0.0
            {
                options.xr = options.vxr;
            }
            if options.vyr.0 != 0.0 || options.vyr.1 != 0.0
            {
                options.yr = options.vyr;
            }
            if options.vzr.0 != 0.0 || options.vzr.1 != 0.0
            {
                options.zr = options.vzr;
            }
            let (xticks, yticks, zticks) = if options.ticks == -2.0
            {
                (Some((Auto, 0)), Some((Auto, 0)), Some((Auto, 0)))
            }
            else if options.ticks == -1.0
            {
                (
                    Some((Fix(1.0), 0)),
                    Some((Fix(1.0), 0)),
                    Some((Fix(1.0), 0)),
                )
            }
            else if options.ticks == 0.0
            {
                (None, None, None)
            }
            else
            {
                (
                    Some((Fix((options.xr.1 - options.xr.0) / options.ticks), 0)),
                    Some((Fix((options.yr.1 - options.yr.0) / options.ticks), 0)),
                    Some((Fix((options.zr.1 - options.zr.0) / options.ticks), 0)),
                )
            };
            if options.surface
                && points3d[1..6]
                    .iter()
                    .all(|p| p[0][2].is_empty() && p[1][2].is_empty())
                && (points3d[0][0][2].is_empty() || points3d[0][1][2].is_empty())
            {
                fg.axes3d()
                    //.set_palette(Custom(&[(0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0)]))
                    .set_x_grid(!cli)
                    .set_y_grid(!cli)
                    .set_z_grid(!cli)
                    .set_x_ticks(xticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_y_ticks(yticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_z_ticks(zticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_x_label(&colors.label.0, &[])
                    .set_y_label(if options.depth { "re" } else { &colors.label.1 }, &[])
                    .set_z_label(if options.depth { "im" } else { &colors.label.2 }, &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .surface(
                        &points3d[0][0][2],
                        options.samples_3d.0 + 1,
                        options.samples_3d.1 + 1,
                        Some((options.xr.0, options.yr.0, options.xr.1, options.yr.1)),
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .surface(
                        &points3d[0][1][2],
                        options.samples_3d.0 + 1,
                        options.samples_3d.1 + 1,
                        Some((options.xr.0, options.yr.0, options.xr.1, options.yr.1)),
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    );
            }
            else if options.lines || lines
            {
                fg.axes3d()
                    .set_x_grid(!cli)
                    .set_y_grid(!cli)
                    .set_z_grid(!cli)
                    .set_x_ticks(xticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_y_ticks(yticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_z_ticks(zticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_x_label(&colors.label.0, &[])
                    .set_y_label(if options.depth { "re" } else { &colors.label.1 }, &[])
                    .set_z_label(if options.depth { "im" } else { &colors.label.2 }, &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .lines_points(
                        &points3d[0][0][0],
                        &points3d[0][0][1],
                        &points3d[0][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .lines_points(
                        if points3d[0][1][0].is_empty()
                        {
                            &points3d[0][0][0]
                        }
                        else
                        {
                            &points3d[0][1][0]
                        },
                        if points3d[0][1][1].is_empty()
                        {
                            &points3d[0][0][1]
                        }
                        else
                        {
                            &points3d[0][1][1]
                        },
                        &points3d[0][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .lines_points(
                        &points3d[1][0][0],
                        &points3d[1][0][1],
                        &points3d[1][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .lines_points(
                        if points3d[1][1][0].is_empty()
                        {
                            &points3d[1][0][0]
                        }
                        else
                        {
                            &points3d[1][1][0]
                        },
                        if points3d[1][1][1].is_empty()
                        {
                            &points3d[1][0][1]
                        }
                        else
                        {
                            &points3d[1][1][1]
                        },
                        &points3d[1][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .lines_points(
                        &points3d[2][0][0],
                        &points3d[2][0][1],
                        &points3d[2][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .lines_points(
                        if points3d[2][1][0].is_empty()
                        {
                            &points3d[2][0][0]
                        }
                        else
                        {
                            &points3d[2][1][0]
                        },
                        if points3d[2][1][1].is_empty()
                        {
                            &points3d[2][0][1]
                        }
                        else
                        {
                            &points3d[2][1][1]
                        },
                        &points3d[2][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .lines_points(
                        &points3d[3][0][0],
                        &points3d[3][0][1],
                        &points3d[3][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .lines_points(
                        if points3d[3][1][0].is_empty()
                        {
                            &points3d[3][0][0]
                        }
                        else
                        {
                            &points3d[3][1][0]
                        },
                        if points3d[3][1][1].is_empty()
                        {
                            &points3d[3][0][1]
                        }
                        else
                        {
                            &points3d[3][1][1]
                        },
                        &points3d[3][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .lines_points(
                        &points3d[4][0][0],
                        &points3d[4][0][1],
                        &points3d[4][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .lines_points(
                        if points3d[4][1][0].is_empty()
                        {
                            &points3d[4][0][0]
                        }
                        else
                        {
                            &points3d[4][1][0]
                        },
                        if points3d[4][1][1].is_empty()
                        {
                            &points3d[4][0][1]
                        }
                        else
                        {
                            &points3d[4][1][1]
                        },
                        &points3d[4][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .lines_points(
                        &points3d[5][0][0],
                        &points3d[5][0][1],
                        &points3d[5][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .lines_points(
                        if points3d[5][1][0].is_empty()
                        {
                            &points3d[5][0][0]
                        }
                        else
                        {
                            &points3d[5][1][0]
                        },
                        if points3d[5][1][1].is_empty()
                        {
                            &points3d[5][0][1]
                        }
                        else
                        {
                            &points3d[5][1][1]
                        },
                        &points3d[5][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
            else
            {
                fg.axes3d()
                    .set_x_grid(!cli)
                    .set_y_grid(!cli)
                    .set_z_grid(!cli)
                    .set_x_ticks(xticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_y_ticks(yticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_z_ticks(zticks, &[TickOption::OnAxis(!options.onaxis && !cli)], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_x_label(&colors.label.0, &[])
                    .set_y_label(if options.depth { "re" } else { &colors.label.1 }, &[])
                    .set_z_label(if options.depth { "im" } else { &colors.label.2 }, &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .points(
                        &points3d[0][0][0],
                        &points3d[0][0][1],
                        &points3d[0][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .points(
                        if points3d[0][1][0].is_empty()
                        {
                            &points3d[0][0][0]
                        }
                        else
                        {
                            &points3d[0][1][0]
                        },
                        if points3d[0][1][1].is_empty()
                        {
                            &points3d[0][0][1]
                        }
                        else
                        {
                            &points3d[0][1][1]
                        },
                        &points3d[0][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .points(
                        &points3d[1][0][0],
                        &points3d[1][0][1],
                        &points3d[1][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .points(
                        if points3d[1][1][0].is_empty()
                        {
                            &points3d[1][0][0]
                        }
                        else
                        {
                            &points3d[1][1][0]
                        },
                        if points3d[1][1][1].is_empty()
                        {
                            &points3d[1][0][1]
                        }
                        else
                        {
                            &points3d[1][1][1]
                        },
                        &points3d[1][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .points(
                        &points3d[2][0][0],
                        &points3d[2][0][1],
                        &points3d[2][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .points(
                        if points3d[2][1][0].is_empty()
                        {
                            &points3d[2][0][0]
                        }
                        else
                        {
                            &points3d[2][1][0]
                        },
                        if points3d[2][1][1].is_empty()
                        {
                            &points3d[2][0][1]
                        }
                        else
                        {
                            &points3d[2][1][1]
                        },
                        &points3d[2][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .points(
                        &points3d[3][0][0],
                        &points3d[3][0][1],
                        &points3d[3][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .points(
                        if points3d[3][1][0].is_empty()
                        {
                            &points3d[3][0][0]
                        }
                        else
                        {
                            &points3d[3][1][0]
                        },
                        if points3d[3][1][1].is_empty()
                        {
                            &points3d[3][0][1]
                        }
                        else
                        {
                            &points3d[3][1][1]
                        },
                        &points3d[3][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .points(
                        &points3d[4][0][0],
                        &points3d[4][0][1],
                        &points3d[4][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .points(
                        if points3d[4][1][0].is_empty()
                        {
                            &points3d[4][0][0]
                        }
                        else
                        {
                            &points3d[4][1][0]
                        },
                        if points3d[4][1][1].is_empty()
                        {
                            &points3d[4][0][1]
                        }
                        else
                        {
                            &points3d[4][1][1]
                        },
                        &points3d[4][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .points(
                        &points3d[5][0][0],
                        &points3d[5][0][1],
                        &points3d[5][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .points(
                        if points3d[5][1][0].is_empty()
                        {
                            &points3d[5][0][0]
                        }
                        else
                        {
                            &points3d[5][1][0]
                        },
                        if points3d[5][1][1].is_empty()
                        {
                            &points3d[5][0][1]
                        }
                        else
                        {
                            &points3d[5][1][1]
                        },
                        &points3d[5][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
        }
        if let Some(time) = watch
        {
            println!("\x1b[G\x1b[K{}ms\x1b[G\n", time.elapsed().as_millis());
        }
        if !colors.graphtofile.is_empty()
        {
            fg.save_to_png(
                colors.graphtofile,
                options.window_size.0 as u32,
                options.window_size.1 as u32,
            )
            .unwrap()
        }
        else if fg.show().is_err()
        {
            print!("\x1b[G\x1b[Kno gnuplot\x1b[G\n{}", prompt(options, &colors));
            stdout().flush().unwrap();
        }
    })
}
#[allow(clippy::type_complexity)]
pub fn get_list_2d(
    mut func: (Vec<NumStr>, Vec<(String, Vec<NumStr>)>, Options),
) -> ([[Vec<f64>; 2]; 2], [Vec<f64>; 2], (bool, bool))
{
    {
        let temp = func.1.clone();
        for i in func.1.iter_mut()
        {
            if let Ok(n) = do_math(i.1.clone(), func.2, temp.clone())
            {
                i.1 = vec![n];
            }
        }
    }
    if let Num(n) = &func.0[0]
    {
        if func.0.len() == 1 && n.number.is_zero()
        {
            return Default::default();
        }
    }
    let mut data: [[Vec<f64>; 2]; 2] = [
        [
            Vec::with_capacity(func.2.samples_2d + 1),
            Vec::with_capacity(func.2.samples_2d + 1),
        ],
        [Vec::new(), Vec::with_capacity(func.2.samples_2d + 1)],
    ];
    let mut data3d: [Vec<f64>; 2] = [
        Vec::with_capacity(func.2.samples_2d + 1),
        Vec::with_capacity(func.2.samples_2d + 1),
    ];
    let mut nan = true;
    let den_range = (func.2.xr.1 - func.2.xr.0) / func.2.samples_2d as f64;
    let mut zero = (false, false);
    let list = func.0.iter().any(|c| {
        if let Str(s) = c
        {
            matches!(s.as_str(), "±" | "cubic" | "quadratic" | "quartic")
        }
        else
        {
            false
        }
    }) || func.1.iter().any(|c| {
        c.1.iter().any(|c| {
            if let Str(s) = c
            {
                matches!(s.as_str(), "±" | "cubic" | "quadratic" | "quartic")
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
                let complex = !num.real().is_infinite();
                if complex
                {
                    nan = false;
                    let f = num.real().to_f64();
                    if (f * 1e8).round() / 1e8 != 0.0
                    {
                        zero.0 = true
                    }
                    data[0][0].push(n);
                    data[0][1].push(f);
                }
                if !num.imag().is_infinite()
                {
                    nan = false;
                    let f = num.imag().to_f64();
                    if (f * 1e8).round() / 1e8 != 0.0
                    {
                        zero.1 = true
                    }
                    if !complex
                    {
                        data[0][0].push(n);
                        data[0][1].push(f64::NAN);
                    }
                    data[1][1].push(f);
                }
                else
                {
                    if !complex
                    {
                        data[0][0].push(n);
                        data[0][1].push(f64::NAN);
                    }
                    data[1][1].push(f64::NAN);
                }
            }
            Ok(Vector(v)) =>
            {
                if list || v.len() == 1 || v.len() > 3
                {
                    for num in v
                    {
                        let num = num.number;
                        if num.real().is_nan() || num.imag().is_nan()
                        {
                            continue;
                        }
                        let complex = !num.real().is_infinite();
                        if complex
                        {
                            nan = false;
                            let f = num.real().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            data[0][0].push(n);
                            data[0][1].push(f);
                        }
                        if !num.imag().is_infinite()
                        {
                            nan = false;
                            let f = num.imag().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            if !complex
                            {
                                data[0][0].push(n);
                                data[0][1].push(f64::NAN);
                            }
                            data[1][1].push(f);
                        }
                        else
                        {
                            if !complex
                            {
                                data[0][0].push(n);
                                data[0][1].push(f64::NAN);
                            }
                            data[1][1].push(f64::NAN);
                        }
                    }
                }
                else if v.len() == 3
                {
                    nan = false;
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
                        zero.0 = true;
                    }
                    if (xi * 1e8).round() / 1e8 != 0.0
                        || (yi * 1e8).round() / 1e8 != 0.0
                        || (zi * 1e8).round() / 1e8 != 0.0
                    {
                        zero.1 = true;
                    }
                    data[0][0].push(xr);
                    data[0][1].push(yr);
                    data3d[0].push(zr);
                    data[1][0].push(xi);
                    data[1][1].push(yi);
                    data3d[1].push(zi);
                }
                else if v.len() == 2
                {
                    nan = false;
                    let xr = v[0].number.real().to_f64();
                    let yr = v[1].number.real().to_f64();
                    let xi = v[0].number.imag().to_f64();
                    let yi = v[1].number.imag().to_f64();
                    if (xr * 1e8).round() / 1e8 != 0.0 || (yr * 1e8).round() / 1e8 != 0.0
                    {
                        zero.0 = true;
                    }
                    if (xi * 1e8).round() / 1e8 != 0.0 || (yi * 1e8).round() / 1e8 != 0.0
                    {
                        zero.1 = true;
                    }
                    data[0][0].push(xr);
                    data[0][1].push(yr);
                    data[1][0].push(xi);
                    data[1][1].push(yi);
                }
            }
            Ok(Matrix(m)) =>
            {
                for v in m
                {
                    for num in v
                    {
                        let num = num.number;
                        if num.real().is_nan() || num.imag().is_nan()
                        {
                            continue;
                        }
                        let complex = !num.real().is_infinite();
                        if complex
                        {
                            nan = false;
                            let f = num.real().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            data[0][0].push(n);
                            data[0][1].push(f);
                        }
                        if !num.imag().is_infinite()
                        {
                            nan = false;
                            let f = num.imag().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            if !complex
                            {
                                data[0][0].push(n);
                                data[0][1].push(f64::NAN);
                            }
                            data[1][1].push(f);
                        }
                        else
                        {
                            if !complex
                            {
                                data[0][0].push(n);
                                data[0][1].push(f64::NAN);
                            }
                            data[1][1].push(f64::NAN);
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
    if !zero.0 && zero.1
    {
        data[0][1] = Vec::new();
        data3d[0] = Vec::new();
    }
    if !zero.1 && zero.0
    {
        data[1][1] = Vec::new();
        data3d[1] = Vec::new();
    }
    if !zero.1 && !zero.0
    {
        data[1][1] = Vec::new();
        data3d[1] = Vec::new();
    }
    if (data[0][0].is_empty()
        && data[1][0].is_empty()
        && data3d[0].is_empty()
        && data3d[1].is_empty())
        || nan
    {
        println!("\x1b[G\ngraph is all 0, nan or infinity");
        return Default::default();
    }
    (data, data3d, zero)
}
#[allow(clippy::type_complexity)]
pub fn get_list_3d(
    mut func: (Vec<NumStr>, Vec<(String, Vec<NumStr>)>, Options),
) -> ([[Vec<f64>; 3]; 2], (bool, bool), bool)
{
    {
        let temp = func.1.clone();
        for i in func.1.iter_mut()
        {
            if let Ok(n) = do_math(i.1.clone(), func.2, temp.clone())
            {
                i.1 = vec![n];
            }
        }
    }
    if let Num(n) = &func.0[0]
    {
        let n = &n.number;
        if func.0.len() == 1 && n.is_zero()
        {
            return Default::default();
        }
    }
    let mut data: [[Vec<f64>; 3]; 2] = [
        [
            Vec::with_capacity(func.2.samples_3d.0 + 1),
            Vec::with_capacity(func.2.samples_3d.1 + 1),
            Vec::with_capacity((func.2.samples_3d.0 + 1) * (func.2.samples_3d.1 + 1)),
        ],
        [
            Vec::new(),
            Vec::new(),
            Vec::with_capacity((func.2.samples_3d.0 + 1) * (func.2.samples_3d.1 + 1)),
        ],
    ];
    let den_x_range = (func.2.xr.1 - func.2.xr.0) / func.2.samples_3d.0 as f64;
    let den_y_range = (func.2.yr.1 - func.2.yr.0) / func.2.samples_3d.1 as f64;
    let mut modified: Vec<NumStr>;
    let mut modifiedvars: Vec<(String, Vec<NumStr>)>;
    let mut zero = (false, false);
    let mut nan = true;
    let list = func.0.iter().any(|c| {
        if let Str(s) = c
        {
            matches!(s.as_str(), "±" | "cubic" | "quadratic" | "quartic")
        }
        else
        {
            false
        }
    }) || func.1.iter().any(|c| {
        c.1.iter().any(|c| {
            if let Str(s) = c
            {
                matches!(s.as_str(), "±" | "cubic" | "quadratic" | "quartic")
            }
            else
            {
                false
            }
        })
    });
    let mut d2 = false;
    for i in 0..=func.2.samples_3d.0
    {
        let n = func.2.xr.0 + i as f64 * den_x_range;
        let num = Num(Number::from(Complex::with_val(func.2.prec, n), None));
        modified = place_var(func.0.clone(), "x", num.clone());
        modifiedvars = place_funcvar(func.1.clone(), "x", num);
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
                    let complex = !num.real().is_infinite();
                    if complex
                    {
                        nan = false;
                        let g = num.real().to_f64();
                        if (g * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        data[0][0].push(n);
                        data[0][1].push(f);
                        data[0][2].push(g);
                    }
                    if !num.imag().is_infinite()
                    {
                        nan = false;
                        let g = num.imag().to_f64();
                        if (g * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        if !complex
                        {
                            data[0][0].push(n);
                            data[0][1].push(f);
                            data[0][2].push(f64::NAN);
                        }
                        data[1][2].push(g);
                    }
                    else
                    {
                        if !complex
                        {
                            data[0][0].push(n);
                            data[0][1].push(f);
                            data[0][2].push(f64::NAN);
                        }
                        data[1][2].push(f64::NAN);
                    }
                }
                Ok(Vector(v)) =>
                {
                    if list || v.len() == 1 || v.len() > 3
                    {
                        for num in v
                        {
                            let num = num.number;
                            if num.real().is_nan() || num.imag().is_nan()
                            {
                                continue;
                            }
                            let complex = !num.real().is_infinite();
                            if complex
                            {
                                nan = false;
                                let g = num.real().to_f64();
                                if (g * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.0 = true
                                }
                                data[0][0].push(n);
                                data[0][1].push(f);
                                data[0][2].push(g);
                            }
                            if !num.imag().is_infinite()
                            {
                                nan = false;
                                let g = num.imag().to_f64();
                                if (g * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.1 = true
                                }
                                if !complex
                                {
                                    data[0][0].push(n);
                                    data[0][1].push(f);
                                    data[0][2].push(f64::NAN);
                                }
                                data[1][2].push(g);
                            }
                            else
                            {
                                if !complex
                                {
                                    data[0][0].push(n);
                                    data[0][1].push(f);
                                    data[0][2].push(f64::NAN);
                                }
                                data[1][2].push(f64::NAN);
                            }
                        }
                    }
                    else if v.len() == 3
                    {
                        nan = false;
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
                            zero.0 = true;
                        }
                        if (xi * 1e8).round() / 1e8 != 0.0
                            || (yi * 1e8).round() / 1e8 != 0.0
                            || (zi * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true;
                        }
                        data[0][0].push(xr);
                        data[0][1].push(yr);
                        data[0][2].push(zr);
                        data[1][0].push(xi);
                        data[1][1].push(yi);
                        data[1][2].push(zi);
                    }
                    else if v.len() == 2
                    {
                        nan = false;
                        d2 = true;
                        let xr = v[0].number.real().to_f64();
                        let yr = v[1].number.real().to_f64();
                        let xi = v[0].number.imag().to_f64();
                        let yi = v[1].number.imag().to_f64();
                        if (xr * 1e8).round() / 1e8 != 0.0 || (yr * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true;
                        }
                        if (xi * 1e8).round() / 1e8 != 0.0 || (yi * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true;
                        }
                        data[0][0].push(xr);
                        data[0][1].push(yr);
                        data[1][0].push(xi);
                        data[1][1].push(yi);
                    }
                }
                Ok(Matrix(m)) =>
                {
                    for v in m
                    {
                        for num in v
                        {
                            let num = num.number;
                            if num.real().is_nan() || num.imag().is_nan()
                            {
                                continue;
                            }
                            let complex = !num.real().is_infinite();
                            if complex
                            {
                                nan = false;
                                let g = num.real().to_f64();
                                if (g * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.0 = true
                                }
                                data[0][0].push(n);
                                data[0][1].push(f);
                                data[0][2].push(g);
                            }
                            if !num.imag().is_infinite()
                            {
                                nan = false;
                                let g = num.imag().to_f64();
                                if (g * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.1 = true
                                }
                                if !complex
                                {
                                    data[0][0].push(n);
                                    data[0][1].push(f);
                                    data[0][2].push(f64::NAN);
                                }
                                data[1][2].push(g);
                            }
                            else
                            {
                                if !complex
                                {
                                    data[0][0].push(n);
                                    data[0][1].push(f);
                                    data[0][2].push(f64::NAN);
                                }
                                data[1][2].push(f64::NAN);
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
    if !zero.0 && zero.1
    {
        data[0][2] = Vec::new();
    }
    if !zero.1 && zero.0
    {
        data[1][2] = Vec::new();
    }
    if !zero.1 && !zero.0
    {
        data[1][2] = Vec::new();
    }
    if (data[0][0].is_empty() && data[1][0].is_empty()) || nan
    {
        println!("\x1b[G\ngraph is all 0, nan or infinity");
        return Default::default();
    }
    (data, zero, d2)
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
    func: (Vec<NumStr>, Vec<(String, Vec<NumStr>)>, Options),
    input: String,
) -> JoinHandle<(
    (bool, bool),
    (bool, bool),
    bool,
    bool,
    [[Vec<f64>; 2]; 2],
    [[Vec<f64>; 3]; 2],
)>
{
    thread::spawn(move || {
        let mut lines = false;
        let mut points2d: [[Vec<f64>; 2]; 2] = Default::default();
        let mut points3d: [[Vec<f64>; 3]; 2] = Default::default();
        let mut d2_or_d3: (bool, bool) = (false, false);
        let mut re_or_im = (false, false);
        let (has_x, has_y) = (
            func.0.iter().any(|i| i.str_is("x"))
                || func.1.iter().any(|i| i.1.iter().any(|i| i.str_is("x"))),
            func.0.iter().any(|i| i.str_is("y"))
                || func.1.iter().any(|i| i.1.iter().any(|i| i.str_is("y"))),
        );
        if !has_y && !has_x
        {
            match match do_math(func.0.clone(), func.2, func.1)
            {
                Ok(n) => n,
                _ =>
                {
                    fail(func.2, &colors, input);
                    return (
                        (false, false),
                        (false, false),
                        false,
                        true,
                        Default::default(),
                        Default::default(),
                    );
                }
            }
            {
                Num(n) =>
                {
                    let n = n.number;
                    d2_or_d3.0 = true;
                    let change = (func.2.xr.1 - func.2.xr.0) / func.2.samples_2d as f64;
                    let im = n.imag().to_f64();
                    let re = n.real().to_f64();
                    for i in 0..func.2.samples_2d
                    {
                        if re != 0.0 || im == 0.0
                        {
                            points2d[0][0].push(func.2.xr.0 + change * i as f64);
                            points2d[0][1].push(re);
                        }
                        if im != 0.0
                        {
                            if re == 0.0
                            {
                                points2d[1][0].push(func.2.xr.0 + change * i as f64);
                            }
                            points2d[1][1].push(im);
                        }
                    }
                    re_or_im = (re != 0.0, im != 0.0);
                    if points2d[0][1].is_empty() && points2d[1][1].is_empty()
                    {
                        fail(func.2, &colors, input);
                        return (
                            (false, false),
                            (false, false),
                            false,
                            true,
                            Default::default(),
                            Default::default(),
                        );
                    }
                }
                Vector(v) =>
                {
                    lines = true;
                    match v.len()
                    {
                        3 =>
                        {
                            d2_or_d3.1 = true;
                            points3d = [
                                [
                                    vec![0.0, v[0].number.real().to_f64()],
                                    vec![0.0, v[1].number.real().to_f64()],
                                    vec![0.0, v[2].number.real().to_f64()],
                                ],
                                [
                                    vec![0.0, v[0].number.imag().to_f64()],
                                    vec![0.0, v[1].number.imag().to_f64()],
                                    vec![0.0, v[2].number.imag().to_f64()],
                                ],
                            ];
                            re_or_im = (
                                points3d[0].iter().flatten().any(|a| *a != 0.0),
                                points3d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        2 if func.0.iter().any(|c| c.str_is("±")) =>
                        {
                            lines = false;
                            d2_or_d3.0 = true;
                            (points2d, _, _) = get_list_2d((
                                vec![Num(Number::from(v[0].number.clone(), None))],
                                Vec::new(),
                                func.2,
                            ));
                            let points2dtemp: [[Vec<f64>; 2]; 2];
                            (points2dtemp, _, re_or_im) = get_list_2d((
                                vec![Num(Number::from(v[1].number.clone(), None))],
                                Vec::new(),
                                func.2,
                            ));
                            points2d[0][0].extend(points2dtemp[0][0].clone());
                            points2d[0][1].extend(points2dtemp[0][1].clone());
                            points2d[1][0].extend(points2dtemp[1][0].clone());
                            points2d[1][1].extend(points2dtemp[1][1].clone());
                            if points2d[0][1].is_empty() && points2d[1][1].is_empty()
                            {
                                fail(func.2, &colors, input);
                                return (
                                    (false, false),
                                    (false, false),
                                    false,
                                    true,
                                    Default::default(),
                                    Default::default(),
                                );
                            }
                        }
                        2 =>
                        {
                            d2_or_d3.0 = true;
                            points2d = [
                                [
                                    vec![0.0, v[0].number.real().to_f64()],
                                    vec![0.0, v[1].number.real().to_f64()],
                                ],
                                [
                                    vec![0.0, v[0].number.imag().to_f64()],
                                    vec![0.0, v[1].number.imag().to_f64()],
                                ],
                            ];
                            re_or_im = (
                                points2d[0].iter().flatten().any(|a| *a != 0.0),
                                points2d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        _ =>
                        {
                            d2_or_d3.0 = true;
                            let mut vec = Vec::with_capacity(v.len());
                            for i in 1..=v.len()
                            {
                                vec.push(i as f64);
                            }
                            points2d = [
                                [
                                    vec,
                                    v.iter()
                                        .map(|c| c.number.real().to_f64())
                                        .collect::<Vec<f64>>(),
                                ],
                                [
                                    Vec::new(),
                                    if v.iter().any(|c| !c.number.imag().is_zero())
                                    {
                                        v.iter()
                                            .map(|c| c.number.imag().to_f64())
                                            .collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                ],
                            ];
                            re_or_im = (
                                points2d[0].iter().flatten().any(|a| *a != 0.0),
                                points2d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                    }
                }
                Matrix(m) =>
                {
                    lines = true;
                    match m[0].len()
                    {
                        3 =>
                        {
                            d2_or_d3.1 = true;
                            points3d = [
                                [
                                    m.iter()
                                        .map(|c| c[0].number.real().to_f64())
                                        .collect::<Vec<f64>>(),
                                    m.iter()
                                        .map(|c| c[1].number.real().to_f64())
                                        .collect::<Vec<f64>>(),
                                    m.iter()
                                        .map(|c| c[2].number.real().to_f64())
                                        .collect::<Vec<f64>>(),
                                ],
                                [
                                    if m.iter().any(|c| !c[0].number.imag().is_zero())
                                    {
                                        m.iter()
                                            .map(|c| c[0].number.imag().to_f64())
                                            .collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                    if m.iter().any(|c| !c[1].number.imag().is_zero())
                                    {
                                        m.iter()
                                            .map(|c| c[1].number.imag().to_f64())
                                            .collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                    if m.iter().any(|c| !c[2].number.imag().is_zero())
                                    {
                                        m.iter()
                                            .map(|c| c[2].number.imag().to_f64())
                                            .collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                ],
                            ];
                            re_or_im = (
                                points3d[0].iter().flatten().any(|a| *a != 0.0),
                                points3d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        2 =>
                        {
                            d2_or_d3.0 = true;
                            points2d = [
                                [
                                    m.iter()
                                        .map(|c| c[0].number.real().to_f64())
                                        .collect::<Vec<f64>>(),
                                    m.iter()
                                        .map(|c| c[1].number.real().to_f64())
                                        .collect::<Vec<f64>>(),
                                ],
                                [
                                    if m.iter().any(|c| !c[0].number.imag().is_zero())
                                    {
                                        m.iter()
                                            .map(|c| c[0].number.imag().to_f64())
                                            .collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                    if m.iter().any(|c| !c[1].number.imag().is_zero())
                                    {
                                        m.iter()
                                            .map(|c| c[1].number.imag().to_f64())
                                            .collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                ],
                            ];
                            re_or_im = (
                                points2d[0].iter().flatten().any(|a| *a != 0.0),
                                points2d[1].iter().flatten().any(|a| *a != 0.0),
                            );
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
            d2_or_d3.0 = true;
            let data3d;
            (points2d, data3d, re_or_im) = get_list_2d(func.clone());
            if !data3d[0].is_empty() || !data3d[1].is_empty()
            {
                d2_or_d3 = (false, true);
                points3d[0][0].clone_from(&points2d[0][0]);
                points3d[0][1].clone_from(&points2d[0][1]);
                points3d[0][2].clone_from(&data3d[0]);
                points3d[1][0].clone_from(&points2d[1][0]);
                points3d[1][1].clone_from(&points2d[1][1]);
                points3d[1][2].clone_from(&data3d[1]);
                points2d = Default::default();
            }
            else if points2d[0][1].is_empty() && points2d[1][1].is_empty()
            {
                fail(func.2, &colors, input);
                return (
                    (false, false),
                    (false, false),
                    false,
                    true,
                    Default::default(),
                    Default::default(),
                );
            }
            if !has_x
            {
                if re_or_im.1
                {
                    points2d[1][0] = points2d[0][0].clone();
                    points2d[1].swap(0, 1);
                }
                points2d[0].swap(0, 1);
            }
            if func.2.flat
            {
                re_or_im.1 = false;
                points2d[0].swap(0, 1);
                points2d[0][1] = points2d[1][1].clone();
                points2d[1] = Default::default();
            }
            else if func.2.depth
            {
                re_or_im.1 = false;
                d2_or_d3 = (false, true);
                points3d[0][0].clone_from(&points2d[0][0]);
                points3d[0][1].clone_from(&points2d[0][1]);
                points3d[0][2].clone_from(&points2d[1][1]);
                points2d = Default::default();
            }
        }
        else
        {
            d2_or_d3.1 = true;
            let d2;
            (points3d, re_or_im, d2) = get_list_3d(func.clone());
            if d2
            {
                d2_or_d3 = (true, false);
                points2d[0][0].clone_from(&points3d[0][0]);
                points2d[0][1].clone_from(&points3d[0][1]);
                points2d[1][0].clone_from(&points3d[1][0]);
                points2d[1][1].clone_from(&points3d[1][1]);
                points3d = Default::default();
            }
            else if points3d[0][2].is_empty() && points3d[1][2].is_empty()
            {
                fail(func.2, &colors, input);
                return (
                    (false, false),
                    (false, false),
                    false,
                    true,
                    Default::default(),
                    Default::default(),
                );
            }
            if func.2.flat
            {
                re_or_im.1 = false;
                points3d[0][1] = points3d[0][2].clone();
                points3d[0][2] = points3d[1][2].clone();
                points3d[1] = Default::default();
            }
            else if func.2.depth
            {
                re_or_im.1 = false;
                points3d[0][0] = points3d[0][1].clone();
                points3d[0][1] = points3d[0][2].clone();
                points3d[0][2] = points3d[1][2].clone();
                points3d[1] = Default::default();
            }
        }
        (d2_or_d3, re_or_im, lines, false, points2d, points3d)
    })
}
