#[cfg(test)]
use std::f64::consts::{E, PI, TAU};
#[cfg(test)]
use crate::math::NumOrString::{Complex, Str};
#[cfg(test)]
use crate::parse::get_func;
#[cfg(test)]
use crate::math::do_math;
#[test]
fn test_math()
{
    let input = "pi+tau*e/2i^(sqrt(2))/3*3-log(2-2i,-3+i)+sqrt(2)^(sqrt(2))";
    let output = get_func(input).unwrap();
    let expected = vec![Complex((PI, 0.0)),
                        Str("+".to_string()),
                        Complex((TAU, 0.0)),
                        Str("*".to_string()),
                        Complex((E, 0.0)),
                        Str("/".to_string()),
                        Complex((2.0, 0.0)),
                        Str("*".to_string()),
                        Complex((0.0, 1.0)),
                        Str("^".to_string()),
                        Str("(".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Complex((2.0, 0.0)),
                        Str(")".to_string()),
                        Str(")".to_string()),
                        Str("/".to_string()),
                        Complex((3.0, 0.0)),
                        Str("*".to_string()),
                        Complex((3.0, 0.0)),
                        Str("-".to_string()),
                        Str("log".to_string()),
                        Str("(".to_string()),
                        Complex((2.0, 0.0)),
                        Str("-".to_string()),
                        Complex((2.0, 0.0)),
                        Str("*".to_string()),
                        Complex((0.0, 1.0)),
                        Str(",".to_string()),
                        Complex((-3.0, 0.0)),
                        Str("+".to_string()),
                        Complex((0.0, 1.0)),
                        Str(")".to_string()),
                        Str("+".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Complex((2.0, 0.0)),
                        Str(")".to_string()),
                        Str("^".to_string()),
                        Str("(".to_string()),
                        Str("sqrt".to_string()),
                        Str("(".to_string()),
                        Complex((2.0, 0.0)),
                        Str(")".to_string()),
                        Str(")".to_string())];
    assert_eq!(output.len(), expected.len());
    for i in 0..expected.len()
    {
        match (&output[i], &expected[i])
        {
            (Complex((o, oi)), Complex((e, ei))) =>
            {
                assert_eq!(o, e);
                assert_eq!(oi, ei);
            }
            (Str(o), Str(e)) => assert_eq!(o, e),
            _ => panic!(),
        }
    }
    let answer = do_math(expected, false).unwrap();
    assert_eq!(answer, (0.2009877988310409, 4.535664430265577));
    // actually         0.2009877988310399 +4.535664430265577i  acording to wolfram alpha and it will be fixed in the future
}