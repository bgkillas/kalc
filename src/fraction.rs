// as per continued fraction expansion
use rug::Float;
use rug::float::Constant::Pi;
pub fn fraction(value:Float, tau:bool) -> String
{
    let prec = value.prec();
    if value.clone().fract() == 0.0
    {
        return String::new();
    }
    let mut nums:Vec<Float> = vec![];
    let values = [Float::with_val(prec, 1.0), if tau { 2 * Float::with_val(prec, Pi) } else { Float::with_val(prec, Pi) }, Float::with_val(prec, 2).sqrt(), Float::with_val(prec, 3).sqrt()];
    let sign:String = if value < 0.0 { "-".to_string() } else { "".to_string() };
    let val = value.abs();
    let (mut number, mut recip, mut fract, mut orig);
    let tau = if tau { "τ" } else { "π" };
    for (i, constant) in values.iter().enumerate()
    {
        orig = val.clone() / constant;
        if orig.clone().fract() == 0.0
        {
            return if i == 0
            {
                String::new()
            }
            else
            {
                format!("{}{}{}", sign, if orig == 1.0 { "".to_string() } else { orig.to_integer().unwrap().to_string() }, match i
                {
                    1 => tau,
                    2 => "sqrt(2)",
                    3 => "sqrt(3)",
                    _ => "",
                })
            };
        }
        number = orig.clone().fract();
        nums.clear();
        for _ in 0..=50
        {
            recip = number.clone().recip();
            fract = recip.clone().fract();
            if fract < 1e-6
            {
                let mut last = Float::with_val(prec, 1.0);
                for j in (0..nums.len()).rev()
                {
                    last = recip.clone();
                    recip *= &nums[j];
                }
                let recip = recip.to_integer().unwrap();
                let last = (last + recip.clone() * orig.trunc()).to_integer().unwrap();
                return format!("{sign}{}{}{}",
                               if last == 1 && i != 0 { "".to_string() } else { last.to_string() },
                               match i
                               {
                                   0 => "",
                                   1 => tau,
                                   2 => "sqrt(2)",
                                   3 => "sqrt(3)",
                                   _ => "",
                               },
                               if recip == 1 { "".to_string() } else { "/".to_owned() + &recip.to_string() });
            }
            nums.push(recip);
            number = fract;
        }
    }
    String::new()
}