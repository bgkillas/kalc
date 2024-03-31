# kalc

[![crates.io](https://img.shields.io/crates/v/kalc.svg)](https://crates.io/crates/kalc) [![AUR](https://img.shields.io/aur/version/kalc.svg)](https://aur.archlinux.org/packages/kalc/)

![image](https://github.com/bgkillas/kalc/assets/55570525/40ef3f19-97de-4da4-a5df-8a6e37ff33b2)

requires gnuplot for graphing, a modern terminal like windows terminal on windows(via wsl)

history file is stored in ```~/.config/kalc.history```

config file is stored in ```~/.config/kalc.config``` example in repo

you can set permanent variables and functions in the file ```~/.config/kalc.vars``` example in repo, also contains more
advanced example usage, suggested not to use entire kalc.vars example on lower end systems as it may cause lag

config defaults listed in kalc.config

# install instructions

use aur or run
```cargo install kalc```

# build instructions

windows is not properly supported due to dependencys being weird, just use wsl

dependencys are: rust>=1.73.0, diffutils, gcc, m4, make

```
git clone https://github.com/bgkillas/kalc
cd kalc
cargo build --release
./target/release/kalc
```

# usage

```
Usage: kalc [FLAGS] equation_1 equation_2 equation_3...
FLAGS: --help (this message)
--help {thing} to get more detail on a function/option/feature, --help help to list all "things"
--interactive/-i allows interaction after finishing the equations given
--units toggles units\x1b[G\n\
--label=[x],[y],[z] sets the labels for the graphs x/y/z axis
--tau fractions are shown in tau instead of pi
--deg compute in degrees
--rad compute in radians
--grad compute in gradians
--2d=[num] number of points to graph in 2D
--3d=[x],[y] number of points to graph in 3D
--xr=[min],[max] x range for graphing
--yr=[min],[max] y range for graphing
--zr=[min],[max] z range for graphing
--range=[num] sets all ranges to [-num],[num]
--vxr=[min],[max] x range for graphing, graph view override, useful for parametric
--vyr=[min],[max] y range for graphing, graph view override, useful for parametric
--vzr=[min],[max] z range for graphing, graph view override, useful for parametric
--vrange=[num] sets all ranges to [-num],[num], graph view override, useful for parametric
--point [char] point style for graphing
--sci toggles scientific notation
--base=[input],[output] sets the numbers base from 2 to 36
--ticks=[num] sets amount of ticks, -2 will be auto, -1 will be at every whole number, 0 will be none
--onaxis toggles showing the ticks on the x/y/z axis on by default for 2d, off by default for 3d
--prompt toggles the prompt
--color toggles color output, toggled by default when running from arguments
--comma toggles comma seperation
--graph toggles graphing
--vars disables default variables and kalc.vars\n\x1b[G\
--default sets to default settings
--line toggles line graphing
--rt toggles real time printing
--polar toggles displaying polar vectors
--frac toggles fraction display
--frac_iter=[num] how many iterations to check for fractions
--prec=[num] sets the output precision(default 512)
--graphprec=[num] sets the graph precision(default 64)
--deci=[num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals
--def ignores config file
--multi toggles multi line display for matrixes
--tabbed toggles tabbed display for matrixes
--depth display 2d complex graphs in 3d with imag #'s going up/down on the z axis
--surface displays a colored surface(based on z value) for 3d graphing, only supports 1 graph
--flat display 2d complex graphs like they are on the 2d number line
--small_e use small e notation, like 5e2=5*10^2, instead of capital 'E' for scientific notation. only works with a number before and number or '-' sign after the 'e' otherwise assumes euler number
--scalegraph scales the y part of a 2d graph to the users screen size, setting --windowsize=x,y makes the ratio more accurate

- flags can be executed in runtime just without the dashes
- "colors=" to see color settings
- "exit" to exit the program
- "clear" to clear the screen
- "history [arg]" to see the history, arg searches for the arg it if specified
- "vars" to list all variables
- "option/var;function" to set a temporal option/var, example: "a=45;deg;sin(a)" = sqrt(2)/2
- "f(x)=var:function" to set a temporal var when defining function, example: "f(x)=a=2:ax" = f(x)=2x
- "_" to use the previous answer
- "a={expr}" to define a variable
- "f(x)=..." to define a function
- "f(x,y,z...)=..." to define a multi variable function
- "...=" display parsed input, show values of stuff like xr/deci/prec etc
- "f...=null" to delete a function or variable
- "{x,y,z...}" to define a cartesian vector
- "[radius,theta,phi]" to define a polar vector (same as car{vec})
- "f(x)#g(x)" to graph multiple things
- "{vec}#" to graph a vector
- "{mat}#" to graph a matrix
- "number#" to graph a complex number
- "[f(x),x]" to graph a polar graph of f(x)
- "{x,y}" to graph a parametric equation, example: {cos(x),sin(x)} unit circle, {f(x)cos(x),f(x)sin(x)} for polar graph
- "{x,y,z}" to graph a parametric equation in 3d, example: {cos(x),sin(x),x} helix, {sin(x)cos(y),sin(x)sin(y),cos(x)} sphere
- "{{a,b,c},{d,e,f},{g,h,i}}" to define a 3x3 matrix
- "rnd" to generate a random number
- "epoch" to get time in seconds since unix epoch
- Alt+Enter will not graph whatever is present
- "help {thing}" to get more detail on a function/option/feature
- "help help" to list all things to query

Order of Operations:
- user defined functions
- functions, !x, x!, x!!, |x|
- % (modulus), .. (a..b creates lists of integers from a to b)
- ^/** (exponentiation), // (a//b is a root b) ^^ (tetration), × internal multiplication for units and some negitive signs
- * (multiplication), / (division)
- + (addition), - (subtraction), +-/± (creates a list of the calculation if plus and the calculation if minus)
- to/-> (unit conversions, ie 2m->yd=2.2, leaves unitless if perfect conversion)
- < (lt), <= (le), > (gt), >= (ge), == (eq), != (!eq), >> (a>>b shifts b bits right), << (a<<b shifts b bits left)
- && (a&&b outputs 1 if both a and b are 1), || (a||b outputs 1 if either a or b are 1)

Functions:
- sin, cos, tan, asin, acos, atan, atan(x,y)
- csc, sec, cot, acsc, asec, acot
- sinh, cosh, tanh, asinh, acosh, atanh
- csch, sech, coth, acsch, asech, acoth
- sqrt, cbrt, square, cube, quadratic(a,b,c) cubic(a,b,c,d) (quadratic and cubic finds the zeros for the given polynomials, you can use cubic(a,b,c,d,1) to only find real solutions for cubics/quadratics)
- ln, log(base,num), W(k,z) (product log, branch k, defaults to k=0)
- root(base,exp), sum(var,func,start,end), prod(var,func,start,end)
- abs, sgn, arg
- ceil, floor, round, int, frac
- fact, doublefact, subfact
- sinc, cis, exp
- zeta, gamma, beta, erf, erfc, digamma, ai, multinomial, binomial/bi/C(n,r), P(n,r), pochhammer(x,n)
- re, im, split(x+yi), next(n,to)
- factors, prime, isprime, gcd, lcm
- slog(a,b), ssrt(k,a) (k is lambert w branch)
- piecewise/pw({value,cond},{value2,cond2}...) (when first condition is met from left to right. value elsewards is nan)
- vec(var,func,start,end) mat(var,func,start,end) (makes a vector/matrix) start..end is a shortcut to vec(n,n,start,end)
- tofreq{a,b,c...}, tolist{{a,b},{c,d}...}, tolist{a,b,c} (sorts and counts how many time each number occurs, tolist takes that kind of data and reverses it)
- variance/var, standarddeviation/σ (sample-bias corrected)
- percentile({vec},nth) (gets number at nth percentile), percentilerank({vec},x) (gets percentile rank for x point), quartiles{vec} (gets quartiles for data set)
- normP(μ,σ,x) (normal distribution pdf) normD(z)/normD(x,μ,σ) (area under curve to the left of z score cdf)
- betaP(α,β,x) (beta distribution pdf) I(x,a,b) (regularized incomplete beta function, or beta distributions cdf)
- roll{a,b,c...} rolls die, dice{a,b,c...} gets the frequency data any amount of different sided die, where a/b/c are number of faces for each die, both also accept {{first_dice_face,# of die},{second_dice_face,# of die}...}
- lim(x,f(x),point (,side)) both sides are checked by default, -1 for left, 1 for right
- slope(x,f(x),point (,side) (,nth derivitive) (,0) ), can add a 0 to the args to not combine the x and y slopes for parametric equations, and for area
- area(x,f(x),from,to (,amount of data points) (,0) ), length(x,f(x),from,to (,amount of data points) ) (bracketed means optional)

Vector functions:
- dot({vec1},{vec2}), cross({vec1},{vec2}), proj/project({vec1},{vec2})
- angle({vec1},{vec2})
- norm, normalize
- abs, len, any, all
- max, min, mean, median, mode, sort
- reverse, link
- part({vec},col), sum, prod
- pol{vec} outputs (radius, theta, phi)
- car{vec} outputs (x, y, z)
- other functions are applied like sqrt{2,4}={sqrt(2),sqrt(4)}

Matrix functions:
- eigenvalues
- trace/tr, determinant/det, inverse/inv
- transpose/trans, adjugate/adj, cofactor/cof, minor
- part({mat},col,row), flatten, sum, prod
- abs, norm
- len, wid
- max, min, mean, mode
- iden(n) produces an n dimension identity matrix
- rotate(theta) produces a rotational matrix
- other functions are applied like sqrt{{2,4},{5,6}}={{sqrt(2),sqrt(4)},{sqrt(5),sqrt(6)}} 

Constants:
- c: speed of light, 299792458 m/s
- gr: gravity, 9.80665 m/s^2
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)
- pl: planck's constant, 6.62607015E-34 J*s
- ec: elementary charge/electron volt, 1.602176634E-19 C
- me: electron mass, 9.1093837015E-31 kg
- mp: proton mass, 1.67262192369E-27 kg
- mn: neutron mass, 1.67492749804E-27 kg
- k: coulomb's constant, 8.9875517923E9 N*m^2/C^2
- Na: avogadro's number, 6.02214076E23 1/mol
- R: gas constant, 8.31446261815324 J/(mol*K)
- bo: boltzmann constant, 1.380649E-23 J/K
- phi/φ: golden ratio, 1.6180339887~
- e: euler's number, 2.7182818284~
- pi/π: pi, 3.1415926535~
- tau/τ: tau, 6.2831853071~

Units:
supports metric and binary prefixes
ignores "s" at the end to allow "meters" and stuff
the following units are supported
"m" | "meter"
"s" | "second"
"A" | "ampere"
"K" | "kelvin"
"mol" | "mole"
"cd" | "candela"
"g" | "gram"
"J" | "joule"
"mph"
"mi" | "mile"
"yd" | "yard"
"ft" | "foot"
"in" | "inch"
"lb" | "pound"
"L" | "litre"
"Hz" | "hertz"
"V" | "volt" | "voltage"
"°C" | "celsius"
"°F" | "fahrenheit"
"Wh"
"Ah"
"year"
"month"
"ly"
"kph"
"T" | "tesla"
"H" | "henry"
"weber" | "Wb"
"siemens" | "S"
"F" | "farad"
"W" | "watt"
"Pa" | "pascal"
"Ω" | "ohm"
"min" | "minute"
"h" | "hour"
"d" | "day"
"week"
"N" | "newton"
"C" | "coulomb"
"°" | "deg" | "degrees"
"rad" | "radians"
"grad" | "gradians"
"lumen" | "lm"
"lux" | "lx"
"nit" | "nt"
"byte" | "B"
"gray" | "Gy"
"sievert" | "Sv"
"katal" | "kat"
"bit" | "b"
"steradian" | "sr"

Digraph:
hit escape then a letter, or hold alt while typing(only lowercase)
a=>α, A=>Α, b=>β, B=>Β, c=>ξ, C=>Ξ, d=>Δ, D=>δ,
e=>ε, E=>Ε, f=>φ, F=>Φ, g=>γ, G=>Γ, h=>η, H=>Η,
i=>ι, I=>Ι, k=>κ, Κ=>Κ, l=>λ, L=>Λ, m=>μ, M=>Μ,
n=>ν, Ν=>Ν, o=>ο, O=>Ο, p=>π, P=>Π, q=>θ, Q=>Θ,
r=>ρ, R=>Ρ, s=>σ, S=>Σ, t=>τ, T=>Τ, u=>υ, U=>Υ,
w=>ω, W=>Ω, y=>ψ, Y=>Ψ, x=>χ, X=>Χ, z=>ζ, Z=>Ζ,
==>±, `=>ⁱ, _=>∞, ;=>°
numbers/minus sign convert to superscript acting as exponents
```

# example usage

```
kalc
> 1+1
2
> f(x)=sin(2x) //define f(x), will display how it was parsed
sin(2*x)
> f(x) // graphs f(x) in 2D
sin(2*x)
> f(pi/2) // evaluates f(x) at x=pi/2, so sin(2pi/2)=sin(pi)=0
0
> f(x,y)=x^2+y^2
x^2+y^2
> f(1,2) // evaluates f(x,y) at x=1, y=2, so 1^2+2^2=5
5
> f(x,y) // graphs f(x,y) in 3D
x^2+y^2
> a=3^3
3^3
> cbrt(a)
3
> im(exp(xi)) // graphs the imag part of exp(xi) in 2D, so sin(x)
im(exp(x*1i))
> f(x,y,z,w)=x+y+z+w
x+y+z+w
> f(1,2,3,4) // evaluates f(x,y,z,w) at x=1, y=2, z=3, w=4, so 1+2+3+4=10
10
> f(x,y,2,5) // graphs f(x,y,2,5) in 3D with z=2 and w=5 so x+y+2+5
x+y+2+5
> f(2,5,x,y) // graphs f(2,5,x,y) in 3D with x=2 and y=5 so 2+5+x+y, to graph x and y have to be the unknown variables
2+5+x+y
> |z| // graphs |(x+yi)| in 3D
norm((x+y+1i))
> deg // enables degrees
> pol({5,3,2}+{1,2,3}) // prints {magnitude, theta, phi} of {5,3,2}+{1,2,3}
{9.273618495496,57.373262293469,39.805571092265}
> piecewise({+-sqrt(2^2-x^2),(x<2)&&(x>-2)}) # 3{cos(x),sin(x)} # [5,x] # flat;exp(ix) //graphing circles 4 different ways
piecewise({0±sqrt(2^2-x^2),(x<2)&&(x>-2)})
3*{cos(x),sin(x)}
exp(1i*x)
```

```
echo -ne 'sqrt(pi) \n pi^2'|kalc
1.7724538509055159
9.869604401089358

kalc 'sqrt(pi)' 'pi^2'
1.7724538509055159
9.869604401089358

echo -ne 'sin(x)#cos(x)'|kalc // graphs sin(x) and cos(x) in 2D
kalc 'sin(x)#cos(x)' // graphs sin(x) and cos(x) in 2D
```

# graphing

my gnuplot config in ~/.gnuplot

```
set terminal x11
set xyplane 0
```

chars available for point style:

```
. - dot
+ - plus
x - cross
* - star
s - empty square
S - filled square
o - empty circle
O - filled circle
t - empty triangle
T - filled triangle
d - empty del (upside down triangle)
D - filled del (upside down triangle)
r - empty rhombus
R - filled rhombus
```
