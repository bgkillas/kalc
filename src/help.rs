pub fn help()
{
    println!(
             "Usage: kalc [FLAGS] function_1 function_2 function_3...\n\x1b[G\
FLAGS: --help (this message)\n\x1b[G\
--tau fractions are shown in tau instead of pi\n\x1b[G\
--deg compute in degrees\n\x1b[G\
--rad compute in radians\n\x1b[G\
--grad compute in gradians\n\x1b[G\
--2d=[num] number of points to graph in 2D\n\x1b[G\
--3d=[x],[y] number of points to graph in 3D\n\x1b[G\
--xr=[min],[max] x range for graphing\n\x1b[G\
--yr=[min],[max] y range for graphing\n\x1b[G\
--zr=[min],[max] z range for graphing\n\x1b[G\
--range=[num] sets all ranges to [-num],[num]\n\x1b[G\
--point [char] point style for graphing\n\x1b[G\
--sci toggles scientific notation\n\x1b[G\
--base=[num] sets the number base (2,8,16)\n\x1b[G\
--prompt toggles the prompt\n\x1b[G\
--color toggles color\n\x1b[G\
--comma toggles comma seperation\n\x1b[G\
--graph toggles graphing\n\x1b[G\
--vars toggles default variables\n\x1b[G\
--line toggles line graphing\n\x1b[G\
--rt toggles real time printing\n\x1b[G\
--polar toggles displaying polar vectors\n\x1b[G\
--frac toggles fraction display\n\x1b[G\
--frac_iter=[num] how many iterations to check for fractions\n\x1b[G\
--prec=[num] sets the output precision(default 512)\n\x1b[G\
--graphprec=[num] sets the graph precision(default 64)\n\x1b[G\
--deci=[num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\n\x1b[G\
--def ignores config file\n\x1b[G\
--multi toggles multi line display for matrixes\n\x1b[G\
--tabbed toggles tabbed display for matrixes\n\x1b[G\
--debug displays computation time in nanoseconds\n\x1b[G\
--depth display 2d complex graphs in 3d with imag #'s going up/down on the z axis\n\x1b[G\
--small_e use small e notation, like 5e2=5*10^2, instead of capital 'E' for scientific notation. only works with a number before and number or '-' sign after the 'e' otherwise assumes euler number\n\x1b[G\n\x1b[G\
- flags can be executed in runtime just without the dashes\n\x1b[G\
- Type \"colors=\" to see color settings\n\x1b[G\
- Type \"exit\" to exit the program\n\x1b[G\
- Type \"clear\" to clear the screen\n\x1b[G\
- Type \"history [arg]\" to see the history, arg indexes it if specified\n\x1b[G\
- Type \"vars\" to list all variables\n\x1b[G\
- Type \"_\" to use the previous answer\n\x1b[G\
- Type \"a={{expr}}\" to define a variable\n\x1b[G\
- Type \"f(x)=...\" to define a function\n\x1b[G\
- Type \"f(x,y,z...)=...\" to define a multi variable function\n\x1b[G\
- Type \"...=\" display parsed input, show values of stuff like xr/deci/prec etc\n\x1b[G\
- Type \"f...=null\" to delete a function or variable\n\x1b[G\
- Type \"{{x,y,z...}}\" to define a cartesian vector\n\x1b[G\
- Type \"[radius,theta,phi]\" to define a polar vector (same as car{{vec}})\n\x1b[G\
- Type \"{{vec}}#\" to graph a vector\n\x1b[G\
- Type \"{{mat}}#\" to graph a matrix\n\x1b[G\
- Type \"number#\" to graph a complex number\n\x1b[G\
- Type \"{{{{a,b,c}},{{d,e,f}},{{g,h,i}}}}\" to define a 3x3 matrix\n\x1b[G\
- Type \"rnd\" to generate a random number\n\x1b[G\
- Alt+Enter will not graph whatever is present\n\x1b[G\n\x1b[G\
Operators:\n\x1b[G\
- +, -, *, /, //, ^, ^^, %, <, >, <=, >=, |x|, ±/+-\n\x1b[G\
- !x (subfact), x! (fact), x!! (doublefact)\n\x1b[G\
- && (and), || (or), == (equals), != (not equals)\n\x1b[G\
- >> (right shift), << (left shift)\n\x1b[G\n\x1b[G\
Functions:\n\x1b[G\
- sin, cos, tan, asin, acos, atan, atan(x,y)\n\x1b[G\
- csc, sec, cot, acsc, asec, acot\n\x1b[G\
- sinh, cosh, tanh, asinh, acosh, atanh\n\x1b[G\
- csch, sech, coth, acsch, asech, acoth\n\x1b[G\
- sqrt, cbrt, square, cube, quadratic(a,b,c) cubic(a,b,c,d)\n\x1b[G\
- ln, log(base,num), root(base,exp), sum(var,func,start,end), prod(var,func,start,end)\n\x1b[G\
- abs, sgn, arg\n\x1b[G\
- ceil, floor, round, int, frac\n\x1b[G\
- fact(real), doublefact(real), subfact(natural)\n\x1b[G\
- sinc, cis, exp\n\x1b[G\
- zeta, gamma, erf, erfc, digamma, ai, binomial/bi/C, P(n,r) (all real only)\n\x1b[G\
- deg, rad, grad (all real only)\n\x1b[G\
- re, im, split(x+yi={{x,y}})\n\x1b[G\
- factors, prime\n\x1b[G\
- slog(a,b)\n\x1b[G\
- piecewise({{value,cond}},{{value2,cond2}}...) (when first condition is met from left to right. value elsewards is nan)\n\x1b[G\
- vec(var,func,start,end) mat(var,func,start,end) (makes a vector/matrix) start..end is a shortcut to vec(n,n,start,end)\n\x1b[G\
- tofreq{{a,b,c...}}, tolist{{{{a,b}},{{c,d}}...}} (sorts and counts how many time each number occurs, tolist takes that kind of data and reverses it)\n\x1b[G\
- variance/var, standarddeviation/σ (sample-bias corrected)\n\x1b[G\
- percentile({{vec}},nth) (gets number at nth percentile), percentilerank({{vec}},x) (gets percentile rank for x point), quartiles{{vec}} (gets quartiles for data set)\n\x1b[G\
- normP(μ,σ,x) (normal distribution) normD(z) (area under curve to the left of z score)\n\x1b[G\n\x1b[G\
Vector operations/functions:\n\x1b[G\
- dot({{vec1}},{{vec2}}), cross({{vec1}},{{vec2}}), proj/project({{vec1}},{{vec2}})\n\x1b[G\
- angle({{vec1}},{{vec2}})\n\x1b[G\
- norm, normalize\n\x1b[G\
- abs, len, any, all\n\x1b[G\
- max, min, mean, median, mode, sort\n\x1b[G\
- reverse, link\n\x1b[G\
- part({{vec}},col), sum, prod\n\x1b[G\
- convert to polar: pol{{vec}} outputs (radius, theta, phi)\n\x1b[G\
- convert to cartesian: car{{vec}} outputs (x, y, z)\n\x1b[G\
- other functions are applied like sqrt{{2,4}}={{sqrt(2),sqrt(4)}}\n\x1b[G\n\x1b[G\
Matrix operations/functions:\n\x1b[G\
- eigenvalues\n\x1b[G\
- trace/tr, determinant/det, inverse/inv\n\x1b[G\
- transpose/trans, adjugate/adj, cofactor/cof, minor\n\x1b[G\
- part({{mat}},col,row), flatten, sum, prod\n\x1b[G\
- abs, norm\n\x1b[G\
- len, wid\n\x1b[G\
- max, min, mean, mode\n\x1b[G\
- I(n) produces n identity matrix\n\x1b[G\
- rotate(theta) produces a rotational matrix\n\x1b[G\
- other functions are applied like sqrt{{{{2,4}},{{5,6}}}}={{{{sqrt(2),sqrt(4)}},{{sqrt(5),sqrt(6)}}}}\n\x1b[G\n\x1b[G\
Constants:\n\x1b[G\
- c: speed of light, 299792458 m/s\n\x1b[G\
- g: gravity, 9.80665 m/s^2\n\x1b[G\
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)\n\x1b[G\
- h: planck's constant, 6.62607015E-34 J*s\n\x1b[G\
- ec: elementary charge/electron volt, 1.602176634E-19 C/J\n\x1b[G\
- me: electron mass, 9.1093837015E-31 kg\n\x1b[G\
- mp: proton mass, 1.67262192369E-27 kg\n\x1b[G\
- mn: neutron mass, 1.67492749804E-27 kg\n\x1b[G\
- k: coulomb's constant, 8.9875517923E9 N*m^2/C^2\n\x1b[G\
- Na: avogadro's number, 6.02214076E23 1/mol\n\x1b[G\
- R: gas constant, 8.31446261815324 J/(mol*K)\n\x1b[G\
- kB: boltzmann constant, 1.380649E-23 J/K\n\x1b[G\
- phi/φ: golden ratio, 1.6180339887~\n\x1b[G\
- e: euler's number, 2.7182818284~\n\x1b[G\
- pi/π: pi, 3.1415926535~\n\x1b[G\
- tau/τ: tau, 6.2831853071~\n\x1b[G\
Digraph:\n\x1b[G\
hit escape then a letter\n\x1b[G\
a=>α, A=>Α, b=>β, B=>Β, c=>ξ, C=>Ξ, d=>Δ, D=>δ,\n\x1b[G\
e=>ε, E=>Ε, f=>φ, F=>Φ, g=>γ, G=>Γ, h=>η, H=>Η,\n\x1b[G\
i=>ι, I=>Ι, k=>κ, Κ=>Κ, l=>λ, L=>Λ, m=>μ, M=>Μ,\n\x1b[G\
n=>ν, Ν=>Ν, o=>ο, O=>Ο, p=>π, P=>Π, q=>θ, Q=>Θ,\n\x1b[G\
r=>ρ, R=>Ρ, s=>σ, S=>Σ, t=>τ, T=>Τ, u=>υ, U=>Υ,\n\x1b[G\
w=>ω, W=>Ω, y=>ψ, Y=>Ψ, x=>χ, X=>Χ, z=>ζ, Z=>Ζ,\n\x1b[G\
==>±, `=>ⁱ _=>∞\n\x1b[G\
numbers/minus sign convert to superscript acting as exponents"
    );
}