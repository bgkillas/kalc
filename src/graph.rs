use gnuplot::{AxesCommon, Caption, Color, Dot, Figure, Fix, LineStyle, PointSymbol};
use crate::complex::parse;
use crate::math::do_math;
pub fn get_list_3d(func:&[String], range:([[f64; 2]; 3], f64, f64)) -> (Vec<[f64; 3]>, Vec<[f64; 3]>)
{
    if func.is_empty()
    {
        return (Vec::new(), Vec::new());
    }
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
    if func.is_empty()
    {
        return (Vec::new(), Vec::new());
    }
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
pub fn graph(input:[&String; 2], func:[&[String]; 2], graph:bool, close:bool, fg:&mut Figure, range:([[f64; 2]; 3], f64, f64))
{
    let xticks = Some((Fix((range.0[0][1] - range.0[0][0]) / 20.0), 1));
    let yticks = Some((Fix((range.0[1][1] - range.0[1][0]) / 20.0), 1));
    fg.close();
    fg.clear_axes();
    if graph
    {
        let zticks = Some((Fix((range.0[2][1] - range.0[2][0]) / 20.0), 1));
        let (mut re, mut im) = get_list_3d(func[0], range);
        let (mut re2, mut im2) = get_list_3d(func[1], range);
        if im.iter().map(|i| (i[2] * 1e15).round() / 1e15).sum::<f64>() == 0.0
        {
            im.clear();
        }
        if im2.iter().map(|i| (i[2] * 1e15).round() / 1e15).sum::<f64>() == 0.0
        {
            im2.clear();
        }
        if re.iter().map(|i| (i[2] * 1e15).round() / 1e15).sum::<f64>() == 0.0
        {
            re.clear();
        }
        if re2.iter().map(|i| (i[2] * 1e15).round() / 1e15).sum::<f64>() == 0.0
        {
            re2.clear();
        }
        let re1c = if input[0] != "0" && !re.is_empty() { "re: ".to_owned() + input[0] } else { "".to_owned() };
        let im1c = if input[0] != "0" && !im.is_empty() { "im: ".to_owned() + input[0] } else { "".to_owned() };
        let re2c = if input[1] != "0" && !re2.is_empty() { "re: ".to_owned() + input[1] } else { "".to_owned() };
        let im2c = if input[1] != "0" && !im2.is_empty() { "im: ".to_owned() + input[1] } else { "".to_owned() };
        fg.axes3d()
          .set_x_ticks(xticks, &[], &[])
          .set_y_ticks(yticks, &[], &[])
          .set_z_ticks(zticks, &[], &[])
          .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
          .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
          .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
          .set_z_label("z", &[])
          .set_y_label("y", &[])
          .set_x_label("x", &[])
          .lines([0], [0], [0], &[Caption(re1c.as_str()), Color("#9400D3")])
          .lines([0], [0], [0], &[Caption(im1c.as_str()), Color("#009E73")])
          .lines([0], [0], [0], &[Caption(re2c.as_str()), Color("#56B4E9")])
          .lines([0], [0], [0], &[Caption(im2c.as_str()), Color("#E69F00")])
          .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.'), Color("#9400D3")])
          .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.'), Color("#009E73")])
          .points(re2.iter().map(|i| i[0]), re2.iter().map(|i| i[1]), re2.iter().map(|i| i[2]), &[PointSymbol('.'), Color("#56B4E9")])
          .points(im2.iter().map(|i| i[0]), im2.iter().map(|i| i[1]), im2.iter().map(|i| i[2]), &[PointSymbol('.'), Color("#E69F00")]);
        if close
        {
            fg.show().unwrap();
            return;
        }
        fg.show_and_keep_running().unwrap();
        return;
    }
    let (mut re, mut im) = get_list_2d(func[0], range);
    let (mut re2, mut im2) = get_list_2d(func[1], range);
    if im.iter().map(|i| (i[1] * 1e15).round() / 1e15).sum::<f64>() == 0.0
    {
        im.clear();
    }
    if im2.iter().map(|i| (i[1] * 1e15).round() / 1e15).sum::<f64>() == 0.0
    {
        im2.clear();
    }
    if re.iter().map(|i| (i[1] * 1e15).round() / 1e15).sum::<f64>() == 0.0
    {
        re.clear();
    }
    if re2.iter().map(|i| (i[1] * 1e15).round() / 1e15).sum::<f64>() == 0.0
    {
        re2.clear();
    }
    let axisline = [-1000.0, -100.0, -10.0, -1.0, -0.1, 0.0, 0.1, 1.0, 10.0, 100.0, 1000.0];
    let zeros = [0.0; 11];
    let re1c = if input[0] != "0" && !re.is_empty() { "re: ".to_owned() + input[0] } else { "".to_owned() };
    let im1c = if input[0] != "0" && !im.is_empty() { "im: ".to_owned() + input[0] } else { "".to_owned() };
    let re2c = if input[1] != "0" && !re2.is_empty() { "re: ".to_owned() + input[1] } else { "".to_owned() };
    let im2c = if input[1] != "0" && !im2.is_empty() { "im: ".to_owned() + input[1] } else { "".to_owned() };
    fg.axes2d()
      .set_x_ticks(xticks, &[], &[])
      .set_y_ticks(yticks, &[], &[])
      .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
      .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
      .lines([0], [0], &[Caption(re1c.as_str()), Color("#9400D3")])
      .lines([0], [0], &[Caption(im1c.as_str()), Color("#009E73")])
      .lines([0], [0], &[Caption(re2c.as_str()), Color("#56B4E9")])
      .lines([0], [0], &[Caption(im2c.as_str()), Color("#E69F00")])
      .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#9400D3")])
      .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#009E73")])
      .points(re2.iter().map(|x| x[0]), re2.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#56B4E9")])
      .points(im2.iter().map(|x| x[0]), im2.iter().map(|x| x[1]), &[PointSymbol('.'), Color("#E69F00")])
      .lines(axisline, zeros, &[Color("black"), LineStyle(Dot)])
      .lines(zeros, axisline, &[Color("black"), LineStyle(Dot)]);
    if close
    {
        fg.show().unwrap();
        return;
    }
    fg.show_and_keep_running().unwrap();
}