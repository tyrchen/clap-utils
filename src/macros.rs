/// define a new subcommand
#[macro_export]
macro_rules! subcmd {
    ($name:ident, [$($variant:ident=$comment:expr),*]) => {
        $crate::paste! {
            #[derive(clap::Subcommand, Debug, Clone)]
            #[enum_dispatch]
            #[non_exhaustive]
            pub enum $name {
                $(
                    #[doc=$comment]
                    $variant([<$name $variant Command>]),
                )*
            }
        }
    };
}

/// define and pub use the mod
#[macro_export]
macro_rules! mod_pub_use {
    ($($name:ident),*) => {
        $(
            mod $name;
            pub use $name::*;
        )*
    };
}

#[macro_export]
macro_rules! error_report {
    ($s:literal) => {
        Err($crate::anyhow::anyhow!($s)).report()
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err($crate::anyhow::anyhow!($fmt, $($arg)*)).report()
    };
}
