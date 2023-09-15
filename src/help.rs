pub fn help()
{
    println!(
             "Usage: kalc [FLAGS] function_1 function_2 function_3...\n\
FLAGS: --help (this message)\n\
--tau fractions are shown in tau instead of pi\n\
--deg compute in degrees\n\
--rad compute in radians\n\
--grad compute in gradians\n\
--2d=[num] number of points to graph in 2D\n\
--3d=[num] number of points to graph in 3D\n\
--xr=[min],[max] x range for graphing\n\
--yr=[min],[max] y range for graphing\n\
--zr=[min],[max] z range for graphing\n\
--point [char] point style for graphing\n\
--sci toggles scientific notation\n\
--base=[num] sets the number base (2,8,16)\n\
--prompt toggles the prompt\n\
--color toggles color\n\
--comma toggles comma seperation\n\
--vars toggles default variables\n\
--line toggles line graphing\n\
--rt toggles real time printing\n\
--polar toggles displaying polar vectors\n\
--frac toggles fraction display\n\
--frac_iter=[num] how many iterations to check for fractions\n\
--prec=[num] sets the precision\n\
--deci=[num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\n\
--def ignores config file\n\
--multi toggles multi line display for matrixes\n\
--tabbed toggles tabbed display for matrixes\n\
--debug displays computation time in nanoseconds\n\
--small_e use small e notation, like 5e2=5*10^2, instead of capital 'E' for scientific notation. only works with a number before and number or '-' sign after the 'e' otherwise assumes euler number\n\n\
- flags can be executed in runtime just without the dashes\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"help [arg]\" to get this message, get help on function if specified\n\
- Type \"vars\" to list all variables\n\
- Type \"lvars\" to list all variables without equating them\n\
- Type \"_\" to use the previous answer\n\
- Type \"a={{expr}}\" to define a variable\n\
- Type \"f(x)=...\" to define a function\n\
- Type \"f(x,y,z...)=...\" to define a multi variable function\n\
- Type \"...=\" display parsed input, show values of stuff like xr/deci/prec etc\n\
- Type \"f...=null\" to delete a function or variable\n\
- Type \"{{x,y,z...}}\" to define a cartesian vector\n\
- Type \"[radius,theta,phi]\" to define a polar vector (same as car{{vec}})\n\
- Type \"{{vec}}#\" to graph a vector\n\
- Type \"{{mat}}#\" to graph a matrix\n\
- Type \"number#\" to graph a complex number\n\
- Type \"{{{{a,b,c}},{{d,e,f}},{{g,h,i}}}}\" to define a 3x3 matrix\n\n\
Operators:\n\
- +, -, *, /, ^, %, <, >, <=, >=, |(norm), ±(works well if only 1 is present, creates a vector with plus being first part and minus being second part)\n\
- !x (subfact), x! (fact)\n\
- && (and), || (or), == (equals), != (not equals)\n\
- >> (right shift), << (left shift)\n\n\
Trigonometric functions:\n\
- sin, cos, tan, asin, acos, atan, atan(x,y)\n\
- csc, sec, cot, acsc, asec, acot\n\
- sinh, cosh, tanh, asinh, acosh, atanh\n\
- csch, sech, coth, acsch, asech, acoth\n\n\
Other functions:\n\
- sqrt, cbrt, square, cube\n\
- ln, log(base,num), root(base,exp), sum(func,var,start,end), prod(func,var,start,end) (start and end are rounded to integers)\n\
- abs, sgn, arg\n\
- ceil, floor, round, int, frac\n\
- fact(real), subfact(natural)\n\
- sinc, cis, exp\n\
- zeta, gamma, erf, erfc, digamma, ai, binomial/bi (all real only)\n\
- deg(to_degrees), rad(to_radians), grad(to_gradians) (all real only)\n\
- re, im, max(x,y), min(x,y)\n\
- factors, prime\n\
- mvec(func,var,start,end) (makes a vector/matrix)\n\n\
Vector operations/functions:\n\
- *,/,+,-,^\n\
- dot({{vec1}},{{vec2}}), cross({{vec1}},{{vec2}}), proj/project({{vec1}},{{vec2}})\n\
- angle({{vec1}},{{vec2}})\n\
- norm, normalize\n\
- abs, len\n\
- part({{vec}},col), add\n\
- convert to polar: pol{{vec}} outputs (radius, theta, phi)\n\
- convert to cartesian: car{{vec}} outputs (x, y, z)\n\
- other functions are applied like sqrt{{2,4}}={{sqrt(2),sqrt(4)}}\n\n\
Matrix operations/functions:\n\
- *,/,+,-,^\n\
- trace/tr, determinant/det, inverse/inv\n\
- transpose/trans, adjugate/adj, cofactor/cof, minor\n\
- part({{mat}},col,row), add\n\
- abs, norm\n\
- len, wid\n\
- rotate(theta) produces a rotational matrix\n\
- other functions are applied like sqrt{{{{2,4}},{{5,6}}}}={{{{sqrt(2),sqrt(4)}},{{sqrt(5),sqrt(6)}}}}\n\n\
Constants:\n\
- c: speed of light, 299792458 m/s\n\
- g: gravity, 9.80665 m/s^2\n\
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)\n\
- h: planck's constant, 6.62607015E-34 J*s\n\
- ec: elementary charge/electron volt, 1.602176634E-19 C/J\n\
- me: electron mass, 9.1093837015E-31 kg\n\
- mp: proton mass, 1.67262192369E-27 kg\n\
- mn: neutron mass, 1.67492749804E-27 kg\n\
- k: coulomb's constant, 8.9875517923E9 N*m^2/C^2\n\
- Na: avogadro's number, 6.02214076E23 1/mol\n\
- R: gas constant, 8.31446261815324 J/(mol*K)\n\
- kB: boltzmann constant, 1.380649E-23 J/K\n\
- phi/φ: golden ratio, 1.6180339887~\n\
- e: euler's number, 2.7182818284~\n\
- pi/π: pi, 3.1415926535~\n\
- tau/τ: tau, 6.2831853071~\n\n\
Digraph:\n\
hit escape then a letter\n\
a=>α, A=>Α, b=>β, B=>Β, c=>ξ, C=>Ξ, d=>Δ, D=>δ,\n\
e=>ε, E=>Ε, f=>φ, F=>Φ, g=>γ, G=>Γ, h=>η, H=>Η,\n\
i=>ι, I=>Ι, k=>κ, Κ=>Κ, l=>λ, L=>Λ, m=>μ, M=>Μ,\n\
n=>ν, Ν=>Ν, o=>ο, O=>Ο, p=>π, P=>Π, q=>θ, Q=>Θ,\n\
r=>ρ, R=>Ρ, s=>σ, S=>Σ, t=>τ, T=>Τ, u=>υ, U=>Υ,\n\
w=>ω, W=>Ω, y=>ψ, Y=>Ψ, x=>χ, X=>Χ, z=>ζ, Z=>Ζ,\n\
= >±, `=>ⁱ\n\
numbers/minus sign convert to superscript acting as exponents"
    );
}
pub fn get_help(s: &str)
{
    println!("{}", s);
}