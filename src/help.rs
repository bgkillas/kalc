const MSG: &str = include_str!("../README.md");
pub fn help()
{
    let lines = MSG.lines();
    let p = lines.clone().position(|l| l == "# usage").unwrap()+3;
    let e = lines.clone().skip(p).position(|l| l == "```").unwrap() - 1;
    println!("{}", lines.skip(p).take(e).map(|l| l.to_string()).collect::<Vec<String>>().join("\x1b[G\n"));
}
fn all_units() -> &'static str
{
    "\"m\" | \"meter\"\x1b[G\n\
\"s\" | \"second\"\x1b[G\n\
\"A\" | \"ampere\"\x1b[G\n\
\"K\" | \"kelvin\"\x1b[G\n\
\"u\" | \"unit\"\x1b[G\n\
\"mol\" | \"mole\"\x1b[G\n\
\"cd\" | \"candela\"\x1b[G\n\
\"g\" | \"gram\"\x1b[G\n\
\"J\" | \"joule\"\x1b[G\n\
\"mph\"\x1b[G\n\
\"mi\" | \"mile\"\x1b[G\n\
\"yd\" | \"yard\"\x1b[G\n\
\"ft\" | \"foot\"\x1b[G\n\
\"in\" | \"inch\"\x1b[G\n\
\"lb\" | \"pound\"\x1b[G\n\
\"L\" | \"l\" | \"litre\"\x1b[G\n\
\"Hz\" | \"hertz\"\x1b[G\n\
\"V\" | \"volt\" | \"voltage\"\x1b[G\n\
\"°C\" | \"celsius\"\x1b[G\n\
\"°F\" | \"fahrenheit\"\x1b[G\n\
\"Wh\"\x1b[G\n\
\"Ah\"\x1b[G\n\
\"year\"\x1b[G\n\
\"month\"\x1b[G\n\
\"ly\"\x1b[G\n\
\"kph\"\x1b[G\n\
\"T\" | \"tesla\"\x1b[G\n\
\"H\" | \"henry\"\x1b[G\n\
\"weber\" | \"Wb\"\x1b[G\n\
\"siemens\" | \"S\"\x1b[G\n\
\"F\" | \"farad\"\x1b[G\n\
\"W\" | \"watt\"\x1b[G\n\
\"Pa\" | \"pascal\"\x1b[G\n\
\"Ω\" | \"ohm\"\x1b[G\n\
\"min\" | \"minute\"\x1b[G\n\
\"h\" | \"hour\"\x1b[G\n\
\"day\"\x1b[G\n\
\"week\"\x1b[G\n\
\"N\" | \"newton\"\x1b[G\n\
\"C\" | \"coulomb\"\x1b[G\n\
\"°\" | \"deg\" | \"degrees\"\x1b[G\n\
\"arcsec\"\x1b[G\n\
\"arcmin\"\x1b[G\n\
\"rad\" | \"radians\"\x1b[G\n\
\"grad\" | \"gradians\"\x1b[G\n\
\"lumen\" | \"lm\"\x1b[G\n\
\"lux\" | \"lx\"\x1b[G\n\
\"nit\" | \"nt\"\x1b[G\n\
\"byte\" | \"B\"\x1b[G\n\
\"gray\" | \"Gy\"\x1b[G\n\
\"sievert\" | \"Sv\"\x1b[G\n\
\"katal\" | \"kat\"\x1b[G\n\
\"bit\"\x1b[G\n\
\"steradian\" | \"sr\"\x1b[G\n\
\"atm\"\x1b[G\n\
\"psi\"\x1b[G\n\
\"bar\"\x1b[G\n\
\"tonne\"\x1b[G\n\
\"hectare\" | \"ha\"\x1b[G\n\
\"acre\" | \"ac\"\x1b[G\n\
\"ton\"\x1b[G\n\
\"oz\"\x1b[G\n\
\"gallon\" | \"gal\"\x1b[G\n\
\"lbf\"\x1b[G\n\
\"parsec\" | \"pc\"\x1b[G\n\
\"au\"\x1b[G\n\
\"floz\"\x1b[G\n\
\"AUD\",\"CAD\",\"CNY\",\"EUR\",\"GBP\",\"HKD\",\"IDR\",\"INR\",\"JPY\",\"KRW\",\"MYR\",\"NZD\",\"PHP\",\"SGD\",\"THB\",\"TWD\",\"VND\",\"BGN\",\"BRL\",\"CHF\",\"CLP\",\"CZK\",\"DKK\",\"HUF\",\"ILS\",\"ISK\",\"MXN\",\"NOK\",\"PLN\",\"RON\",\"SEK\",\"TRY\",\"UAH\",\"ZAR\",\"EGP\",\"JOD\",\"LBP\",\"AED\",\"MDL\",\"RSD\",\"RUB\",\"AMD\",\"AZN\",\"BDT\",\"DOP\",\"DZD\",\"GEL\",\"IQD\",\"IRR\",\"KGS\",\"KZT\",\"LYD\",\"MAD\",\"PKR\",\"SAR\",\"TJS\",\"TMT\",\"TND\",\"UZS\",\"XAF\",\"XOF\",\"BYN\",\"PEN\",\"VES\",\"ARS\",\"BOB\",\"COP\",\"CRC\",\"HTG\",\"PAB\",\"PYG\",\"UYU\",\"NGN\",\"AFN\",\"ALL\",\"ANG\",\"AOA\",\"AWG\",\"BAM\",\"BBD\",\"BHD\",\"BIF\",\"BND\",\"BSD\",\"BWP\",\"BZD\",\"CDF\",\"CUP\",\"CVE\",\"DJF\",\"ERN\",\"ETB\",\"FJD\",\"GHS\",\"GIP\",\"GMD\",\"GNF\",\"GTQ\",\"GYD\",\"HNL\",\"JMD\",\"KES\",\"KHR\",\"KMF\",\"KWD\",\"LAK\",\"LKR\",\"LRD\",\"LSL\",\"MGA\",\"MKD\",\"MMK\",\"MNT\",\"MOP\",\"MRU\",\"MUR\",\"MVR\",\"MWK\",\"MZN\",\"NAD\",\"NIO\",\"NPR\",\"OMR\",\"PGK\",\"QAR\",\"RWF\",\"SBD\",\"SCR\",\"SDG\",\"SOS\",\"SRD\",\"SSP\",\"STN\",\"SVC\",\"SYP\",\"SZL\",\"TOP\",\"TTD\",\"TZS\",\"UGX\",\"VUV\",\"WST\",\"XCD\",\"XPF\",\"YER\",\"ZMW\""
}
pub fn help_for(thing: &str) -> String
{
    match thing
    {
        "W" | "productlog" | "lambertw" =>
        {
            "W(k,z), W(z)\x1b[G\n\
            kth branch of the inverse of z*e^z\x1b[G\n\
            given one argument assumes k=0"
        }
        "atan" | "arctan" | "atan2" =>
        {
            "atan(y/x), atan(x,y), atan2(y,x)\x1b[G\n\
        inverse of tan(z)\x1b[G\n\
        using the 2 arg version gives you an angle from 0 instead of from the x axis\x1b[G\n\
        example using cardinal directions: atan(-2,-3)=-2.15 E->N, atan(-3/-2)=0.98 W->S"
        }
        "->"|"to"=>"divides the left number and unit by the right unit after the '+'/'-' step of order of operations, bit more complex for fereignheit/celsius",
        "units" => "see \"units list\" for a list of all units supported\x1b[G\nsupports metric and binary prefixes, \"units\" function extracts the units of the given input",
        "units list" =>
            all_units(),
        "help" => "W, atan\x1b[G\nunits, ->",
        "point"|"points"=>". - dot\x1b[G\n\
+ - plus\x1b[G\n\
x - cross\x1b[G\n\
* - star\x1b[G\n\
s - empty square\x1b[G\n\
S - filled square\x1b[G\n\
o - empty circle\x1b[G\n\
O - filled circle\x1b[G\n\
t - empty triangle\x1b[G\n\
T - filled triangle\x1b[G\n\
d - empty del (upside down triangle)\x1b[G\n\
D - filled del (upside down triangle)\x1b[G\n\
r - empty rhombus\x1b[G\n\
R - filled rhombus",
        "" => "",
        _ => "not in database",
    }
    .to_string()
}