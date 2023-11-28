use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;
use std::collections::LinkedList;

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

#[derive(Debug)]
enum Source {
    Code(String),
    Doc(String),
}

fn to_source_elements(source_text: String) -> LinkedList<Source> {
    let doc_prefix = "##";
    let mut source_lines: LinkedList<Source> = LinkedList::new();

    for source_line in source_text.lines() {
       if source_line.trim().starts_with(doc_prefix) {
           source_lines.push_back(
               Source::Doc(
                   source_line.trim_start_matches(doc_prefix).trim().to_string()
               )
           );
       } else {
           source_lines.push_back(Source::Code(source_line.to_string()));
       }
    }

    // TODO Lines with same type should be meld to single elements...
    source_lines
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world! Opts: {:?}", opt);
    println!("File name: {}", opt.input.display());
    let content = fs::read_to_string(opt.input)
        .expect("Cannot load source file");

    println!("I've got a file: \"\"\"\n{content}\"\"\"");

    println!("\"Parsed\" source lines: {:?}", to_source_elements(content));

    println!(
        "Documentation comment indentifier: \"{0}\"",
        opt.doc_comment_identifier
    );

    println!("Source: {:?}", Source::Code("fn x() = || y;".to_string()));
    println!("Source: {:?}", Source::Doc("@param x".to_string()));
    println!("To-Source-Elements: {:?}", to_source_elements("".to_string()));


}
