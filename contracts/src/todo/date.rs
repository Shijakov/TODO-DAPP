use chrono::{Datelike, NaiveDate, Weekday, Days, NaiveDateTime};

use crate::errors::TodoError;

pub type UncheckedDate = (u16, u8, u8);

pub type Date = UncheckedDate;

#[derive(Debug, PartialEq, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

pub trait DateMethods {
    fn from_timestamp(timestamp_milis: u64) -> Self;

    fn from_naive(date: NaiveDate) -> Self;

    fn from_unchecked(date: UncheckedDate) -> Result<Date, TodoError>;

    fn day_of_week(&self) -> Result<DayOfWeek, TodoError>;

    fn compare(&self, other: Date) -> i32;

    fn add_days(&self, days: u64) -> Self;

    fn subtract_days(&self, days: u64) -> Self;
}

impl DateMethods for Date {
    fn from_timestamp(timestamp_milis: u64) -> Self {
        let timestamp_seconds = (timestamp_milis / 1000) as i64;

        let today = NaiveDateTime::from_timestamp_opt(timestamp_seconds, 0).unwrap();

        let year = today.year();
        let month = today.month();
        let day = today.day();

        (year as u16, month as u8, day as u8)
    }

    fn from_naive(date: NaiveDate) -> Self {
        (
            date.year_ce().1 as u16,
            date.month0() as u8 + 1,
            date.day0() as u8 + 1,
        )
    }

    fn from_unchecked(date: UncheckedDate) -> Result<Date, TodoError> {
        let date = NaiveDate::from_ymd_opt(date.0.into(), date.1.into(), date.2.into());

        match date {
            Some(d) => Ok(Date::from_naive(d)),
            None => Err(TodoError::InvalidDate),
        }
    }

    fn day_of_week(&self) -> Result<DayOfWeek, TodoError> {
        let date = NaiveDate::from_ymd_opt(self.0.into(), self.1.into(), self.2.into());

        match date {
            Some(d) => Ok(DayOfWeek::from(d.weekday())),
            None => Err(TodoError::InvalidDate),
        }
    }

    fn compare(&self, other: Date) -> i32 {
        (self.0 as i32 * 365 + self.1 as i32 * 30 + self.2 as i32)
            - (other.0 as i32 * 365 + other.1 as i32 * 30 + other.2 as i32)
    }

    fn add_days(&self, days: u64) -> Self {
        let date = NaiveDate::from_ymd_opt(self.0.into(), self.1.into(), self.2.into()).unwrap();

        let tmp = date.checked_add_days(Days::new(days)).unwrap();

        Self::from_naive(tmp)
    }

    fn subtract_days(&self, days: u64) -> Self {
        let date = NaiveDate::from_ymd_opt(self.0.into(), self.1.into(), self.2.into()).unwrap();

        let tmp = date.checked_sub_days(Days::new(days)).unwrap();

        Self::from_naive(tmp)
    }
}

impl From<Weekday> for DayOfWeek {
    fn from(item: Weekday) -> Self {
        match item {
            Weekday::Mon => Self::Mon,
            Weekday::Tue => Self::Tue,
            Weekday::Wed => Self::Wed,
            Weekday::Thu => Self::Thu,
            Weekday::Fri => Self::Fri,
            Weekday::Sat => Self::Sat,
            Weekday::Sun => Self::Sun,
        }
    }
}
