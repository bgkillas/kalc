# calc
requires gnuplot for graphing

history file is stored in ``~/.config/.calc_history`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\calc.history``

# usage
```
Usage: calc [FLAGS] function_1 function_2 function_3...
FLAGS: --help (this message),--debug for computation time in nanoseconds

- Type "exit" to exit the program.
- Type "clear" to clear the screen.
- Type "help" to see this message.

Trigonometric functions:
- sin, cos, tan, asin, acos, atan
- csc, sec, cot, acsc, asec, acot
- sinh, cosh, tanh, asinh, acosh, atanh
- csch, sech, coth, acsch, asech, acoth

Other functions:
- sqrt, cbrt
- ln, log(base,num), root(base,exp)
- abs, sgn, arg
- ceil, floor, round, int, frac
- fact, subfact
- sinc, exp
- deg(to_degrees), rad(to_radians)
- re(real part), im(imaginary part)

Special features:
- Graphing: type a function with one variable and add "graphs" to graph it.
- Graphing multiple functions: use the "#" character to separate the functions.
- Change the x range of the graph: use "xr=min,max".
- Change the number of points in the graph: use "2d=num_points" for 2D graphs or "3d=num_points" for 3D graphs.

Examples:
- To calculate the sine of 0.5, type: sin(0.5)
- To calculate the logarithm base 2 of 8, type: log(2,8)
- To graph x^2, type: x^2 graphs
- To graph x^2, x^3, and x, type: x^2#x^3#x graphs
- To change the x range to -10 to 10, type: xr=-10,10
- To change the number of points to 100000 for a 2D graph, type: 2d=100000
```