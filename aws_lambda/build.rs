extern crate skeptic;

use skeptic::*;

fn main() {
    let mut files = markdown_files_of_directory("../docs/");
    files.push("../README.md".into());

    generate_doc_tests(&files);
}
