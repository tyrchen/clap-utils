use crate::prelude::*;
use std::io::Write;

macro_rules! highlight_ext {
    ($($name:ident), *) => {
        $crate::paste! {
            $(
                #[doc=concat!("write highlighted ", stringify!($name), " to writer")]
                fn [<highlight_ $name>](&mut self, text: &str) -> Result<(), Error> {
                    self.highlight(text, stringify!($name), None)
                }
            )*
        }
    };
}
pub trait Highlight: Write {
    /// write highlighted text to writer
    fn highlight(&mut self, text: &str, extension: &str, theme: Option<&str>) -> Result<(), Error>;

    fn highlight_ext(&mut self, text: &str, extension: &str) -> Result<(), Error> {
        self.highlight(text, extension, None)
    }

    highlight_ext!(json, yaml, toml, xml, html, css, js, rs, py, rb, sh, md, txt);
}
