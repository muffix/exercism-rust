extern crate num;

use num::Num;

pub struct Triangle<N>
where
    N: Num + PartialOrd + Copy,
{
    a: N,
    b: N,
    c: N,
}

impl<N> Triangle<N>
where
    N: Num + PartialOrd + Copy,
{
    pub fn build(sides: [N; 3]) -> Option<Triangle<N>> {
        let mut sorted = sides.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        if sorted[2] <= N::zero() || sorted[2] > sorted[1] + sorted[0] {
            return None;
        }

        Some(Triangle {
            a: sorted[2],
            b: sorted[1],
            c: sorted[0],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c
    }
}
