
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

macro_rules! Years {
    ($struct_name:ident, $period:expr) => {
        pub struct $struct_name;

        impl Planet for $struct_name {
                fn years_during(d: &Duration) -> f64 {
                    d.0 / ( $period * EARTH_SECONDS )
                }
        }
    }
}

Years![Mercury, 0.2408467];
Years![Venus, 0.61519726];
Years![Earth, 1.0];
Years![Mars, 1.8808158];
Years![Jupiter,11.862615];
Years![Saturn,29.447498];
Years![Uranus, 84.016846 ];
Years![Neptune, 164.79132];