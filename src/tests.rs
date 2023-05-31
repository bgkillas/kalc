use std::f64::consts::{E, PI, TAU};
use crate::math::Complex::{Num, Str};
use crate::parse::{get_func, get_vars, input_var};
use crate::math::do_math;
#[test]
fn test_math()
{
    let input = input_var("pi+tau*e/2i^(sqrt(2))/3*3-log(2-2i,-3+i)+sqrt(2)^(sqrt(2))", &get_vars(true));
    let output = get_func(&input).unwrap();
    let expected = vec![Num((PI, 0.0)),
                        Str("+".to_string()),
                        Num((TAU, 0.0)),
                        Str("*".to_string()),
                        Num((E, 0.0)),
                        Str("/".to_string()),
                        Num((2.0, 0.0)),
                        Str("*".to_string()),
                        Num((0.0, 1.0)),
                        Str("^".to_string()),
                        Str("(".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Num((2.0, 0.0)),
                        Str(")".to_string()),
                        Str(")".to_string()),
                        Str("/".to_string()),
                        Num((3.0, 0.0)),
                        Str("*".to_string()),
                        Num((3.0, 0.0)),
                        Str("-".to_string()),
                        Str("log".to_string()),
                        Str("(".to_string()),
                        Num((2.0, 0.0)),
                        Str("-".to_string()),
                        Num((2.0, 0.0)),
                        Str("*".to_string()),
                        Num((0.0, 1.0)),
                        Str(",".to_string()),
                        Num((-3.0, 0.0)),
                        Str("+".to_string()),
                        Num((0.0, 1.0)),
                        Str(")".to_string()),
                        Str("+".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Num((2.0, 0.0)),
                        Str(")".to_string()),
                        Str("^".to_string()),
                        Str("(".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Num((2.0, 0.0)),
                        Str(")".to_string()),
                        Str(")".to_string())];
    let answer = do_math(expected, false).unwrap();
    assert_eq!(do_math(output, false).unwrap(), answer);
    assert_eq!(answer, (0.2009877988310409, 4.535664430265577));
    // actually         0.2009877988310399 +4.535664430265577i  acording to wolfram alpha and it will be fixed in the future
}