use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "exodocia",
    about = "Simple documentation extraction tool.",
    rename_all = "kebab-case"
)]
/// Simple tool for extracting documentation from _any_ source.
/// Specifically shell scripts et consortes.
///
struct Opt {
    #[structopt(name = "SOURCE_FILE", parse(from_os_str))]
    input: PathBuf,

    #[structopt(default_value = "##")]
    doc_comment_identifier: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world! Opts: {:?}", opt);
    println!("File name: {}", opt.input.display());
    let content = fs::read_to_string(opt.input)
        .expect("Cannot load source file");

    println!("I've got a file: \"\"\"\n{content}\"\"\"");

    println!(
        "Documentation comment indentifier: \"{0}\"",
        opt.doc_comment_identifier
    );

}
