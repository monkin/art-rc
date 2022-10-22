use super::mix::Mix;
use num_traits::{one, zero};

pub trait Curve<T: Mix + Clone>: Clone {
    fn value_at(&self, t: T::Fraction) -> T;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Curve0<T: Mix + Clone>(T);

#[derive(Clone, PartialEq, Debug)]
pub struct Curve1<T: Mix + Clone>(T, T);

#[derive(Clone, PartialEq, Debug)]
pub struct Curve2<T: Mix + Clone>(T, T, T);

#[derive(Clone, PartialEq, Debug)]
pub struct Curve3<T: Mix + Clone>(T, T, T, T);

impl<T: Mix + Clone + Copy> Copy for Curve0<T> {}
impl<T: Mix + Clone + Copy> Copy for Curve1<T> {}
impl<T: Mix + Clone + Copy> Copy for Curve2<T> {}
impl<T: Mix + Clone + Copy> Copy for Curve3<T> {}

impl<T: Mix + Clone> Curve<T> for Curve0<T> {
    fn value_at(&self, _t: T::Fraction) -> T {
        self.0.clone()
    }
}

impl<T: Mix + Clone> Curve<T> for Curve1<T> {
    fn value_at(&self, t: T::Fraction) -> T {
        if t == zero() {
            self.0.clone()
        } else if t == one() {
            self.1.clone()
        } else {
            self.0.clone().mix(self.1.clone(), t)
        }
    }
}

impl<T: Mix + Clone> Curve<T> for Curve2<T> {
    fn value_at(&self, t: T::Fraction) -> T {
        if t == zero() {
            self.0.clone()
        } else if t == one() {
            self.2.clone()
        } else {
            let v1 = self.0.clone().mix(self.1.clone(), t);
            let v2 = self.1.clone().mix(self.2.clone(), t);

            v1.mix(v2, t)
        }
    }
}

impl<T: Mix + Clone> Curve<T> for Curve3<T> {
    fn value_at(&self, t: T::Fraction) -> T {
        if t == zero() {
            self.0.clone()
        } else if t == one() {
            self.1.clone()
        } else {
            let v1 = self.0.clone().mix(self.1.clone(), t);
            let v2 = self.1.clone().mix(self.2.clone(), t);
            let v3 = self.2.clone().mix(self.3.clone(), t);

            let x1 = v1.mix(v2.clone(), t);
            let x2 = v2.mix(v3, t);

            x1.mix(x2, t)
        }
    }
}
