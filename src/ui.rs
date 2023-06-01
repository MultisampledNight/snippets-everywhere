use clap::{Arg, ArgGroup, ArgMatches, Args, Command, Parser};

use crate::Backend;

pub fn cmdline(backends: &[Box<dyn Backend>]) -> ArgMatches {
    let mut in_args = ArgGroup::new("inputs").required(true);
    let mut out_args = ArgGroup::new("outputs").multiple(true).required(true);

    for backend in backends {
        todo!()
    }

    Command::new("snippets-everywhere")
        .group(in_args)
        .group(out_args)
        .get_matches()
}
