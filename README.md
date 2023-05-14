# calc
requires gnuplot for graphing

the following functions in the complex plane

``+,-,*,/,^,``

``sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh``

``csc, sec, cot, acsc, asec, acot, csch, sech, coth, acsch, asech, acoth``

``sqrt, cbrt, ln, log(base,num), root(base,exp), exp, abs, arg, sgn``

``fact, int, frac, ceil, floor, round`` 

``deg,rad``

``re(real part),im(imag part)``

constants: ``pi, e``

history file is stored in ``~/.config/.calc_history`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\calc.history``

# usage
```
calc 1+1
2
```
```
echo 1+1 | calc
2
```
```
calc
>asin(0.5)
π/6
0.5235987755982988
```
```
calc x^2
graphs x^2
```
```
calc x^2#x^3#x
graphs x^2, x^3 and x
```
```
calc
> -e^(x)=e^(x+ipi)
true
```
```
calc z^2
graphs (x+yi)^2
```
```
calc
> x=4
> x^2
16
```
```
calc
> x^2
graphs x^2
```
```
calc
> x^y
graphs x^y
```
```
calc
> clear
clears terminal and graph data
```
```
calc
> xrange=-10,10
change the x range of the graph
```
```
calc
> 3d=400
change the number of points in the graph
```
```
calc
> 2d=100000
change the number of points in the graph
```