use num::{
    traits::{NumAssignOps, NumOps},
    Integer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Coordinate3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Coordinate3D { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Coordinate3D<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Coordinate3D { x, y, z }
    }
}

impl<T: Integer + Copy> Coordinate3D<T> {
    pub fn neighbors(&self) -> Vec<Coordinate3D<T>> {
        vec![
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y,
                z: self.z,
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y,
                z: self.z,
            },
            Coordinate3D {
                x: self.x,
                y: self.y - num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x,
                y: self.y + num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x,
                y: self.y,
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y,
                z: self.z - num::one(),
            },
        ]
    }

    pub fn neighbors_extended(&self) -> Vec<Coordinate3D<T>> {
        vec![
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y - num::one(),
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y - num::one(),
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y - num::one(),
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y,
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y,
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y,
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y + num::one(),
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y + num::one(),
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y + num::one(),
                z: self.z - num::one(),
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y - num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x,
                y: self.y - num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y - num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y,
                z: self.z,
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y,
                z: self.z,
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y + num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x,
                y: self.y + num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y + num::one(),
                z: self.z,
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y - num::one(),
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y - num::one(),
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y - num::one(),
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y,
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y,
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y,
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x - num::one(),
                y: self.y + num::one(),
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x,
                y: self.y + num::one(),
                z: self.z + num::one(),
            },
            Coordinate3D {
                x: self.x + num::one(),
                y: self.y + num::one(),
                z: self.z + num::one(),
            },
        ]
    }

    pub fn manhattan(&self, other: &Self) -> T {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
            + self.z.max(other.z)
            - self.z.min(other.z)
    }
}

impl<T: NumOps> std::ops::Add for Coordinate3D<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Coordinate3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: NumAssignOps> std::ops::AddAssign for Coordinate3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: NumOps> std::ops::Sub for Coordinate3D<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Coordinate3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: NumAssignOps> std::ops::SubAssign for Coordinate3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: NumOps> std::ops::Mul for Coordinate3D<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Coordinate3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: NumAssignOps> std::ops::MulAssign for Coordinate3D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Eq + PartialEq + Ord + Copy> Ord for Coordinate3D<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x, self.z).cmp(&(other.y, other.x, other.z))
    }
}

impl<T: Ord + PartialOrd + std::marker::Copy> PartialOrd for Coordinate3D<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::coordinate3d::Coordinate3D;

    #[test]
    fn test_add() {
        assert_eq!(
            Coordinate3D::new(23, 2, -3) + Coordinate3D::new(2, -6, 4),
            Coordinate3D::new(25, -4, 1)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut c = Coordinate3D::new(12, 32, 4);
        c += Coordinate3D::new(3, 10, 0);
        assert_eq!(c, Coordinate3D::new(15, 42, 4));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Coordinate3D::new(12, 2, 5) - Coordinate3D::new(12, 3, 6),
            Coordinate3D::new(0, -1, -1)
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut c = Coordinate3D::new(12, 32, 3);
        c -= Coordinate3D::new(3, 10, 3);
        assert_eq!(c, Coordinate3D::new(9, 22, 0));
    }

    #[test]
    fn test_ord() {
        assert_eq!(
            Coordinate3D::new(4, 3, 5) < Coordinate3D::new(3, 4, 2),
            true
        );
        assert_eq!(
            Coordinate3D::new(3, 4, 6) == Coordinate3D::new(3, 4, 6),
            true
        );
        assert_eq!(
            Coordinate3D::new(4, 3, 3) > Coordinate3D::new(2, 1, 3),
            true
        );
        assert_eq!(
            Coordinate3D::new(3, 4, 8) < Coordinate3D::new(2, 1, 2),
            false
        );
    }

    #[test]
    fn test_manhattan() {
        let a = Coordinate3D::new(4, 3, 2);
        let b = Coordinate3D::new(3, 4, 4);
        assert_eq!(a.manhattan(&b), 4);
        assert_eq!(b.manhattan(&a), 4);
        assert_eq!(b.manhattan(&b), 0);
    }
}
