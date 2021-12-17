use std::fmt::Write;

/// Title line for a manpage.
pub struct Title {
    title: String,
    section: ManSection,
    date: Option<String>,
    source: Option<String>,
    manual: Option<String>,
}

impl Title {
    pub fn new(title: &str, section: ManSection) -> Self {
        Title {
            title: title.into(),
            section,
            date: None,
            source: None,
            manual: None,
        }
    }
}

impl Troffable for Title {
    fn render(&self) -> String {
        let manual = self.manual.as_deref().unwrap_or_default();
        let date = self.date.as_deref().unwrap_or_default();
        let source = self.source.as_deref().unwrap_or_default();

        format!(
            r#".TH "{}" "{}" "{}" "{}" "{}""#,
            self.title.to_uppercase(),
            self.section.value(),
            date,
            source,
            manual
        )
    }
}

/// Manpage sections.
///
/// The most common is [`ManSection::Executable`], and is the recommended default.
#[derive(Clone, Copy)]
pub enum ManSection {
    /// Executable programs or shell commands
    Executable,
    /// System calls (functions provided by the kernel)
    SystemCalls,
    /// Library calls (functions within program libraries)
    LibraryCalls,
    /// Special files (usually found in /dev)
    SpecialFiles,
    /// File formats and conventions, e.g. /etc/passwd
    FileFormats,
    /// Games
    Games,
    /// Miscellaneous (including macro packages and conventions), e.g. man(7), groff(7)
    Miscellaneous,
    /// System administration commands (usually only for root)
    SystemAdministrationCommands,
    /// Kernel routines [Non standard]
    KernelRoutines,
}

impl ManSection {
    pub fn value(&self) -> i8 {
        match self {
            ManSection::Executable => 1,
            ManSection::SystemCalls => 2,
            ManSection::LibraryCalls => 3,
            ManSection::SpecialFiles => 4,
            ManSection::FileFormats => 5,
            ManSection::Games => 6,
            ManSection::Miscellaneous => 7,
            ManSection::SystemAdministrationCommands => 8,
            ManSection::KernelRoutines => 9,
        }
    }
}

pub struct Roff {
    title: Title,
    content: Vec<Section>,
}

impl Roff {
    pub fn new(title: &str, section: ManSection) -> Self {
        Roff {
            title: Title::new(title, section),
            content: Vec::new(),
        }
    }

    /// Date of the last nontrivial change to the manpage. Should be formatted
    /// in `YYYY-MM-DD`.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.title.date = Some(date.into());
        self
    }

    /// The source of the command, function or system call.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.title.source = Some(source.into());
        self
    }

    /// The title of the manual.
    pub fn manual(mut self, manual: impl Into<String>) -> Self {
        self.title.manual = Some(manual.into());
        self
    }

    pub fn section<'a, C, I>(mut self, title: &str, content: I) -> Self
    where
        I: IntoIterator<Item = &'a C>,
        C: Troffable + 'a,
    {
        let title = title.into();
        let content = content.into_iter().map(Troffable::render).collect();

        self.content.push(Section { title, content });
        self
    }
}

impl Troffable for Roff {
    fn render(&self) -> String {
        let mut res = String::new();

        writeln!(&mut res, "{}", self.title.render()).unwrap();

        // Compatibility settings:
        //
        // Set sentence_space_size to 0 to prevent extra space between sentences separated
        // by a newline the alternative is to add \& at the end of the line
        writeln!(&mut res, ".ss \\n[.ss] 0").unwrap();
        // Disable hyphenation
        writeln!(&mut res, ".nh").unwrap();
        // Disable justification (adjust text to the left margin only)
        writeln!(&mut res, ".ad l").unwrap();

        for section in &self.content {
            writeln!(&mut res, "{}", escape(&section.render())).unwrap();
        }

        res
    }
}

struct Section {
    title: String,
    content: String,
}

impl Troffable for Section {
    fn render(&self) -> String {
        let mut res = String::new();

        writeln!(&mut res, ".SH \"{}\"", self.title.to_uppercase()).unwrap();
        res.push_str(&self.content);

        res
    }
}

pub trait Troffable {
    fn render(&self) -> String;
}

impl Troffable for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl<'a> Troffable for &'a str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl<'a, C: Troffable> Troffable for &'a [C] {
    fn render(&self) -> String {
        self.iter().map(Troffable::render).collect()
    }
}

impl<C: Troffable> Troffable for Vec<C> {
    fn render(&self) -> String {
        self.iter().map(Troffable::render).collect()
    }
}

pub fn bold(input: &str) -> String {
    format!(r"\fB{}\fP", input)
}

pub fn italic(input: &str) -> String {
    format!(r"\fI{}\fP", input)
}

pub fn list<C1: Troffable, C2: Troffable>(header: &[C1], content: &[C2]) -> String {
    format!(".TP\n{}\n{}", header.render(), content.render())
}

pub fn escape(input: &str) -> String {
    input.replace("-", r"\-")
}
