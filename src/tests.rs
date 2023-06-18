use rug::Complex;
use rug::float::Constant::Pi;
use crate::math::NumStr::{Num, Str};
use crate::parse::{get_func, get_vars, input_var};
use crate::math::do_math;
#[test]
fn test_math()
{
    let input = input_var("pi+tau*e/2i^(sqrt(2))/3*3-log(2-2i,-3+i)+sqrt(2)^(sqrt(2))", &get_vars(256));
    let output = get_func(&input, 256, false).unwrap();
    let expected = vec![Num(Complex::with_val(256, Pi)),
                        Str("+".to_string()),
                        Num(2 * Complex::with_val(256, Pi)),
                        Str("*".to_string()),
                        Num(Complex::with_val(256, 1).exp()),
                        Str("/".to_string()),
                        Num(Complex::with_val(256, 2.0)),
                        Str("*".to_string()),
                        Num(Complex::with_val(256, (0.0, 1.0))),
                        Str("^".to_string()),
                        Str("(".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Num(Complex::with_val(256, 2.0)),
                        Str(")".to_string()),
                        Str(")".to_string()),
                        Str("/".to_string()),
                        Num(Complex::with_val(256, 3.0)),
                        Str("*".to_string()),
                        Num(Complex::with_val(256, 3.0)),
                        Str("-".to_string()),
                        Str("log".to_string()),
                        Str("(".to_string()),
                        Num(Complex::with_val(256, 2.0)),
                        Str("-".to_string()),
                        Num(Complex::with_val(256, 2.0)),
                        Str("*".to_string()),
                        Num(Complex::with_val(256, (0.0, 1.0))),
                        Str(",".to_string()),
                        Num(Complex::with_val(256, -3.0)),
                        Str("+".to_string()),
                        Num(Complex::with_val(256, (0.0, 1.0))),
                        Str(")".to_string()),
                        Str("+".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Num(Complex::with_val(256, 2.0)),
                        Str(")".to_string()),
                        Str("^".to_string()),
                        Str("(".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Num(Complex::with_val(256, 2.0)),
                        Str(")".to_string()),
                        Str(")".to_string())];
    let out = do_math(output, false, 256).unwrap().num().unwrap();
    let answer = do_math(expected, false, 256).unwrap().num().unwrap();
    assert_eq!(out.real().to_string(), answer.real().to_string());
    assert_eq!(out.imag().to_string(), answer.imag().to_string());
    assert_eq!(&out.real().to_string()[..20], "2.009877988310399125");
    assert_eq!(&out.imag().to_string()[..20], "4.535664430265577075");
}