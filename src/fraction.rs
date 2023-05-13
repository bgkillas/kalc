use std::f64::consts::PI;
pub fn fraction(num:f64) -> String
{
    if (num * 1e12).round() / 1e12 == 0.0 || ((num * 1e12).round() / 1e12).fract() == 0.0
    {
        return "0".to_string();
    }
    let mut p;
    let values = [(3f64.sqrt(), "sqrt(3)"), (2f64.sqrt(), "sqrt(2)"), (PI, "π")];
    for i in 1..=20
    {
        for &(constant, name) in &values
        {
            p = (1e12 * num / constant * i as f64).round() / 1e12;
            if p.fract() == 0.0
            {
                let prefix = match p.is_sign_positive()
                {
                    true => "".to_string(),
                    false => "-".to_string(),
                };
                let suffix = if i == 1 { "".to_string() } else { format!("/{}", i) };
                let multiplier = if p == 1.0 { "".to_string() } else { p.to_string() };
                return format!("{}{}{}{}", multiplier, prefix, name, suffix);
            }
        }
    }
    let mut i = 1;
    while i <= 10000
    {
        let product = num * i as f64;
        if product.fract() == 0.0
        {
            let denominator = if i == 1 { "".to_string() } else { format!("/{}", i) };
            return format!("{}{}", product, denominator);
        }
        i += 1;
        if i <= 10000 && product > i as f64 * num && product.is_sign_positive()
        {
            i = (product / num) as usize;
        }
    }
    num.to_string()
}