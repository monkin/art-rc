use super::Vector;

pub trait Normal {
    fn normal(p1: Self, p2: Self) -> Vector;
}
