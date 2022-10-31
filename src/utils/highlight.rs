use error_stack::IntoReportCompat;
use std::fmt;
use std::fmt::Write as _;
use std::io::Stdout;
use std::io::Write as _;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::{Highlight, Result};

struct Line(Option<usize>);

impl Highlight for Stdout {
    fn highlight(
        &mut self,
        text: &str,
        extension: &str,
        theme: Option<&str>,
    ) -> Result<(), crate::Error> {
        if atty::is(atty::Stream::Stdout) {
            writeln!(self, "{}", highlight_text(text, extension, theme)?)
                .map_err(Into::into)
                .into_report()?;
        } else {
            writeln!(self, "{}", text)
                .map_err(Into::into)
                .into_report()?;
        }

        Ok(())
    }
}

pub fn highlight_text(
    text: &str,
    extension: &str,
    theme: Option<&str>,
) -> Result<String, crate::Error> {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = if let Some(s) = ps.find_syntax_by_extension(extension) {
        s
    } else {
        ps.find_syntax_plain_text()
    };
    let mut h = HighlightLines::new(syntax, &ts.themes[theme.unwrap_or("base16-ocean.dark")]);

    let mut output = String::new();

    for line in LinesWithEndings::from(text) {
        let ranges = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        write!(&mut output, "{}", escaped)
            .map_err(Into::into)
            .into_report()?;
    }

    Ok(output)
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn highlight_text_should_work() {
        let v = json!({
            "foo": "bar",
            "baz": "qux"
        });
        let text = serde_json::to_string_pretty(&v).unwrap();

        insta::assert_snapshot!(highlight_text(&text, "json", None).unwrap());
    }

    #[test]
    fn stdout_highlight_should_work() {
        let v = json!({
            "foo": "bar",
            "baz": "qux"
        });
        let text = serde_json::to_string_pretty(&v).unwrap();

        let mut stdout = std::io::stdout();
        stdout.highlight(&text, "json", None).unwrap();
    }
}
