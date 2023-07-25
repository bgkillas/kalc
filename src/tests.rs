use crate::{
    complex::{
        Float,
        NumStr::{Num, Str},
    },
    math::do_math,
    options::AngleType::Radians,
    parse::get_func,
    vars::{get_vars, input_var},
    Options,
};
use rug::{float::Constant::Pi, Complex};
#[test]
fn test_math()
{
    let input = input_var(
        "pi+tau*e/2i^(sqrt(2))/3*3-log(2-2i,-3+i)+sqrt(2)^(sqrt(2))",
        &get_vars(512),
        None,
        Options::default(),
    );
    let output = get_func(&input, Options::default()).unwrap();
    let expected = vec![
        Num(Complex::with_val(512, Pi)),
        Str("+".to_string()),
        Num(2 * Complex::with_val(512, Pi)),
        Str("*".to_string()),
        Num(Complex::with_val(512, 1).exp()),
        Str("/".to_string()),
        Num(Complex::with_val(512, 2.0)),
        Str("*".to_string()),
        Num(Complex::with_val(512, (0.0, 1.0))),
        Str("^".to_string()),
        Str("(".to_string()),
        Str("sqrt".to_string()),
        Str("(".to_string()),
        Num(Complex::with_val(512, 2.0)),
        Str(")".to_string()),
        Str(")".to_string()),
        Str("/".to_string()),
        Num(Complex::with_val(512, 3.0)),
        Str("*".to_string()),
        Num(Complex::with_val(512, 3.0)),
        Str("-".to_string()),
        Str("log".to_string()),
        Str("(".to_string()),
        Num(Complex::with_val(512, 2.0)),
        Str("-".to_string()),
        Num(Complex::with_val(512, 2.0)),
        Str("*".to_string()),
        Num(Complex::with_val(512, (0.0, 1.0))),
        Str(",".to_string()),
        Num(Complex::with_val(512, -3.0)),
        Str("+".to_string()),
        Num(Complex::with_val(512, (0.0, 1.0))),
        Str(")".to_string()),
        Str("+".to_string()),
        Str("sqrt".to_string()),
        Str("(".to_string()),
        Num(Complex::with_val(512, 2.0)),
        Str(")".to_string()),
        Str("^".to_string()),
        Str("(".to_string()),
        Str("sqrt".to_string()),
        Str("(".to_string()),
        Num(Complex::with_val(512, 2.0)),
        Str(")".to_string()),
        Str(")".to_string()),
    ];
    let out = do_math(output, Radians, 512).unwrap().num().unwrap();
    let answer = do_math(expected, Radians, 512).unwrap().num().unwrap();
    assert_eq!(out.real().to_string(), answer.real().to_string());
    assert_eq!(out.imag().to_string(), answer.imag().to_string());
    assert_eq!(&out.real().to_string()[..20], "2.009877988310399125");
    assert_eq!(&out.imag().to_string()[..20], "4.535664430265577075");
}
#[test]
fn test_complex()
{
    let a = (4.0, -2.0);
    let b = (-5.0, 4.0);
    assert_eq!(a.add(b), (-1.0, 2.0));
    assert_eq!(a.sub(b), (9.0, -6.0));
    assert_eq!(a.mul(b), (-12.0, 26.0));
    assert_eq!(a.div(b), (-0.6829268292682927, -0.14634146341463414));
    assert_eq!(a.sin(), (-2.8472390868488278, 2.370674169352002));
    assert_eq!(a.cos(), (-2.4591352139173837, -2.7448170067921542));
    assert_eq!(a.tan(), (0.03642336924740369, -1.004682312190235));
}