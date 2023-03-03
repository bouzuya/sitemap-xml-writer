use std::io::Write;

use crate::url::Url;

use self::private::SealedTryIntoUrl;

use super::sitemap_xml_writer::SitemapXmlWriter;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid changefreq")]
    InvalidChangefreq,
    #[error("invalid lastmod")]
    InvalidLastmod,
    #[error("invalid loc")]
    InvalidLoc,
    #[error("invalid priority")]
    InvalidPriority,
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("max byte length is 50 MiB (52,428,800 bytes)")]
    MaxByteLength,
    #[error("max number of urls is 50,000")]
    MaxNumberOfUrls,
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

/// A writer for sitemap file.
///
/// # Examples
///
/// The following example is a sitemap containing only one URL specified by `&str`.
///
/// ```rust
/// use sitemap_xml_writer::{Changefreq, Lastmod, Loc, Priority, SitemapWriter, Url};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
/// writer.write("http://www.example.com/")?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<url>"#,
///         r#"<loc>http://www.example.com/</loc>"#,
///         r#"</url>"#,
///         r#"</urlset>"#
///     )
/// );
/// #    Ok(())
/// # }
/// ```
///
/// The following example is a sitemap that uses all the optional tags. It also includes an example using non-string types.
///
/// ```rust
/// use sitemap_xml_writer::{Changefreq, SitemapWriter, Url};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     Url::loc("http://www.example.com/")?
///         .lastmod("2005-01-01")?
///         .changefreq("monthly")?
///         .priority("0.8")?,
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<url>"#,
///         r#"<loc>http://www.example.com/</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"<changefreq>monthly</changefreq>"#,
///         r#"<priority>0.8</priority>"#,
///         r#"</url>"#,
///         r#"</urlset>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// The following an example using `chrono` crate types.
///
#[cfg_attr(feature = "chrono", doc = "```rust")]
#[cfg_attr(not(feature = "chrono"), doc = "```rust,ignore")]
/// use sitemap_xml_writer::{Changefreq, SitemapWriter, Url};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     Url::loc("http://www.example.com/")?
///         // `::chrono::NaiveDate` and `::chrono::DateTime` are supported.
///         .lastmod(::chrono::NaiveDate::parse_from_str("2005-01-01", "%Y-%m-%d")?)?
///         .changefreq(Changefreq::Monthly)?
///         .priority(0.8)?
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<url>"#,
///         r#"<loc>http://www.example.com/</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"<changefreq>monthly</changefreq>"#,
///         r#"<priority>0.8</priority>"#,
///         r#"</url>"#,
///         r#"</urlset>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// The following an example using `time` crate types.
///
#[cfg_attr(feature = "time", doc = "```rust")]
#[cfg_attr(not(feature = "time"), doc = "```rust,ignore")]
/// use sitemap_xml_writer::{Changefreq, SitemapWriter, Url};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     Url::loc("http://www.example.com/")?
///         // `::time::Date` and `::time::OffsetDateTime` are supported.
///         .lastmod(::time::macros::date!(2005-01-01))?
///         .changefreq(Changefreq::Monthly)?
///         .priority(0.8)?
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<url>"#,
///         r#"<loc>http://www.example.com/</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"<changefreq>monthly</changefreq>"#,
///         r#"<priority>0.8</priority>"#,
///         r#"</url>"#,
///         r#"</urlset>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// The following an example using `url` crate types.
///
#[cfg_attr(feature = "url", doc = "```rust")]
#[cfg_attr(not(feature = "url"), doc = "```rust,ignore")]
/// use sitemap_xml_writer::{Changefreq, SitemapWriter, Url};
/// use std::io::Cursor;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
/// writer.write(
///     // <https://crates.io/crates/url> support
///     // You can specify `::url::Url`.
///     // If you want to ensure that the URL is valid, use `::url::Url`.
///     // If you use &str, the URL is assumed to be valid and only the length
///     // check and XML entity escaping are performed.
///     Url::loc(::url::Url::parse("http://www.example.com/")?)?
///         .lastmod("2005-01-01")?
///         .changefreq(Changefreq::Monthly)?
///         .priority(0.8)?
/// )?;
/// writer.end()?;
///
/// assert_eq!(
///     String::from_utf8(writer.into_inner().into_inner())?,
///     concat!(
///         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
///         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
///         r#"<url>"#,
///         r#"<loc>http://www.example.com/</loc>"#,
///         r#"<lastmod>2005-01-01</lastmod>"#,
///         r#"<changefreq>monthly</changefreq>"#,
///         r#"<priority>0.8</priority>"#,
///         r#"</url>"#,
///         r#"</urlset>"#
///     )
/// );
/// #     Ok(())
/// # }
/// ```
pub struct SitemapWriter<W: Write> {
    writer: SitemapXmlWriter<W>,
    number_of_urls: usize,
}

impl<W: Write> SitemapWriter<W> {
    const MAX_NUMBER_OF_URLS: usize = 50_000;

    /// Creates a new `SitemapWriter<W>`. At the same time, write the XML declaration and an opening `<urlset>` tag.
    pub fn start(inner: W) -> Result<Self> {
        Self::start_inner(inner, false)
    }

    /// Creates a new `SitemapWriter<W>` with indentation enabled. At the same time, write the XML declaration and an opening `<urlset>` tag.
    pub fn start_with_indent(inner: W) -> Result<Self> {
        Self::start_inner(inner, true)
    }

    /// Writes a `url` element.
    pub fn write<'a, U>(&mut self, url: U) -> Result<()>
    where
        U: SealedTryIntoUrl<'a>,
    {
        if self.number_of_urls + 1 > Self::MAX_NUMBER_OF_URLS {
            return Err(Error::MaxNumberOfUrls);
        }
        self.number_of_urls += 1;

        let url: Url<'a> = url.try_into_url()?;
        self.writer.start_tag(b"url")?;

        let content = url.loc;
        self.writer.element(b"loc", content.as_ref())?;

        if let Some(content) = url.lastmod {
            self.writer.element(b"lastmod", content.as_ref())?;
        }

        if let Some(content) = url.changefreq {
            self.writer.element(b"changefreq", content.as_ref())?;
        }

        if let Some(content) = url.priority {
            self.writer.element(b"priority", content.as_ref())?;
        }

        self.writer.end_tag(b"url")?;
        Ok(())
    }

    /// Writes a closing `</urlset>` tag.
    pub fn end(&mut self) -> Result<()> {
        self.writer.end_tag(b"urlset")?;
        Ok(())
    }

    /// Unwraps this `SitemapWrite<W>`, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    fn start_inner(inner: W, pretty: bool) -> Result<Self> {
        let mut s = Self {
            writer: SitemapXmlWriter::new(inner, pretty),
            number_of_urls: 0_usize,
        };
        s.writer.declaration()?;
        s.writer.start_tag_with_default_ns(b"urlset")?;
        Ok(s)
    }
}

mod private {
    use crate::Url;

    use super::Error;

    pub trait SealedTryIntoUrl<'a> {
        fn try_into_url(self) -> Result<Url<'a>, Error>;
    }

    impl<'a> SealedTryIntoUrl<'a> for Url<'a> {
        fn try_into_url(self) -> Result<Url<'a>, Error> {
            Ok(self)
        }
    }

    impl<'a> SealedTryIntoUrl<'a> for &'a str {
        fn try_into_url(self) -> Result<Url<'a>, Error> {
            Url::loc(self)
        }
    }
}
