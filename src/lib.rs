mod macros;
mod traits;
mod utils;

pub use traits::*;
pub use utils::*;

// re-export
pub use anyhow::{self, bail, ensure, Context, Error, Result};
pub use async_trait::async_trait;
pub use atty;
pub use clap::{self, Parser};
pub use dialoguer;
pub use enum_dispatch::enum_dispatch;
pub use paste::paste;
pub use strum::{self, EnumString};

pub mod prelude {
    pub use crate::{
        anyhow, anyhow::Result, async_trait, atty, bail, clap, dialoguer, enum_dispatch,
        mod_pub_use, paste, subcmd, Context, EnumString, Error, Highlight, Parser, ShellType,
    };
}
