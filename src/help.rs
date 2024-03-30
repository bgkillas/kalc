pub fn help()
{
    print!(
             "Usage: kalc [FLAGS] equation_1 equation_2 equation_3...\x1b[G\n\
FLAGS: --help (this message)\x1b[G\n\
--help {{thing}} to get more detail on a function/option/feature, --help help to list all \"things\"\x1b[G\n\
--interactive/-i allows interaction after finishing the equations given\x1b[G\n\
--units toggles units\x1b[G\n\
--label=[x],[y],[z] sets the labels for the graphs x/y/z axis\x1b[G\n\
--tau fractions are shown in tau instead of pi\x1b[G\n\
--deg compute in degrees\x1b[G\n\
--rad compute in radians\x1b[G\n\
--grad compute in gradians\x1b[G\n\
--2d=[num] number of points to graph in 2D\x1b[G\n\
--3d=[x],[y] number of points to graph in 3D\x1b[G\n\
--xr=[min],[max] x range for graphing\x1b[G\n\
--yr=[min],[max] y range for graphing\x1b[G\n\
--zr=[min],[max] z range for graphing\x1b[G\n\
--vxr=[min],[max] x range for graphing, graph view override, useful for parametric\x1b[G\n\
--vyr=[min],[max] y range for graphing, graph view override, useful for parametric\x1b[G\n\
--vzr=[min],[max] z range for graphing, graph view override, useful for parametric\x1b[G\n\
--vrange=[num] sets all ranges to [-num],[num], graph view override, useful for parametric\x1b[G\n\
--range=[num] sets all ranges to [-num],[num]\x1b[G\n\
--point [char] point style for graphing\x1b[G\n\
--sci toggles scientific notation\x1b[G\n\
--base=[input],[output] sets the numbers base from 2 to 36\x1b[G\n\
--ticks=[num] sets amount of ticks, -2 will be auto, -1 will be at every whole number, 0 will be none\x1b[G\n\
--onaxis toggles showing the ticks on the x/y/z axis\x1b[G\n\
--prompt toggles the prompt\x1b[G\n\
--color toggles color output, toggled by default when running from arguments\x1b[G\n\
--comma toggles comma seperation\x1b[G\n\
--graph toggles graphing\x1b[G\n\
--vars disables default variables and kalc.vars\x1b[G\n\
--default sets to default settings\x1b[G\n\
--line toggles line graphing\x1b[G\n\
--rt toggles real time printing\x1b[G\n\
--polar toggles displaying polar vectors\x1b[G\n\
--frac toggles fraction display\x1b[G\n\
--frac_iter=[num] how many iterations to check for fractions\x1b[G\n\
--prec=[num] sets the output precision(default 512)\x1b[G\n\
--graphprec=[num] sets the graph precision(default 64)\x1b[G\n\
--deci=[num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\x1b[G\n\
--def ignores config file\x1b[G\n\
--multi toggles multi line display for matrixes\x1b[G\n\
--tabbed toggles tabbed display for matrixes\x1b[G\n\
--depth display 2d complex graphs in 3d with imag #'s going up/down on the z axis\x1b[G\n\
--surface displays a colored surface(based on z value) for 3d graphing, only supports 1 graph\x1b[G\n\
--flat display 2d complex graphs like they are on the 2d number line\x1b[G\n\
--small_e use small e notation, like 5e2=5*10^2, instead of capital 'E' for scientific notation. only works with a number before and number or '-' sign after the 'e' otherwise assumes euler number\x1b[G\n\
--scalegraph scales the y part of a 2d graph to the users screen size\x1b[G\x1b[G\n\n\
- flags can be executed in runtime just without the dashes\x1b[G\n\
- \"colors=\" to see color settings\x1b[G\n\
- \"exit\" to exit the program\x1b[G\n\
- \"clear\" to clear the screen\x1b[G\n\
- \"history [arg]\" to see the history, arg searches for the arg it if specified\x1b[G\n\
- \"vars\" to list all variables\x1b[G\n\
- \"option/var;function\" to set a temporal option/var, example: \"a=45;deg;sin(a)\" = sqrt(2)/2\x1b[G\n\
- \"f(x)=var;function\" to set a temporal var when defining function, example: \"f(x)=a=2:ax\" = f(x)=2x\x1b[G\n\
- \"_\" to use the previous answer\x1b[G\n\
- \"a={{expr}}\" to define a variable\x1b[G\n\
- \"f(x)=...\" to define a function\x1b[G\n\
- \"f(x,y,z...)=...\" to define a multi variable function\x1b[G\n\
- \"...=\" display parsed input, show values of stuff like xr/deci/prec etc\x1b[G\n\
- \"f...=null\" to delete a function or variable\x1b[G\n\
- \"{{x,y,z...}}\" to define a cartesian vector\x1b[G\n\
- \"[radius,theta,phi]\" to define a polar vector (same as car{{vec}})\x1b[G\n\
- \"f(x)#g(x)\" to graph multiple things\x1b[G\n\
- \"{{vec}}#\" to graph a vector\x1b[G\n\
- \"{{mat}}#\" to graph a matrix\x1b[G\n\
- \"number#\" to graph a complex number\x1b[G\n\
- \"{{x,y}}\" to graph a parametric equation, example: {{cos(x),sin(x)}} unit circle, {{f(x)cos(x),f(x)sin(x)}} for polar graph\x1b[G\n\
- \"{{x,y,z}}\" to graph a parametric equation in 3d, example: {{cos(x),sin(x),x}} helix, {{sin(x)cos(y),sin(x)sin(y),cos(x)}} sphere\x1b[G\n\
- \"{{{{a,b,c}},{{d,e,f}},{{g,h,i}}}}\" to define a 3x3 matrix\x1b[G\n\
- \"rnd\" to generate a random number\x1b[G\n\
- Alt+Enter will not graph whatever is present\x1b[G\n\
- \"help {{thing}}\" to get more detail on a function/option/feature\x1b[G\n\
- \"help help\" to list all things to query\x1b[G\n\n\
Operators:\x1b[G\n\
- +, -, *, /, //, ^, ^^, %(modulo), <, >, <=, >=, |x|, ±/+-\x1b[G\n\
- !x (subfact), x! (fact), x!! (doublefact)\x1b[G\n\
- && (and), || (or), == (equals), != (not equals)\x1b[G\n\
- >> (right shift), << (left shift)\x1b[G\n\
- ->, to (unit conversions, ie 2m->yd)\x1b[G\n\n\
Functions:\x1b[G\n\
- sin, cos, tan, asin, acos, atan, atan(x,y)\x1b[G\n\
- csc, sec, cot, acsc, asec, acot\x1b[G\n\
- sinh, cosh, tanh, asinh, acosh, atanh\x1b[G\n\
- csch, sech, coth, acsch, asech, acoth\x1b[G\n\
- sqrt, cbrt, square, cube, quadratic(a,b,c) cubic(a,b,c,d) (quadratic and cubic finds the zeros for the given polynomials, you can use cubic(a,b,c,d,1) to only find real solutions for cubics/quadratics)\x1b[G\n\
- ln, log(base,num), W(k,z) (product log, branch k, defaults to k=0)\x1b[G\n\
- root(base,exp), sum(var,func,start,end), prod(var,func,start,end)\x1b[G\n\
- abs, sgn, arg\x1b[G\n\
- ceil, floor, round, int, frac\x1b[G\n\
- fact, doublefact, subfact\x1b[G\n\
- sinc, cis, exp\x1b[G\n\
- zeta, gamma, beta, erf, erfc, digamma, ai, multinomial, binomial/bi/C(n,r), P(n,r), pochhammer(x,n)\x1b[G\n\
- deg, rad, grad\x1b[G\n\
- re, im, split(x+yi), next(n,to)\x1b[G\n\
- factors, prime, isprime, gcd, lcm\x1b[G\n\
- slog(a,b), ssrt(k,a) (k is lambert w branch)\x1b[G\n\
- piecewise/pw({{value,cond}},{{value2,cond2}}...) (when first condition is met from left to right. value elsewards is nan)\x1b[G\n\
- vec(var,func,start,end) mat(var,func,start,end) (makes a vector/matrix) start..end is a shortcut to vec(n,n,start,end)\x1b[G\n\
- tofreq{{a,b,c...}}, tolist{{{{a,b}},{{c,d}}...}}, tolist{{a,b,c}} (sorts and counts how many time each number occurs, tolist takes that kind of data and reverses it)\x1b[G\n\
- variance/var, standarddeviation/σ (sample-bias corrected)\x1b[G\n\
- percentile({{vec}},nth) (gets number at nth percentile), percentilerank({{vec}},x) (gets percentile rank for x point), quartiles{{vec}} (gets quartiles for data set)\x1b[G\n\
- normP(μ,σ,x) (normal distribution pdf) normD(z)/normD(x,μ,σ) (area under curve to the left of z score cdf)\x1b[G\n\
- betaP(α,β,x) (beta distribution pdf) I(x,a,b) (regularized incomplete beta function, or beta distributions cdf)\x1b[G\n\
- roll{{a,b,c...}} rolls die, dice{{a,b,c...}} gets the frequency data any amount of different sided die, where a/b/c are number of faces for each die, both also accept {{{{first_dice_face,# of die}},{{second_dice_face,# of die}}...}}\x1b[G\n\
- lim(x,f(x),point (,side)) both sides are checked by default, -1 for left, 1 for right\x1b[G\n\
- slope(x,f(x),point (,side) (,nth derivitive) (,0) ), can add a 0 to the args to not combine the x and y slopes for parametric equations, and for area\x1b[G\n\
- area(x,f(x),from,to (,amount of data points) (,0) ), length(x,f(x),from,to (,amount of data points) ) (bracketed means optional)\x1b[G\n\n\
Vector operations/functions:\x1b[G\n\
- dot({{vec1}},{{vec2}}), cross({{vec1}},{{vec2}}), proj/project({{vec1}},{{vec2}})\x1b[G\n\
- angle({{vec1}},{{vec2}})\x1b[G\n\
- norm, normalize\x1b[G\n\
- abs, len, any, all\x1b[G\n\
- max, min, mean, median, mode, sort\x1b[G\n\
- reverse, link\x1b[G\n\
- part({{vec}},col), sum, prod\x1b[G\n\
- convert to polar: pol{{vec}} outputs (radius, theta, phi)\x1b[G\n\
- convert to cartesian: car{{vec}} outputs (x, y, z)\x1b[G\n\
- other functions are applied like sqrt{{2,4}}={{sqrt(2),sqrt(4)}}\x1b[G\x1b[G\n\n\
Matrix operations/functions:\x1b[G\n\
- eigenvalues\x1b[G\n\
- trace/tr, determinant/det, inverse/inv\x1b[G\n\
- transpose/trans, adjugate/adj, cofactor/cof, minor\x1b[G\n\
- part({{mat}},col,row), flatten, sum, prod\x1b[G\n\
- abs, norm\x1b[G\n\
- len, wid\x1b[G\n\
- max, min, mean, mode\x1b[G\n\
- iden(n) produces an n dimension identity matrix\x1b[G\n\
- rotate(theta) produces a rotational matrix\x1b[G\n\
- other functions are applied like sqrt{{{{2,4}},{{5,6}}}}={{{{sqrt(2),sqrt(4)}},{{sqrt(5),sqrt(6)}}}}\x1b[G\n\n\
Constants:\x1b[G\n\
- c: speed of light, 299792458 m/s\x1b[G\n\
- gr: gravity, 9.80665 m/s^2\x1b[G\n\
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)\x1b[G\n\
- pl: planck's constant, 6.62607015E-34 J*s\x1b[G\n\
- ec: elementary charge/electron volt, 1.602176634E-19 C\x1b[G\n\
- me: electron mass, 9.1093837015E-31 kg\x1b[G\n\
- mp: proton mass, 1.67262192369E-27 kg\x1b[G\n\
- mn: neutron mass, 1.67492749804E-27 kg\x1b[G\n\
- k: coulomb's constant, 8.9875517923E9 N*m^2/C^2\x1b[G\n\
- Na: avogadro's number, 6.02214076E23 1/mol\x1b[G\n\
- R: gas constant, 8.31446261815324 J/(mol*K)\x1b[G\n\
- bo: boltzmann constant, 1.380649E-23 J/K\x1b[G\n\
- phi/φ: golden ratio, 1.6180339887~\x1b[G\n\
- e: euler's number, 2.7182818284~\x1b[G\n\
- pi/π: pi, 3.1415926535~\x1b[G\n\
- tau/τ: tau, 6.2831853071~\x1b[G\n\n\
Units:\x1b[G\n\
supports metric prefixes and the following units\x1b[G\n\
\"m\" | \"meter\"\x1b[G\n\
\"s\" | \"second\"\x1b[G\n\
\"A\" | \"ampere\"\x1b[G\n\
\"K\" | \"kelvin\"\x1b[G\n\
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
\"L\" | \"litre\"\x1b[G\n\
\"Hz\" | \"hertz\"\x1b[G\n\
\"V\" | \"volt\" | \"voltage\"\x1b[G\n\
\"°C\" | \"celsius\"\x1b[G\n\
\"°F\" | \"fahrenheit\"\x1b[G\n\
\"Wh\"\x1b[G\n\
\"Ah\"\x1b[G\n\
\"year\"\x1b[G\n\
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
\"d\" | \"day\"\x1b[G\n\
\"week\"\x1b[G\n\
\"N\" | \"newton\"\x1b[G\n\
\"C\" | \"coulomb\"\x1b[G\n\
\"°\" | \"deg\" | \"degrees\"\x1b[G\n\
\"rad\" | \"radians\"\x1b[G\n\
\"grad\" | \"gradians\"\x1b[G\n\
\"lumen\" | \"lm\"\x1b[G\n\
\"lux\" | \"lx\"\x1b[G\n\
\"byte\" | \"B\"\x1b[G\n\
\"gray\" | \"Gy\"\x1b[G\n\
\"sievert\" | \"Sv\"\x1b[G\n\
\"katal\" | \"kat\"\x1b[G\n\
\"bit\" | \"b\"\x1b[G\n\
\"steradian\" | \"sr\"\x1b[G\n\n\
Digraph:\x1b[G\n\
hit escape then a letter\x1b[G\n\
a=>α, A=>Α, b=>β, B=>Β, c=>ξ, C=>Ξ, d=>Δ, D=>δ,\x1b[G\n\
e=>ε, E=>Ε, f=>φ, F=>Φ, g=>γ, G=>Γ, h=>η, H=>Η,\x1b[G\n\
i=>ι, I=>Ι, k=>κ, Κ=>Κ, l=>λ, L=>Λ, m=>μ, M=>Μ,\x1b[G\n\
n=>ν, Ν=>Ν, o=>ο, O=>Ο, p=>π, P=>Π, q=>θ, Q=>Θ,\x1b[G\n\
r=>ρ, R=>Ρ, s=>σ, S=>Σ, t=>τ, T=>Τ, u=>υ, U=>Υ,\x1b[G\n\
w=>ω, W=>Ω, y=>ψ, Y=>Ψ, x=>χ, X=>Χ, z=>ζ, Z=>Ζ,\x1b[G\n\
==>±, `=>ⁱ, _=>∞, ;=>°\x1b[G\n\
numbers/minus sign convert to superscript acting as exponents\x1b[G\n"
);
}
//TODO
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
        "help" => "W, atan",
        "" => "",
        _ => "not in database",
    }
    .to_string()
}
