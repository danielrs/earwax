use num::rational::Rational64;

/// Used for representing points in time of the audio
/// stream.
#[derive(Debug, Copy, Clone)]
pub struct Timestamp {
    time_base: Rational64,
    pts: i64,
}

impl Timestamp {
    pub fn new(time_base: Rational64) -> Self {
        Timestamp {
            time_base: time_base,
            pts: 0,
        }
    }

    pub fn from_seconds(time_base: Rational64, seconds: i64) -> Self {
        let pts = seconds * time_base.denom() * time_base.numer();
        Timestamp {
            time_base: time_base,
            pts: pts,
        }
    }

    pub fn from_pts(time_base: Rational64, pts: i64) -> Self {
        Timestamp {
            time_base: time_base,
            pts: pts,
        }
    }

    pub fn seconds(&self) -> i64 {
        (self.pts * self.time_base.numer()) / self.time_base.denom()
    }

    pub fn set_seconds(&mut self, seconds: i64) {
        self.pts = seconds * self.time_base.denom() * self.time_base.numer();
    }

    pub fn pts(&self) -> i64 {
        self.pts
    }

    pub fn set_pts(&mut self, pts: i64) {
        self.pts = pts
    }
}
