
const EARTH_SECONDS : f64 = 31557600.0;


#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self ( s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 ;
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
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 0.2408467 ;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 0.61519726;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 1.0;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 1.8808158;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 11.862615;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 29.447498;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 84.016846;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period = 164.79132;
        d.0 / ( orbital_period * EARTH_SECONDS )
    }
}
