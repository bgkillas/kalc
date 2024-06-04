pub fn help()
{
    print!(
             "Usage: kalc [FLAGS] equation_1 equation_2 equation_3...\x1b[G\n\
FLAGS: --help (this message)\x1b[G\n\
--help {{thing}} to get more detail on a function/option/feature, --help help to list all \"things\"\x1b[G\n\
--interactive/-i allows interaction after finishing the equations given\x1b[G\n\
--units toggles units\x1b[G\n\
--notation=e/E/s/n defines what kind of notation you should use,(e) 3e2,(E) 3E2,(s) 3*10^2,(n) 300\x1b[G\n\
--graph=normal/depth/flat/none changes how a function is graphed\x1b[G\n\
--label=[x],[y],[z] sets the labels for the graphs x/y/z axis\x1b[G\n\
--angle=deg/rad/grad sets your angletype\x1b[G\n\
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
--base=[input],[output] sets the numbers base from 2 to 36\x1b[G\n\
--ticks=[num](,[num](,[num])) sets amount of ticks, optionally set different x/y/z ticks, -2 will be auto, -1 will be at every whole number, 0 will be none\x1b[G\n\
--onaxis toggles showing the ticks on the x/y/z axis\x1b[G\n\
--prompt toggles the prompt\x1b[G\n\
--color=true/false/auto toggles color output, toggled by default when running from arguments\x1b[G\n\
--comma toggles comma seperation\x1b[G\n\
--graph toggles graphing\x1b[G\n\
--vars disables default variables and kalc.vars\x1b[G\n\
--default sets to default settings and ignores kalc.vars\x1b[G\n\
--line=true/false/auto toggles line graphing\x1b[G\n\
--rt toggles real time printing\x1b[G\n\
--polar toggles displaying polar vectors\x1b[G\n\
--frac toggles fraction display\x1b[G\n\
--prec=[num] sets the output precision(default 512)\x1b[G\n\
--graphprec=[num] sets the graph precision(default 64)\x1b[G\n\
--deci=[num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\x1b[G\n\
--multi toggles multi line display for matrixes\x1b[G\n\
--tabbed toggles tabbed display for matrixes\x1b[G\n\
--surface displays a colored surface(based on z value) for 3d graphing, only supports 1 graph\x1b[G\n\
--scalegraph scales the y part of a 2d graph to the users screen size, setting --windowsize=x,y makes the ratio more accurate\x1b[G\n\
--saveto=[file] saves the graph as a png to the given file, --windowsize=x,y for resolution\x1b[G\n\
--siunits toggles keeping stuff in si units, a newton will show as 'm s^-2 kg' instead of 'N'\x1b[G\n\
--keepzeros dont remove trailing zeros\x1b[G\n\n\
- flags can be executed in runtime just without the dashes\x1b[G\n\
- \"colors=\" to see color settings\x1b[G\n\
- \"exit\" to exit the program\x1b[G\n\
- \"clear\" to clear the screen\x1b[G\n\
- \"history [arg]\" to see the history, arg searches for the arg it if specified\x1b[G\n\
- \"vars\" to list all variables\x1b[G\n\
- \"option/var;function\" to set a temporal option/var, example: \"a=45;angle=deg;sin(a)\" = sqrt(2)/2\x1b[G\n\
- \"f(x)=var:function\" to set a temporal var when defining function, example: \"f(x)=a=2:ax\" = f(x)=2x\x1b[G\n\
- \"_\" or \"ans\" or \"ANS\" to use the previous answer\x1b[G\n\
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
- \"epoch\" to get time in seconds since unix epoch\x1b[G\n\
- Alt+Enter will not print output while still graphing/defining variables\x1b[G\n\
- \"help {{thing}}\" to get more detail on a function/option/feature\x1b[G\n\
- \"help help\" to list all things to query\x1b[G\n\n\
Order of Operations:\x1b[G\n\
- user defined functions\x1b[G\n\
- functions, !x, x!, x!!, |x|\x1b[G\n\
- % (modulus), .. (a..b creates lists of integers from a to b)\x1b[G\n\
- ^/** (exponentiation), // (a//b is a root b), ^^ (tetration), computed from right to left\x1b[G\n\
- × internal multiplication for units and negitive signs\x1b[G\n\
- * (multiplication), / (division)\x1b[G\n\
- + (addition), - (subtraction), +-/± (creates a list of the calculation if plus and the calculation if minus)\x1b[G\n\
- to/-> (unit conversions, ie 2m->yd=2.2, leaves unitless if perfect conversion)\x1b[G\n\
- < (lt), <= (le), > (gt), >= (ge), == (eq), != (!eq), >> (a>>b shifts b bits right), << (a<<b shifts b bits left)\x1b[G\n\
- && (a&&b outputs 1 if both a and b are 1), || (a||b outputs 1 if either a or b are 1)\x1b[G\n\n\
Functions:\x1b[G\n\
- sin, cos, tan, asin, acos, atan, atan(x,y), atan2(y,x), sincos(x)={{sin(x),cos(x)}}, cossin(x)={{cos(x),sin(x)}}\x1b[G\n\
- csc, sec, cot, acsc, asec, acot\x1b[G\n\
- sinh, cosh, tanh, asinh, acosh, atanh\x1b[G\n\
- csch, sech, coth, acsch, asech, acoth\x1b[G\n\
- sqrt, cbrt, square, cube, quadratic(a,b,c), cubic(a,b,c,d), quartic(a,b,c,d,e) (finds the zeros for the given polynomial, you can add a '1' to the args to only find real roots)\x1b[G\n\
- ln, log(base,num), W(k,z) (product log, branch k, defaults to k=0)\x1b[G\n\
- root(base,exp), sum(var,func,start,end), prod(var,func,start,end)\x1b[G\n\
- abs, sgn, arg\x1b[G\n\
- ceil, floor, round, int, frac\x1b[G\n\
- fact, doublefact, subfact\x1b[G\n\
- sinc, cis, exp\x1b[G\n\
- zeta, eta, gamma, lower_gamma, beta, erf, erfc, digamma, ai, multinomial, binomial/bi/C(n,r), P(n,r), pochhammer(x,n)\x1b[G\n\
- re, im, onlyreal, onlyimag, split(x+yi), next(n,to)\x1b[G\n\
- unity(n,k) gets all solutions for x in x^k=n\x1b[G\n\
- factors, nth_prime, is_prime, is_nan, is_inf, is_finite, gcd, lcm\x1b[G\n\
- slog(a,b), ssrt(k,a) (k is lambert w branch)\x1b[G\n\
- piecewise/pw({{value,cond}},{{value2,cond2}}...) (when first condition is met from left to right. value elsewards is nan)\x1b[G\n\
- vec(var,func,start,end) mat(var,func,start,end) (makes a vector/matrix) start..end is a shortcut to vec(n,n,start,end)\x1b[G\n\
- to_freq{{a,b,c...}}, to_list{{{{a,b}},{{c,d}}...}}, to_list{{a,b,c}} (sorts and counts how many time each number occurs, to_list takes that kind of data and reverses it)\x1b[G\n\
- variance/var, covariance/cov, standarddeviation/sd/σ (sample-bias corrected), skew/skewness, kurtosis (excess)\x1b[G\n\
- percentile({{vec}},nth) (gets number at nth percentile), percentilerank({{vec}},x) (gets percentile rank for x point), quartiles{{vec}} (gets quartiles for data set)\x1b[G\n\
- norm_pdf(x,μ,σ) (normal distribution pdf) normD(z)/norm_cdf(x,μ,σ) (area under curve to the left of z score cdf)\x1b[G\n\
- beta_pdf(x,α,β) (beta distribution pdf) beta_cdf/I(x,a,b) (regularized incomplete beta function, or beta distributions cdf)\x1b[G\n\
- gamma_pdf(x,k,θ), gamma_cdf(x,k,θ), lognorm_pdf(x,μ,σ), lognorm_cdf(x,μ,σ), binomial_pmf(k,n,p), binomial_cdf(k,n,p), geometric_pmf(k,p), geometric_cdf(k,p), poisson_pmf(x,λ), poisson_cdf(x,λ)\x1b[G\n\
- rand_norm(μ,σ), rand_uniform(a,b), rand_int(a,b), rand_gamma(k,θ), rand_lognorm(μ,σ), rand_binomial(n,p), rand_geometric(k,p), rand_bernoulli(p), rand_poisson(λ)\x1b[G\n\
- roll{{a,b,c...}} rolls die, dice{{a,b,c...}} gets the frequency data any amount of different sided die, where a/b/c are number of faces for each die, both also accept {{{{first_dice_face,# of die}},{{second_dice_face,# of die}}...}}\x1b[G\n\
- rand_weighted{{{{a,n1}},{{b,n2}}..}} rolls a weighted die where a and b are face values and n1 and n2 are their weights\x1b[G\n\
- An(n,k), Ap(n,t) eulerian numbers and polynomials\x1b[G\n\
- lim(x,f(x),point (,side)) both sides are checked by default, -1 for left, 1 for right\x1b[G\n\
- slope(x,f(x),point (,nth derivitive) (,0) ), can add a 0 to the args to not combine the x and y slopes for parametric equations, same for area\x1b[G\n\
- area(x,f(x),from,to (,0) ), length(x,f(x),from,to), surfacearea(a,b,z(a,b),startb,endb,starta,enda)\x1b[G\n\
- solve(x,f(x) (,point)) employs newtons method to find the root of a function at a starting point, assumes 0 if no point given, outputs Nan if newton method fails\x1b[G\n\
- extrema(x,f(x) (,point)) employs newtons method to find the extrema of a function at a starting point, assumes 0 if no point given, outputs Nan if newton method fails, outputs {{x,y,positive/negitive concavity}}\x1b[G\n\
- iter(x,f(x),p,n), f(x) iterated n times at point p, add \",1\" to args to show steps\x1b[G\n\n\
Vector operations/functions:\x1b[G\n\
- dot({{vec1}},{{vec2}}), cross({{vec1}},{{vec2}}), proj/project({{vec1}},{{vec2}})\x1b[G\n\
- angle({{vec1}},{{vec2}})\x1b[G\n\
- norm, normalize\x1b[G\n\
- abs, len, any, all\x1b[G\n\
- max, min, mean, median, mode, sort, geo_mean\x1b[G\n\
- reverse, link\x1b[G\n\
- part({{vec}},col), sum, prod\x1b[G\n\
- convert to polar: pol{{vec}} outputs (radius, theta, phi)\x1b[G\n\
- convert to cartesian: car{{vec}} outputs (x, y, z)\x1b[G\n\
- other functions are applied like sqrt{{2,4}}={{sqrt(2),sqrt(4)}}\x1b[G\n\n\
Matrix operations/functions:\x1b[G\n\
- eigenvalues, eigenvectors\x1b[G\n\
- trace/tr, determinant/det, inverse/inv\x1b[G\n\
- transpose/trans, adjugate/adj, cofactor/cof, minor\x1b[G\n\
- part({{mat}},col,row), flatten, sum, prod\x1b[G\n\
- abs, norm\x1b[G\n\
- len, wid\x1b[G\n\
- max, min, mean, mode, weighted_mean{{{{n,weight}}...}}\x1b[G\n\
- iden(n) produces an n dimension identity matrix\x1b[G\n\
- rotate(theta), rotate(yaw,pitch,roll) produces a rotational matrix\x1b[G\n\
- sort(mat) sorts rows by first column\x1b[G\n\
- interpolate/inter(mat,x) using lagrange interpolation interpolates a 2xN matrix along x, matrix should be organized like {{{{x0,y0}},{{x1,y1}} ... {{xN,yN}}}}\x1b[G\n\
- lineofbestfit/lobf(mat,x) line of best fit for numerous 2d values, with no x values it will spit out the m/b values for line equation in form of mx+b, mat should be organized like {{{{x0,y0}},{{x1,y1}} ... {{xN,yN}}}}\x1b[G\n\
- plane(mat,x,y) finds the plane that 3, 3d points lie on, with no x/y arg it will spit out the a/b/c values for the equation of plane in ax+by+c form, mat should be in form of {{{{x0,y0,z0}},{{x1,y1,z1}},{{x2,y2,z2}}}}\x1b[G\n\
- other functions are applied like sqrt{{{{2,4}},{{5,6}}}}={{{{sqrt(2),sqrt(4)}},{{sqrt(5),sqrt(6)}}}}\x1b[G\n\n\
Constants:\x1b[G\n\
- c: speed of light, 299792458 m/s\x1b[G\n\
- gravity: gravity, 9.80665 m/s^2\x1b[G\n\
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)\x1b[G\n\
- planck: planck's constant, 6.62607015E-34 J*s\x1b[G\n\
- eV: electron volt, 1.602176634E-19 J\x1b[G\n\
- eC: elementary charge, 1.602176634E-19 C\x1b[G\n\
- eM: electron mass, 9.1093837015E-31 kg\x1b[G\n\
- pM: proton mass, 1.67262192369E-27 kg\x1b[G\n\
- nM: neutron mass, 1.67492749804E-27 kg\x1b[G\n\
- ke: coulomb's constant, 8.9875517923E9 N*m^2/C^2\x1b[G\n\
- Na: avogadro's number, 6.02214076E23 1/mol\x1b[G\n\
- R: gas constant, 8.31446261815324 J/(mol*K)\x1b[G\n\
- boltzmann: boltzmann constant, 1.380649E-23 J/K\x1b[G\n\
- phi/φ: golden ratio, 1.6180339887~\x1b[G\n\
- e: euler's number, 2.7182818284~\x1b[G\n\
- pi/π: pi, 3.1415926535~\x1b[G\n\
- tau/τ: tau, 6.2831853071~\x1b[G\n\n\
Units:\x1b[G\n\
supports metric and binary prefixes\x1b[G\n\
ignores \"s\" at the end to allow \"meters\" and stuff\x1b[G\n\
\"units\" function will extract the units of a number for == checks and stuff\x1b[G\n\
the following units are supported\x1b[G\n\
{}\x1b[G\n\n\
Digraph:\x1b[G\n\
hit escape then a letter, or hold alt while typing(only lowercase)\x1b[G\n\
a=>α, A=>Α, b=>β, B=>Β, c=>ξ, C=>Ξ, d=>Δ, D=>δ,\x1b[G\n\
e=>ε, E=>Ε, f=>φ, F=>Φ, g=>γ, G=>Γ, h=>η, H=>Η,\x1b[G\n\
i=>ι, I=>Ι, k=>κ, Κ=>Κ, l=>λ, L=>Λ, m=>μ, M=>Μ,\x1b[G\n\
n=>ν, Ν=>Ν, o=>ο, O=>Ο, p=>π, P=>Π, q=>θ, Q=>Θ,\x1b[G\n\
r=>ρ, R=>Ρ, s=>σ, S=>Σ, t=>τ, T=>Τ, u=>υ, U=>Υ,\x1b[G\n\
w=>ω, W=>Ω, y=>ψ, Y=>Ψ, x=>χ, X=>Χ, z=>ζ, Z=>Ζ,\x1b[G\n\
+=>±, ==>≈, `=>ⁱ, _=>∞, ;=>°\x1b[G\n\
numbers/minus sign convert to superscript acting as exponents\x1b[G\n"
,all_units());
}
fn all_units() -> &'static str
{
    "\"m\" | \"meter\"\x1b[G\n\
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
\"d\" | \"day\"\x1b[G\n\
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
\"bit\" | \"b\"\x1b[G\n\
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
\"floz\""
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
