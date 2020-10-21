use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod const_eval_no_pass;

/// A simple tool which applies a pass to a Relay program and
/// serialized the output to disk.
#[derive(StructOpt, Debug)]
#[structopt(name = "tvm_opt")]
struct Opt {
    /// The input file path.
    #[structopt(short, long)]
    input: PathBuf,
    /// Which pass to execute.
    #[structopt(short, long)]
    pass_name: String,
    /// The output file path.
    #[structopt(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let output: PathBuf = opt.output.unwrap_or(Path::new("output.rly").into());
    match &opt.pass_name[..] {
        "const_eval_no_pass" => const_eval_no_pass::run(opt.input, output).expect(),
        _ => panic!()
    }
    println!("{:#?}", opt);
}
