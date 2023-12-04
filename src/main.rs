use std::collections::LinkedList;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

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

#[derive(Debug, PartialEq)]
enum Source {
    Code(String),
    Doc(String),
}

#[derive(Debug, PartialEq)]
enum SourceType {
    Code,
    Doc,
}

fn to_source_elements(source_text: String) -> LinkedList<Source> {
    let doc_prefix = "##";
    let mut source_lines: LinkedList<Source> = LinkedList::new();

    for source_line in source_text.lines() {
        if source_line.trim().starts_with(doc_prefix) {
            source_lines.push_back(Source::Doc(
                source_line
                    .trim_start_matches(doc_prefix)
                    .trim()
                    .to_string(),
            ));
        } else {
            source_lines.push_back(Source::Code(source_line.to_string()));
        }
    }

    source_lines
}

fn meld_neighbors(source_lines: LinkedList<Source>) -> LinkedList<Source> {
    let mut part: String = "".to_owned();
    let mut documentation: LinkedList<Source> = LinkedList::new();

    let mut last_part_type = match source_lines.front() {
        Some(Source::Code(_)) => SourceType::Code,
        Some(Source::Doc(_)) => SourceType::Doc,
        _ => return documentation,
    };

    for line in source_lines.iter() {
        match line {
            Source::Code(line_str) => {
                if SourceType::Code == last_part_type {
                    part.push_str(line_str);
                    part.push_str("\n");
                } else {
                    documentation.push_back(Source::Doc(part));

                    part = line_str.to_owned();
                    part.push_str("\n");
                    last_part_type = SourceType::Code;
                }
            }
            Source::Doc(line_str) => {
                if SourceType::Doc == last_part_type {
                    part.push_str(line_str);
                    part.push_str("\n");
                } else {
                    documentation.push_back(Source::Code(part));

                    part = line_str.to_owned();
                    part.push_str("\n");
                    last_part_type = SourceType::Doc;
                }
            }
        }
    }
    match last_part_type {
        SourceType::Code => documentation.push_back(Source::Code(part)),
        SourceType::Doc => documentation.push_back(Source::Doc(part)),
    }

    documentation
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world! Opts: {:?}", opt);
    println!("File name: {}", opt.input.display());
    let content = fs::read_to_string(opt.input).expect("Cannot load source file");

    println!("I've got a file: \"\"\"\n{content}\"\"\"");

    let lines_of_source = to_source_elements(content);
    println!("\"Parsed\" source lines: {:?}\n", lines_of_source);
    let parts_of_source = meld_neighbors(lines_of_source);
    println!("\"Meld\" source parts: {:?}\n", parts_of_source);

    println!(
        "Documentation comment indentifier: \"{0}\"",
        opt.doc_comment_identifier
    );

    println!("Source: {:?}", Source::Code("fn x() = || y;".to_string()));
    println!("Source: {:?}", Source::Doc("@param x".to_string()));
    println!(
        "To-Source-Elements: {:?}",
        to_source_elements("".to_string())
    );
}
