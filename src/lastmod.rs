use once_cell::sync::Lazy;
use regex::Regex;
use std::{borrow::Cow, fmt::Debug};

#[cfg(feature = "time")]
use time::format_description::well_known::Iso8601;

static DATE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\A-?([1-9][0-9]{3,}|0[0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?\z"#
            ).unwrap()
});

static DATE_TIME_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
                r#"\A-?([1-9][0-9]{3,}|0[0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T(([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\.[0-9]+)?|(24:00:00(\.0+)?))(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?\z"#
            ).unwrap()
});

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

/// A `lastmod` child entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lastmod<'a>(Cow<'a, str>);

impl<'a> Lastmod<'a> {
    pub(crate) fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> TryFrom<&'a str> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // <https://www.w3.org/TR/xmlschema11-2/#date>
        // <https://www.w3.org/TR/xmlschema11-2/#dateTime>
        if !DATE_RE.is_match(value) && !DATE_TIME_RE.is_match(value) {
            return Err(Error);
        }
        Ok(Self(Cow::Borrowed(value)))
    }
}

#[cfg(feature = "chrono")]
impl<'a, T> TryFrom<::chrono::DateTime<T>> for Lastmod<'a>
where
    T: ::chrono::TimeZone,
    <T as ::chrono::TimeZone>::Offset: ::std::fmt::Display,
{
    type Error = Error;

    fn try_from(value: ::chrono::DateTime<T>) -> Result<Self, Self::Error> {
        let s = value.to_rfc3339();
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(feature = "chrono")]
impl<'a> TryFrom<::chrono::NaiveDate> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: ::chrono::NaiveDate) -> Result<Self, Self::Error> {
        // `chrono::NaiveDate` debug output format is "%Y-%m-%d"
        let s = format!("{:?}", value);
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(feature = "time")]
impl<'a> TryFrom<time::Date> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: time::Date) -> Result<Self, Self::Error> {
        let format = time::macros::format_description!("[year]-[month]-[day]");
        let s = value.format(&format).map_err(|_| Error)?;
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(feature = "time")]
impl<'a> TryFrom<time::OffsetDateTime> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: time::OffsetDateTime) -> Result<Self, Self::Error> {
        let s = value.format(&Iso8601::DEFAULT).map_err(|_| Error)?;
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let lastmod = Lastmod::try_from("2005-01-01")?;
        assert_eq!(lastmod.into_inner(), "2005-01-01");

        let lastmod = Lastmod::try_from("2004-12-23T18:00:15+00:00")?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15+00:00");
        Ok(())
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_chrono_date_time() -> anyhow::Result<()> {
        let lastmod = Lastmod::try_from(::chrono::DateTime::parse_from_rfc3339(
            "2004-12-23T18:00:15+00:00",
        )?)?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15+00:00");

        let lastmod = Lastmod::try_from(::chrono::DateTime::parse_from_rfc3339(
            "2004-12-23T18:00:15.123456789+09:00",
        )?)?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15.123456789+09:00");
        Ok(())
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_chrono_naive_date() -> anyhow::Result<()> {
        let lastmod = Lastmod::try_from(::chrono::NaiveDate::parse_from_str(
            "2005-01-01",
            "%Y-%m-%d",
        )?)?;
        assert_eq!(lastmod.into_inner(), "2005-01-01");

        let lastmod = Lastmod::try_from(::chrono::NaiveDate::parse_from_str(
            "2023-01-02",
            "%Y-%m-%d",
        )?)?;
        assert_eq!(lastmod.into_inner(), "2023-01-02");
        Ok(())
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_time_date() -> anyhow::Result<()> {
        #[rustfmt::skip]
        let lastmod = Lastmod::try_from(time::macros::date!(2005-01-01))?;
        assert_eq!(lastmod.into_inner(), "2005-01-01");
        Ok(())
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_time_offset_date_time() -> anyhow::Result<()> {
        #[rustfmt::skip]
        let lastmod = Lastmod::try_from(time::macros::datetime!(2004-12-23 18:00:15 +00:00))?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15.000000000Z");
        Ok(())
    }
}
