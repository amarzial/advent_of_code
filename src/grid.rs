use num::Signed;

pub fn manhattan<T: Signed>(p1: (T, T), p2: (T, T)) -> T {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
