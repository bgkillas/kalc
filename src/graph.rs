use gnuplot::{AxesCommon, Color, Dot, Figure, Fix, LineStyle, PointSymbol, SmallDot};
use crate::complex::parse;
use crate::math::do_math;
pub fn get_list_3d(func:&[String], range:([[f64; 2]; 3], f64, f64)) -> (Vec<[f64; 3]>, Vec<[f64; 3]>)
{
    let mut re = Vec::new();
    let mut im = Vec::new();
    let den = range.2;
    let min_x = range.0[0][0];
    let max_x = range.0[0][1];
    let den_x_range = (max_x - min_x) / den;
    let min_y = range.0[1][0];
    let max_y = range.0[1][1];
    let den_y_range = (max_y - min_y) / den;
    for i in 0..=den as i32
    {
        let n = min_x + i as f64 * den_x_range;
        let modified:Vec<String> = func.iter().map(|i| i.replace('x', &(n).to_string())).collect();
        for g in 0..=den as i32
        {
            let f = min_y + g as f64 * den_y_range;
            let num = match do_math(modified.iter().map(|j| j.replace('y', &(f).to_string())).collect())
            {
                Ok(n) => n,
                Err(_) =>
                {
                    println!("0");
                    continue;
                }
            };
            let (a, b) = parse(&num);
            re.push([n, f, a]);
            im.push([n, f, b]);
        }
    }
    (re, im)
}
pub fn get_list_2d(func:&[String], range:([[f64; 2]; 3], f64, f64)) -> (Vec<[f64; 2]>, Vec<[f64; 2]>)
{
    let mut re = Vec::new();
    let mut im = Vec::new();
    let min = range.0[0][0];
    let max = range.0[0][1];
    let den = range.1;
    let den_range = (max - min) / den;
    for i in 0..=den as i32
    {
        let n = min + i as f64 * den_range;
        let num = match do_math(func.iter().map(|i| i.replace('x', &(n).to_string())).collect())
        {
            Ok(n) => n,
            Err(_) =>
            {
                println!("0");
                continue;
            }
        };
        let (a, b) = parse(&num);
        re.push([n, a]);
        im.push([n, b]);
    }
    (re, im)
}
pub fn graph(func:&[String], graph:bool, close:bool, fg:&mut Figure, older:Option<Vec<[Vec<[f64; 2]>; 2]>>, range:([[f64; 2]; 3], f64, f64)) -> Option<[Vec<[f64; 2]>; 2]>
{
    fg.close();
    fg.clear_axes();
    if graph
    {
        let (re, im) = get_list_3d(func, range);
        let i = im.iter().map(|i| (i[2] * 1e15).round() / 1e15).sum::<f64>() != 0.0;
        let r = re.iter().map(|i| (i[2] * 1e15).round() / 1e15).sum::<f64>() != 0.0;
        if i && r
        {
            fg.axes3d()
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .set_z_label("z", &[])
              .set_y_label("y", &[])
              .set_x_label("x", &[])
              .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')])
              .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if r
        {
            fg.axes3d()
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .set_z_label("z", &[])
              .set_y_label("y", &[])
              .set_x_label("x", &[])
              .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if i
        {
            fg.axes3d()
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .set_z_label("z", &[])
              .set_y_label("y", &[])
              .set_x_label("x", &[])
              .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        if close
        {
            fg.show().unwrap();
            return None;
        }
        fg.show_and_keep_running().unwrap();
        return None;
    }
    let (mut re, mut im) = get_list_2d(func, range);
    let i = im.iter().map(|i| (i[1] * 1e15).round() / 1e15).sum::<f64>() != 0.0;
    let r = re.iter().map(|i| (i[1] * 1e15).round() / 1e15).sum::<f64>() != 0.0;
    let xticks = Some((Fix((range.0[0][1] - range.0[0][0]) / 20.0), 1));
    let yticks = Some((Fix((range.0[1][1] - range.0[1][0]) / 20.0), 1));
    let axisline = [-1000.0, -100.0, -10.0, -1.0, -0.1, 0.0, 0.1, 1.0, 10.0, 100.0, 1000.0];
    let zeros = [0.0; 11];
    if let Some(..) = older
    {
        let older = older.unwrap();
        if !older.is_empty()
        {
            let mut older_re = older[0][0].to_vec();
            let mut older_im = older[0][1].to_vec();
            for i in older
            {
                if i[0].iter().map(|i| i[1]).sum::<f64>() != 0.0
                {
                    older_re.extend_from_slice(&i[0]);
                }
                if i[1].iter().map(|i| i[1]).sum::<f64>() != 0.0
                {
                    older_im.extend_from_slice(&i[1]);
                }
            }
            if i && r
            {
                fg.axes2d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
                  .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
                  .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
                  .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)])
                  .lines(axisline, zeros, &[Color("white"), LineStyle(SmallDot)])
                  .lines(zeros, axisline, &[Color("white"), LineStyle(SmallDot)])
                  .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#9400D3")])
                  .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#009E73")])
                  .points(older_re.iter().map(|x| x[0]), older_re.iter().map(|x| x[1]), &[PointSymbol('.')])
                  .points(older_im.iter().map(|x| x[0]), older_im.iter().map(|x| x[1]), &[PointSymbol('.')]);
            }
            else if r
            {
                fg.axes2d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
                  .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
                  .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
                  .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)])
                  .lines(axisline, zeros, &[Color("white"), LineStyle(SmallDot)])
                  .lines(zeros, axisline, &[Color("white"), LineStyle(SmallDot)])
                  .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#9400D3")])
                  .points(older_re.iter().map(|x| x[0]), older_re.iter().map(|x| x[1]), &[PointSymbol('.')])
                  .points(older_im.iter().map(|x| x[0]), older_im.iter().map(|x| x[1]), &[PointSymbol('.')]);
                im.clear();
            }
            else if i
            {
                fg.axes2d()
                  .set_x_ticks(xticks, &[], &[])
                  .set_y_ticks(yticks, &[], &[])
                  .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
                  .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
                  .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
                  .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)])
                  .lines(axisline, zeros, &[Color("white"), LineStyle(SmallDot)])
                  .lines(zeros, axisline, &[Color("white"), LineStyle(SmallDot)])
                  .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#009E73")])
                  .points(older_re.iter().map(|x| x[0]), older_re.iter().map(|x| x[1]), &[PointSymbol('.')])
                  .points(older_im.iter().map(|x| x[0]), older_im.iter().map(|x| x[1]), &[PointSymbol('.')]);
                im.clear();
            }
            if close
            {
                fg.show().unwrap();
                return None;
            }
            fg.show_and_keep_running().unwrap();
            return Some([re, im]);
        }
    }
    if i && r
    {
        fg.axes2d()
          .set_x_ticks(xticks, &[], &[])
          .set_y_ticks(yticks, &[], &[])
          .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
          .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
          .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
          .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)])
          .lines(axisline, zeros, &[Color("white"), LineStyle(SmallDot)])
          .lines(zeros, axisline, &[Color("white"), LineStyle(SmallDot)])
          .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#9400D3")])
          .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#009E73")]);
    }
    else if r
    {
        fg.axes2d()
          .set_x_ticks(xticks, &[], &[])
          .set_y_ticks(yticks, &[], &[])
          .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
          .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
          .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
          .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)])
          .lines(axisline, zeros, &[Color("white"), LineStyle(SmallDot)])
          .lines(zeros, axisline, &[Color("white"), LineStyle(SmallDot)])
          .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#9400D3")]);
        im.clear();
    }
    else if i
    {
        fg.axes2d()
          .set_x_ticks(xticks, &[], &[])
          .set_y_ticks(yticks, &[], &[])
          .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
          .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
          .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
          .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)])
          .lines(axisline, zeros, &[Color("white"), LineStyle(SmallDot)])
          .lines(zeros, axisline, &[Color("white"), LineStyle(SmallDot)])
          .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#009E73")]);
        re.clear();
    }
    if close
    {
        fg.show().unwrap();
        return None;
    }
    fg.show_and_keep_running().unwrap();
    Some([re, im])
}