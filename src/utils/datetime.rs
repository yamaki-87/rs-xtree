use std::{ops::Deref, time::SystemTime};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, TimeZone, Utc,
};

pub fn yyyy_mm_dd_format(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.format("%Y-%m-%d").to_string()
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DateTimeWrap(DateTime<Utc>);

impl From<i64> for DateTimeWrap {
    fn from(value: i64) -> Self {
        Self(Utc.timestamp_opt(value, 0).unwrap())
    }
}

impl From<SystemTime> for DateTimeWrap {
    fn from(value: SystemTime) -> Self {
        Self(value.into())
    }
}

impl DateTimeWrap {
    pub fn yyyy_mm_dd_format(&self) -> DelayedFormat<StrftimeItems<'_>> {
        self.format("%Y-%m-%d")
    }
}

impl Deref for DateTimeWrap {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
