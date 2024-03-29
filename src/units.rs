use crate::Units;
use rug::{float::Constant::Pi, ops::Pow, Complex};
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
            radian: self.radian + b.radian,
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
            radian: self.radian - b.radian,
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
            radian: self.radian * b,
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
            radian: self.radian / b,
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
            radian: 0.0,
        }
    }
}
impl fmt::Display for Units
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
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
            },
            if self.radian != 0.0
            {
                " rad".to_owned()
                    + &if self.radian != 1.0
                    {
                        "^".to_owned() + &self.radian.to_string()
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
pub fn prefixes(mut unit: String) -> (String, isize)
{
    let bak = unit.clone();
    let mut word = String::new();
    while !unit.is_empty() && word.len() < 7
    {
        word.push(unit.remove(0));
        match word.as_str()
        {
            "quetta" | "Q" if units().contains(unit.as_str()) => return (unit, 30),
            "ronna" | "R" if units().contains(unit.as_str()) => return (unit, 27),
            "yotta" | "Y" if units().contains(unit.as_str()) => return (unit, 24),
            "zetta" | "Z" if units().contains(unit.as_str()) => return (unit, 21),
            "exa" | "E" if units().contains(unit.as_str()) => return (unit, 18),
            "peta" | "P" if units().contains(unit.as_str()) => return (unit, 15),
            "tera" | "T" if units().contains(unit.as_str()) => return (unit, 12),
            "giga" | "G" if units().contains(unit.as_str()) => return (unit, 9),
            "mega" | "M" if units().contains(unit.as_str()) => return (unit, 6),
            "kilo" | "k" if units().contains(unit.as_str()) => return (unit, 3),
            "hecto" | "h" if units().contains(unit.as_str()) => return (unit, 2),
            "deca" | "da" if units().contains(unit.as_str()) => return (unit, 1),
            "deci" | "d" if units().contains(unit.as_str()) => return (unit, -1),
            "centi" | "c" if units().contains(unit.as_str()) => return (unit, -2),
            "milli" | "m" if units().contains(unit.as_str()) => return (unit, -3),
            "micro" | "μ" if units().contains(unit.as_str()) => return (unit, -6),
            "nano" | "n" if units().contains(unit.as_str()) => return (unit, -9),
            "pico" | "p" if units().contains(unit.as_str()) => return (unit, -12),
            "femto" | "f" if units().contains(unit.as_str()) => return (unit, -15),
            "atto" | "a" if units().contains(unit.as_str()) => return (unit, -18),
            "zepto" | "z" if units().contains(unit.as_str()) => return (unit, -21),
            "yocto" | "y" if units().contains(unit.as_str()) => return (unit, -24),
            "ronto" | "r" if units().contains(unit.as_str()) => return (unit, -27),
            "qecto" | "q" if units().contains(unit.as_str()) => return (unit, -30),
            _ =>
            {}
        }
    }
    (bak, 0)
}
pub fn units() -> HashSet<&'static str>
{
    [
        "m",
        "meter",
        "s",
        "second",
        "kg",
        "kilogram",
        "A",
        "ampere",
        "K",
        "kelvin",
        "mol",
        "mole",
        "cd",
        "candela",
        "J",
        "joule",
        "min",
        "minute",
        "C",
        "coulomb",
        "N",
        "newton",
        "°",
        "deg",
        "degrees",
        "rad",
        "radians",
        "grad",
        "gradians",
        "hour",
        "day",
        "week",
        "Ω",
        "ohm",
        "V",
        "volt",
        "voltage",
        "Hz",
        "hertz",
        "Pa",
        "pascal",
        "W",
        "watt",
        "farad",
        "F",
        "siemens",
        "S",
        "weber",
        "Wb",
        "T",
        "tesla",
        "H",
        "henry",
        "°C",
        "°F",
        "kWh",
        "celsius",
        "fahrenheit",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn to_unit(
    mut unit: String,
    prec: (u32, u32),
) -> ((Complex, Option<Units>), Option<(Complex, Option<Units>)>)
{
    let pow;
    (unit, pow) = prefixes(unit);
    let mut num = Complex::with_val(prec, 1);
    let mut units = Units::default();
    let mut add = None;
    match unit.as_str()
    {
        "m" | "meter" => units.meter = 1.0,
        "s" | "second" => units.second = 1.0,
        "kg" | "kilogram" => units.kilogram = 1.0,
        "A" | "ampere" => units.ampere = 1.0,
        "K" | "kelvin" => units.kelvin = 1.0,
        "mol" | "mole" => units.mole = 1.0,
        "cd" | "candela" => units.candela = 1.0,
        "J" | "joule" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "Hz" | "hertz" => units.second = -1.0,
        "V" | "volt" | "voltage" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -3.0;
            units.ampere = -1.0;
        }
        "°C" | "celsius" =>
        {
            units.kelvin = 1.0;
            let unit = Units {
                kelvin: 1.0,
                ..Units::default()
            };
            add = Some((Complex::with_val(prec, 273.15), Some(unit)));
        }
        "°F" | "fahrenheit" =>
        {
            num *= 5;
            num /= 9;
            units.kelvin = 1.0;
            let unit = Units {
                kelvin: 1.0,
                ..Units::default()
            };
            add = Some((Complex::with_val(prec, 45967) / 180, Some(unit)));
        }
        "kWh" =>
        {
            num *= 3600000;
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "T" | "tesla" =>
        {
            units.kilogram = 1.0;
            units.second = -2.0;
            units.ampere = -1.0;
        }
        "H" | "henry" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
            units.ampere = -2.0;
        }
        "weber" | "Wb" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
            units.ampere = -1.0;
        }
        "siemens" | "S" =>
        {
            units.kilogram = -1.0;
            units.meter = -2.0;
            units.second = 3.0;
            units.ampere = 2.0;
        }
        "F" | "farad" =>
        {
            units.kilogram = -1.0;
            units.meter = -2.0;
            units.second = 4.0;
            units.ampere = 2.0;
        }
        "W" | "watt" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -3.0;
        }
        "Pa" | "pascal" =>
        {
            units.kilogram = 1.0;
            units.meter = -1.0;
            units.second = -2.0;
        }
        "Ω" | "ohm" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -3.0;
            units.ampere = -2.0;
        }
        "min" | "minute" =>
        {
            units.second = 1.0;
            num *= 60;
        }
        "hour" =>
        {
            units.second = 1.0;
            num *= 3600;
        }
        "day" =>
        {
            units.second = 1.0;
            num *= 86400;
        }
        "week" =>
        {
            units.second = 1.0;
            num *= 604800;
        }
        "N" | "newton" =>
        {
            units.kilogram = 1.0;
            units.meter = 1.0;
            units.second = -2.0;
        }
        "C" | "coulomb" =>
        {
            units.ampere = 1.0;
            units.second = 1.0;
        }
        "°" | "deg" | "degrees" =>
        {
            num *= Complex::with_val(prec, Pi) / 180;
            units.radian = 1.0;
        }
        "rad" | "radians" => units.radian = 1.0,
        "grad" | "gradians" =>
        {
            num *= Complex::with_val(prec, Pi) / 200;
            units.radian = 1.0;
        }
        _ =>
        {}
    }
    (
        (num * Complex::with_val(prec, 10).pow(pow), Some(units)),
        add,
    )
}
