use crate::graph::{get_list_2d, get_list_3d};
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
        let l = match get_func(l)
        {
            Ok(i) => i,
            Err(()) =>
            {
                return true;
            }
        };
        let r = match get_func(r)
        {
            Ok(i) => i,
            Err(()) =>
            {
                return true;
            }
        };
        let mut success = true;
        if input.contains('y')
        {
            let (lre, lim) = get_list_3d(&l, range, false);
            let (rre, rim) = get_list_3d(&r, range, false);
            for i in 0..lre.len()
            {
                if (lre[i][2] * 1e9).round() / 1e9 != (rre[i][2] * 1e9).round() / 1e9 || (lim[i][2] * 1e9).round() / 1e9 != (rim[i][2] * 1e9).round() / 1e9
                {
                    success = false;
                }
            }
        }
        else
        {
            let (lre, lim) = get_list_2d(&l, range, false);
            let (rre, rim) = get_list_2d(&r, range, false);
            for i in 0..lre.len()
            {
                if (lre[i][1] * 1e9).round() / 1e9 != (rre[i][1] * 1e9).round() / 1e9 || (lim[i][1] * 1e9).round() / 1e9 != (rim[i][1] * 1e9).round() / 1e9
                {
                    success = false;
                }
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
    if l.chars().all(|x| x.is_alphabetic())
    {
        return false;
    }
    let l = match do_math(match get_func(l)
                          {
                              Ok(i) => i,
                              Err(()) =>
                              {
                                  return true;
                              }
                          },
                          false)
    {
        Ok(i) => i,
        Err(()) =>
        {
            return true;
        }
    };
    let r = match do_math(match get_func(r)
                          {
                              Ok(i) => i,
                              Err(()) =>
                              {
                                  return true;
                              }
                          },
                          false)
    {
        Ok(i) => i,
        Err(()) =>
        {
            return true;
        }
    };
    if l == r
    {
        println!("true");
        return true;
    }
    println!("false");
    true
}