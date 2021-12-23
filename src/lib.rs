//! A document in the ROFF format.
//!
//! [ROFF] is a family of Unix text-formatting languages, implemented
//! by the `nroff`, `troff`, and `groff` programs, among others. See
//! [groff(7)] for a description of the language. This structure is an
//! abstract representation of a document in ROFF format. It is meant
//! for writing code to generate ROFF documents, such as manual pages.
//!
//! # Example
//!
//! ```
//! # use roff::*;
//! let doc = RoffBuilder::default().text(vec![roman("hello, world")]).build().render();
//! assert!(doc.ends_with("hello, world\n"));
//! ```
//!
//! [ROFF]: https://en.wikipedia.org/wiki/Roff_(software)
//! [groff(7)]: https://manpages.debian.org/bullseye/groff/groff.7.en.html

#![deny(missing_docs)]

use std::io::Write;
use std::write;

/// A preamble added to the start of rendered output.
///
/// This defines a string variable that contains an apostrophe. For
/// historical reasons, there seems to be no other portable way to
/// represent apostrophes across various implementations of the ROFF
/// language. In implementations that produce output like PostScript
/// or PDF, an apostrophe gets typeset as a right single quote, which
/// looks different from an apostrophe. For terminal output ("ASCII"),
/// such as when using nroff, an apostrophe looks indistinguishable
/// from a right single quote. For manual pages, and similar content,
/// an apostrophe is more generally desired than the right single
/// quote, so we convert all apostrophe characters in input text into
/// a use of the string variable defined in the preamble.
///
/// The special handling of apostrophes is avoided in the
/// [`to_roff`](Roff::to_roff) method, but it's used in the
/// [`render`](Roff::render) and [`to_writer`](Roff::to_writer)
/// methods.
///
/// See: https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=507673#65
const APOSTROPHE_PREABMLE: &str = r#".ie \n(.g .ds Aq \(aq
.el .ds Aq '
"#;

// Use the apostrophe string variable.
const APOSTROPHE: &str = r"\*(Aq";

#[derive(Eq, PartialEq)]
enum Apostrophes {
    Handle,
    DontHandle,
}

/// A ROFF document, consisting of lines.
///
/// Lines are either control lines (requests that are built in, or
/// invocations of macros), or text lines.
///
/// # Example
///
/// ```
/// # use roff::*;
/// let doc = RoffBuilder::default()
///     .control("TH", ["FOO", "1"])
///     .control("SH", ["NAME"])
///     .text([roman("foo - do a foo thing")])
///     .build()
///     .render();
/// assert!(doc.ends_with(".TH FOO 1\n.SH NAME\nfoo \\- do a foo thing\n"));
/// ```
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Roff {
    lines: Vec<Line>,
}

impl Roff {
    /// Append a control line.
    ///
    /// The line consist of the name of a built-in command or macro,
    /// and some number of arguments. Arguments that contain spaces
    /// will be enclosed with double quotation marks.
    pub fn control<'a>(
        &mut self,
        name: impl Into<String>,
        args: impl IntoIterator<Item = &'a str>,
    ) {
        self.lines.push(Line::control(
            name.into(),
            args.into_iter().map(|s| s.to_string()).collect(),
        ));
    }

    /// Append a text line.
    ///
    /// The line will be rendered in a way that ensures it can't be
    /// interpreted as a control line. The caller does not need to
    /// ensure, for example, that the line doesn't start with a
    /// period ("`.`") or an apostrophe ("`'`").
    pub fn text(&mut self, inlines: impl Into<Vec<Inline>>) {
        self.lines.push(Line::text(inlines.into()));
    }

    /// Render as ROFF source text that can be fed to a ROFF implementation.
    pub fn render(&self) -> String {
        let mut buf = vec![];
        self.to_writer(&mut buf).unwrap(); // writing to a Vec always works
        std::str::from_utf8(&buf)
            .expect("output is utf8 if all input is utf8 and our API guarentees that")
            .to_owned()
    }

    /// Write to a writer.
    pub fn to_writer(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        w.write_all(APOSTROPHE_PREABMLE.as_bytes())?;
        for line in self.lines.iter() {
            line.render(w, Apostrophes::Handle)?;
        }
        Ok(())
    }

    /// Render without handling apostrophes specially.
    ///
    /// You probably want [`render`](Roff::render) or
    /// [`to_writer`](Roff::to_writer) instead of this method.
    ///
    /// Without special handling, apostrophes get typeset as right
    /// single quotes, including in words like "don't". In most
    /// situations, such as in manual pages, that's unwanted. The
    /// other methods handle apostrophes specially to prevent it, but
    /// for completeness, and for testing, this method is provided to
    /// avoid it.
    pub fn to_roff(&self) -> String {
        let mut buf = vec![];
        for line in self.lines.iter() {
            // Writing to a Vec always works, so we discard any error.
            line.render(&mut buf, Apostrophes::DontHandle).unwrap();
        }
        String::from_utf8_lossy(&buf).into_owned()
    }
}

/// Build a Roff.
///
/// This exists to make it more convenient to create a [`Roff`], by
/// chaining [`control`](Builder::control) and [`text`](Builder::text)
/// calls. With a `Roff`, the similar calls can't be chained. The
/// chaining approach is more convenient when you can build the whole
/// document at once.
#[derive(Default)]
pub struct RoffBuilder {
    roff: Roff,
}

impl RoffBuilder {
    /// Append a control line.
    ///
    /// The line consist of the name of a built-in command or macro,
    /// and some number of arguments. Arguments that contain spaces
    /// will be enclosed on double quotation marks.
    pub fn control<'a>(mut self, name: &'a str, args: impl IntoIterator<Item = &'a str>) -> Self {
        self.roff.control(name, args);
        self
    }

    /// Append a text line.
    ///
    /// The line will be rendered in a way that ensures it can't be
    /// interpreted as a control line. The caller does not need to
    /// ensure, for example, that the line doesn't start with a
    /// period ("`.`") or an apostrophe ("`'`").
    pub fn text(mut self, inlines: impl Into<Vec<Inline>>) -> Self {
        self.roff.text(inlines);
        self
    }

    /// Return a built Roff.
    pub fn build(self) -> Roff {
        self.roff
    }
}

/// A line in a ROFF document.
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Line {
    /// A control line.
    Control {
        /// Name of control request or macro being invoked.
        name: String,

        /// Arguments on control line.
        args: Vec<String>,
    },

    /// A text line.
    Text(Vec<Inline>),
}

impl Line {
    /// Append a control line.
    pub(crate) fn control(name: String, args: Vec<String>) -> Self {
        Self::Control { name, args }
    }

    /// Append a text line, consisting of inline elements.
    pub(crate) fn text(parts: Vec<Inline>) -> Self {
        Self::Text(parts)
    }

    // Generate a ROFF line.
    //
    // All the ROFF code generation and special handling happens here.
    fn render(
        &self,
        out: &mut dyn Write,
        handle_apostrophes: Apostrophes,
    ) -> Result<(), std::io::Error> {
        match self {
            Self::Control { name, args } => {
                write!(out, ".{}", name)?;
                for arg in args {
                    write!(out, " {}", &escape_spaces(arg))?;
                }
            }
            Self::Text(inlines) => {
                let mut at_line_start = true;
                for inline in inlines.iter() {
                    // We need to handle line breaking specially: it
                    // introduces a control line to the ROFF, and the
                    // leading period of that mustn't be escaped.
                    match inline {
                        Inline::LineBreak => {
                            if at_line_start {
                                writeln!(out, ".br")?;
                            } else {
                                writeln!(out, "\n.br")?;
                            }
                        }
                        Inline::Roman(text) | Inline::Italic(text) | Inline::Bold(text) => {
                            let mut text = escape_inline(text);
                            if handle_apostrophes == Apostrophes::Handle {
                                text = escape_apostrophes(&text)
                            };
                            let text = escape_leading_cc(&text);
                            if let Inline::Bold(_) = inline {
                                write!(out, r"\fB{}\fR", text)?;
                            } else if let Inline::Italic(_) = inline {
                                write!(out, r"\fI{}\fR", text)?;
                            } else {
                                if at_line_start && starts_with_cc(&text) {
                                    // Line would start with a period, so we
                                    // insert a non-printable, zero-width glyph to
                                    // prevent it from being interpreted as such.
                                    // We only do that when it's needed, though,
                                    // to avoid making the output ugly.
                                    //
                                    // Note that this isn't handled by
                                    // escape_leading_cc, as it
                                    // doesn't know when an inline
                                    // element is at the start of a
                                    // line.
                                    write!(out, r"\&").unwrap();
                                }
                                write!(out, "{}", text)?;
                            }
                        }
                    }
                    at_line_start = false;
                }
            }
        };
        writeln!(out)?;
        Ok(())
    }
}

// Does line start with a control character?
fn starts_with_cc(line: &str) -> bool {
    line.starts_with('.') || line.starts_with('\'')
}

// This quotes strings with spaces. This doesn't handle strings with
// quotes in any way: there doesn't seem to a way to escape them.
fn escape_spaces(w: &str) -> String {
    if w.contains(' ') {
        format!("\"{}\"", w)
    } else {
        w.to_string()
    }
}

// Prevent leading periods or apostrophes on lines to be interpreted
// as control lines. Note that this needs to be done for apostrophes
// whether they need special handling for typesetting or not: a
// leading apostrophe on a line indicates a control line.
fn escape_leading_cc(s: &str) -> String {
    s.replace("\n.", "\n\\&.").replace("\n'", "\n\\&'")
}

// Escape anything that may be interpreted by the roff processor in a
// text line: dashes and backslashes are escaped with a backslash.
// Apostrophes are not handled.
fn escape_inline(text: &str) -> String {
    text.replace(r"\", r"\\").replace('-', r"\-")
}

// Handle apostrophes.
fn escape_apostrophes(text: &str) -> String {
    text.replace('\'', APOSTROPHE)
}

/// A part of a text line.
///
/// Text will be escaped for ROFF. No inline escape sequences will be
/// passed to ROFF. The text may contain newlines, but leading periods
/// will be escaped so that they won't be interpreted by ROFF as
/// control lines.
// Note that the strings stored in the variants are stored as they're
// received from the API user. The Line::render function handles
// escaping etc.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Inline {
    /// Text in the "roman" font, which is the normal font if nothing
    /// else is specified.
    Roman(String),

    /// Text in the italic (slanted) font.
    Italic(String),

    /// Text in a bold face font.
    Bold(String),

    /// A hard line break. This is an inline element so it's easy to
    /// insert a line break in a paragraph.
    LineBreak,
}

/// Turn a string slice into inline text in the roman font.
///
/// This is equivalent to the [roman] function, but may be more
/// convenient to use.
impl From<&str> for Inline {
    fn from(s: &str) -> Self {
        roman(s)
    }
}

/// Return some inline text in the "roman" font.
///
/// The roman font is the normal font, if no other font is chosen.
pub fn roman(input: &str) -> Inline {
    Inline::Roman(input.to_string())
}

/// Return some inline text in the bold font.
pub fn bold(input: &str) -> Inline {
    Inline::Bold(input.to_string())
}

/// Return some inline text in the italic font.
pub fn italic(input: &str) -> Inline {
    Inline::Italic(input.to_string())
}

/// Return an inline element for a hard line break.
pub fn line_break() -> Inline {
    Inline::LineBreak
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn escape_dash() {
        assert_eq!(r"\-", escape_inline("-"));
    }

    #[test]
    fn escape_backslash() {
        assert_eq!(r"\\x", escape_inline(r"\x"));
    }

    #[test]
    fn escape_backslash_and_dash() {
        assert_eq!(r"\\\-", escape_inline(r"\-"));
    }

    #[test]
    fn escapes_leading_control_chars() {
        assert_eq!("foo\n\\&.bar\n\\&'yo", escape_leading_cc("foo\n.bar\n'yo"));
    }

    #[test]
    fn escape_plain() {
        assert_eq!("abc", escape_inline("abc"));
    }

    #[test]
    fn render_roman() {
        let text = RoffBuilder::default().text([roman("foo")]).build();
        assert_eq!(text.to_roff(), "foo\n");
    }

    #[test]
    fn render_dash() {
        let text = RoffBuilder::default().text([roman("foo-bar")]).build();
        assert_eq!(text.to_roff(), "foo\\-bar\n");
    }

    #[test]
    fn render_italic() {
        let text = RoffBuilder::default().text([italic("foo")]).build();
        assert_eq!(text.to_roff(), "\\fIfoo\\fR\n");
    }

    #[test]
    fn render_bold() {
        let text = RoffBuilder::default().text([bold("foo")]).build();
        assert_eq!(text.to_roff(), "\\fBfoo\\fR\n");
    }

    #[test]
    fn render_text() {
        let text = RoffBuilder::default().text([roman("roman")]).build();
        assert_eq!(text.to_roff(), "roman\n");
    }

    #[test]
    fn render_text_with_leading_period() {
        let text = RoffBuilder::default().text([roman(".roman")]).build();
        assert_eq!(text.to_roff(), "\\&.roman\n");
    }

    #[test]
    fn render_text_with_newline_period() {
        let text = RoffBuilder::default().text([roman("foo\n.roman")]).build();
        assert_eq!(text.to_roff(), "foo\n\\&.roman\n");
    }
    #[test]
    fn render_line_break() {
        let text = RoffBuilder::default()
            .text([roman("roman"), Inline::LineBreak, roman("more")])
            .build();
        assert_eq!(text.to_roff(), "roman\n.br\nmore\n");
    }

    #[test]
    fn render_control() {
        let text = RoffBuilder::default()
            .control("foo", ["bar", "foo and bar"])
            .build();
        assert_eq!(text.to_roff(), ".foo bar \"foo and bar\"\n");
    }
}
