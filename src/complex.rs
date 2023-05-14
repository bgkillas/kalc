use std::f64::consts::{FRAC_PI_2, E, PI};
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
pub fn to_string(t:(f64, f64)) -> String
{
    let (a, b) = t;
    if b == 0.0
    {
        return a.to_string();
    }
    if a == 0.0
    {
        return format!("{}i", b);
    }
    if b < 0.0
    {
        return format!("{}{}i", a, b);
    }
    format!("{}+{}i", a, b)
}
pub fn add(a:f64, b:f64, c:f64, d:f64) -> (f64, f64)
{
    // (a+bi)+(c+di)=(a+c)+(b+d)i
    let re = a + c;
    let im = b + d;
    (re, im)
}
pub fn mul(a:f64, b:f64, c:f64, d:f64) -> (f64, f64)
{
    // (a+bi)(c+di)=(ac-bd)+i(ad+bc)
    if b == 0.0 && d == 0.0
    {
        return (a * c, 0.0);
    }
    let re = a * c - b * d;
    let im = a * d + b * c;
    (re, im)
}
pub fn div(a:f64, b:f64, c:f64, d:f64) -> (f64, f64)
{
    // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2)+i(bc-ad)/(c^2+d^
    if b == 0.0 && d == 0.0
    {
        return (a / c, 0.0);
    }
    let den = c * c + d * d;
    let re = (a * c + b * d) / den;
    let im = (b * c - a * d) / den;
    (re, im)
}
pub fn pow(a:f64, b:f64, c:f64, d:f64) -> (f64, f64)
{
    // (a+bi)^(c+di)=e^((c+di)(ln(a^2+b^2)/2+i*atan2(b,a)))
    // re=e^(c*ln(a^2+b^2)/2-d*atan2(b,a))*cos(d*ln(a^2+b^2)/2+c*atan2(b,a))
    // im=e^(c*ln(a^2+b^2)/2-d*atan2(b,a))*sin(d*ln(a^2+b^2)/2+c*atan2(b,a))
    if b == 0.0 && d == 0.0 && c.fract() == 0.0
    {
        return (a.powf(c), 0.0);
    }
    let angle = c * (b.atan2(a)) + d * (0.5 * (a * a + b * b).ln());
    let scaler = E.powf(c * (0.5 * (a * a + b * b).ln()) - d * (b.atan2(a)));
    let im = scaler * angle.sin();
    let re = scaler * angle.cos();
    (re, im)
}
pub fn abs(a:f64, b:f64) -> f64
{
    // abs(a+bi)=sqrt(a^2+b^2)
    (a * a + b * b).sqrt()
}
pub fn ln(a:f64, b:f64) -> (f64, f64)
{
    // ln(a+bi)=ln(a^2+b^2)/2+i*atan2(b,a)
    if b == 0.0
    {
        return if a.is_sign_positive() { (a.ln(), 0.0) } else { (a.abs().ln(), PI) };
    }
    let re = (a * a + b * b).ln() / 2.0;
    let im = b.atan2(a);
    (re, im)
}
pub fn sin(a:f64, b:f64) -> (f64, f64)
{
    // sin(a+bi)=sin(a)cosh(b)+i*cos(a)sinh(b)
    if b == 0.0
    {
        if a % PI == 0.0
        {
            return (0.0, 0.0);
        }
        if a % FRAC_PI_2 == 0.0
        {
            if (a.abs() * 2.0 / PI) % 4.0 == 1.0
            {
                return (if a.is_sign_positive() { 1.0 } else { -1.0 }, 0.0);
            }
            return (if a.is_sign_positive() { -1.0 } else { 1.0 }, 0.0);
        }
        let re = a.sin();
        return (re, 0.0);
    }
    let im = a.cos() * b.sinh();
    let re = a.sin() * b.cosh();
    (re, im)
}
pub fn csc(a:f64, b:f64) -> (f64, f64)
{
    // csc(a+bi)=1/sin(a+bi)
    let (re, im) = sin(a, b);
    div(1.0, 0.0, re, im)
}
pub fn cos(a:f64, b:f64) -> (f64, f64)
{
    // cos(a+bi)=cos(a)cosh(b)-i*sin(a)sinh(b)
    if b == 0.0
    {
        if a % PI == 0.0
        {
            if (a.abs() / PI) % 2.0 == 1.0
            {
                return (-1.0, 0.0);
            }
            return (1.0, 0.0);
        }
        if a % FRAC_PI_2 == 0.0
        {
            return (0.0, 0.0);
        }
        let re = a.cos();
        return (re, 0.0);
    }
    let im = -a.sin() * b.sinh();
    let re = a.cos() * b.cosh();
    (re, im)
}
pub fn sec(a:f64, b:f64) -> (f64, f64)
{
    // sec(a+bi)=1/cos(a+bi)
    let (re, im) = cos(a, b);
    div(1.0, 0.0, re, im)
}
pub fn tan(a:f64, b:f64) -> (f64, f64)
{
    // tan(a+bi)=sin(a+bi)/cos(a+bi)
    if b == 0.0
    {
        if a % PI == 0.0
        {
            return (0.0, 0.0);
        }
        if a % FRAC_PI_2 == 0.0
        {
            return (f64::INFINITY, 0.0);
        }
        let re = a.tan();
        return (re, 0.0);
    }
    let (re, im) = div(a.sin() * b.cosh(), a.cos() * b.sinh(), a.cos() * b.cosh(), -a.sin() * b.sinh());
    (re, im)
}
pub fn cot(a:f64, b:f64) -> (f64, f64)
{
    // cot(a+bi)=1/tan(a+bi)
    let (re, im) = tan(a, b);
    div(1.0, 0.0, re, im)
}
pub fn log(c:f64, d:f64, a:f64, b:f64) -> (f64, f64)
{
    // log(c,a+bi)=ln(a+bi)/ln(c)
    let (a, b) = ln(a, b);
    let (c, d) = ln(c, d);
    let (re, im) = div(a, b, c, d);
    (re, im)
}
pub fn asin(a:f64, b:f64) -> (f64, f64)
{
    // asin(a+bi)=-i*ln(i(a+bi)+sqrt(1-(a+bi)^2))
    if b == 0.0
    {
        // asin(a)=-i*ln(sqrt(1-a^2)+ai)
        let (d, c) = pow(1.0 - a * a, 0.0, 0.5, 0.0);
        let (e, f) = add(d, c, 0.0, a);
        let (x, y) = ln(e, f);
        let (re, im) = mul(0.0, -1.0, x, y);
        return (re, im);
    }
    let (c, d) = pow(1.0 - a * a + b * b, -2.0 * a * b, 0.5, 0.0);
    let (a, b) = add(-b, a, c, d);
    let (a, b) = ln(a, b);
    let (re, im) = mul(a, b, 0.0, -1.0);
    (re, im)
}
pub fn acsc(a:f64, b:f64) -> (f64, f64)
{
    // acsc(a+bi)=asin(1/(a+bi))
    let (re, im) = div(1.0, 0.0, a, b);
    asin(re, im)
}
pub fn acos(a:f64, b:f64) -> (f64, f64)
{
    // acos(a+bi)=pi/2-asin(a+bi)
    let (a, b) = asin(a, b);
    let re = FRAC_PI_2 - a;
    let im = -b;
    (re, im)
}
pub fn asec(a:f64, b:f64) -> (f64, f64)
{
    // asec(a+bi)=acos(1/(a+bi))
    let (re, im) = div(1.0, 0.0, a, b);
    acos(re, im)
}
pub fn atan(a:f64, b:f64) -> (f64, f64)
{
    // atan(a+bi)=i*atanh(-i(a+bi))
    if b == 0.0
    {
        return (a.atan(), 0.0);
    }
    let (a, b) = atanh(b, -a);
    let re = -b;
    let im = a;
    (re, im)
}
pub fn acot(a:f64, b:f64) -> (f64, f64)
{
    // acot(a+bi)=atan(1/(a+bi))
    let (re, im) = div(1.0, 0.0, a, b);
    atan(re, im)
}
pub fn sinh(a:f64, b:f64) -> (f64, f64)
{
    // sinh(a+bi)=sinh(a)cos(b)+i*cosh(a)sin(b)
    if b == 0.0
    {
        return (a.sinh(), 0.0);
    }
    let im = a.cosh() * b.sin();
    let re = a.sinh() * b.cos();
    (re, im)
}
pub fn csch(a:f64, b:f64) -> (f64, f64)
{
    // csch(a+bi)=1/sinh(a+bi)
    let (re, im) = sinh(a, b);
    div(1.0, 0.0, re, im)
}
pub fn cosh(a:f64, b:f64) -> (f64, f64)
{
    // cosh(a+bi)=cosh(a)cos(b)+i*sinh(a)sin(b)
    if b == 0.0
    {
        return (a.cosh(), 0.0);
    }
    let im = a.sinh() * b.sin();
    let re = a.cosh() * b.cos();
    (re, im)
}
pub fn sech(a:f64, b:f64) -> (f64, f64)
{
    // sech(a+bi)=1/cosh(a+bi)
    let (re, im) = cosh(a, b);
    div(1.0, 0.0, re, im)
}
pub fn tanh(a:f64, b:f64) -> (f64, f64)
{
    // tanh(a+bi)=sinh(a+bi)/cosh(a+bi)
    if b == 0.0
    {
        return (a.tanh(), 0.0);
    }
    let (re, im) = div(a.sinh() * b.cos(), a.cosh() * b.sin(), a.cosh() * b.cos(), a.sinh() * b.sin());
    (re, im)
}
pub fn coth(a:f64, b:f64) -> (f64, f64)
{
    // coth(a+bi)=cosh(a+bi)/sinh(a+bi)
    let (re, im) = tanh(a, b);
    div(1.0, 0.0, re, im)
}
pub fn asinh(a:f64, b:f64) -> (f64, f64)
{
    // asinh(a+bi)=ln(sqrt((a+bi)^2+1)+a+bi)
    if b == 0.0
    {
        return (a.asinh(), 0.0);
    }
    if a == 0.0
    {
        let (a, b) = asin(b, 0.0);
        let re = -b;
        let im = a;
        return (re, im);
    }
    let (c, d) = pow(a, b, 2.0, 0.0);
    let (e, f) = add(c, d, 1.0, 0.0);
    let (g, h) = pow(e, f, 0.5, 0.0);
    let (a, b) = add(g, h, a, b);
    let (re, im) = ln(a, b);
    (re, im)
}
pub fn acsch(a:f64, b:f64) -> (f64, f64)
{
    // acsch(a+bi)=asinh(1/(a+bi))
    let (re, im) = div(1.0, 0.0, a, b);
    asinh(re, im)
}
pub fn acosh(a:f64, b:f64) -> (f64, f64)
{
    // acosh(a+bi)=ln(sqrt(a+ib-1)*sqrt(a+ib+1)+a+ib)
    let (e, f) = pow(a - 1.0, b, 0.5, 0.0);
    let (g, h) = pow(a + 1.0, b, 0.5, 0.0);
    let (c, d) = mul(e, f, g, h);
    let (a, b) = add(c, d, a, b);
    let (re, im) = ln(a, b);
    (re, im)
}
pub fn asech(a:f64, b:f64) -> (f64, f64)
{
    // asech(a+bi)=acosh(1/(a+bi))
    let (re, im) = div(1.0, 0.0, a, b);
    acosh(re, im)
}
pub fn atanh(a:f64, b:f64) -> (f64, f64)
{
    // atanh(a+bi)=ln(a+bi+1)/2-ln(-a-bi+1)/2
    let (c, d) = add(a, b, 1.0, 0.0);
    let (e, f) = add(-a, -b, 1.0, 0.0);
    let (g, h) = ln(c, d);
    let (i, j) = ln(e, f);
    let (k, l) = div(g, h, 2.0, 0.0);
    let (m, n) = div(i, j, 2.0, 0.0);
    let (re, im) = add(k, l, -m, -n);
    (re, im)
}
pub fn acoth(a:f64, b:f64) -> (f64, f64)
{
    // acoth(a+bi)=atanh(1/(a+bi))
    let (re, im) = div(1.0, 0.0, a, b);
    atanh(re, im)
}
pub fn arg(a:f64, b:f64) -> f64
{
    // arg(a+bi)=-ilog(sgn(a+bi))
    let (a, b) = sgn(a, b);
    let (_, re) = ln(a, b);
    re
}
pub fn sgn(a:f64, b:f64) -> (f64, f64)
{
    // sign(a+bi)=(a+bi)/|a+bi|
    if b == 0.0
    {
        return (a.signum(), 0.0);
    }
    let (re, im) = div(a, b, abs(a, b), 0.0);
    (re, im)
}
pub fn fact(a:f64) -> f64
{
    // fact(a)=a!
    if a == 0.0
    {
        return 1.0;
    }
    let mut b = a;
    let mut c = 1.0;
    while b > 1.0
    {
        c *= b;
        b -= 1.0;
    }
    c
}
pub fn int(a:f64, b:f64) -> (f64, f64)
{
    (a.trunc(), b.trunc())
}
pub fn frac(a:f64, b:f64) -> (f64, f64)
{
    (a.fract(), b.fract())
}