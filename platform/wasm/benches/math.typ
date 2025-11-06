Review Example

1. Let $f(x,y) = sqrt(4-x^2)+sqrt(2-y^2)$

a) Find and sketch the domain

$ 4-x^2 &>= 0 \ x^2 &<= 4 \ |x| &<= 2 $

$ 2-y^2 &>= 0 \ y^2 &<= 2 \ |y| &<= sqrt(2) $

Domain: $ D = {(x,y)| |x| <= 2 "and" |y| <= sqrt(2)} $

b) Find the critial point of $z=f(x,y)$

$ f_x(x,y) &= 0 \
  (-2x)/(2 sqrt(4-x^2)) &= 0 \
   x &= 0 $

$ f_y(x,y) &= 0 \
  (-2y)/(2 sqrt(2-y^2)) &= 0 \
  y &= 0 $

c) Find the equation of the tangent plane at the points $(0,0)$ and $(sqrt(3), 1)$

$ z - z_0 &= f_x(x,y)(x-x_0) + f_y(x,y)(y-y_0) \
  z - f(0,0) &= f_x(0,0)x + f_y(0,0)y \
  z &= f(0,0) \
  &= 2 + sqrt(2) $

$ z - z_0 &= f_x(x,y)(x-x_0) + f_y(x,y)(y-y_0) \
  z - f(sqrt(3), 1) &= f_x(sqrt(3),1)(x-sqrt(3)) + f_y(sqrt(3),1)(y-1) \
  z - 2 &= -sqrt(3)(x-sqrt(3)) + (y-1) \
  z &= -sqrt(3)x + 3 - y + 1 + 2 \
  z &= -sqrt(3)x - y + 6 \
  $

d) Find the local extrema
- We already have the critial point $(0,0)$

$ f_(x x) &= (-x dot 1/sqrt(4-x^2) - -1 dot sqrt(4-x^2))/(4-x^2) \
  &= (-4)/(4-x^2)^(3\/2) \
  f_(y y) &= (-2x dot 1/sqrt(2-y^2) - -2 dot 2 sqrt(2-y^2))/(2-y^2) \
  &= (-2)/(2-y^2)^(3\/2) \
  f_(x y) &= 0 \
  f_(y x) &= 0 \
$

$ f_(x,x)(0,0) dot f_(y,y)(0,0) &= (-4)/(4-x^2)^(3\/2) dot (-2)/(2-y^2)^(3\/2) \
  &= (-4)/(4^(3\/2)) dot (-2)/(2^(3\/2)) \
  &= (-1)/2 dot (-1)/sqrt(2) \
  &= sqrt(2)/4 > 0 \
  &therefore f(0,0) "is a local max"
$

e) Find the absolute extrema on the domain of $f=(x,y)$

- *Step 1:* Find the critical points and evaluate $f(x,y)$ at the critial points.
- *Step 2:* Check the value on the boundary

  On $L_1$: $ y &= -sqrt(2) quad -2 <= x <= 2 $ So, $ f(x, sqrt(2)) = sqrt(4-x^2) $

$ f(plus.minus 2, -sqrt(2)) &= sqrt(4-(plus.minus 2) ^2) \ & = 0 \
  f'(x) &= (-x)/sqrt(4-x^2) \ &= 0 \ x &= 0 \
  f(0, -sqrt(2))  &= 2 $

On $L_2$: $ x=2 quad -sqrt(2) <= y <= sqrt(2) $ So, $ z = f(2,y) = sqrt(2-y^2) $

$ f(2, plus.minus sqrt(2)) &= 0 \
  f'(2,y) &= (-y)/sqrt(2-y) \ &= 0 \
  y &= 0 \
  f(2,0) &= sqrt(2) $

On $L_3$: $ y = sqrt(2) quad -2 <= x <= 2 $ So, $ f(x,sqrt(2)) = sqrt(4-x^2) $

On $L_4$: $ x = -2 quad -sqrt(2) <= y sqrt(2) $ So, $ f(-2,y) = sqrt(2-y^2) $

f.

Show that the limit exists

$ lim_((x,y)->(0,0)) (e^(-x^2-y^2)+1)/(x^2+y^2) $

$ "Let" quad x &= r cos(theta) \
  y &= r sin(theta) \
  x^2 + y^2 &= r^2 $

$ &= lim_(r->0) (e^(-r^2)-1)/r \
  &=^"L'H" lim_(r->0) (-2r e^(-r^2))/(2r) \
  &= lim_(r->0) -e^(-r^2) $

Show that the limit exists

$ lim_((x,y)->(0,0)) &= (x^2+y^3)/(x^2+y^2) \
  &= lim_(r->0) (r^3cos^3(theta)+r^3sin^3(theta))/(r^2) \ &= 0 $

