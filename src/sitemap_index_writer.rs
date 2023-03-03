use std::io::Write;

use self::private::SealedTryIntoSitemap;

use super::{sitemap_xml_writer::SitemapXmlWriter, Sitemap};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid lastmod")]
    InvalidLastmod,
    #[error("invalid loc")]
    InvalidLoc,
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("max byte length is 50 MiB (52,428,800 bytes)")]
    MaxByteLength,
    #[error("max number of sitemaps is 50,000")]
    MaxNumberOfSitemaps,
}

impl From<crate::sitemap_xml_writer::Error> for Error {
    fn from(value: crate::sitemap_xml_writer::Error) -> Self {
        match value {
            super::sitemap_xml_writer::Error::Io(e) => Error::Io(e),
            super::sitemap_xml_writer::Error::MaxByteLength => Error::MaxByteLength,
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

/// A writer for sitemap index file.
///
/// # Examples
///
/// The following example is a sitemap index containing only one URL specified by `&str`.
///
/// ```rust
/// use sitemap_xml_writer::{SitemapIndexWriter, Sitemap};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
/// writer.write("http://www.example.com/sitemap1.xml.gz")?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
///         r#"</sitemap>"#,
///         r#"</sitemapindex>"#
///     )
/// );
/// #    Ok(())
/// # }
/// ```
///
/// The following example is a sitemap that uses all the optional tags.
///
/// ```rust
/// use sitemap_xml_writer::{SitemapIndexWriter, Sitemap};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
///         .lastmod("2004-10-01T18:23:17+00:00")?
/// )?;
/// writer.write(
///     Sitemap::loc("http://www.example.com/sitemap2.xml.gz")?
///         .lastmod("2005-01-01")?,
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
///         r#"<lastmod>2004-10-01T18:23:17+00:00</lastmod>"#,
///         r#"</sitemap>"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap2.xml.gz</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"</sitemap>"#,
///         r#"</sitemapindex>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// The following example using `time` crate types.
///
#[cfg_attr(feature = "time", doc = "```rust")]
#[cfg_attr(not(feature = "time"), doc = "```rust,ignore")]
/// use sitemap_xml_writer::{SitemapIndexWriter, Sitemap};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
///         // `::time::OffsetDateTime` support
///         .lastmod(::time::macros::datetime!(2004-10-01 18:23:17+00:00))?
/// )?;
/// writer.write(
///     Sitemap::loc("http://www.example.com/sitemap2.xml.gz")?
///         // `::time::Date` support
///         .lastmod(::time::macros::date!(2005-01-01))?,
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
///         r#"<lastmod>2004-10-01T18:23:17.000000000Z</lastmod>"#,
///         r#"</sitemap>"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap2.xml.gz</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"</sitemap>"#,
///         r#"</sitemapindex>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// The following example using `url` crate types.
///
#[cfg_attr(feature = "url", doc = "```rust")]
#[cfg_attr(not(feature = "url"), doc = "```rust,ignore")]
/// use sitemap_xml_writer::{SitemapIndexWriter, Sitemap};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     // <https://crates.io/crates/url> support
///     // If you want to ensure that the URL is Valid, use `::url::Url`.
///     // If you use &str, the URL is assumed to be valid and only the length
///     // check and XML entity escaping are performed.
///     Sitemap::loc(::url::Url::parse("http://www.example.com/sitemap1.xml.gz")?)?
///         .lastmod("2004-10-01T18:23:17+00:00")?
/// )?;
/// writer.write(
///     Sitemap::loc(::url::Url::parse("http://www.example.com/sitemap2.xml.gz")?)?
///         .lastmod("2005-01-01")?,
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
///         r#"<lastmod>2004-10-01T18:23:17+00:00</lastmod>"#,
///         r#"</sitemap>"#,
///         r#"<sitemap>"#,
///         r#"<loc>http://www.example.com/sitemap2.xml.gz</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"</sitemap>"#,
///         r#"</sitemapindex>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
pub struct SitemapIndexWriter<W: Write> {
    writer: SitemapXmlWriter<W>,
    number_of_sitemaps: usize,
}

impl<W: Write> SitemapIndexWriter<W> {
    const MAX_NUMBER_OF_SITEMAPS: usize = 50_000;

    /// Creates a new `SitemapIndexWriter<W>`. At the same time, write the XML declaration and an opening `<sitemapindex>` tag.
    pub fn start(inner: W) -> Result<Self> {
        Self::start_inner(inner, false)
    }

    /// Creates a new `SitemapIndexWriter<W>` with indentation enabled. At the same time, write the XML declaration and an opening `<sitemapindex>` tag.
    pub fn start_with_indent(inner: W) -> Result<Self> {
        Self::start_inner(inner, true)
    }

    /// Writes a `sitemap` element.
    pub fn write<'a, S>(&mut self, sitemap: S) -> Result<()>
    where
        S: SealedTryIntoSitemap<'a>,
    {
        if self.number_of_sitemaps + 1 > Self::MAX_NUMBER_OF_SITEMAPS {
            return Err(Error::MaxNumberOfSitemaps);
        }
        self.number_of_sitemaps += 1;

        let sitemap: Sitemap<'a> = sitemap.try_into_sitemap()?;
        self.writer.start_tag(b"sitemap")?;

        let content = sitemap.loc;
        self.writer.element(b"loc", content.as_ref())?;

        if let Some(content) = sitemap.lastmod {
            self.writer.element(b"lastmod", content.as_ref())?;
        }

        self.writer.end_tag(b"sitemap")?;
        Ok(())
    }

    /// Writes a closing `</sitemapindex>` tag.
    pub fn end(&mut self) -> Result<()> {
        self.writer.end_tag(b"sitemapindex")?;
        Ok(())
    }

    /// Unwraps this `SitemapIndexWrite<W>`, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    fn start_inner(inner: W, pretty: bool) -> Result<Self> {
        let mut s = Self {
            writer: SitemapXmlWriter::new(inner, pretty),
            number_of_sitemaps: 0_usize,
        };
        s.writer.declaration()?;
        s.writer.start_tag_with_default_ns(b"sitemapindex")?;
        Ok(s)
    }
}

mod private {
    use crate::Sitemap;

    use super::Error;

    pub trait SealedTryIntoSitemap<'a> {
        fn try_into_sitemap(self) -> Result<Sitemap<'a>, Error>;
    }

    impl<'a> SealedTryIntoSitemap<'a> for Sitemap<'a> {
        fn try_into_sitemap(self) -> Result<Sitemap<'a>, Error> {
            Ok(self)
        }
    }

    impl<'a> SealedTryIntoSitemap<'a> for &'a str {
        fn try_into_sitemap(self) -> Result<Sitemap<'a>, Error> {
            Sitemap::loc(self)
        }
    }
}
