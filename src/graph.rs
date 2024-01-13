use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    math::do_math,
    misc::prompt,
    Colors, Options,
};
use gnuplot::{AxesCommon, Caption, Color, Figure, Fix, PointSymbol};
use rug::Complex;
use std::{
    io::{stdout, Write},
    thread,
    thread::JoinHandle,
    time::Instant,
};
pub fn graph(
    input: Vec<String>,
    unmod: Vec<String>,
    func: Vec<Vec<NumStr>>,
    mut options: Options,
    watch: Option<Instant>,
    colors: Colors,
) -> JoinHandle<()>
{
    //TODO remove most extra axes2d/3d by just making a var to be the lines stuff
    //TODO hack together polar maybe
    //TODO set amount of ticks
    //see if x=2 and y=2 lines are plausible
    thread::spawn(move || {
        if input.iter().all(|i| i.is_empty())
        {
            return;
        }
        options.prec = options.graph_prec;
        let mut fg = Figure::new();
        fg.set_enhanced_text(false);
        let xticks = Some((Fix((options.xr.1 - options.xr.0) / 20.0), 1));
        let yticks = Some((Fix((options.yr.1 - options.yr.0) / 20.0), 1));
        let mut re_cap: [String; 6] = Default::default();
        let mut im_cap: [String; 6] = Default::default();
        let mut points2d: [[Vec<f64>; 3]; 6] = Default::default();
        let mut points3d: [[Vec<f64>; 4]; 6] = Default::default();
        let mut d2_or_d3 = (false, false);
        for (i, func) in func.iter().enumerate()
        {
            let re_or_im;
            let (has_x, has_y) = (
                func.iter().any(|i| i.str_is("x")),
                func.iter().any(|i| i.str_is("y")),
            );
            if !has_y && !has_x
            {
                match match do_math(func.clone(), options)
                {
                    Ok(n) => n,
                    _ =>
                    {
                        fail(options, &colors);
                        return;
                    }
                }
                {
                    Num(_n) =>
                    {
                        if d2_or_d3.1
                        {
                            (points3d[i], re_or_im) = get_list_3d(func, options);
                            if points3d[i][2].is_empty() && points3d[i][3].is_empty()
                            {
                                fail(options, &colors);
                                return;
                            }
                        }
                        else
                        {
                            d2_or_d3.0 = true;
                            (points2d[i], re_or_im) = get_list_2d(func, options);
                            if points2d[i][1].is_empty() && points2d[i][2].is_empty()
                            {
                                fail(options, &colors);
                                return;
                            }
                        }
                        if re_or_im.0
                        {
                            re_cap[i] = unmod[i].to_owned() + ":re";
                        }
                        if re_or_im.1
                        {
                            im_cap[i] = unmod[i].to_owned() + ":im";
                        }
                    }
                    Vector(v) => match v.len()
                    {
                        3 =>
                        {}
                        2 =>
                        {}
                        _ =>
                        {}
                    },
                    Matrix(m) => match m[0].len()
                    {
                        3 =>
                        {}
                        2 =>
                        {}
                        _ =>
                        {}
                    },
                    _ =>
                    {}
                }
            }
            else if !has_y || !has_x
            {
                d2_or_d3.0 = true;
                (points2d[i], re_or_im) = get_list_2d(func, options);
                if re_or_im.0
                {
                    re_cap[i] = unmod[i].to_owned() + ":re";
                }
                if re_or_im.1
                {
                    im_cap[i] = unmod[i].to_owned() + ":im";
                }
                if points2d[i][1].is_empty() && points2d[i][2].is_empty()
                {
                    fail(options, &colors);
                    return;
                }
                if !has_x
                {
                    points2d[i].swap(0, 1);
                }
            }
            else
            {
                d2_or_d3.1 = true;
                (points3d[i], re_or_im) = get_list_3d(func, options);
                if re_or_im.0
                {
                    re_cap[i] = unmod[i].to_owned() + ":re";
                }
                if re_or_im.1
                {
                    im_cap[i] = unmod[i].to_owned() + ":im";
                }
                if points3d[i][2].is_empty() && points3d[i][3].is_empty()
                {
                    fail(options, &colors);
                    return;
                }
            }
        }
        if d2_or_d3.0 == d2_or_d3.1
        {
            print!(
                "\x1b[G\x1b[Kcant graph 2d and 3d\n\x1b[G{}",
                prompt(options, &colors)
            );
            stdout().flush().unwrap();
            return;
        }
        if d2_or_d3.0
        {
            if options.lines
            {
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_x_label("x", &[])
                    .set_y_label("y", &[])
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
                    .lines(
                        &points2d[0][0],
                        &points2d[0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .lines(
                        &points2d[0][0],
                        &points2d[0][2],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .lines(
                        &points2d[1][0],
                        &points2d[1][1],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .lines(
                        &points2d[1][0],
                        &points2d[1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .lines(
                        &points2d[2][0],
                        &points2d[2][1],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .lines(
                        &points2d[2][0],
                        &points2d[2][2],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .lines(
                        &points2d[3][0],
                        &points2d[3][1],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .lines(
                        &points2d[3][0],
                        &points2d[3][2],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .lines(
                        &points2d[4][0],
                        &points2d[4][1],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .lines(
                        &points2d[4][0],
                        &points2d[4][2],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .lines(
                        &points2d[5][0],
                        &points2d[5][1],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .lines(
                        &points2d[5][0],
                        &points2d[5][2],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
            else
            {
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_x_label("x", &[])
                    .set_y_label("y", &[])
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
                        &points2d[0][0],
                        &points2d[0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .points(
                        &points2d[0][0],
                        &points2d[0][2],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .points(
                        &points2d[1][0],
                        &points2d[1][1],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .points(
                        &points2d[1][0],
                        &points2d[1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .points(
                        &points2d[2][0],
                        &points2d[2][1],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .points(
                        &points2d[2][0],
                        &points2d[2][2],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .points(
                        &points2d[3][0],
                        &points2d[3][1],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .points(
                        &points2d[3][0],
                        &points2d[3][2],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .points(
                        &points2d[4][0],
                        &points2d[4][1],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .points(
                        &points2d[4][0],
                        &points2d[4][2],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .points(
                        &points2d[5][0],
                        &points2d[5][1],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .points(
                        &points2d[5][0],
                        &points2d[5][2],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
        }
        if d2_or_d3.1
        {
            let zticks = Some((Fix((options.zr.1 - options.zr.0) / 20.0), 1));
            fg.axes3d()
                .set_x_ticks(xticks, &[], &[])
                .set_y_ticks(yticks, &[], &[])
                .set_z_ticks(zticks, &[], &[])
                .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                .set_x_label("x", &[])
                .set_y_label("y", &[])
                .set_z_label("z", &[])
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
                    &points3d[0][0],
                    &points3d[0][1],
                    &points3d[0][2],
                    &[PointSymbol(options.point_style), Color(&colors.re1col)],
                )
                .points(
                    &points3d[0][0],
                    &points3d[0][1],
                    &points3d[0][3],
                    &[PointSymbol(options.point_style), Color(&colors.im1col)],
                )
                .points(
                    &points3d[1][0],
                    &points3d[1][1],
                    &points3d[1][2],
                    &[PointSymbol(options.point_style), Color(&colors.re2col)],
                )
                .points(
                    &points3d[1][0],
                    &points3d[1][1],
                    &points3d[1][3],
                    &[PointSymbol(options.point_style), Color(&colors.im2col)],
                )
                .points(
                    &points3d[2][0],
                    &points3d[2][1],
                    &points3d[2][2],
                    &[PointSymbol(options.point_style), Color(&colors.re3col)],
                )
                .points(
                    &points3d[2][0],
                    &points3d[2][1],
                    &points3d[2][3],
                    &[PointSymbol(options.point_style), Color(&colors.im3col)],
                )
                .points(
                    &points3d[3][0],
                    &points3d[3][1],
                    &points3d[3][2],
                    &[PointSymbol(options.point_style), Color(&colors.re4col)],
                )
                .points(
                    &points3d[3][0],
                    &points3d[3][1],
                    &points3d[3][3],
                    &[PointSymbol(options.point_style), Color(&colors.im4col)],
                )
                .points(
                    &points3d[4][0],
                    &points3d[4][1],
                    &points3d[4][2],
                    &[PointSymbol(options.point_style), Color(&colors.re5col)],
                )
                .points(
                    &points3d[4][0],
                    &points3d[4][1],
                    &points3d[4][3],
                    &[PointSymbol(options.point_style), Color(&colors.im5col)],
                )
                .points(
                    &points3d[5][0],
                    &points3d[5][1],
                    &points3d[5][2],
                    &[PointSymbol(options.point_style), Color(&colors.re6col)],
                )
                .points(
                    &points3d[5][0],
                    &points3d[5][1],
                    &points3d[5][3],
                    &[PointSymbol(options.point_style), Color(&colors.im6col)],
                );
        }
        if let Some(time) = watch
        {
            print!("\x1b[G\x1b[K{}ms\n\x1b[G", time.elapsed().as_millis(),);
        }
        if fg.show().is_err()
        {
            print!("\x1b[G\x1b[Kno gnuplot\n\x1b[G{}", prompt(options, &colors));
        }
        stdout().flush().unwrap();
    })
}
pub fn get_list_2d(func: &[NumStr], options: Options) -> ([Vec<f64>; 3], (bool, bool))
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.is_zero()
        {
            return Default::default();
        }
    }
    let mut data: [Vec<f64>; 3] = [
        Vec::with_capacity(options.samples_2d + 1),
        Vec::with_capacity(options.samples_2d + 1),
        Vec::with_capacity(options.samples_2d + 1),
    ];
    let den_range = (options.xr.1 - options.xr.0) / options.samples_2d as f64;
    let mut zero = (false, false);
    for i in 0..=options.samples_2d
    {
        let n = options.xr.0 + i as f64 * den_range;
        let num = Num(Complex::with_val(options.prec, n));
        match do_math(
            func.iter()
                .map(|i| match i
                {
                    Str(s) if s == "x" || s == "y" => num.clone(),
                    _ => i.clone(),
                })
                .collect(),
            options,
        )
        {
            Ok(Num(num)) =>
            {
                let complex = num.real().is_finite();
                if complex
                {
                    let f = num.real().to_f64();
                    if (f * 1e8).round() / 1e8 != 0.0
                    {
                        zero.0 = true
                    }
                    data[0].push(n);
                    data[1].push(f);
                }
                if num.imag().is_finite()
                {
                    let f = num.imag().to_f64();
                    if (f * 1e8).round() / 1e8 != 0.0
                    {
                        zero.1 = true
                    }
                    if !complex
                    {
                        data[0].push(n);
                        data[1].push(f64::INFINITY);
                    }
                    data[2].push(f);
                }
            }
            Ok(Vector(v)) =>
            {
                for num in v
                {
                    let complex = num.real().is_finite();
                    if complex
                    {
                        let f = num.real().to_f64();
                        if (f * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        data[0].push(n);
                        data[1].push(f);
                    }
                    if num.imag().is_finite()
                    {
                        let f = num.imag().to_f64();
                        if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        if !complex
                        {
                            data[0].push(n);
                            data[1].push(f64::INFINITY);
                        }
                        data[2].push(f);
                    }
                }
            }
            Ok(Matrix(m)) =>
            {
                for v in m
                {
                    for num in v
                    {
                        let complex = num.real().is_finite();
                        if complex
                        {
                            let f = num.real().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            data[0].push(n);
                            data[1].push(f);
                        }
                        if num.imag().is_finite()
                        {
                            let f = num.imag().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            if !complex
                            {
                                data[0].push(n);
                                data[1].push(f64::INFINITY);
                            }
                            data[2].push(f);
                        }
                    }
                }
            }
            _ =>
            {}
        }
    }
    if !zero.0
    {
        data[1] = Vec::new();
    }
    if !zero.1
    {
        data[2] = Vec::new();
    }
    (data, zero)
}
pub fn get_list_3d(func: &[NumStr], options: Options) -> ([Vec<f64>; 4], (bool, bool))
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.is_zero()
        {
            return Default::default();
        }
    }
    let mut data: [Vec<f64>; 4] = [
        Vec::with_capacity(options.samples_3d.0 + 1),
        Vec::with_capacity(options.samples_3d.1 + 1),
        Vec::with_capacity((options.samples_3d.0 + 1) * (options.samples_3d.1 + 1)),
        Vec::with_capacity((options.samples_3d.0 + 1) * (options.samples_3d.1 + 1)),
    ];
    let den_x_range = (options.xr.1 - options.xr.0) / options.samples_3d.0 as f64;
    let den_y_range = (options.yr.1 - options.yr.0) / options.samples_3d.1 as f64;
    let mut modified: Vec<NumStr>;
    let mut zero = (false, false);
    for i in 0..=options.samples_3d.0
    {
        let n = options.xr.0 + i as f64 * den_x_range;
        let num = Num(Complex::with_val(options.prec, n));
        modified = func
            .iter()
            .map(|i| match i
            {
                Str(s) if s == "x" => num.clone(),
                _ => i.clone(),
            })
            .collect();
        for g in 0..=options.samples_3d.1
        {
            let f = options.yr.0 + g as f64 * den_y_range;
            let num = Num(Complex::with_val(options.prec, f));
            match do_math(
                modified
                    .iter()
                    .map(|j| match j
                    {
                        Str(s) if s == "y" => num.clone(),
                        _ => j.clone(),
                    })
                    .collect(),
                options,
            )
            {
                Ok(Num(num)) =>
                {
                    let complex = num.real().is_finite();
                    if complex
                    {
                        if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        data[0].push(n);
                        data[1].push(f);
                        data[2].push(num.real().to_f64());
                    }
                    if num.imag().is_finite()
                    {
                        if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        if !complex
                        {
                            data[0].push(n);
                            data[1].push(f);
                            data[2].push(f64::INFINITY);
                        }
                        data[3].push(num.imag().to_f64());
                    }
                }
                Ok(Vector(v)) =>
                {
                    for num in v
                    {
                        let complex = num.real().is_finite();
                        if complex
                        {
                            if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            data[0].push(n);
                            data[1].push(f);
                            data[2].push(num.real().to_f64());
                        }
                        if num.imag().is_finite()
                        {
                            if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            if !complex
                            {
                                data[0].push(n);
                                data[1].push(f);
                                data[2].push(f64::INFINITY);
                            }
                            data[3].push(num.imag().to_f64());
                        }
                    }
                    continue;
                }
                Ok(Matrix(m)) =>
                {
                    for v in m
                    {
                        for num in v
                        {
                            let complex = num.real().is_finite();
                            if complex
                            {
                                if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.0 = true
                                }
                                data[0].push(n);
                                data[1].push(f);
                                data[2].push(num.real().to_f64());
                            }
                            if num.imag().is_finite()
                            {
                                if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.1 = true
                                }
                                if !complex
                                {
                                    data[0].push(n);
                                    data[1].push(f);
                                    data[2].push(f64::INFINITY);
                                }
                                data[3].push(num.imag().to_f64());
                            }
                        }
                    }
                }
                _ =>
                {}
            }
        }
    }
    if !zero.0
    {
        data[2] = Vec::new();
    }
    if !zero.1
    {
        data[3] = Vec::new();
    }
    (data, zero)
}
fn fail(options: Options, colors: &Colors)
{
    print!(
        "\x1b[G\x1b[KNo data to plot\n\x1b[G{}",
        prompt(options, colors)
    );
    stdout().flush().unwrap();
}