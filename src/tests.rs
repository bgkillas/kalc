use rug::{Complex, Float};
use rug::float::Constant::Pi;
use crate::math::NumStr::{Num, Str};
use crate::parse::{get_func, get_vars, input_var};
use crate::math::do_math;
#[test]
fn test_math()
{
    let input = input_var("pi+tau*e/2i^(sqrt(2))/3*3-log(2-2i,-3+i)+sqrt(2)^(sqrt(2))", &get_vars(true));
    let output = get_func(&input, 256).unwrap();
    let expected = vec![Num(Complex::with_val(256, Pi)),
                        Str("+".to_string()),
                        Num(2 * Complex::with_val(256, Pi)),
                        Str("*".to_string()),
                        Num(Complex::with_val(256, Float::parse("2.71828182845904523536028747135266249775724709369995957496696763").unwrap())),
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
    let out = do_math(output, false, 256).unwrap();
    let answer = do_math(expected, false, 256).unwrap();
    assert_eq!(out.real().to_string()[..20], answer.real().to_string()[..20]);
    assert_eq!(out.imag().to_string()[..20], answer.imag().to_string()[..20]);
    assert_eq!(&out.real().to_string()[..20], "2.009877988310399125");
    assert_eq!(&out.imag().to_string()[..20], "4.535664430265577075");
}