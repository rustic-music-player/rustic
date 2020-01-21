use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub(crate) struct CliOptions {
    /// Verbosity (-v = debug, -vv = trace)
    #[structopt(short, long, parse(from_occurrences))]
    pub(crate) verbose: u8,

    /// Config file
    #[structopt(
        short,
        long,
        default_value = "config.toml",
        parse(from_os_str)
    )]
    pub(crate) config: PathBuf,

    /// Extensions path
    #[structopt(long = "extensions")]
    pub(crate) extensions_path: Option<String>
}
