/// this module manages reading and translating
/// the arguments passed on launch of the application.
use crate::errors::ProgramError;
use clap;
use std::path::PathBuf;
use std::result::Result;

pub struct AppLaunchArgs {
    pub target: PathBuf,
    pub no_scroll: bool,
    pub width: usize,
}

/// declare the possible CLI arguments, and gets the values
fn get_cli_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("clima")
        .version(env!("CARGO_PKG_VERSION"))
        .author("dystroy <denys.seguret@gmail.com>")
        .about("minimal rough markdown viewer")
        .arg(clap::Arg::with_name("target").help("sets the file to open"))
        .arg(
            clap::Arg::with_name("no-scroll")
                .help("Disables use of scroll view")
                .long("no-scroll")
                .short("s"),
        )
        .arg(
            clap::Arg::with_name("width")
                .help("Sets output width")
                .long("width")
                .short("w")
                .takes_value(true),
        )
        .get_matches()
}

/// return the parsed launch arguments
pub fn read_launch_args() -> Result<AppLaunchArgs, ProgramError> {
    let cli_args = get_cli_args();
    let target = match cli_args.value_of("target") {
        Some(path) => PathBuf::from(path),
        None => {
            return Err(ProgramError::NoPathProvided {});
        }
    };
    if !target.exists() {
        Err(ProgramError::FileNotFound {
            path: format!("{:?}", &target),
        })?;
    }
    if target.is_dir() {
        Err(ProgramError::NotRegular {
            path: format!("{:?}", &target),
        })?;
    }
    let target = target.canonicalize()?;
    let no_scroll = cli_args.is_present("no-scroll");
    let width = cli_args.value_of("width")
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);
    Ok(AppLaunchArgs { target, no_scroll, width })
}
