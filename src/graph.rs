use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use gnuplot::{AxesCommon, Caption, Color, Dot, Figure, Fix, LineStyle, PointSymbol};
use rug::Complex;
use crate::GraphOptions;
use crate::math::{
    do_math, NumStr, NumStr::{Num, Str}
};
pub fn graph(input:Vec<String>, func:Vec<Vec<NumStr>>, options:GraphOptions, deg:bool, prec:u32, watch:Option<Instant>) -> JoinHandle<()>
{
    thread::spawn(move || {
        let mut fg = Figure::new();
        fg.set_enhanced_text(false);
        let re1col = "#9400D3";
        let im1col = "#009E73";
        let re2col = "#56B4E9";
        let im2col = "#E69F00";
        let re3col = "#F0E442";
        let im3col = "#0072B2";
        let re4col = "#D55E00";
        let im4col = "#CC79A7";
        let re5col = "#000000";
        let im5col = "#0033cc";
        let xticks = Some((Fix((options.xr[1] - options.xr[0]) / 20.0), 1));
        let yticks = Some((Fix((options.yr[1] - options.yr[0]) / 20.0), 1));
        let mut re_cap:[String; 5] = Default::default();
        let mut im_cap:[String; 5] = Default::default();
        if !input[0].contains('x')
        {
            let mut re = Vec::new();
            for (i, f) in func.iter().enumerate()
            {
                re.push(do_math(f.to_vec(), deg, prec).unwrap().vec().unwrap());
                re_cap[i] = input[i].to_owned().replace("graph(", "").replace("graph", "");
            }
            if re[0].len() == 2
            {
                for _ in 0..5 - func.len()
                {
                    re.push(vec![Complex::with_val(prec, 0.0), Complex::with_val(prec, 0.0)]);
                }
                let axisline = [-1000000.0, -100000.0, -10000.0, -1000.0, -100.0, -10.0, -1.0, -0.1, 0.0, 0.1, 1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0];
                let zeros = [0.0; 16];
                fg.axes2d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_y_range(Fix(options.yr[0]), Fix(options.yr[1]))
                  .set_x_range(Fix(options.xr[0]), Fix(options.xr[1]))
                  .lines([0.0, re[0][0].real().to_f64()], [0.0, re[0][1].real().to_f64()], &[Caption(&re_cap[0]), Color(re1col)])
                  .lines([0.0, re[1][0].real().to_f64()], [0.0, re[1][1].real().to_f64()], &[Caption(&re_cap[1]), Color(re2col)])
                  .lines([0.0, re[2][0].real().to_f64()], [0.0, re[2][1].real().to_f64()], &[Caption(&re_cap[2]), Color(re3col)])
                  .lines([0.0, re[3][0].real().to_f64()], [0.0, re[3][1].real().to_f64()], &[Caption(&re_cap[3]), Color(re4col)])
                  .lines([0.0, re[4][0].real().to_f64()], [0.0, re[4][1].real().to_f64()], &[Caption(&re_cap[4]), Color(re5col)])
                  .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
                  .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)]);
            }
            else if re[0].len() == 3
            {
                for _ in 0..5 - func.len()
                {
                    re.push(vec![Complex::with_val(prec, 0.0), Complex::with_val(prec, 0.0), Complex::with_val(prec, 0.0)]);
                }
                let zticks = Some((Fix((options.zr[1] - options.zr[0]) / 20.0), 1));
                fg.axes3d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_z_ticks(zticks, &[], &[])
                  .set_x_range(Fix(options.xr[0]), Fix(options.xr[1]))
                  .set_y_range(Fix(options.yr[0]), Fix(options.yr[1]))
                  .set_z_range(Fix(options.zr[0]), Fix(options.zr[1]))
                  .set_z_label("z", &[])
                  .set_y_label("y", &[])
                  .set_x_label("x", &[])
                  .lines([0.0, re[0][0].real().to_f64()], [0.0, re[0][1].real().to_f64()], [0.0, re[0][2].real().to_f64()], &[Caption(&re_cap
                                                                                                                                          [0]),
                                                                                                                              Color(re1col)])
                  .lines([0.0, re[1][0].real().to_f64()], [0.0, re[1][1].real().to_f64()], [0.0, re[1][2].real().to_f64()], &[Caption(&re_cap
                                                                                                                                          [1]),
                                                                                                                              Color(re2col)])
                  .lines([0.0, re[2][0].real().to_f64()], [0.0, re[2][1].real().to_f64()], [0.0, re[2][2].real().to_f64()], &[Caption(&re_cap
                                                                                                                                          [2]),
                                                                                                                              Color(re3col)])
                  .lines([0.0, re[3][0].real().to_f64()], [0.0, re[3][1].real().to_f64()], [0.0, re[3][2].real().to_f64()], &[Caption(&re_cap
                                                                                                                                          [3]),
                                                                                                                              Color(re4col)])
                  .lines([0.0, re[4][0].real().to_f64()], [0.0, re[4][1].real().to_f64()], [0.0, re[4][2].real().to_f64()], &[Caption(&re_cap
                                                                                                                                          [4]),
                                                                                                                              Color(re5col)]);
            }
        }
        else if input[0].contains('y')
        {
            let zticks = Some((Fix((options.zr[1] - options.zr[0]) / 20.0), 1));
            let mut re = vec![Vec::new(); 5];
            let mut im = vec![Vec::new(); 5];
            let (mut re2, mut im2);
            for (i, f) in func.iter().enumerate()
            {
                (re2, im2) = get_list_3d(f, options, deg, prec);
                if re2.iter().map(|i| ((i[2] * 1e15).round() / 1e15) == 0.0).all(|i| i)
                {
                    re2.clear();
                }
                else
                {
                    re_cap[i] = input[i].to_owned() + ":re";
                }
                if im2.iter().map(|i| ((i[2] * 1e15).round() / 1e15) == 0.0).all(|i| i)
                {
                    im2.clear();
                }
                else
                {
                    im_cap[i] = input[i].to_owned() + ":im";
                }
                re[i].extend(re2);
                im[i].extend(im2);
            }
            if re.iter().all(|x| x.is_empty()) && im.iter().all(|x| x.is_empty())
            {
                println!("No data to plot");
                return;
            }
            fg.axes3d()
              .set_x_ticks(xticks, &[], &[])
              .set_y_ticks(yticks, &[], &[])
              .set_z_ticks(zticks, &[], &[])
              .set_x_range(Fix(options.xr[0]), Fix(options.xr[1]))
              .set_y_range(Fix(options.yr[0]), Fix(options.yr[1]))
              .set_z_range(Fix(options.zr[0]), Fix(options.zr[1]))
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
              .points(re[0].iter().map(|i| i[0]), re[0].iter().map(|i| i[1]), re[0].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(re1col)])
              .points(im[0].iter().map(|i| i[0]), im[0].iter().map(|i| i[1]), im[0].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(im1col)])
              .points(re[1].iter().map(|i| i[0]), re[1].iter().map(|i| i[1]), re[1].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(re2col)])
              .points(im[1].iter().map(|i| i[0]), im[1].iter().map(|i| i[1]), im[1].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(im2col)])
              .points(re[2].iter().map(|i| i[0]), re[2].iter().map(|i| i[1]), re[2].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(re3col)])
              .points(im[2].iter().map(|i| i[0]), im[2].iter().map(|i| i[1]), im[2].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(im3col)])
              .points(re[3].iter().map(|i| i[0]), re[3].iter().map(|i| i[1]), re[3].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(re4col)])
              .points(im[3].iter().map(|i| i[0]), im[3].iter().map(|i| i[1]), im[3].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(im4col)])
              .points(re[4].iter().map(|i| i[0]), re[4].iter().map(|i| i[1]), re[4].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(re5col)])
              .points(im[4].iter().map(|i| i[0]), im[4].iter().map(|i| i[1]), im[4].iter().map(|i| i[2]), &[PointSymbol(options.point_style),
                                                                                                            Color(im5col)]);
        }
        else
        {
            let axisline = [-1000000.0, -100000.0, -10000.0, -1000.0, -100.0, -10.0, -1.0, -0.1, 0.0, 0.1, 1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0];
            let zeros = [0.0; 16];
            let mut re = vec![Vec::new(); 5];
            let mut im = vec![Vec::new(); 5];
            let (mut re2, mut im2);
            for (i, f) in func.iter().enumerate()
            {
                (re2, im2) = get_list_2d(f, options, deg, prec);
                if re2.iter().map(|i| ((i[1] * 1e15).round() / 1e15) == 0.0).all(|i| i)
                {
                    re2.clear();
                }
                else
                {
                    re_cap[i] = input[i].to_owned() + ":re";
                }
                if im2.iter().map(|i| ((i[1] * 1e15).round() / 1e15) == 0.0).all(|i| i)
                {
                    im2.clear();
                }
                else
                {
                    im_cap[i] = input[i].to_owned() + ":im";
                }
                re[i].extend(re2);
                im[i].extend(im2);
            }
            if re.iter().all(|x| x.is_empty()) && im.iter().all(|x| x.is_empty())
            {
                println!("No data to plot");
                return;
            }
            if options.lines
            {
                fg.axes2d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_y_range(Fix(options.yr[0]), Fix(options.yr[1]))
                  .set_x_range(Fix(options.xr[0]), Fix(options.xr[1]))
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
                  .lines(re[0].iter().map(|x| x[0]), re[0].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re1col)])
                  .lines(im[0].iter().map(|x| x[0]), im[0].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im1col)])
                  .lines(re[1].iter().map(|x| x[0]), re[1].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re2col)])
                  .lines(im[1].iter().map(|x| x[0]), im[1].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im2col)])
                  .lines(re[2].iter().map(|x| x[0]), re[2].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re3col)])
                  .lines(im[2].iter().map(|x| x[0]), im[2].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im3col)])
                  .lines(re[3].iter().map(|x| x[0]), re[3].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re4col)])
                  .lines(im[3].iter().map(|x| x[0]), im[3].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im4col)])
                  .lines(re[4].iter().map(|x| x[0]), re[4].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re5col)])
                  .lines(im[4].iter().map(|x| x[0]), im[4].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im5col)])
                  .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
                  .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)]);
            }
            else
            {
                fg.axes2d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_y_range(Fix(options.yr[0]), Fix(options.yr[1]))
                  .set_x_range(Fix(options.xr[0]), Fix(options.xr[1]))
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
                  .points(re[0].iter().map(|x| x[0]), re[0].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re1col)])
                  .points(im[0].iter().map(|x| x[0]), im[0].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im1col)])
                  .points(re[1].iter().map(|x| x[0]), re[1].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re2col)])
                  .points(im[1].iter().map(|x| x[0]), im[1].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im2col)])
                  .points(re[2].iter().map(|x| x[0]), re[2].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re3col)])
                  .points(im[2].iter().map(|x| x[0]), im[2].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im3col)])
                  .points(re[3].iter().map(|x| x[0]), re[3].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re4col)])
                  .points(im[3].iter().map(|x| x[0]), im[3].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im4col)])
                  .points(re[4].iter().map(|x| x[0]), re[4].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(re5col)])
                  .points(im[4].iter().map(|x| x[0]), im[4].iter().map(|x| x[1]), &[PointSymbol(options.point_style), Color(im5col)])
                  .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
                  .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)]);
            }
        }
        if let Some(time) = watch
        {
            println!("{}ms", time.elapsed().as_millis());
        }
        fg.show().unwrap();
    })
}
pub fn get_list_2d(func:&[NumStr], range:GraphOptions, deg:bool, prec:u32) -> (Vec<[f64; 2]>, Vec<[f64; 2]>)
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.eq0()
        {
            return (Vec::new(), Vec::new());
        }
    }
    let mut re = Vec::new();
    let mut im = Vec::new();
    let min = range.xr[0];
    let max = range.xr[1];
    let den = range.samples_2d;
    let den_range = (max - min) / den;
    let (mut n, mut num);
    for i in 0..=den as i32
    {
        n = min + i as f64 * den_range;
        num = match do_math(func.iter()
                                .map(|i| {
                                    match i
                                    {
                                        Str(s) if s == "x" => Num(Complex::with_val(prec, n)),
                                        _ => i.clone(),
                                    }
                                })
                                .collect(),
                            deg,
                            prec)
        {
            Ok(n) =>
            {
                match n.num()
                {
                    Ok(n) => n,
                    _ => continue,
                }
            }
            Err(_) => continue,
        };
        if num.real().is_finite()
        {
            re.push([n, num.real().to_f64()]);
        }
        if num.imag().is_finite()
        {
            im.push([n, num.imag().to_f64()]);
        }
    }
    (re, im)
}
pub fn get_list_3d(func:&[NumStr], range:GraphOptions, deg:bool, prec:u32) -> (Vec<[f64; 3]>, Vec<[f64; 3]>)
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.eq0()
        {
            return (Vec::new(), Vec::new());
        }
    }
    let mut re = Vec::new();
    let mut im = Vec::new();
    let den = range.samples_3d;
    let min_x = range.xr[0];
    let max_x = range.xr[1];
    let den_x_range = (max_x - min_x) / den;
    let min_y = range.yr[0];
    let max_y = range.yr[1];
    let den_y_range = (max_y - min_y) / den;
    let (mut n, mut f, mut num);
    let mut modified:Vec<NumStr>;
    for i in 0..=den as i32
    {
        n = min_x + i as f64 * den_x_range;
        modified = func.iter()
                       .map(|i| {
                           match i
                           {
                               Str(s) if s == "x" => Num(Complex::with_val(prec, n)),
                               _ => i.clone(),
                           }
                       })
                       .collect();
        for g in 0..=den as i32
        {
            f = min_y + g as f64 * den_y_range;
            num = match do_math(modified.iter()
                                        .map(|j| {
                                            match j
                                            {
                                                Str(s) if s == "y" => Num(Complex::with_val(prec, f)),
                                                _ => j.clone(),
                                            }
                                        })
                                        .collect(),
                                deg,
                                prec)
            {
                Ok(n) =>
                {
                    match n.num()
                    {
                        Ok(n) => n,
                        _ => continue,
                    }
                }
                Err(_) => continue,
            };
            if num.real().is_finite()
            {
                re.push([n, f, num.real().to_f64()]);
            }
            if num.imag().is_finite()
            {
                im.push([n, f, num.imag().to_f64()]);
            }
        }
    }
    (re, im)
}