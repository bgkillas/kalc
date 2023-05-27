// as per continued fraction expansion
use std::f64::consts::{E, PI, TAU, SQRT_2};
pub fn fraction(value:f64, tau:bool, scientific:bool) -> String
{
    let mut nums:Vec<f64> = vec![];
    let values = [1.0, if tau { TAU } else { PI }, SQRT_2, 3f64.sqrt(), 5f64.sqrt(), E];
    let sign:String = if value < 0.0 { "-".to_string() } else { "".to_string() };
    let (value, exponent) = if scientific { split_f64(value.abs()) } else { (value.abs(), 0) };
    let (mut number, mut recip, mut fract, mut orig);
    let tau = if tau { "τ" } else { "π" };

    for (i, constant) in values.iter().enumerate()
    {
        orig = value / constant;
        if orig.fract() == 0.0
        {
            return if i == 0
            {
                String::new()
            }
            else
            {
                format!("{}{}{}{}",
                        sign,
                        if orig == 1.0 { "".to_string() } else { orig.to_string() },
                        match i
                        {
                            1 => tau,
                            2 => "sqrt(2)",
                            3 => "sqrt(3)",
                            4 => "sqrt(5)",
                            5 => "e",
                            _ => "",
                        },
                        if exponent == 0 { "".to_string() } else { format!("\x1b[92mE{}\x1b[0m", exponent) })
            };
        }
        number = orig.fract();
        nums.clear();
        for _ in 0..=20
        {
            recip = number.recip();
            fract = recip.fract();
            if fract < 1e-6
            {
                let mut last = 1.0;
                for j in (0..nums.len()).rev()
                {
                    last = recip;
                    recip *= nums[j];
                }
                recip = recip.round();
                last = (last + recip * orig.trunc()).round();
                if recip.to_string().len() > 9 && last.to_string().len() > 9
                {
                    continue;
                }
                return format!("{sign}{}{}{}{}",
                               if last == 1.0 && i != 0 { "".to_string() } else { last.to_string() },
                               match i
                               {
                                   0 => "",
                                   1 => tau,
                                   2 => "sqrt(2)",
                                   3 => "sqrt(3)",
                                   4 => "sqrt(5)",
                                   5 => "e",
                                   _ => "",
                               },
                               if recip == 1.0 { "".to_string() } else { "/".to_owned() + &recip.to_string() },
                               if exponent == 0 { "".to_string() } else { format!("\x1b[92mE{}\x1b[0m", exponent) });
            }
            nums.push(recip);
            number = fract;
        }
    }
    String::new()
}
fn split_f64(value:f64) -> (f64, i32)
{
    if !value.is_finite()
    {
        return (value, 0);
    }
    let formatted_value = format!("{:e}", value);
    let parts:Vec<&str> = formatted_value.split('e').collect();
    let whole_number = parts[0].parse::<f64>().unwrap();
    let exponent = parts[1].parse::<i32>().unwrap();
    (whole_number, exponent)
}