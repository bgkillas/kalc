use crate::Units;
use rug::Complex;
use std::{collections::HashSet, fmt};
impl Units
{
    pub fn mul(&self, b: &Self) -> Self
    {
        Self {
            second: self.second + b.second,
            meter: self.meter + b.meter,
            kilogram: self.kilogram + b.kilogram,
            ampere: self.ampere + b.ampere,
            kelvin: self.kelvin + b.kelvin,
            mole: self.mole + b.mole,
            candela: self.candela + b.candela,
        }
    }
    pub fn div(&self, b: &Self) -> Self
    {
        Self {
            second: self.second - b.second,
            meter: self.meter - b.meter,
            kilogram: self.kilogram - b.kilogram,
            ampere: self.ampere - b.ampere,
            kelvin: self.kelvin - b.kelvin,
            mole: self.mole - b.mole,
            candela: self.candela - b.candela,
        }
    }
    pub fn pow(&self, b: f64) -> Self
    {
        Self {
            second: self.second * b,
            meter: self.meter * b,
            kilogram: self.kilogram * b,
            ampere: self.ampere * b,
            kelvin: self.kelvin * b,
            mole: self.mole * b,
            candela: self.candela * b,
        }
    }
    pub fn root(&self, b: f64) -> Self
    {
        Self {
            second: self.second / b,
            meter: self.meter / b,
            kilogram: self.kilogram / b,
            ampere: self.ampere / b,
            kelvin: self.kelvin / b,
            mole: self.mole / b,
            candela: self.candela / b,
        }
    }
}
impl Default for Units
{
    fn default() -> Self
    {
        Self {
            second: 0.0,
            meter: 0.0,
            kilogram: 0.0,
            ampere: 0.0,
            kelvin: 0.0,
            mole: 0.0,
            candela: 0.0,
        }
    }
}
impl fmt::Display for Units
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            if self.meter != 0.0
            {
                " m".to_owned()
                    + &if self.meter != 1.0
                    {
                        "^".to_owned() + &self.meter.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.second != 0.0
            {
                " s".to_owned()
                    + &if self.second != 1.0
                    {
                        "^".to_owned() + &self.second.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.kilogram != 0.0
            {
                " kg".to_owned()
                    + &if self.kilogram != 1.0
                    {
                        "^".to_owned() + &self.kilogram.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.ampere != 0.0
            {
                " A".to_owned()
                    + &if self.ampere != 1.0
                    {
                        "^".to_owned() + &self.ampere.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.kelvin != 0.0
            {
                " K".to_owned()
                    + &if self.kelvin != 1.0
                    {
                        "^".to_owned() + &self.kelvin.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.mole != 0.0
            {
                " mol".to_owned()
                    + &if self.mole != 1.0
                    {
                        "^".to_owned() + &self.mole.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.candela != 0.0
            {
                " cd".to_owned()
                    + &if self.candela != 1.0
                    {
                        "^".to_owned() + &self.candela.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            }
        )
    }
}
pub fn units() -> HashSet<&'static str>
{
    ["m", "s", "kg", "A", "K", "mol", "cd", "J", "min", "C", "N"]
        .iter()
        .cloned()
        .collect::<HashSet<&str>>()
}
pub fn to_unit(unit: String, prec: (u32, u32)) -> (Complex, Option<Units>)
{
    let mut num = 1;
    let mut units = Units::default();
    match unit.as_str()
    {
        "m" => units.meter = 1.0,
        "s" => units.second = 1.0,
        "kg" => units.kilogram = 1.0,
        "A" => units.ampere = 1.0,
        "K" => units.kelvin = 1.0,
        "mol" => units.mole = 1.0,
        "cd" => units.candela = 1.0,
        "J" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "min" =>
        {
            units.second = 1.0;
            num = 60;
        }
        "N" =>
        {
            units.kilogram = 1.0;
            units.meter = 1.0;
            units.second = -2.0;
        }
        "C" =>
        {
            units.ampere = 1.0;
            units.second = 1.0;
        }
        _ =>
        {}
    }
    (Complex::with_val(prec, num), Some(units))
}
