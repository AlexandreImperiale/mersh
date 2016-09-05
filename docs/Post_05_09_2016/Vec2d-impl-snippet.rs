pub fn amplify(&mut self, a: f64) -> &mut Vec2d
{
    self.coords.amplify(a); self
}

pub fn norm(&self) -> f64
{
    self.coords.norm()
}
