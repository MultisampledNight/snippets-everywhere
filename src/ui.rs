use std::path::PathBuf;

use clap::{value_parser, Arg, ArgGroup, ArgMatches, Command};

use crate::Backend;

pub fn cmdline(backends: &[Box<dyn Backend>]) -> ArgMatches {
    let mut cmd = Command::new("snippets-everywhere");
    let mut in_args = ArgGroup::new("inputs").required(true);
    let mut out_args = ArgGroup::new("outputs").multiple(true).required(true);

    for backend in backends {
        if let Some(name_in) = backend.name_in() {
            cmd = cmd.arg(
                Arg::new(&name_in)
                    .long(&name_in)
                    .value_parser(value_parser!(PathBuf)),
            );
            in_args = in_args.arg(name_in);
        }

        if let Some(name_out) = backend.name_out() {
            cmd = cmd.arg(
                Arg::new(&name_out)
                    .long(&name_out)
                    .value_parser(value_parser!(PathBuf)),
            );
            out_args = out_args.arg(name_out);
        }
    }

    cmd.group(in_args).group(out_args).get_matches()
}
