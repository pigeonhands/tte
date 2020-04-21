
use std::time::{Instant, Duration};
use std::default::Default;
use std::fmt;
use std::ops::{Add, Sub};
use rust_decimal::Decimal;

#[derive(Default, Clone, Copy)]
pub struct TimeSpan {
    nanos: u128,
}

impl From<Duration> for TimeSpan {
    fn from(d:Duration) -> Self {
        Self{
            nanos: d.as_nanos(),
        }
    }
}

impl Into<Duration> for TimeSpan{
    fn into(self) -> Duration{
        Duration::from_nanos(self.nanos as u64)
    } 
}

impl Add<TimeSpan> for TimeSpan{
    type Output = TimeSpan;
    fn add(self, t: TimeSpan) -> TimeSpan {
        TimeSpan{
            nanos: self.nanos + t.nanos
        }
    }
}

impl Add<Duration> for TimeSpan{
    type Output = TimeSpan;
    fn add(self, d: Duration) -> TimeSpan {
        TimeSpan{
            nanos: self.nanos + d.as_nanos()
        }
    }
}


impl Sub<TimeSpan> for TimeSpan{
    type Output = TimeSpan;
    fn sub(self, t: TimeSpan) -> TimeSpan {
        TimeSpan{
            nanos: self.nanos - t.nanos
        }
    }
}

impl Sub<Duration> for TimeSpan{
    type Output = TimeSpan;
    fn sub(self, d: Duration) -> TimeSpan {
        TimeSpan{
            nanos: self.nanos - d.as_nanos()
        }
    }
}




impl TimeSpan {
    const NANOS_PER_MICRO: u32 = 1_000;
    const NANOS_PER_SEC: u128 = 1_000_000_000;
    const NANOS_PER_MINUTE: u128 = Self::NANOS_PER_SEC * 60;
    const NANOS_PER_HOUR: u128 = Self::NANOS_PER_MINUTE * 60;
    const NANOS_PER_DAY: u128 = Self::NANOS_PER_HOUR * 24;


  
    pub fn total_days(&self) -> Decimal{
        Decimal::new(self.nanos as i64, 0) * (Decimal::new(1, 0) / Decimal::new(Self::NANOS_PER_DAY as i64, 0))
    }

    pub fn total_hours(&self) -> Decimal{
        Decimal::new(self.nanos as i64, 0) *  (Decimal::new(1, 0) / Decimal::new(Self::NANOS_PER_HOUR as i64, 0))
    }

    pub fn total_minutes(&self) -> Decimal{
        Decimal::new(self.nanos as i64, 0) *  (Decimal::new(1, 0) / Decimal::new(Self::NANOS_PER_MINUTE as i64, 0))
    }

    pub fn total_seconds(&self) -> Decimal{
        Decimal::new(self.nanos as i64, 0) *  (Decimal::new(1, 0) / Decimal::new(Self::NANOS_PER_SEC as i64, 0))
    }

    pub fn total_ms(&self) -> Decimal {
        Decimal::new(self.nanos as i64, 0) *  (Decimal::new(1, 0) / Decimal::new(Self::NANOS_PER_MICRO as i64, 0))
    }
    pub fn total_nano(&self) -> u128 {
        self.nanos
    }

    pub fn days(&self) -> u32 {
        (self.nanos / Self::NANOS_PER_DAY) as u32
    }
    pub fn hours(&self) -> u32 {
        ((self.nanos / Self::NANOS_PER_HOUR) % 24) as u32
    }
    pub fn minutes(&self) -> u32 {
        ((self.nanos / Self::NANOS_PER_MINUTE) % 60) as u32
    }
    pub fn seconds(&self) -> u32 {
        ((self.nanos / Self::NANOS_PER_SEC) % 60) as u32
    }

    pub fn ms(&self) -> u32 {
        ((self.nanos / Self::NANOS_PER_MINUTE) % 1000) as u32
    }

    pub fn nanos(&self) -> u128 {
        self.nanos
    }
    
}

#[derive(Default, Clone, Copy)]
pub struct Profiler {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    elapsed_time: TimeSpan,
}

impl fmt::Display for Profiler {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elapsed = self.elapsed();
		return write!(f, "{}ms", elapsed.total_minutes());
	}
}

impl Profiler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn start_new() -> Self{
        let mut inst = Self::default();
        inst.start();
        inst
    }

    pub fn start(&mut self) {
        if self.start_time.is_none(){
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if self.end_time.is_none(){
            self.end_time = Some(Instant::now());
        }

        if let (Some(start), Some(end)) = (self.start_time, self.end_time){
            self.elapsed_time = (end - start).into();
        }else{
            self.elapsed_time = TimeSpan::default();
        }
    }

    pub fn clear(&mut self) {
        self.start_time = None;
        self.end_time = None
    }

    pub fn since_started(&self) -> TimeSpan{
        let current_inst = Instant::now();
        if let Some(started) = self.start_time{
            (current_inst - started).into()
        }else{
            Duration::from_secs(0).into()
        }
    }

    pub fn elapsed(&self) -> TimeSpan {
        self.elapsed_time
    }

}