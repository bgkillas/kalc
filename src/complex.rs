pub fn parse(num:&String) -> (f64, f64)
{
    let im = num.contains('i');
    let mut index = None;
    if let Some(i) = num.find('+')
    {
        index = Some(i);
    }
    else if let Some(i) = num.rfind('-')
    {
        if i != 0
        {
            index = Some(i);
        }
    }
    if let Some(i) = index
    {
        let a = num[..i].parse::<f64>();
        let mut b = num[i..].replace('i', "").parse::<f64>();
        if &num[i..] == "i"
        {
            b = Ok(1.0);
        }
        if &num[i..] == "-i"
        {
            b = Ok(-1.0);
        }
        match (a, b)
        {
            (Ok(a), Ok(b)) => (a, b),
            _ => (0.0, 0.0),
        }
    }
    else if im
    {
        let mut b = num[..num.len() - 1].parse::<f64>();
        if num == "i"
        {
            b = Ok(1.0);
        }
        if num == "-i"
        {
            b = Ok(-1.0);
        }
        match b
        {
            Ok(b) => (0.0, b),
            _ => (0.0, 0.0),
        }
    }
    else
    {
        let a = num.parse::<f64>();
        match a
        {
            Ok(a) => (a, 0.0),
            _ => (0.0, 0.0),
        }
    }
}
pub fn add(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)+(c+di)=(a+c)+(b+d)i
    let im = (b + d).to_string();
    let sign = if im.contains('-') { "" } else { "+" };
    (a + c).to_string() + sign + im.as_str() + "i"
}
pub fn mul(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)(c+di)=(ac-bd)+i(ad+bc)
    let im = (a * d + b * c).to_string();
    let sign = if im.contains('-') { "" } else { "+" };
    (a * c - b * d).to_string() + sign + im.as_str() + "i"
}
pub fn div(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2)+i(bc-ad)/(c^2+d^2)
    let im = b * c - a * d;
    let den = c * c + d * d;
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    ((a * c + b * d) / den).to_string() + sign + (im / den).to_string().as_str() + "i"
}
pub fn pow(a:f64, b:f64, c:f64, d:f64) -> String
{
    // (a+bi)^(c+di)=e^((c+di)(ln(a^2+b^2)/2+i*atan2(b,a)))
    // re=e^(c*ln(a^2+b^2)/2-d*atan2(b,a))*cos(d*ln(a^2+b^2)/2+c*atan2(b,a))
    // im=e^(c*ln(a^2+b^2)/2-d*atan2(b,a))*sin(d*ln(a^2+b^2)/2+c*atan2(b,a))
    if b == 0.0 && d == 0.0 && c.fract() == 0.0
    {
        return (a.powf(c)).to_string();
    }
    let r = c * (b.atan2(a)) + d * (0.5 * (a * a + b * b).ln());
    let m = std::f64::consts::E.powf(c * (0.5 * (a * a + b * b).ln()) - d * (b.atan2(a)));
    let im = m * r.sin();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (m * r.cos()).to_string() + sign + im.to_string().as_str() + "i"
    //((m * r.cos() * 1e15).round() / 1e15).to_string() + sign + ((im * 1e15).round() / 1e15).to_string().as_str() + "i"
}
pub fn abs(a:f64, b:f64) -> String
{
    // abs(a+bi)=sqrt(a^2+b^2)
    (a * a + b * b).sqrt().to_string()
}
pub fn ln(a:f64, b:f64) -> String
{
    // ln(a+bi)=ln(a^2+b^2)/2+i*atan2(b,a)
    let i = b.atan2(a);
    let sign = if i.to_string().contains('-') { "" } else { "+" };
    (0.5 * (a * a + b * b).ln()).to_string() + sign + i.to_string().as_str() + "i"
}
pub fn sin(a:f64, b:f64) -> String
{
    // sin(a+bi)=sin(a)cosh(b)+i*cos(a)sinh(b)
    if b == 0.0
    {
        return a.sin().to_string();
    }
    let im = a.cos() * b.sinh();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.sin() * b.cosh()).to_string() + sign + im.to_string().as_str() + "i"
}
pub fn cos(a:f64, b:f64) -> String
{
    // cos(a+bi)=cos(a)cosh(b)-i*sin(a)sinh(b)
    if b == 0.0
    {
        return a.cos().to_string();
    }
    let im = -a.sin() * b.sinh();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.cos() * b.cosh()).to_string() + sign + im.to_string().as_str() + "i"
}
pub fn tan(a:f64, b:f64) -> String
{
    // tan(a+bi)=sin(a+bi)/cos(a+bi)
    if b == 0.0
    {
        return (a.tan()).to_string();
    }
    div(a.sin() * b.cosh(), a.cos() * b.sinh(), a.cos() * b.cosh(), -a.sin() * b.sinh())
}
pub fn log(c:f64, d:f64, a:f64, b:f64) -> String
{
    // log(c,a+bi)=ln(a+bi)/ln(c)
    let (a, b) = parse(&ln(a, b));
    let (c, d) = parse(&ln(c, d));
    div(a, b, c, d)
}
pub fn asin(a:f64, b:f64) -> String
{
    // asin(a+bi)=-i*ln(i(a+bi)+sqrt(1-(a+bi)^2))
    if b == 0.0
    {
        // asin(a)=-i*ln(sqrt(1-a^2)+ai)
        let (d, c) = parse(&pow(1.0 - a * a, 0.0, 0.5, 0.0));
        let (e, f) = parse(&add(d, c, 0.0, a));
        let (x, y) = parse(&ln(e, f));
        let (a, b) = parse(&mul(0.0, -1.0, x, y));
        let sign = if b.to_string().contains('-') { "" } else { "+" };
        return a.to_string() + sign + b.to_string().as_str() + "i";
        // return ((a * 1e15).round() / 1e15).to_string() + sign + ((b * 1e15).round() / 1e15).to_string().as_str() + "i";
    }
    let (c, d) = parse(&pow(1.0 - a * a + b * b, -2.0 * a * b, 0.5, 0.0));
    let (a, b) = parse(&add(-b, a, c, d));
    let (a, b) = parse(&ln(a, b));
    let (a, b) = parse(&mul(a, b, 0.0, -1.0));
    let sign = if b.to_string().contains('-') { "" } else { "+" };
    a.to_string() + sign + b.to_string().as_str() + "i"
    //((a * 1e15).round() / 1e15).to_string() + sign + ((b * 1e15).round() / 1e15).to_string().as_str() + "i"
}
pub fn acos(a:f64, b:f64) -> String
{
    // acos(a+bi)=pi/2-asin(a+bi)
    let (a, b) = parse(&asin(a, b));
    let sign = if !b.to_string().contains('-') { "" } else { "+" };
    (std::f64::consts::PI / 2.0 - a).to_string() + sign + ((-b * 1e15).round() / 1e15).to_string().as_str() + "i"
    //(((std::f64::consts::PI / 2.0 - a) * 1e15).round() / 1e15).to_string() + sign + ((-b * 1e15).round() / 1e15).to_string().as_str() + "i"
}
pub fn atan(a:f64, b:f64) -> String
{
    // atan(a+bi)=i*atanh(-i(a+bi))
    if b == 0.0
    {
        return (a.atan()).to_string();
    }
    let (a, b) = parse(&atanh(b, -a));
    let sign = if a.to_string().contains('-') { "" } else { "+" };
    (-b).to_string() + sign + a.to_string().as_str() + "i"
    //((-b * 1e15).round() / 1e15).to_string() + sign + ((a * 1e15).round() / 1e15).to_string().as_str() + "i"
}
pub fn sinh(a:f64, b:f64) -> String
{
    // sinh(a+bi)=sinh(a)cos(b)+i*cosh(a)sin(b)
    if b == 0.0
    {
        return (a.sinh()).to_string();
    }
    let im = a.cosh() * b.sin();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.sinh() * b.cos()).to_string() + sign + im.to_string().as_str() + "i"
}
pub fn cosh(a:f64, b:f64) -> String
{
    // cosh(a+bi)=cosh(a)cos(b)+i*sinh(a)sin(b)
    if b == 0.0
    {
        return (a.cosh()).to_string();
    }
    let im = a.sinh() * b.sin();
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    (a.cosh() * b.cos()).to_string() + sign + im.to_string().as_str() + "i"
}
pub fn tanh(a:f64, b:f64) -> String
{
    // tanh(a+bi)=sinh(a+bi)/cosh(a+bi)
    if b == 0.0
    {
        return (a.tanh()).to_string();
    }
    div(a.sinh() * b.cos(), a.cosh() * b.sin(), a.cosh() * b.cos(), a.sinh() * b.sin())
}
pub fn asinh(a:f64, b:f64) -> String
{
    // asinh(a+bi)=ln(sqrt(a^2+b^2)+a+bi)
    if b == 0.0
    {
        return (a.asinh()).to_string();
    }
    if a == 0.0
    {
        let (a, b) = parse(&asin(b, 0.0));
        let sign = if a.to_string().contains('-') { "" } else { "+" };
        return ((-b * 1e15).round() / 1e15).to_string() + sign + ((a * 1e15).round() / 1e15).to_string().as_str() + "i";
    }
    let (e, f) = parse(&pow(a, b, 2.0, 0.0));
    let (c, d) = parse(&pow(e, (f * 1e10).round() / 1e10, 0.5, 0.0));
    let (a, b) = parse(&add(c, d, a, b));
    let (re, im) = parse(&ln(a, b));
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    re.to_string() + sign + im.to_string().as_str() + "i"
    //((re * 1e15).round() / 1e15).to_string() + sign + ((im * 1e15).round() / 1e15).to_string().as_str() + "i"
}
pub fn acosh(a:f64, b:f64) -> String
{
    // acosh(a+bi)=ln(sqrt(a+ib-1)*sqrt(a+ib+1)+a+ib)
    let (e, f) = parse(&pow(a - 1.0, b, 0.5, 0.0));
    let (g, h) = parse(&pow(a + 1.0, b, 0.5, 0.0));
    let (c, d) = parse(&mul(e, f, g, h));
    let (a, b) = parse(&add(c, d, a, b));
    let (re, im) = parse(&ln(a, b));
    let sign = if im.to_string().contains('-') { "" } else { "+" };
    re.to_string() + sign + im.to_string().as_str() + "i"
    //((re * 1e15).round() / 1e15).to_string() + sign + ((im * 1e15).round() / 1e15).to_string().as_str() + "i"
}
pub fn atanh(a:f64, b:f64) -> String
{
    // atanh(a+bi)=ln(a+bi+1)/2-ln(-a-bi+1)/2
    let (c, d) = parse(&add(a, b, 1.0, 0.0));
    let (e, f) = parse(&add(-a, -b, 1.0, 0.0));
    let (g, h) = parse(&ln(c, d));
    let (i, j) = parse(&ln(e, f));
    let (k, l) = parse(&div(g, h, 2.0, 0.0));
    let (m, n) = parse(&div(i, j, 2.0, 0.0));
    let (o, p) = parse(&add(k, l, -m, -n));
    let sign = if p.to_string().contains('-') { "" } else { "+" };
    o.to_string() + sign + p.to_string().as_str() + "i"
    //((o * 1e15).round() / 1e15).to_string() + sign + ((p * 1e15).round() / 1e15).to_string().as_str() + "i"
}