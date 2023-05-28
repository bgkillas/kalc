# kalc
[![AUR](https://img.shields.io/aur/version/kalc.svg)](https://aur.archlinux.org/packages/kalc/)

requires gnuplot for graphing, a modern terminal like windows terminal on windows

history file is stored in ``~/.config/kalc.history`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.history``

config file is stored in ``~/.config/kalc.config`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.config``

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
--debug displays computation time in nanoseconds

- Type "exit" to exit the program
- Type "clear" to clear the screen
- Type "help" to see this message
- Type "history" to see the history of calculations
- Type "deg" to switch to degrees mode
- Type "rad" to switch to radians mode
- Type "tau" to show fractions in tau
- Type "pi" to show fractions in pi
- Type "2d=[num]" to set the number of points in 2D graphs
- Type "3d=[num]" to set the number of points in 3D graphs
- Type "xr=[min],[max]" to set the x range for graphing
- Type "yr=[min],[max]" to set the y range for graphing
- Type "zr=[min],[max]" to set the z range for graphing
- Type "point=[char]" to set the point style for graphing
- Type "sci" to toggle scientific notation
- Type "base=[num]" to set the number base (2,8,16)
- Type "debug" toggles displaying computation time in nanoseconds
- Type "_" to use the previous answer
- Type "a=[num]" to define a variable
- Type "f(x)=..." to define a function

Trigonometric functions:
- sin, cos, tan, asin, acos, atan
- csc, sec, cot, acsc, asec, acot
- sinh, cosh, tanh, asinh, acosh, atanh
- csch, sech, coth, acsch, asech, acoth

Other functions:
- sqrt, cbrt, square, cube
- ln, log(base,num), root(base,exp)
- abs, sgn, arg
- ceil, floor, round, int, frac
- fact, subfact
- sinc, cis, exp
- deg(to_degrees), rad(to_radians)
- re(real part), im(imaginary part)

Examples:
- To calculate the sine of 0.5, type: sin(0.5)
- To calculate the logarithm base 2 of 8, type: log(2,8)
- To graph x^2, type: x^2
- To graph (x+yi)^2 type z^2 or (x+yi)^2
- To graph x^2, x^3, and x, type: x^2#x^3#x
- To change the x range to -10 to 10, type: xr=-10,10
- To change the number of points to 100000 for a 2D graph, type: 2d=100000

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