use gnuplot::{AxesCommon, Figure, Fix, PointSymbol};
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
            let a = (a * 1e12).round() / 1e12;
            let b = (b * 1e12).round() / 1e12;
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
        let a = (a * 1e12).round() / 1e12;
        let b = (b * 1e12).round() / 1e12;
        re.push([n, a]);
        im.push([n, b]);
    }
    (re, im)
}
pub fn graph(func:&[String], graph:bool, im3d:bool, re3d:bool, fg:&mut Figure, older:Option<Vec<[Vec<[f64; 2]>; 2]>>, range:([[f64; 2]; 3], f64, f64)) -> Option<[Vec<[f64; 2]>; 2]>
{
    fg.close();
    if graph
    {
        let (re, im) = get_list_3d(func, range);
        let i = im.iter().map(|i| i[2]).sum::<f64>() != 0.0;
        let r = re.iter().map(|i| i[2]).sum::<f64>() != 0.0;
        if re3d && im3d && i && r
        {
            fg.axes3d()
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')])
              .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if re3d && r
        {
            fg.axes3d()
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if im3d && i
        {
            fg.axes3d()
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        fg.show_and_keep_running().unwrap();
        return None;
    }
    let (re, im) = get_list_2d(func, range);
    if let Some(..) = older
    {
        let older = older.unwrap();
        if !older.is_empty()
        {
            let mut older_re = older[0][0].to_vec();
            let mut older_im = older[0][1].to_vec();
            for i in older
            {
                older_re.extend_from_slice(&i[0]);
                older_im.extend_from_slice(&i[1]);
            }
            fg.axes2d()
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.')])
              .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.')])
              .points(older_re.iter().map(|x| x[0]), older_re.iter().map(|x| x[1]), &[PointSymbol('.')])
              .points(older_im.iter().map(|x| x[0]), older_im.iter().map(|x| x[1]), &[PointSymbol('.')]);
            fg.show_and_keep_running().unwrap();
            return Some([re, im]);
        }
    }
    fg.axes2d()
      .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
      .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
      .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.')])
      .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.')]);
    fg.show_and_keep_running().unwrap();
    Some([re, im])
}