use clap::{Arg, ArgGroup, ArgMatches, Command};

use crate::Backend;

pub fn cmdline(backends: &[Box<dyn Backend>]) -> ArgMatches {
    let mut cmd = Command::new("snippets-everywhere");
    let mut in_args = ArgGroup::new("inputs").required(true);
    let mut out_args = ArgGroup::new("outputs").multiple(true).required(true);

    for backend in backends {
        let name_in = format!("{}-in", backend.name());
        let name_out = format!("{}-out", backend.name());
        cmd = cmd
            .arg(Arg::new(&name_in).long(&name_in))
            .arg(Arg::new(&name_out).long(&name_out));
        in_args = in_args.arg(name_in);
        out_args = out_args.arg(name_out);
    }

    cmd.group(in_args).group(out_args).get_matches()
}
