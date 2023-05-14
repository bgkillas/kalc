use std::f64::consts::PI;
pub fn fraction(num:f64) -> String
{
    if (num * 1e12).round() / 1e12 == 0.0 || ((num * 1e12).round() / 1e12).fract() == 0.0
    {
        return "0".to_string();
    }
    // 0.6
    // 1.666 -> 0.666
    // 1.5 -> 0.5
    // 1/2 -> 2
    // once whole multiply up
    // 1.5 * 2
    // 3
    // 1.666 * 3
    // 5
    // 0.6 * 5 = 3
    // 0.6 = 3/5
    // therefore 3/5 is the fraction
    let mut nums:Vec<f64> = vec![];
    let values = [(1.0, "1"), (3f64.sqrt(), "3"), (2f64.sqrt(), "2"), (PI, "p")];
    let mut which = "";
    let mut number = num;
    let mut c = 0;
    'outer: for &(constant, name) in &values
    {
        number = num / constant;
        nums.clear();
        for i in 0..=10
        {
            let recip = number.recip();
            let fract = recip.fract();
            if (fract * 1e9).round() / 1e9 == 0.0
            {
                number = (recip * 1e9).round() / 1e9;
                which = name;
                break 'outer;
            }
            nums.push(recip);
            number = fract;
            if i == 10
            {
                c += 1;
            }
        }
    }
    if c == values.len()
    {
        return num.to_string();
    }
    let mut last = 1.0;
    for i in 0..nums.len()
    {
        last = number;
        number *= nums[nums.len() - 1 - i];
    }
    format!("{}{}{}",
            if last.round() == 1.0 && which != "1" { "".to_string() } else { last.round().to_string() },
            match which
            {
                "1" => "",
                "3" => "sqrt(3)",
                "2" => "sqrt(2)",
                "p" => "π",
                _ => "",
            },
            if number.round() == 1.0 { "".to_string() } else { "/".to_owned() + &number.round().to_string() })
}