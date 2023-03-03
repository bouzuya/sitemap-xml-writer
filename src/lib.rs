//! This crate provides a library for writing [`sitemap.xml`](https://www.sitemaps.org/).
//!
//! # Usage
//!
//! This crate is on crates.io and can be used by adding `sitemap-xml-writer` to your dependencies in your project’s Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! sitemap-xml-writer = "0.1.0"
//! ```
//!
//! # Writers
//!
//! - [`SitemapWriter`]: A writer for sitemap file.
//! - [`SitemapIndexWriter`]: A writer for sitemap index file.
//!
//! # Example: Write sitemap file
//!
//! ```rust
//! use sitemap_xml_writer::{Changefreq, SitemapWriter, Url};
//! use std::io::Cursor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
//! writer.write(
//!     Url::loc("http://www.example.com/")?
//!         .lastmod("2005-01-01")?
//!         .changefreq("monthly")?
//!         .priority("0.8")?,
//! )?;
//! writer.end()?;
//!
//! assert_eq!(
//!     String::from_utf8(writer.into_inner().into_inner())?,
//!     concat!(
//!         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
//!         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
//!         r#"<url>"#,
//!         r#"<loc>http://www.example.com/</loc>"#,
//!         r#"<lastmod>2005-01-01</lastmod>"#,
//!         r#"<changefreq>monthly</changefreq>"#,
//!         r#"<priority>0.8</priority>"#,
//!         r#"</url>"#,
//!         r#"</urlset>"#
//!     )
//! );
//! #     Ok(())
//! # }
//! ```
//!
//! # Example: Write sitemap index file
//!
//! ```rust
//! use sitemap_xml_writer::{SitemapIndexWriter, Sitemap};
//! use std::io::Cursor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
//! writer.write(
//!     Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
//!         .lastmod("2004-10-01T18:23:17+00:00")?
//! )?;
//! writer.end()?;
//!
//! assert_eq!(
//!     String::from_utf8(writer.into_inner().into_inner())?,
//!     concat!(
//!         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
//!         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
//!         r#"<sitemap>"#,
//!         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
//!         r#"<lastmod>2004-10-01T18:23:17+00:00</lastmod>"#,
//!         r#"</sitemap>"#,
//!         r#"</sitemapindex>"#
//!     )
//! );
//! #     Ok(())
//! # }
//!
mod changefreq;
mod lastmod;
mod loc;
mod priority;
mod sitemap;
mod sitemap_index_writer;
mod sitemap_writer;
mod sitemap_xml_writer;
mod url;

pub use self::changefreq::Changefreq;
pub use self::lastmod::Lastmod;
pub use self::loc::Loc;
pub use self::priority::Priority;
pub use self::sitemap::Sitemap;
pub use self::sitemap_index_writer::SitemapIndexWriter;
pub use self::sitemap_writer::SitemapWriter;
pub use self::url::Url;
