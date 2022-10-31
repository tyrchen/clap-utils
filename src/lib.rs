mod macros;
mod traits;
mod utils;

pub use traits::*;
pub use utils::*;

// re-export
pub use anyhow::{self, Error};
pub use async_trait::async_trait;
pub use atty;
pub use clap::{self, Parser};
pub use dialoguer;
pub use enum_dispatch::enum_dispatch;
pub use error_stack::{self, Report, Result, ResultExt};
pub use paste::paste;
pub use strum::{self, EnumString};

pub mod prelude {
    pub use crate::{
        anyhow, async_trait, atty, clap, dialoguer, enum_dispatch, error_report, error_stack,
        mod_pub_use, paste, subcmd, EnumString, Error, Highlight, Parser, Report, Result,
        ResultExt, ShellType, ToReport,
    };
}
