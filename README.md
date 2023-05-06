# calc
a fast rust calculator that supports the following functions in the complex plane

``+,-,*,/,^,``

``sin, cos, tan, asin, acos, atan, ``

``sinh, cosh, tanh, asinh, acosh, atanh, ``

``sqrt, cbrt, ln, log(base,num), abs,`` 

``dg(to_degrees),rd(to_radians)``

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
calc x^2
gives graph data for x^2 from -10 to 10
```
```
calc x^y
gives graph data for x^y from x=-10..10 and y=-10..10
```
```
calc
> x=4
> x^2
16
```