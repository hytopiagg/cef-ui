use cef_ui_sys::{cef_basetime_t, cef_time_from_basetime, cef_time_t};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::mem::zeroed;

/// A wrapper for DateTime<Utc> for interacting with CEF.
pub struct CefTime(DateTime<Utc>);

impl From<CefTime> for DateTime<Utc> {
    fn from(value: CefTime) -> Self {
        value.0
    }
}

impl From<&CefTime> for DateTime<Utc> {
    fn from(value: &CefTime) -> Self {
        value.0
    }
}

impl TryFrom<cef_basetime_t> for CefTime {
    type Error = &'static str;

    fn try_from(value: cef_basetime_t) -> Result<Self, Self::Error> {
        let mut time: cef_time_t = unsafe { zeroed() };

        unsafe {
            cef_time_from_basetime(value, &mut time);
        }

        CefTime::try_from(time)
    }
}

impl TryFrom<cef_time_t> for CefTime {
    type Error = &'static str;

    fn try_from(value: cef_time_t) -> Result<Self, Self::Error> {
        let naive_datetime = NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(
                value.year,
                value.month as u32,
                value.day_of_month as u32
            )
            .ok_or("Invalid date!")?,
            chrono::NaiveTime::from_hms_milli_opt(
                value.hour as u32,
                value.minute as u32,
                value.second as u32,
                value.millisecond as u32
            )
            .ok_or("Invalid time!")?
        );

        Ok(Self(DateTime::<Utc>::from_naive_utc_and_offset(
            naive_datetime,
            Utc
        )))
    }
}
