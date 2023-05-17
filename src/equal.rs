use crate::graph::get_list_2d;
use crate::math::do_math;
use crate::parse::get_func;
pub fn equal(range:([[f64; 2]; 3], f64, f64), input:&str, l:&str, r:&str) -> bool
{
    if input.contains('x')
    {
        if l.len() == 1
        {
            return false;
        }
        let l = match get_func(l, true)
        {
            Ok(i) => i,
            Err(()) =>
            {
                return true;
            }
        };
        let r = match get_func(r, true)
        {
            Ok(i) => i,
            Err(()) =>
            {
                return true;
            }
        };
        let (lre, lim) = get_list_2d(&l, range);
        let (rre, rim) = get_list_2d(&r, range);
        let mut success = true;
        for i in 0..lre.len()
        {
            if (lre[i][1] * 1e9).round() / 1e9 != (rre[i][1] * 1e9).round() / 1e9 || (lim[i][1] * 1e9).round() / 1e9 != (rim[i][1] * 1e9).round() / 1e9
            {
                success = false;
            }
        }
        if success
        {
            println!("true");
            return true;
        }
        println!("false");
        return true;
    }
    if l.parse::<f64>().is_err()
    {
        return false;
    }
    let l = match do_math(match get_func(l, false)
          {
              Ok(i) => i,
              Err(()) =>
              {
                  return true;
              }
          })
    {
        Ok(i) => i,
        Err(()) =>
        {
            return true;
        }
    };
    let r = match do_math(match get_func(r, false)
          {
              Ok(i) => i,
              Err(()) =>
              {
                  return true;
              }
          })
    {
        Ok(i) => i,
        Err(()) =>
        {
            return true;
        }
    };
    if (l.parse::<f64>().unwrap() * 1e12).round() / 1e12 == (r.parse::<f64>().unwrap() * 1e12).round() / 1e12
    {
        println!("true");
        return true;
    }
    println!("false");
    true
}