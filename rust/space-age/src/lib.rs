// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#![allow(unused_variables)]

const SECONNDS_IN_EARTH_YEAR: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { earth_years: s as f64 / SECONNDS_IN_EARTH_YEAR }
    }
}

pub trait Planet {
    const N: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::N
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const N: f64 = 0.2408467;
}
impl Planet for Venus {
    const N: f64 = 0.61519726;
}
impl Planet for Earth {
    const N: f64 = 1.0;
}
impl Planet for Mars {
    const N: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const N: f64 = 11.862615;
}
impl Planet for Saturn {
    const N: f64 = 29.447498;
}
impl Planet for Uranus {
    const N: f64 = 84.016846;
}
impl Planet for Neptune {
    const N: f64 = 164.79132;
}
