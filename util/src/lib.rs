use anyhow::{Context, Result};
use ariadne::{Color, Label, Report, ReportKind, sources};
use chumsky::prelude::*;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub mod collections;

pub struct InputFile {
    pub path: OsString,
    pub contents: String,
}

impl InputFile {
    pub fn read(relative_path: &str) -> Result<Self> {
        let path = Path::new(relative_path).to_path_buf();
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read input file '{}'", path.display()))?;

        Ok(Self {
            path: path.into_os_string(),
            contents,
        })
    }

    pub fn print_diagnostics<'src>(&self, errs: Vec<Rich<'src, char>>) {
        let escape = |s: String| {
            s.replace("\r", "\\r")
                .replace("\n", "\\n")
                .replace("\t", "\\t")
        };

        for err in errs {
            Report::build(
                ReportKind::Error,
                (self.path.display().to_string(), err.span().into_range()),
            )
            .with_message(escape(err.to_string()))
            .with_label(
                Label::new((self.path.display().to_string(), err.span().into_range()))
                    .with_message(escape(err.reason().to_string()))
                    .with_color(Color::Red),
            )
            .finish()
            .eprint(sources([(
                self.path.display().to_string(),
                self.contents.clone(),
            )]))
            .unwrap();
        }
    }
}

impl From<&str> for InputFile {
    fn from(value: &str) -> Self {
        Self {
            path: OsString::from_str("<MEMORY>").unwrap(),
            contents: value.to_string(),
        }
    }
}

pub fn unsigned<'a, T: FromStr>(
    radix: u32,
) -> impl Parser<'a, &'a str, T, extra::Err<Rich<'a, char>>>
where
    <T as FromStr>::Err: ToString,
{
    text::int(radix).try_map(|s: &str, span| s.parse::<T>().map_err(|e| Rich::custom(span, e)))
}
