use crate::{AngleType, Number, Options, Units};
use rug::{
    float::Constant::Pi,
    ops::{DivRounding, Pow},
    Complex, Float,
};
use std::collections::HashSet;
impl Units
{
    pub fn is_none(&self) -> bool
    {
        self.second == 0.0
            && self.meter == 0.0
            && self.kilogram == 0.0
            && self.ampere == 0.0
            && self.kelvin == 0.0
            && self.mole == 0.0
            && self.candela == 0.0
            && self.angle == 0.0
            && self.byte == 0.0
    }
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
            angle: self.angle + b.angle,
            byte: self.byte + b.byte,
            usd: self.usd + b.usd,
            unit: self.unit + b.unit,
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
            angle: self.angle - b.angle,
            byte: self.byte - b.byte,
            usd: self.usd - b.usd,
            unit: self.unit - b.unit,
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
            angle: self.angle * b,
            byte: self.byte * b,
            usd: self.usd * b,
            unit: self.unit * b,
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
            angle: self.angle / b,
            byte: self.byte / b,
            usd: self.usd / b,
            unit: self.unit / b,
        }
    }
    pub fn to_string(mut self, options: Options) -> String
    {
        let mut siunits = String::new();
        if !options.si_units
        {
            let farad = self
                .meter
                .div_floor(-2.0)
                .min(self.second.div_floor(4.0))
                .min(self.kilogram.div_floor(-1.0))
                .min(self.ampere.div_floor(2.0))
                .max(0.0);

            if farad != 0.0
            {
                self.meter += 2.0 * farad;
                self.second -= 4.0 * farad;
                self.kilogram += 1.0 * farad;
                self.ampere -= 2.0 * farad;
            }
            let ohm = self
                .meter
                .div_floor(2.0)
                .min(self.second.div_floor(-3.0))
                .min(self.kilogram.div_floor(1.0))
                .min(self.ampere.div_floor(-2.0))
                .max(0.0);
            if ohm != 0.0
            {
                self.meter -= 2.0 * ohm;
                self.second += 3.0 * ohm;
                self.kilogram -= 1.0 * ohm;
                self.ampere += 2.0 * ohm;
            }
            let henry = self
                .meter
                .div_floor(2.0)
                .min(self.second.div_floor(-2.0))
                .min(self.kilogram.div_floor(1.0))
                .min(self.ampere.div_floor(-2.0))
                .max(0.0);
            if henry != 0.0
            {
                self.meter -= 2.0 * henry;
                self.second += 2.0 * henry;
                self.kilogram -= 1.0 * henry;
                self.ampere += 2.0 * henry;
            }
            let volt = self
                .meter
                .div_floor(2.0)
                .min(self.second.div_floor(-3.0))
                .min(self.kilogram.div_floor(1.0))
                .min(self.ampere.div_floor(-1.0))
                .max(0.0);
            if volt != 0.0
            {
                self.meter -= 2.0 * volt;
                self.second += 3.0 * volt;
                self.kilogram -= 1.0 * volt;
                self.ampere += 1.0 * volt;
            }
            let watt = self
                .meter
                .div_floor(2.0)
                .min(self.second.div_floor(-3.0))
                .min(self.kilogram.div_floor(1.0))
                .max(0.0);
            if watt != 0.0
            {
                self.meter -= 2.0 * watt;
                self.second += 3.0 * watt;
                self.kilogram -= 1.0 * watt;
            }
            let joules = self
                .meter
                .div_floor(2.0)
                .min(self.second.div_floor(-2.0))
                .min(self.kilogram.div_floor(1.0))
                .max(0.0);
            if joules != 0.0
            {
                self.meter -= 2.0 * joules;
                self.second += 2.0 * joules;
                self.kilogram -= 1.0 * joules;
            }
            let newtons = self
                .meter
                .div_floor(1.0)
                .min(self.second.div_floor(-2.0))
                .min(self.kilogram.div_floor(1.0))
                .max(0.0);
            if newtons != 0.0
            {
                self.meter -= 1.0 * newtons;
                self.second += 2.0 * newtons;
                self.kilogram -= 1.0 * newtons;
            }
            let pascal = self
                .meter
                .div_floor(-1.0)
                .min(self.second.div_floor(-2.0))
                .min(self.kilogram.div_floor(1.0))
                .max(0.0);
            if pascal != 0.0
            {
                self.meter += 1.0 * pascal;
                self.second += 2.0 * pascal;
                self.kilogram -= 1.0 * pascal;
            }
            let tesla = self
                .ampere
                .div_floor(-1.0)
                .min(self.second.div_floor(-2.0))
                .min(self.kilogram.div_floor(1.0))
                .max(0.0);
            if tesla != 0.0
            {
                self.ampere += 1.0 * tesla;
                self.second += 2.0 * tesla;
                self.kilogram -= 1.0 * tesla;
            }
            let coulomb = self
                .ampere
                .div_floor(1.0)
                .min(self.second.div_floor(1.0))
                .max(0.0);
            if coulomb != 0.0
            {
                self.ampere -= 1.0 * coulomb;
                self.second -= 1.0 * coulomb;
            }
            if farad != 0.0
            {
                siunits.push_str(
                    &(" F".to_owned()
                        + &if farad != 1.0
                        {
                            format!("^{}", farad)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if ohm != 0.0
            {
                siunits.push_str(
                    &(" Ω".to_owned()
                        + &if ohm != 1.0
                        {
                            format!("^{}", ohm)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if henry != 0.0
            {
                siunits.push_str(
                    &(" H".to_owned()
                        + &if henry != 1.0
                        {
                            format!("^{}", henry)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if volt != 0.0
            {
                siunits.push_str(
                    &(" V".to_owned()
                        + &if volt != 1.0
                        {
                            format!("^{}", volt)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if watt != 0.0
            {
                siunits.push_str(
                    &(" W".to_owned()
                        + &if watt != 1.0
                        {
                            format!("^{}", watt)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if joules != 0.0
            {
                siunits.push_str(
                    &(" J".to_owned()
                        + &if joules != 1.0
                        {
                            format!("^{}", joules)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if newtons != 0.0
            {
                siunits.push_str(
                    &(" N".to_owned()
                        + &if newtons != 1.0
                        {
                            format!("^{}", newtons)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if pascal != 0.0
            {
                siunits.push_str(
                    &(" Pa".to_owned()
                        + &if pascal != 1.0
                        {
                            format!("^{}", pascal)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if tesla != 0.0
            {
                siunits.push_str(
                    &(" T".to_owned()
                        + &if tesla != 1.0
                        {
                            format!("^{}", tesla)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
            if coulomb != 0.0
            {
                siunits.push_str(
                    &(" C".to_owned()
                        + &if coulomb != 1.0
                        {
                            format!("^{}", coulomb)
                        }
                        else
                        {
                            String::new()
                        }),
                )
            }
        }
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            siunits,
            if self.meter != 0.0
            {
                " m".to_owned()
                    + &if self.meter != 1.0
                    {
                        format!("^{:.12}", self.meter)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
                        format!("^{:.12}", self.second)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
                        format!("^{:.12}", self.kilogram)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
                        format!("^{:.12}", self.ampere)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
                        format!("^{:.12}", self.kelvin)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
                        format!("^{:.12}", self.mole)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
                        format!("^{:.12}", self.candela)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
            if self.angle != 0.0
            {
                match options.angle
                {
                    AngleType::Degrees => " deg",
                    AngleType::Radians => " rad",
                    AngleType::Gradians => " grad",
                }
                .to_owned()
                    + &if self.angle != 1.0
                    {
                        format!("^{:.12}", self.angle)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
            if self.byte != 0.0
            {
                " B".to_owned()
                    + &if self.byte != 1.0
                    {
                        format!("^{:.12}", self.byte)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
            if self.usd != 0.0
            {
                " USD".to_owned()
                    + &if self.usd != 1.0
                    {
                        format!("^{:.12}", self.usd)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
            if self.unit != 0.0
            {
                " u".to_owned()
                    + &if self.unit != 1.0
                    {
                        format!("^{:.12}", self.unit)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
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
        )
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
            angle: 0.0,
            byte: 0.0,
            usd: 0.0,
            unit: 0.0,
        }
    }
}
pub fn is_unit(unit: &mut String) -> bool
{
    units().contains(unit.as_str())
        || (unit.len() > 2 && unit.ends_with('s') && {
            let is_true = units().contains(&unit[..unit.len().saturating_sub(1)]);
            is_true && {
                unit.pop();
                true
            }
        })
}
pub fn prefixes(mut unit: String, prec: (u32, u32)) -> (String, Complex)
{
    if is_unit(&mut unit)
    {
        return (unit, Complex::with_val(prec, 1));
    }
    let bak = unit.clone();
    let mut word = String::new();
    while !unit.is_empty() && word.len() < 7
    {
        word.push(unit.remove(0));
        match word.as_str()
        {
            "quetta" | "Q" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(30))
            }
            "ronna" | "R" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(27))
            }
            "yotta" | "Y" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(24))
            }
            "zetta" | "Z" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(21))
            }
            "exa" | "E" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(18))
            }
            "peta" | "P" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(15))
            }
            "tera" | "T" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(12))
            }
            "giga" | "G" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(9))
            }
            "mega" | "M" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(6))
            }
            "kilo" | "k" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(3))
            }
            "hecto" | "h" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(2))
            }
            "deca" | "da" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(1))
            }
            "deci" | "d" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-1))
            }
            "centi" | "c" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-2))
            }
            "milli" | "m" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-3))
            }
            "micro" | "μ" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-6))
            }
            "nano" | "n" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-9))
            }
            "pico" | "p" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-12))
            }
            "femto" | "f" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-15))
            }
            "atto" | "a" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-18))
            }
            "zepto" | "z" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-21))
            }
            "yocto" | "y" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-24))
            }
            "ronto" | "r" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-27))
            }
            "qecto" | "q" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-30))
            }
            "kibi" | "Ki" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(10))
            }
            "mebi" | "Mi" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(20))
            }
            "gibi" | "Gi" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(30))
            }
            "tebi" | "Ti" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(40))
            }
            "pebi" | "Pi" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(50))
            }
            "exbi" | "Ei" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(60))
            }
            "zebi" | "Zi" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(70))
            }
            "yobi" | "Yi" if is_unit(&mut unit) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(80))
            }
            _ =>
            {}
        }
    }
    (bak, Complex::with_val(prec, 1))
}
pub fn units() -> HashSet<&'static str>
{
    [
        "m",
        "meter",
        "s",
        "second",
        "A",
        "ampere",
        "K",
        "kelvin",
        "mol",
        "mole",
        "cd",
        "month",
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
        "degree",
        "rad",
        "radian",
        "grad",
        "gradian",
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
        "Wh",
        "Ah",
        "celsius",
        "fahrenheit",
        "litre",
        "L",
        "l",
        "lb",
        "pound",
        "inch",
        "in",
        "ft",
        "yd",
        "yard",
        "foot",
        "feet",
        "inches",
        "mi",
        "mile",
        "mph",
        "gram",
        "g",
        "h",
        "lumen",
        "lm",
        "lux",
        "lx",
        "byte",
        "B",
        "gray",
        "Gy",
        "sievert",
        "Sv",
        "katal",
        "kat",
        "bit",
        "steradian",
        "sr",
        "kph",
        "year",
        "ly",
        "nit",
        "nt",
        "usd",
        "USD",
        "$",
        "¢",
        "dollar",
        "cent",
        "atm",
        "psi",
        "bar",
        "tonne",
        "hectare",
        "ha",
        "acre",
        "ac",
        "ton",
        "oz",
        "gal",
        "gallon",
        "floz",
        "lbf",
        "parsec",
        "pc",
        "au",
        "arcsec",
        "arcmin",
        "micron",
        "unit",
        "u",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn to_unit(unit: String, mut num: Complex, options: Options) -> (Number, Option<Number>)
{
    let mut units = Units::default();
    let mut add = None;
    match unit.as_str()
    {
        "u" | "unit" => units.unit = 1.0,
        "m" | "meter" => units.meter = 1.0,
        "s" | "second" => units.second = 1.0,
        "A" | "ampere" => units.ampere = 1.0,
        "K" | "kelvin" => units.kelvin = 1.0,
        "mol" | "mole" => units.mole = 1.0,
        "cd" | "candela" => units.candela = 1.0,
        "byte" | "B" => units.byte = 1.0,
        "usd" | "USD" | "$" | "dollar" => units.usd = 1.0,
        "¢" | "cent" =>
        {
            num /= 100;
            units.usd = 1.0
        }
        "steradian" | "sr" =>
        {
            match options.angle
            {
                AngleType::Gradians => num *= 40000 / Float::with_val(options.prec, Pi).pow(2),
                AngleType::Degrees => num *= 32400 / Float::with_val(options.prec, Pi).pow(2),
                AngleType::Radians =>
                {}
            };
            units.angle = 2.0
        }
        "bit" =>
        {
            num /= 8;
            units.byte = 1.0;
        }
        "micron" =>
        {
            num /= 1000000;
            units.meter = 1.0
        }
        "g" | "gram" =>
        {
            num /= 1000;
            units.kilogram = 1.0
        }
        "nit" | "nt" =>
        {
            units.candela = 1.0;
            units.meter = -2.0
        }
        "gray" | "Gy" =>
        {
            units.second = -2.0;
            units.meter = 2.0;
        }
        "sievert" | "Sv" =>
        {
            units.second = -2.0;
            units.meter = 2.0;
        }
        "katal" | "kat" =>
        {
            units.second = -1.0;
            units.mole = 1.0;
        }
        "lumen" | "lm" =>
        {
            match options.angle
            {
                AngleType::Gradians => num *= 40000 / Float::with_val(options.prec, Pi).pow(2),
                AngleType::Degrees => num *= 32400 / Float::with_val(options.prec, Pi).pow(2),
                AngleType::Radians =>
                {}
            };
            units.angle = 2.0;
            units.candela = 1.0;
        }
        "lux" | "lx" =>
        {
            match options.angle
            {
                AngleType::Gradians => num *= 40000 / Float::with_val(options.prec, Pi).pow(2),
                AngleType::Degrees => num *= 32400 / Float::with_val(options.prec, Pi).pow(2),
                AngleType::Radians =>
                {}
            };
            units.angle = 2.0;
            units.candela = 1.0;
            units.meter = -2.0;
        }
        "J" | "joule" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "mph" =>
        {
            num *= 1397;
            num /= 3125;
            units.meter = 1.0;
            units.second = -1.0;
        }
        "kph" =>
        {
            num *= 5;
            num /= 18;
            units.meter = 1.0;
        }
        "mi" | "mile" =>
        {
            num *= 201168;
            num /= 125;
            units.meter = 1.0;
        }
        "yd" | "yard" =>
        {
            num *= 1143;
            num /= 1250;
            units.meter = 1.0;
        }
        "parsec" | "pc" =>
        {
            units.meter = 1.0;
            num *= 648000 / Float::with_val(options.prec, Pi);
            num *= 149597870700u64;
        }
        "au" =>
        {
            units.meter = 1.0;
            num *= 149597870700u64;
        }
        "ft" | "foot" | "feet" =>
        {
            units.meter = 1.0;
            num *= 381;
            num /= 1250;
        }
        "in" | "inch" | "inches" =>
        {
            units.meter = 1.0;
            num *= 127;
            num /= 5000;
        }
        "lb" | "pound" =>
        {
            units.kilogram = 1.0;
            num *= 45359237;
            num /= 100000000;
        }
        "L" | "l" | "litre" =>
        {
            num /= 1000;
            units.meter = 3.0;
        }
        "floz" =>
        {
            num *= 473176473;
            num /= 16000000000000u64;
            units.meter = 3.0;
        }
        "gallon" | "gal" =>
        {
            num *= 473176473;
            num /= 125000000000u64;
            units.meter = 3.0;
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
            add = Some(Number::from(
                Complex::with_val(options.prec, 5463) / 20,
                Some(unit),
            ));
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
            add = Some(Number::from(
                Complex::with_val(options.prec, 45967) / 180,
                Some(unit),
            ));
        }
        "Wh" =>
        {
            num *= 3600;
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "Ah" =>
        {
            num *= 3600;
            units.ampere = 1.0;
            units.second = 1.0;
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
        "atm" =>
        {
            num *= 101325;
            units.kilogram = 1.0;
            units.meter = -1.0;
            units.second = -2.0;
        }
        "psi" =>
        {
            num *= 6894757;
            num /= 1000;
            units.kilogram = 1.0;
            units.meter = -1.0;
            units.second = -2.0;
        }
        "bar" =>
        {
            num *= 100000;
            units.kilogram = 1.0;
            units.meter = -1.0;
            units.second = -2.0;
        }
        "tonne" =>
        {
            num *= 1000;
            units.kilogram = 1.0;
        }
        "hectare" | "ha" =>
        {
            num *= 10000;
            units.meter = 2.0;
        }
        "acre" | "ac" =>
        {
            num *= 316160658;
            num /= 78125;
            units.meter = 2.0;
        }
        "ton" =>
        {
            num *= 45359237;
            num /= 50000;
            units.kilogram = 1.0;
        }
        "oz" =>
        {
            num *= 45359237;
            num /= 1600000000;
            units.kilogram = 1.0;
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
        "h" | "hour" =>
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
        "month" =>
        {
            num *= 2629800;
            units.second = 1.0
        }
        "year" =>
        {
            num *= 31557600;
            units.second = 1.0;
        }
        "ly" =>
        {
            num *= 9460730472580800u128;
            units.meter = 1.0;
        }
        "N" | "newton" =>
        {
            units.kilogram = 1.0;
            units.meter = 1.0;
            units.second = -2.0;
        }
        "lbf" =>
        {
            num *= 8896443230521u64;
            num /= 2000000000000u64;
            units.kilogram = 1.0;
            units.meter = 1.0;
            units.second = -2.0;
        }
        "C" | "coulomb" =>
        {
            units.ampere = 1.0;
            units.second = 1.0;
        }
        "arcmin" =>
        {
            match options.angle
            {
                AngleType::Degrees =>
                {
                    num /= 60;
                }
                AngleType::Gradians =>
                {
                    num *= 200;
                    num /= 180;
                    num /= 60;
                }
                AngleType::Radians =>
                {
                    num *= Float::with_val(options.prec, Pi) / 180;
                    num /= 60;
                }
            };
            units.angle = 1.0;
        }
        "arcsec" =>
        {
            match options.angle
            {
                AngleType::Degrees =>
                {
                    num /= 3600;
                }
                AngleType::Gradians =>
                {
                    num *= 200;
                    num /= 180;
                    num /= 3600;
                }
                AngleType::Radians =>
                {
                    num *= Float::with_val(options.prec, Pi) / 180;
                    num /= 3600;
                }
            };
            units.angle = 1.0;
        }
        "°" | "deg" | "degree" =>
        {
            match options.angle
            {
                AngleType::Degrees =>
                {}
                AngleType::Gradians =>
                {
                    num *= 200;
                    num /= 180
                }
                AngleType::Radians => num *= Float::with_val(options.prec, Pi) / 180,
            };
            units.angle = 1.0;
        }
        "rad" | "radian" =>
        {
            match options.angle
            {
                AngleType::Degrees => num *= 180 / Float::with_val(options.prec, Pi),
                AngleType::Gradians => num *= 200 / Float::with_val(options.prec, Pi),
                AngleType::Radians =>
                {}
            };
            units.angle = 1.0
        }
        "grad" | "gradian" =>
        {
            match options.angle
            {
                AngleType::Degrees =>
                {
                    num *= 180;
                    num /= 200
                }
                AngleType::Gradians =>
                {}
                AngleType::Radians => num *= Float::with_val(options.prec, Pi) / 200,
            };
            units.angle = 1.0;
        }
        _ =>
        {}
    }
    (Number::from(num, Some(units)), add)
}
