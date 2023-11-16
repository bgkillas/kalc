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
    thread::spawn(move || {
        if input.iter().all(|i| i.is_empty())
        {
            return;
        }
        let mut fg = Figure::new();
        fg.set_enhanced_text(false);
        let re1col = "#ff5555";
        let re2col = "#55ff55";
        let re3col = "#ffff55";
        let re4col = "#5555ff";
        let re5col = "#ff55ff";
        let re6col = "#55ffff";
        let im1col = "#aa0000";
        let im2col = "#00aa00";
        let im3col = "#aaaa00";
        let im4col = "#0000aa";
        let im5col = "#aa00aa";
        let im6col = "#00aaaa";
        let xticks = Some((Fix((options.xr.1 - options.xr.0) / 20.0), 1));
        let yticks = Some((Fix((options.yr.1 - options.yr.0) / 20.0), 1));
        let zticks = Some((Fix((options.zr.1 - options.zr.0) / 20.0), 1));
        let mut re_cap: [String; 6] = Default::default();
        let mut im_cap: [String; 6] = Default::default();
        if !input.iter().all(|i| i.contains('x'))
        {
            let mut re = Vec::new();
            let mut matrix = false;
            let mut x = (Vec::new(), Vec::new());
            let mut y = (Vec::new(), Vec::new());
            let mut z = (Vec::new(), Vec::new());
            for (i, f) in func.iter().enumerate()
            {
                re.push(
                    match match do_math(f.to_vec(), options)
                    {
                        Ok(n) => n,
                        _ =>
                        {
                            fail(options, &colors);
                            return;
                        }
                    }
                    {
                        Vector(n) =>
                        {
                            for j in &n
                            {
                                if !j.real().is_zero()
                                {
                                    re_cap[i] = unmod[i].to_owned() + ":re";
                                }
                                if !j.imag().is_zero()
                                {
                                    im_cap[i] = unmod[i].to_owned() + ":im";
                                }
                            }
                            n
                        }
                        Matrix(n) =>
                        {
                            if n[0].len() <= 3 && n[0].len() > 1
                            {
                                matrix = true;
                                x.0.push(
                                    n.iter().map(|x| x[0].real().to_f64()).collect::<Vec<f64>>(),
                                );
                                x.1.push(
                                    n.iter().map(|x| x[0].imag().to_f64()).collect::<Vec<f64>>(),
                                );
                                y.0.push(
                                    n.iter().map(|x| x[1].real().to_f64()).collect::<Vec<f64>>(),
                                );
                                y.1.push(
                                    n.iter().map(|x| x[1].imag().to_f64()).collect::<Vec<f64>>(),
                                );
                                if n[0].len() == 3
                                {
                                    z.0.push(
                                        n.iter()
                                            .map(|x| x[2].real().to_f64())
                                            .collect::<Vec<f64>>(),
                                    );
                                    z.1.push(
                                        n.iter()
                                            .map(|x| x[2].imag().to_f64())
                                            .collect::<Vec<f64>>(),
                                    );
                                }
                            }
                            for k in &n
                            {
                                for j in k
                                {
                                    if !j.real().is_zero()
                                    {
                                        re_cap[i] = unmod[i].to_owned() + ":re";
                                    }
                                    if !j.imag().is_zero()
                                    {
                                        im_cap[i] = unmod[i].to_owned() + ":im";
                                    }
                                }
                            }
                            if n[0].len() <= 3 && n[0].len() > 1
                            {
                                continue;
                            }
                            matrix = false;
                            n.into_iter().flatten().collect()
                        }
                        Num(n) =>
                        {
                            if !n.real().is_zero()
                            {
                                re_cap[i] = unmod[i].to_owned() + ":re";
                            }
                            if !n.imag().is_zero()
                            {
                                im_cap[i] = unmod[i].to_owned() + ":im";
                            }
                            vec![
                                Complex::with_val(options.prec, n.real()),
                                Complex::with_val(options.prec, n.imag()),
                            ]
                        }
                        _ =>
                        {
                            fail(options, &colors);
                            return;
                        }
                    },
                );
            }
            if matrix
            {
                if !z.0.is_empty()
                {
                    if Options::default().yr == options.yr
                    {
                        options.yr = (
                            y.0.iter().zip(y.1.clone()).fold(f64::MAX, |min, x| {
                                min.min(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MAX, |min, x| min.min(*x.0).min(x.1)),
                                )
                            }),
                            y.0.iter().zip(y.1.clone()).fold(f64::MIN, |max, x| {
                                max.max(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MIN, |max, x| max.max(*x.0).max(x.1)),
                                )
                            }),
                        )
                    }
                    if Options::default().xr == options.xr
                    {
                        options.xr = (
                            x.0.iter().zip(x.1.clone()).fold(f64::MAX, |min, x| {
                                min.min(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MAX, |min, x| min.min(*x.0).min(x.1)),
                                )
                            }),
                            x.0.iter().zip(x.1.clone()).fold(f64::MIN, |max, x| {
                                max.max(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MIN, |max, x| max.max(*x.0).max(x.1)),
                                )
                            }),
                        )
                    }
                    if Options::default().zr == options.zr
                    {
                        options.zr = (
                            z.0.iter().zip(z.1.clone()).fold(f64::MAX, |min, z| {
                                min.min(
                                    z.0.iter()
                                        .zip(z.1)
                                        .fold(f64::MAX, |min, z| min.min(*z.0).min(z.1)),
                                )
                            }),
                            z.0.iter().zip(z.1.clone()).fold(f64::MIN, |max, z| {
                                max.max(
                                    z.0.iter()
                                        .zip(z.1)
                                        .fold(f64::MIN, |max, z| max.max(*z.0).max(z.1)),
                                )
                            }),
                        )
                    }
                    let xticks = Some((Fix((options.xr.1 - options.xr.0) / 20.0), 1));
                    let yticks = Some((Fix((options.yr.1 - options.yr.0) / 20.0), 1));
                    let zticks = Some((Fix((options.zr.1 - options.zr.0) / 20.0), 1));
                    let n = vec![0.0; 3];
                    for _ in 0..6 - func.len()
                    {
                        x.0.push(n.clone());
                        y.0.push(n.clone());
                        z.0.push(n.clone());
                        x.1.push(n.clone());
                        y.1.push(n.clone());
                        z.1.push(n.clone());
                    }
                    fg.axes3d()
                        .set_x_ticks(xticks, &[], &[])
                        .set_y_ticks(yticks, &[], &[])
                        .set_z_ticks(zticks, &[], &[])
                        .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                        .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                        .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                        .set_z_label("z", &[])
                        .set_y_label("y", &[])
                        .set_x_label("x", &[])
                        .lines_points(
                            x.0[0].clone(),
                            y.0[0].clone(),
                            z.0[0].clone(),
                            &[
                                Caption(&re_cap[0]),
                                Color(re1col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[1].clone(),
                            y.0[1].clone(),
                            z.0[1].clone(),
                            &[
                                Caption(&re_cap[1]),
                                Color(re2col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[2].clone(),
                            y.0[2].clone(),
                            z.0[2].clone(),
                            &[Caption(&re_cap[2]), Color(re3col)],
                        )
                        .lines_points(
                            x.0[3].clone(),
                            y.0[3].clone(),
                            z.0[3].clone(),
                            &[
                                Caption(&re_cap[3]),
                                Color(re4col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[4].clone(),
                            y.0[4].clone(),
                            z.0[4].clone(),
                            &[
                                Caption(&re_cap[4]),
                                Color(re5col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[5].clone(),
                            y.0[5].clone(),
                            z.0[5].clone(),
                            &[
                                Caption(&re_cap[5]),
                                Color(re6col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[0].clone(),
                            y.1[0].clone(),
                            z.1[0].clone(),
                            &[
                                Caption(&im_cap[0]),
                                Color(im1col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[1].clone(),
                            y.1[1].clone(),
                            z.1[1].clone(),
                            &[
                                Caption(&im_cap[1]),
                                Color(im2col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[2].clone(),
                            y.1[2].clone(),
                            z.1[2].clone(),
                            &[
                                Caption(&im_cap[2]),
                                Color(im3col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[3].clone(),
                            y.1[3].clone(),
                            z.1[3].clone(),
                            &[
                                Caption(&im_cap[3]),
                                Color(im4col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[4].clone(),
                            y.1[4].clone(),
                            z.1[4].clone(),
                            &[
                                Caption(&im_cap[4]),
                                Color(im5col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[5].clone(),
                            y.1[5].clone(),
                            z.1[5].clone(),
                            &[
                                Caption(&im_cap[5]),
                                Color(im6col),
                                PointSymbol(options.point_style),
                            ],
                        );
                }
                else
                {
                    if Options::default().yr == options.yr
                    {
                        options.yr = (
                            y.0.iter().zip(y.1.clone()).fold(f64::MAX, |min, x| {
                                min.min(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MAX, |min, x| min.min(*x.0).min(x.1)),
                                )
                            }),
                            y.0.iter().zip(y.1.clone()).fold(f64::MIN, |max, x| {
                                max.max(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MIN, |max, x| max.max(*x.0).max(x.1)),
                                )
                            }),
                        )
                    }
                    if Options::default().xr == options.xr
                    {
                        options.xr = (
                            x.0.iter().zip(x.1.clone()).fold(f64::MAX, |min, x| {
                                min.min(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MAX, |min, x| min.min(*x.0).min(x.1)),
                                )
                            }),
                            x.0.iter().zip(x.1.clone()).fold(f64::MIN, |max, x| {
                                max.max(
                                    x.0.iter()
                                        .zip(x.1)
                                        .fold(f64::MIN, |max, x| max.max(*x.0).max(x.1)),
                                )
                            }),
                        )
                    }
                    let xticks = Some((Fix((options.xr.1 - options.xr.0) / 20.0), 1));
                    let yticks = Some((Fix((options.yr.1 - options.yr.0) / 20.0), 1));
                    let z = vec![0.0; 2];
                    for _ in 0..6 - func.len()
                    {
                        x.0.push(z.clone());
                        y.0.push(z.clone());
                        x.1.push(z.clone());
                        y.1.push(z.clone());
                    }
                    fg.axes2d()
                        .set_x_ticks(xticks, &[], &[])
                        .set_y_ticks(yticks, &[], &[])
                        .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                        .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                        .lines_points(
                            x.0[0].clone(),
                            y.0[0].clone(),
                            &[
                                Caption(&re_cap[0]),
                                Color(re1col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[1].clone(),
                            y.0[1].clone(),
                            &[
                                Caption(&re_cap[1]),
                                Color(re2col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[2].clone(),
                            y.0[2].clone(),
                            &[
                                Caption(&re_cap[2]),
                                Color(re3col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[3].clone(),
                            y.0[3].clone(),
                            &[
                                Caption(&re_cap[3]),
                                Color(re4col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[4].clone(),
                            y.0[4].clone(),
                            &[
                                Caption(&re_cap[4]),
                                Color(re5col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.0[5].clone(),
                            y.0[5].clone(),
                            &[
                                Caption(&re_cap[5]),
                                Color(re6col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[0].clone(),
                            y.1[0].clone(),
                            &[
                                Caption(&im_cap[0]),
                                Color(im1col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[1].clone(),
                            y.1[1].clone(),
                            &[
                                Caption(&im_cap[1]),
                                Color(im2col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[2].clone(),
                            y.1[2].clone(),
                            &[
                                Caption(&im_cap[2]),
                                Color(im3col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[3].clone(),
                            y.1[3].clone(),
                            &[
                                Caption(&im_cap[3]),
                                Color(im4col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[4].clone(),
                            y.1[4].clone(),
                            &[
                                Caption(&im_cap[4]),
                                Color(im5col),
                                PointSymbol(options.point_style),
                            ],
                        )
                        .lines_points(
                            x.1[5].clone(),
                            y.1[5].clone(),
                            &[
                                Caption(&im_cap[5]),
                                Color(im6col),
                                PointSymbol(options.point_style),
                            ],
                        );
                }
            }
            else if re.iter().all(|re| re.len() == 2)
            {
                let z = vec![Complex::new(options.prec); 2];
                for _ in 0..6 - func.len()
                {
                    re.push(z.clone());
                }
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .lines_points(
                        [0.0, re[0][0].real().to_f64()],
                        [0.0, re[0][1].real().to_f64()],
                        &[
                            Caption(&re_cap[0]),
                            Color(re1col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[1][0].real().to_f64()],
                        [0.0, re[1][1].real().to_f64()],
                        &[
                            Caption(&re_cap[1]),
                            Color(re2col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[2][0].real().to_f64()],
                        [0.0, re[2][1].real().to_f64()],
                        &[
                            Caption(&re_cap[2]),
                            Color(re3col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[3][0].real().to_f64()],
                        [0.0, re[3][1].real().to_f64()],
                        &[
                            Caption(&re_cap[3]),
                            Color(re4col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[4][0].real().to_f64()],
                        [0.0, re[4][1].real().to_f64()],
                        &[
                            Caption(&re_cap[4]),
                            Color(re5col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[5][0].real().to_f64()],
                        [0.0, re[5][1].real().to_f64()],
                        &[
                            Caption(&re_cap[5]),
                            Color(re6col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[0][0].imag().to_f64()],
                        [0.0, re[0][1].imag().to_f64()],
                        &[
                            Caption(&im_cap[0]),
                            Color(im1col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[1][0].imag().to_f64()],
                        [0.0, re[1][1].imag().to_f64()],
                        &[
                            Caption(&im_cap[1]),
                            Color(im2col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[2][0].imag().to_f64()],
                        [0.0, re[2][1].imag().to_f64()],
                        &[
                            Caption(&im_cap[2]),
                            Color(im3col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[3][0].imag().to_f64()],
                        [0.0, re[3][1].imag().to_f64()],
                        &[
                            Caption(&im_cap[3]),
                            Color(im4col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[4][0].imag().to_f64()],
                        [0.0, re[4][1].imag().to_f64()],
                        &[
                            Caption(&im_cap[4]),
                            Color(im5col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[5][0].imag().to_f64()],
                        [0.0, re[5][1].imag().to_f64()],
                        &[
                            Caption(&im_cap[5]),
                            Color(im6col),
                            PointSymbol(options.point_style),
                        ],
                    );
            }
            else if re.iter().all(|re| re.len() == 3)
            {
                let z = vec![Complex::new(options.prec); 3];
                for _ in 0..6 - func.len()
                {
                    re.push(z.clone());
                }
                fg.axes3d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_z_ticks(zticks, &[], &[])
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_z_label("z", &[])
                    .set_y_label("y", &[])
                    .set_x_label("x", &[])
                    .lines_points(
                        [0.0, re[0][0].real().to_f64()],
                        [0.0, re[0][1].real().to_f64()],
                        [0.0, re[0][2].real().to_f64()],
                        &[
                            Caption(&re_cap[0]),
                            Color(re1col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[1][0].real().to_f64()],
                        [0.0, re[1][1].real().to_f64()],
                        [0.0, re[1][2].real().to_f64()],
                        &[
                            Caption(&re_cap[1]),
                            Color(re2col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[2][0].real().to_f64()],
                        [0.0, re[2][1].real().to_f64()],
                        [0.0, re[2][2].real().to_f64()],
                        &[
                            Caption(&re_cap[2]),
                            Color(re3col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[3][0].real().to_f64()],
                        [0.0, re[3][1].real().to_f64()],
                        [0.0, re[3][2].real().to_f64()],
                        &[
                            Caption(&re_cap[3]),
                            Color(re4col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[4][0].real().to_f64()],
                        [0.0, re[4][1].real().to_f64()],
                        [0.0, re[4][2].real().to_f64()],
                        &[
                            Caption(&re_cap[4]),
                            Color(re5col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[5][0].real().to_f64()],
                        [0.0, re[5][1].real().to_f64()],
                        [0.0, re[5][2].real().to_f64()],
                        &[
                            Caption(&re_cap[5]),
                            Color(re6col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[0][0].imag().to_f64()],
                        [0.0, re[0][1].imag().to_f64()],
                        [0.0, re[0][2].imag().to_f64()],
                        &[
                            Caption(&im_cap[0]),
                            Color(im1col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[1][0].imag().to_f64()],
                        [0.0, re[1][1].imag().to_f64()],
                        [0.0, re[1][2].imag().to_f64()],
                        &[
                            Caption(&im_cap[1]),
                            Color(im2col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[2][0].imag().to_f64()],
                        [0.0, re[2][1].imag().to_f64()],
                        [0.0, re[2][2].imag().to_f64()],
                        &[
                            Caption(&im_cap[2]),
                            Color(im3col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[3][0].imag().to_f64()],
                        [0.0, re[3][1].imag().to_f64()],
                        [0.0, re[3][2].imag().to_f64()],
                        &[
                            Caption(&im_cap[3]),
                            Color(im4col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[4][0].imag().to_f64()],
                        [0.0, re[4][1].imag().to_f64()],
                        [0.0, re[4][2].imag().to_f64()],
                        &[
                            Caption(&im_cap[4]),
                            Color(im5col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        [0.0, re[5][0].imag().to_f64()],
                        [0.0, re[5][1].imag().to_f64()],
                        [0.0, re[5][2].imag().to_f64()],
                        &[
                            Caption(&im_cap[5]),
                            Color(im6col),
                            PointSymbol(options.point_style),
                        ],
                    );
            }
            else
            {
                for _ in 0..6 - func.len()
                {
                    re.push(Vec::new());
                }
                if Options::default().yr == options.yr
                {
                    options.yr = (
                        re.iter().fold(f64::MAX, |min, x| {
                            min.min(x.iter().fold(f64::MAX, |min, x| {
                                min.min(x.real().to_f64()).min(x.imag().to_f64())
                            }))
                        }),
                        re.iter().fold(f64::MIN, |max, x| {
                            max.max(x.iter().fold(f64::MIN, |max, x| {
                                max.max(x.real().to_f64()).max(x.imag().to_f64())
                            }))
                        }),
                    )
                }
                if Options::default().xr == options.xr
                {
                    options.xr = (
                        0.0,
                        (re.iter().map(|re| re.len()).max().unwrap() - 1) as f64,
                    )
                }
                let xticks = Some((Fix((options.xr.1 - options.xr.0) / 20.0), 1));
                let yticks = Some((Fix((options.yr.1 - options.yr.0) / 20.0), 1));
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .lines_points(
                        0..re[0].len(),
                        re[0].iter().map(|x| x.real().to_f64()),
                        &[
                            Caption(&re_cap[0]),
                            Color(re1col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        0..re[1].len(),
                        re[1].iter().map(|x| x.real().to_f64()),
                        &[
                            Caption(&re_cap[1]),
                            Color(re2col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        0..re[2].len(),
                        re[2].iter().map(|x| x.real().to_f64()),
                        &[
                            Caption(&re_cap[2]),
                            Color(re3col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        0..re[3].len(),
                        re[3].iter().map(|x| x.real().to_f64()),
                        &[
                            Caption(&re_cap[3]),
                            Color(re4col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        0..re[4].len(),
                        re[4].iter().map(|x| x.real().to_f64()),
                        &[
                            Caption(&re_cap[4]),
                            Color(re5col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        0..re[5].len(),
                        re[5].iter().map(|x| x.real().to_f64()),
                        &[
                            Caption(&re_cap[5]),
                            Color(re6col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        if !im_cap[0].is_empty()
                        {
                            0..re[0].len()
                        }
                        else
                        {
                            0..0
                        },
                        re[0].iter().map(|x| x.imag().to_f64()),
                        &[
                            Caption(&im_cap[0]),
                            Color(im1col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        if !im_cap[1].is_empty()
                        {
                            0..re[1].len()
                        }
                        else
                        {
                            0..0
                        },
                        re[1].iter().map(|x| x.imag().to_f64()),
                        &[
                            Caption(&im_cap[1]),
                            Color(im2col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        if !im_cap[2].is_empty()
                        {
                            0..re[2].len()
                        }
                        else
                        {
                            0..0
                        },
                        re[2].iter().map(|x| x.imag().to_f64()),
                        &[
                            Caption(&im_cap[2]),
                            Color(im3col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        if !im_cap[3].is_empty()
                        {
                            0..re[3].len()
                        }
                        else
                        {
                            0..0
                        },
                        re[3].iter().map(|x| x.imag().to_f64()),
                        &[
                            Caption(&im_cap[3]),
                            Color(im4col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        if !im_cap[4].is_empty()
                        {
                            0..re[4].len()
                        }
                        else
                        {
                            0..0
                        },
                        re[4].iter().map(|x| x.imag().to_f64()),
                        &[
                            Caption(&im_cap[4]),
                            Color(im5col),
                            PointSymbol(options.point_style),
                        ],
                    )
                    .lines_points(
                        if !im_cap[5].is_empty()
                        {
                            0..re[5].len()
                        }
                        else
                        {
                            0..0
                        },
                        re[5].iter().map(|x| x.imag().to_f64()),
                        &[
                            Caption(&im_cap[5]),
                            Color(im6col),
                            PointSymbol(options.point_style),
                        ],
                    );
            }
        }
        else if input.iter().any(|i| i.contains('y'))
        {
            let mut re = vec![Vec::new(); 6];
            let mut im = vec![Vec::new(); 6];
            for (i, f) in func.iter().enumerate()
            {
                let (re2, im2) = get_list_3d(f, options);
                if !re2.is_empty()
                {
                    re_cap[i] = unmod[i].to_owned() + ":re";
                }
                if !im2.is_empty()
                {
                    im_cap[i] = unmod[i].to_owned() + ":im";
                }
                re[i] = re2;
                im[i] = im2;
            }
            if re.iter().all(|x| x.is_empty()) && im.iter().all(|x| x.is_empty())
            {
                fail(options, &colors);
                return;
            }
            if options.depth
            {
                fg.axes3d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_z_ticks(zticks, &[], &[])
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_z_label("z", &[])
                    .set_y_label("y", &[])
                    .set_x_label("x", &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(re1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(im1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(re2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(im2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(re3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(im3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(re4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(im4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(re5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(im5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(re6col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(im6col)])
                    .points(
                        re[0].iter().map(|x| x[0]),
                        re[0].iter().map(|x| x[2]),
                        im[0].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(re1col)],
                    )
                    .points(
                        re[0].iter().map(|x| x[1]),
                        re[0].iter().map(|x| x[2]),
                        im[0].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(im1col)],
                    )
                    .points(
                        re[1].iter().map(|x| x[0]),
                        re[1].iter().map(|x| x[2]),
                        im[1].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(re2col)],
                    )
                    .points(
                        re[1].iter().map(|x| x[1]),
                        re[1].iter().map(|x| x[2]),
                        im[1].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(im2col)],
                    )
                    .points(
                        re[2].iter().map(|x| x[0]),
                        re[2].iter().map(|x| x[2]),
                        im[2].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(re3col)],
                    )
                    .points(
                        re[2].iter().map(|x| x[1]),
                        re[2].iter().map(|x| x[2]),
                        im[2].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(im3col)],
                    )
                    .points(
                        re[3].iter().map(|x| x[0]),
                        re[3].iter().map(|x| x[2]),
                        im[3].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(re4col)],
                    )
                    .points(
                        re[3].iter().map(|x| x[1]),
                        re[3].iter().map(|x| x[2]),
                        im[3].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(im4col)],
                    )
                    .points(
                        re[4].iter().map(|x| x[0]),
                        re[4].iter().map(|x| x[2]),
                        im[4].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(re5col)],
                    )
                    .points(
                        re[4].iter().map(|x| x[1]),
                        re[4].iter().map(|x| x[2]),
                        im[4].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(im5col)],
                    )
                    .points(
                        re[5].iter().map(|x| x[0]),
                        re[5].iter().map(|x| x[2]),
                        im[5].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(re6col)],
                    )
                    .points(
                        re[5].iter().map(|x| x[1]),
                        re[5].iter().map(|x| x[2]),
                        im[5].iter().map(|x| x[2]),
                        &[PointSymbol(options.point_style), Color(im6col)],
                    );
            }
            else
            {
                fg.axes3d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_z_ticks(zticks, &[], &[])
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_z_label("z", &[])
                    .set_y_label("y", &[])
                    .set_x_label("x", &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(re1col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[0]), Color(im1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(re2col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[1]), Color(im2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(re3col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[2]), Color(im3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(re4col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[3]), Color(im4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(re5col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[4]), Color(im5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(re6col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[5]), Color(im6col)])
                    .points(
                        re[0].iter().map(|i| i[0]),
                        re[0].iter().map(|i| i[1]),
                        re[0].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(re1col)],
                    )
                    .points(
                        im[0].iter().map(|i| i[0]),
                        im[0].iter().map(|i| i[1]),
                        im[0].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(im1col)],
                    )
                    .points(
                        re[1].iter().map(|i| i[0]),
                        re[1].iter().map(|i| i[1]),
                        re[1].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(re2col)],
                    )
                    .points(
                        im[1].iter().map(|i| i[0]),
                        im[1].iter().map(|i| i[1]),
                        im[1].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(im2col)],
                    )
                    .points(
                        re[2].iter().map(|i| i[0]),
                        re[2].iter().map(|i| i[1]),
                        re[2].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(re3col)],
                    )
                    .points(
                        im[2].iter().map(|i| i[0]),
                        im[2].iter().map(|i| i[1]),
                        im[2].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(im3col)],
                    )
                    .points(
                        re[3].iter().map(|i| i[0]),
                        re[3].iter().map(|i| i[1]),
                        re[3].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(re4col)],
                    )
                    .points(
                        im[3].iter().map(|i| i[0]),
                        im[3].iter().map(|i| i[1]),
                        im[3].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(im4col)],
                    )
                    .points(
                        re[4].iter().map(|i| i[0]),
                        re[4].iter().map(|i| i[1]),
                        re[4].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(re5col)],
                    )
                    .points(
                        im[4].iter().map(|i| i[0]),
                        im[4].iter().map(|i| i[1]),
                        im[4].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(im5col)],
                    )
                    .points(
                        re[5].iter().map(|i| i[0]),
                        re[5].iter().map(|i| i[1]),
                        re[5].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(re6col)],
                    )
                    .points(
                        im[5].iter().map(|i| i[0]),
                        im[5].iter().map(|i| i[1]),
                        im[5].iter().map(|i| i[2]),
                        &[PointSymbol(options.point_style), Color(im6col)],
                    );
            }
        }
        else
        {
            let mut re = vec![Vec::new(); 6];
            let mut im = vec![Vec::new(); 6];
            for (i, f) in func.iter().enumerate()
            {
                let (re2, im2) = get_list_2d(f, options);
                if !re2.is_empty()
                {
                    re_cap[i] = unmod[i].to_owned() + ":re";
                }
                if !im2.is_empty()
                {
                    im_cap[i] = unmod[i].to_owned() + ":im";
                }
                re[i] = re2;
                im[i] = im2;
            }
            if re.iter().all(|x| x.is_empty()) && im.iter().all(|x| x.is_empty())
            {
                fail(options, &colors);
                return;
            }
            if options.lines
            {
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .lines([0], [0], &[Caption(&re_cap[0]), Color(re1col)])
                    .lines([0], [0], &[Caption(&im_cap[0]), Color(im1col)])
                    .lines([0], [0], &[Caption(&re_cap[1]), Color(re2col)])
                    .lines([0], [0], &[Caption(&im_cap[1]), Color(im2col)])
                    .lines([0], [0], &[Caption(&re_cap[2]), Color(re3col)])
                    .lines([0], [0], &[Caption(&im_cap[2]), Color(im3col)])
                    .lines([0], [0], &[Caption(&re_cap[3]), Color(re4col)])
                    .lines([0], [0], &[Caption(&im_cap[3]), Color(im4col)])
                    .lines([0], [0], &[Caption(&re_cap[4]), Color(re5col)])
                    .lines([0], [0], &[Caption(&im_cap[4]), Color(im5col)])
                    .lines([0], [0], &[Caption(&re_cap[5]), Color(re6col)])
                    .lines([0], [0], &[Caption(&im_cap[5]), Color(im6col)])
                    .lines(
                        re[0].iter().map(|x| x[0]),
                        re[0].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re1col)],
                    )
                    .lines(
                        im[0].iter().map(|x| x[0]),
                        im[0].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im1col)],
                    )
                    .lines(
                        re[1].iter().map(|x| x[0]),
                        re[1].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re2col)],
                    )
                    .lines(
                        im[1].iter().map(|x| x[0]),
                        im[1].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im2col)],
                    )
                    .lines(
                        re[2].iter().map(|x| x[0]),
                        re[2].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re3col)],
                    )
                    .lines(
                        im[2].iter().map(|x| x[0]),
                        im[2].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im3col)],
                    )
                    .lines(
                        re[3].iter().map(|x| x[0]),
                        re[3].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re4col)],
                    )
                    .lines(
                        im[3].iter().map(|x| x[0]),
                        im[3].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im4col)],
                    )
                    .lines(
                        re[4].iter().map(|x| x[0]),
                        re[4].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re5col)],
                    )
                    .lines(
                        im[4].iter().map(|x| x[0]),
                        im[4].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im5col)],
                    )
                    .lines(
                        re[5].iter().map(|x| x[0]),
                        re[5].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re6col)],
                    )
                    .lines(
                        im[5].iter().map(|x| x[0]),
                        im[5].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im6col)],
                    );
            }
            else if options.depth
            {
                fg.axes3d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_z_ticks(zticks, &[], &[])
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_z_label("z", &[])
                    .set_y_label("y", &[])
                    .set_x_label("x", &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(re1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(re2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(re3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(re4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(re5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(re6col)])
                    .points(
                        re[0].iter().map(|x| x[0]),
                        re[0].iter().map(|x| x[1]),
                        im[0].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re1col)],
                    )
                    .points(
                        re[1].iter().map(|x| x[0]),
                        re[1].iter().map(|x| x[1]),
                        im[1].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re2col)],
                    )
                    .points(
                        re[2].iter().map(|x| x[0]),
                        re[2].iter().map(|x| x[1]),
                        im[2].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re3col)],
                    )
                    .points(
                        re[3].iter().map(|x| x[0]),
                        re[3].iter().map(|x| x[1]),
                        im[3].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re4col)],
                    )
                    .points(
                        re[4].iter().map(|x| x[0]),
                        re[4].iter().map(|x| x[1]),
                        im[4].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re5col)],
                    )
                    .points(
                        re[5].iter().map(|x| x[0]),
                        re[5].iter().map(|x| x[1]),
                        im[5].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re6col)],
                    );
            }
            else
            {
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .lines([0], [0], &[Caption(&re_cap[0]), Color(re1col)])
                    .lines([0], [0], &[Caption(&im_cap[0]), Color(im1col)])
                    .lines([0], [0], &[Caption(&re_cap[1]), Color(re2col)])
                    .lines([0], [0], &[Caption(&im_cap[1]), Color(im2col)])
                    .lines([0], [0], &[Caption(&re_cap[2]), Color(re3col)])
                    .lines([0], [0], &[Caption(&im_cap[2]), Color(im3col)])
                    .lines([0], [0], &[Caption(&re_cap[3]), Color(re4col)])
                    .lines([0], [0], &[Caption(&im_cap[3]), Color(im4col)])
                    .lines([0], [0], &[Caption(&re_cap[4]), Color(re5col)])
                    .lines([0], [0], &[Caption(&im_cap[4]), Color(im5col)])
                    .lines([0], [0], &[Caption(&re_cap[5]), Color(re6col)])
                    .lines([0], [0], &[Caption(&im_cap[5]), Color(im6col)])
                    .points(
                        re[0].iter().map(|x| x[0]),
                        re[0].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re1col)],
                    )
                    .points(
                        im[0].iter().map(|x| x[0]),
                        im[0].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im1col)],
                    )
                    .points(
                        re[1].iter().map(|x| x[0]),
                        re[1].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re2col)],
                    )
                    .points(
                        im[1].iter().map(|x| x[0]),
                        im[1].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im2col)],
                    )
                    .points(
                        re[2].iter().map(|x| x[0]),
                        re[2].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re3col)],
                    )
                    .points(
                        im[2].iter().map(|x| x[0]),
                        im[2].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im3col)],
                    )
                    .points(
                        re[3].iter().map(|x| x[0]),
                        re[3].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re4col)],
                    )
                    .points(
                        im[3].iter().map(|x| x[0]),
                        im[3].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im4col)],
                    )
                    .points(
                        re[4].iter().map(|x| x[0]),
                        re[4].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re5col)],
                    )
                    .points(
                        im[4].iter().map(|x| x[0]),
                        im[4].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im5col)],
                    )
                    .points(
                        re[5].iter().map(|x| x[0]),
                        re[5].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(re6col)],
                    )
                    .points(
                        im[5].iter().map(|x| x[0]),
                        im[5].iter().map(|x| x[1]),
                        &[PointSymbol(options.point_style), Color(im6col)],
                    );
            }
        }
        if let Some(time) = watch
        {
            print!(
                "\x1b[G\x1b[K{}ms\n\x1b[G{}",
                time.elapsed().as_millis(),
                prompt(options, &colors)
            );
        }
        if fg.show().is_err()
        {
            print!("\x1b[G\x1b[Kno gnuplot\n\x1b[G{}", prompt(options, &colors));
        }
        stdout().flush().unwrap();
    })
}
pub fn get_list_2d(func: &[NumStr], options: Options) -> (Vec<[f64; 2]>, Vec<[f64; 2]>)
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.is_zero()
        {
            return (Vec::new(), Vec::new());
        }
    }
    let mut re = Vec::new();
    let mut im = Vec::new();
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
                    Str(s) if s == "x" => num.clone(),
                    _ => i.clone(),
                })
                .collect(),
            options,
        )
        {
            Ok(Num(num)) =>
            {
                if num.real().is_finite()
                {
                    if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                    {
                        zero.0 = true
                    }
                    re.push([n, num.real().to_f64()]);
                }
                if num.imag().is_finite()
                {
                    if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                    {
                        zero.1 = true
                    }
                    im.push([n, num.imag().to_f64()]);
                }
            }
            Ok(Vector(v)) =>
            {
                for num in v
                {
                    if num.real().is_finite()
                    {
                        if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        re.push([n, num.real().to_f64()]);
                    }
                    if num.imag().is_finite()
                    {
                        if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        im.push([n, num.imag().to_f64()]);
                    }
                }
            }
            Ok(Matrix(m)) =>
            {
                for v in m
                {
                    for num in v
                    {
                        if num.real().is_finite()
                        {
                            if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            re.push([n, num.real().to_f64()]);
                        }
                        if num.imag().is_finite()
                        {
                            if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            im.push([n, num.imag().to_f64()]);
                        }
                    }
                }
            }
            _ =>
            {}
        }
    }
    (
        if zero.0 { re } else { Vec::new() },
        if zero.1 { im } else { Vec::new() },
    )
}
pub fn get_list_3d(func: &[NumStr], options: Options) -> (Vec<[f64; 3]>, Vec<[f64; 3]>)
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.is_zero()
        {
            return (Vec::new(), Vec::new());
        }
    }
    let mut re = Vec::new();
    let mut im = Vec::new();
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
                    if num.real().is_finite()
                    {
                        if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        re.push([n, f, num.real().to_f64()]);
                    }
                    if num.imag().is_finite()
                    {
                        if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        im.push([n, f, num.imag().to_f64()]);
                    }
                }
                Ok(Vector(v)) =>
                {
                    for num in v
                    {
                        if num.real().is_finite()
                        {
                            if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            re.push([n, f, num.real().to_f64()]);
                        }
                        if num.imag().is_finite()
                        {
                            if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            im.push([n, f, num.imag().to_f64()]);
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
                            if num.real().is_finite()
                            {
                                if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.0 = true
                                }
                                re.push([n, f, num.real().to_f64()]);
                            }
                            if num.imag().is_finite()
                            {
                                if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.1 = true
                                }
                                im.push([n, f, num.imag().to_f64()]);
                            }
                        }
                    }
                }
                _ =>
                {}
            }
        }
    }
    (
        if zero.0 { re } else { Vec::new() },
        if zero.1 { im } else { Vec::new() },
    )
}
pub fn can_graph(input: &str) -> bool
{
    input.contains('#')
        || input.replace("exp", "").replace("max", "").contains('x')
        || input.replace("any", "").contains('y')
        || input
            .replace("==", "")
            .replace("!=", "")
            .replace(">=", "")
            .replace("<=", "")
            .contains('=')
}
fn fail(options: Options, colors: &Colors)
{
    print!(
        "\x1b[G\x1b[KNo data to plot\n\x1b[G{}",
        prompt(options, colors)
    );
    stdout().flush().unwrap();
}