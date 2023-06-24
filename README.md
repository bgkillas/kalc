# kalc
[![AUR](https://img.shields.io/aur/version/kalc.svg)](https://aur.archlinux.org/packages/kalc/)

![image](https://github.com/bgkillas/kalc/assets/55570525/d6b6775e-0080-409a-be0b-9aa4e3fae871)

requires gnuplot for graphing, a modern terminal like windows terminal on windows

history file is stored in ``~/.config/kalc.history`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.history``

config file is stored in ``~/.config/kalc.config`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.config``

you can set permanent variables and functions in the file ``~/.config/kalc.vars`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.vars``

parsing tries to comply with wolfram alpha

# known issues
- setting precision will remove vars if you change or delete any of the default vars
- sin^-4!(2) fails to parse
- sin^(-4+2)(2) will not parse as sin(2)^(-4+2)

# build instructions
rust>=1.70.0 diffutils gcc m4 make
```
git clone https://github.com/bgkillas/kalc
cd kalc
cargo build --release
./target/release/kalc
```

# usage
```
Usage: kalc [FLAGS] function_1 function_2 function_3...
FLAGS: --help (this message)
--tau fractions are shown in tau instead of pi
--deg compute in degrees, gets rid of complex support for non hyperbolic trig functions
--2d [num] number of points to graph in 2D
--3d [num] number of points to graph in 3D
--xr [min] [max] x range for graphing
--yr [min] [max] y range for graphing
--zr [min] [max] z range for graphing
--point [char] point style for graphing
--sci toggles scientific notation
--base [num] sets the number base (2,8,16)
--prompt toggles the prompt
--color toggles color
--comma toggles comma seperation
--vars toggles default variables
--line toggles line graphing
--rt toggles real time printing
--polar toggles displaying polar vectors
--frac toggles fraction display
--prec [num] sets the precision
--deci [num] sets how many decimals to display, also max length of numerator and denominator in fractions
--def ignores config file
--debug displays computation time in nanoseconds

- Type "exit" to exit the program
- Type "clear" to clear the screen
- Type "help" to see this message
- Type "history" to see the history of calculations
- Type "deg" to switch to degrees mode
- Type "rad" to switch to radians mode
- Type "tau" to show fractions in tau
- Type "pi" to show fractions in pi
- Type "prompt" to toggle the prompt
- Type "line" to toggle line graphing
- Type "rt" to toggle real time printing
- Type "color" to toggle color
- Type "comma" to toggle comma seperation
- Type "2d=[num]" to set the number of points in 2D graphs
- Type "3d=[num]" to set the number of points in 3D graphs
- Type "xr=[min],[max]" to set the x range for graphing
- Type "yr=[min],[max]" to set the y range for graphing
- Type "zr=[min],[max]" to set the z range for graphing
- Type "prec=[num]" to set the precision
- Type "deci=[num]" to set how many decimals to display, also max length of numerator and denominator in fractions
- Type "point=[char]" to set the point style for graphing
- Type "sci" to toggle scientific notation
- Type "vars" to list all variables
- Type "base=[num]" to set the number base (2-36)
- Type "_" to use the previous answer
- Type "a=[num]" to define a variable
- Type "f(x)=..." to define a function
- Type "f(x,y)=..." to define a 2 variable function
- Type "f(x,y,z...)=..." to define a multi variable function
- Type "...=" display parsed input, show values of stuff like xr/deci/prec etc
- Type "f...=null" to delete a function or variable
- Type "{x,y,z...}" to define a cartesian vector
- Type "[radius,theta,phi]" to define a polar vector (same as car{vec})
- Type "{vec}#" to graph a vector
- Type "number#" to graph a complex number
- Type "polar" to toggle polar output
- Type "frac" to toggle fraction display
- Type "debug" toggles displaying computation time in nanoseconds

Operators:
- +, -, *, /, ^, %, <, >, <=, >=
- !x (subfact), x! (fact)
- && (and), || (or), == (equals), != (not equals)
- >> (right shift), << (left shift)

Trigonometric functions:
- sin, cos, tan, asin, acos, atan, atan(x,y)
- csc, sec, cot, acsc, asec, acot
- sinh, cosh, tanh, asinh, acosh, atanh
- csch, sech, coth, acsch, asech, acoth

Other functions:
- sqrt, cbrt, square, cube
- ln, log(base,num), root(base,exp), sum(func,var,start,end), prod(func,var,start,end)
- abs, sgn, arg
- ceil, floor, round, int, frac
- fact, subfact
- sinc, cis, exp
- zeta, gamma, erf, erfc, digamma, ai
- deg(to_degrees), rad(to_radians)
- re(real part), im(imaginary part)

Vector operations/functions:
- dot product: {vec1}.{vec2}
- cross product: {vec1}x{vec2}
- magnitude: |{vec}|
- normal operations: {vec}^{vec}, {vec}*{vec}, {vec}/{vec}, {vec}+{vec}, {vec}-{vec} (works with scalars too)
- convert to polar: pol{vec} outputs (radius, theta, phi)
- convert to cartesian: car{vec} outputs (x, y, z)

Constants:
- c: speed of light, 299792458 m/s
- g: gravity, 9.80665 m/s^2
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)
- h: planck's constant, 6.62607015E-34 J*s
- ec: elementary charge, 1.602176634E-19 C
- me: electron mass, 9.1093837015E-31 kg
- mp: proton mass, 1.67262192369E-27 kg
- mn: neutron mass, 1.67492749804E-27 kg
- ev: electron volt, 1.602176634E-19 J
- kc: coulomb's constant, 8.9875517923E9 N*m^2/C^2
- na: avogadro's number, 6.02214076E23 1/mol
- r: gas constant, 8.31446261815324 J/(mol*K)
- kb: boltzmann constant, 1.380649E-23 J/K
- phi: golden ratio, 1.6180339887~
- e: euler's number, 2.7182818284~
- pi: pi, 3.1415926535~
- tau: tau, 6.2831853071~
```
# example usage
```
kalc
> 1+1
2
> f(x)=sin(2x)
> f(x) // graphs f(x) in 2D
> f(pi/2) // evaluates f(x) at x=pi/2, so sin(2pi/2)=sin(pi)=0
0
> f(x,y)=x^2+y^2
> f(1,2) // evaluates f(x,y) at x=1, y=2, so 1^2+2^2=5
5
> f(x,y) // graphs f(x,y) in 3D
> a=3^3
> cbrt(a)
3
> im(exp(xi)) // graphs the imag part of exp(xi) in 2D, so sin(x)
> f(x,y,z,w)=x+y+z+w
> f(1,2,3,4) // evaluates f(x,y,z,w) at x=1, y=2, z=3, w=4, so 1+2+3+4=10
10
> f(x,y,2,5) // graphs f(x,y,2,5) in 3D with z=2 and w=5 so x+y+2+5
> f(x,y,2,5)= // displays how its parsed
((x)+(y)+(2)+(5))
> f(2,5,x,y) // graphs f(2,5,x,y) in 3D with x=2 and y=5 so 2+5+x+y, to graph x and y have to be the unknown variables
> |z| // graphs |(x+yi)| in 3D
> deg // enables degrees
> pol({5,3,2}+{1,2,3}) // prints {magnitude, theta, phi} of {5,3,2}+{1,2,3}
{9.273618495496,57.373262293469,39.805571092265}
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
```
chars available for point style:
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