use rug::Complex;
pub trait Float
{
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;
}
impl Float for (f32, f32)
{
    fn cos(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.cos(), 0.0)
        }
        else if self.0 == 0.0
        {
            (self.1.cosh(), 0.0)
        }
        else
        {
            (self.0.cos() * self.1.cosh(), -self.0.sin() * self.1.sinh())
        }
    }
    fn sin(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.sin(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.sinh())
        }
        else
        {
            (self.0.sin() * self.1.cosh(), self.0.cos() * self.1.sinh())
        }
    }
    fn tan(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.tan(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.tanh())
        }
        else
        {
            let c = (2.0 * self.0).cos() + (2.0 * self.1).cosh();
            ((2.0 * self.0).sin() / c, (2.0 * self.1).sinh() / c)
        }
    }
}
impl Float for (f64, f64)
{
    fn cos(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.cos(), 0.0)
        }
        else if self.0 == 0.0
        {
            (self.1.cosh(), 0.0)
        }
        else
        {
            (self.0.cos() * self.1.cosh(), -self.0.sin() * self.1.sinh())
        }
    }
    fn sin(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.sin(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.sinh())
        }
        else
        {
            (self.0.sin() * self.1.cosh(), self.0.cos() * self.1.sinh())
        }
    }
    fn tan(self) -> Self
    {
        if self.1 == 0.0
        {
            (self.0.tan(), 0.0)
        }
        else if self.0 == 0.0
        {
            (0.0, self.1.tanh())
        }
        else
        {
            let c = (2.0 * self.0).cos() + (2.0 * self.1).cosh();
            ((2.0 * self.0).sin() / c, (2.0 * self.1).sinh() / c)
        }
    }
}
impl Float for Complex
{
    fn cos(self) -> Self
    {
        self.cos()
    }
    fn sin(self) -> Self
    {
        self.sin()
    }
    fn tan(self) -> Self
    {
        self.tan()
    }
}