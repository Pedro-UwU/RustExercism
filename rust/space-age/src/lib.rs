// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
//

macro_rules! planet_time {
    ($orbital_period:expr, $duration:expr) => {
        $duration / (SECONDS_PER_YEAR * $orbital_period)
    };
}

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub fn as_seconds(&self) -> u64 {
        self.seconds
    }
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration { seconds }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

const SECONDS_PER_YEAR: f64 = 31557600.0;

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
        planet_time!(0.2408467, d.as_seconds() as f64)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        planet_time!(0.61519726, d.as_seconds() as f64)
    }

}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        planet_time!(1.0, d.as_seconds() as f64)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        planet_time!(1.8808158, d.as_seconds() as f64)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        planet_time!(11.862615, d.as_seconds() as f64)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        planet_time!(29.447498, d.as_seconds() as f64)
    }
}
impl Planet for Uranus { 
    fn years_during(d: &Duration) -> f64 {
        planet_time!(84.016846, d.as_seconds() as f64)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        planet_time!(164.79132, d.as_seconds() as f64)
    }
}
