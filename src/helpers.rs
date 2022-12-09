use num::{
    traits::{NumAssignOps, NumOps},
    Integer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Coordinate { x, y }
    }
}

impl<T> From<(T, T)> for Coordinate<T> {
    fn from((x, y): (T, T)) -> Self {
        Coordinate { x, y }
    }
}

impl<T: Integer + Copy> Coordinate<T> {
    pub fn neighbors(&self) -> Vec<Coordinate<T>> {
        vec![
            Coordinate {
                x: self.x - num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x,
                y: self.y + num::one(),
            },
        ]
    }

    pub fn neighbors_extended(&self) -> Vec<Coordinate<T>> {
        vec![
            Coordinate {
                x: self.x - num::one(),
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x,
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y - num::one(),
            },
            Coordinate {
                x: self.x - num::one(),
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y + num::one(),
            },
            Coordinate {
                x: self.x - num::one(),
                y: self.y + num::one(),
            },
            Coordinate {
                x: self.x,
                y: self.y + num::one(),
            },
            Coordinate {
                x: self.x + num::one(),
                y: self.y + num::one(),
            },
        ]
    }

    pub fn manhattan(&self, other: &Self) -> T {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl<T: NumOps> std::ops::Add for Coordinate<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: NumAssignOps> std::ops::AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: NumOps> std::ops::Sub for Coordinate<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: NumAssignOps> std::ops::SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Eq + PartialEq + Ord + Copy> Ord for Coordinate<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl<T: Ord + PartialOrd + std::marker::Copy> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::Coordinate;

    #[test]
    fn test_add() {
        assert_eq!(
            Coordinate::new(23, 2) + Coordinate::new(2, -6),
            Coordinate::new(25, -4)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut c = Coordinate::new(12, 32);
        c += Coordinate::new(3, 10);
        assert_eq!(c, Coordinate::new(15, 42));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Coordinate::new(12, 2) - Coordinate::new(12, 3),
            Coordinate::new(0, -1)
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut c = Coordinate::new(12, 32);
        c -= Coordinate::new(3, 10);
        assert_eq!(c, Coordinate::new(9, 22));
    }

    #[test]
    fn test_ord() {
        assert_eq!(Coordinate::new(4, 3) < Coordinate::new(3, 4), true);
        assert_eq!(Coordinate::new(3, 4) == Coordinate::new(3, 4), true);
        assert_eq!(Coordinate::new(4, 3) > Coordinate::new(2, 1), true);
        assert_eq!(Coordinate::new(3, 4) < Coordinate::new(2, 1), false);
    }

    #[test]
    fn test_manhattan() {
        let a = Coordinate::new(4, 3);
        let b = Coordinate::new(3, 4);
        assert_eq!(a.manhattan(&b), 2);
        assert_eq!(b.manhattan(&a), 2);
        assert_eq!(b.manhattan(&b), 0);
    }
}
