use std::borrow::Cow;

use crate::{sitemap_index_writer::Error, Lastmod, Loc};

type Result<T, E = Error> = std::result::Result<T, E>;

/// A builder for `sitemap` entry.
///
/// # Examples
///
/// ```rust
/// # use sitemap_xml_writer::Sitemap;
/// # fn main() -> anyhow::Result<()> {
/// Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
///     .lastmod("2004-10-01T18:23:17+00:00")?;
/// #     Ok(())
/// # }
/// ```
///
#[cfg_attr(all(feature = "chrono", feature = "url"), doc = "```rust")]
#[cfg_attr(not(all(feature = "chrono", feature = "url")), doc = "```rust,ignore")]
/// # use sitemap_xml_writer::Sitemap;
/// # fn main() -> anyhow::Result<()> {
/// Sitemap::loc(::url::Url::parse("http://www.example.com/sitemap1.xml.gz")?)?
///     .lastmod(::chrono::DateTime::parse_from_rfc3339("2004-10-01T18:23:17+00:00")?)?;
/// #     Ok(())
/// # }
/// ```
///
#[cfg_attr(all(feature = "time", feature = "url"), doc = "```rust")]
#[cfg_attr(not(all(feature = "time", feature = "url")), doc = "```rust,ignore")]
/// # use sitemap_xml_writer::Sitemap;
/// # fn main() -> anyhow::Result<()> {
/// Sitemap::loc(::url::Url::parse("http://www.example.com/sitemap1.xml.gz")?)?
///     .lastmod(::time::macros::datetime!(2004-10-01 18:23:17+00:00))?;
/// #     Ok(())
/// # }
/// ```
pub struct Sitemap<'a> {
    pub(crate) loc: Cow<'a, str>,
    pub(crate) lastmod: Option<Cow<'a, str>>,
}

impl<'a> TryFrom<&'a str> for Sitemap<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::loc(value)
    }
}

impl<'a> Sitemap<'a> {
    /// Builds a `sitemap` entry with the specified URL as the content of the
    /// `loc` child entry.
    ///
    /// If you want to ensure that the URL is Valid, use `::url::Url`. If you
    /// use &str, the URL is assumed to be valid and only the length check and
    /// XML entity escaping are performed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sitemap_xml_writer::Sitemap;
    /// # fn main() -> anyhow::Result<()> {
    /// Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?;
    /// #     Ok(())
    /// # }
    /// ```
    ///
    #[cfg_attr(feature = "url", doc = "```rust")]
    #[cfg_attr(not(feature = "url"), doc = "```rust,ignore")]
    /// # use sitemap_xml_writer::Sitemap;
    /// # fn main() -> anyhow::Result<()> {
    /// let url = ::url::Url::parse("http://www.example.com/sitemap1.xml.gz")?;
    /// Sitemap::loc(url)?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn loc<S>(loc: S) -> Result<Self>
    where
        S: TryInto<Loc<'a>>,
    {
        let loc = loc.try_into().map_err(|_| Error::InvalidLoc)?.into_inner();
        Ok(Self { loc, lastmod: None })
    }

    /// Changes the `lastmod` child entry to the specified date or datetime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sitemap_xml_writer::Sitemap;
    /// # fn main() -> anyhow::Result<()> {
    /// Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
    ///     .lastmod("2004-10-01T18:23:17+00:00")?;
    /// #     Ok(())
    /// # }
    /// ```
    ///
    #[cfg_attr(feature = "chrono", doc = "```rust")]
    #[cfg_attr(not(feature = "chrono"), doc = "```rust,ignore")]
    /// # use sitemap_xml_writer::Sitemap;
    /// # fn main() -> anyhow::Result<()> {
    /// Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
    ///     .lastmod(::chrono::DateTime::parse_from_rfc3339("2004-10-01T18:23:17+00:00")?)?;
    /// #     Ok(())
    /// # }
    /// ```
    ///
    #[cfg_attr(feature = "time", doc = "```rust")]
    #[cfg_attr(not(feature = "time"), doc = "```rust,ignore")]
    /// # use sitemap_xml_writer::Sitemap;
    /// # fn main() -> anyhow::Result<()> {
    /// Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
    ///     .lastmod(::time::macros::datetime!(2004-10-01 18:23:17+00:00))?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn lastmod<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Lastmod<'a>>,
    {
        let lastmod = s
            .try_into()
            .map_err(|_| Error::InvalidLastmod)?
            .into_inner();
        self.lastmod = Some(lastmod);
        Ok(self)
    }
}
