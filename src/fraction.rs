use std::f64::consts::PI;
pub fn fraction(value:f64) -> String
{
    let eps = 1e-6;
    // as per continued fraction expansion
    let mut nums:Vec<f64> = vec![];
    let values = [(1.0, '1'), (PI, 'p'), (2f64.sqrt(), '2'), (3f64.sqrt(), '3')];
    let mut number;
    let mut recip;
    let mut fract;
    let mut orig;
    for (constant, name) in values
    {
        orig = value / constant;
        number = orig.fract();
        nums.clear();
        for _ in 0..=10
        {
            recip = number.recip();
            fract = recip.fract();
            if fract < eps
            {
                let mut last = 1.0;
                for i in (0..nums.len()).rev()
                {
                    last = recip;
                    recip *= nums[i];
                }
                recip = recip.round();
                last = (last + recip * orig.trunc() * constant).round();
                return format!("{}{}{}",
                               if last == 1.0 && name != '1' { "".to_string() } else { last.to_string() },
                               match name
                               {
                                   '1' => "",
                                   '3' => "sqrt(3)",
                                   '2' => "sqrt(2)",
                                   'p' => "π",
                                   _ => "",
                               },
                               if recip == 1.0 { "".to_string() } else { "/".to_owned() + &recip.to_string() });
            }
            nums.push(recip);
            number = fract;
        }
    }
    String::new()
}