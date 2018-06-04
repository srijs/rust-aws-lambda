extern crate go_to_rust;
#[macro_use]
extern crate quicli;

use quicli::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct ParsedFile {
    path: PathBuf,
    go: go_to_rust::GoCode,
    rust: go_to_rust::RustCode,
}

/// Generate rust code for AWS lambda events sourced from `aws-go-sdk`
#[derive(Debug, StructOpt)]
#[structopt(author = "")]
struct Cli {
    /// Path to `aws-go-sdk` checkout
    #[structopt(long = "input", name = "AWS_GO_SDK_DIRECTORY", parse(from_os_str))]
    sdk_location: PathBuf,
    /// Output directory
    #[structopt(long = "output", short = "o", name = "DIRECTORY", parse(from_os_str))]
    output_location: PathBuf,
    /// Overwrite existing files
    #[structopt(long = "overwrite")]
    overwrite: bool,
    /// Verbose output. Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

fn get_blacklist() -> HashSet<String> {
    let mut blacklist = HashSet::new();
    // https://github.com/aws/aws-lambda-go/blob/master/events/attributevalue.go
    blacklist.insert("attributevalue".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/firehose.go
    blacklist.insert("firehose".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/cloudwatch_logs.go
    blacklist.insert("cloudwatch_logs".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/code_commit.go
    blacklist.insert("code_commit".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/dynamodb.go
    blacklist.insert("dynamodb".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/epoch_time.go
    blacklist.insert("epoch_time".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/epoch_time.go
    blacklist.insert("epoch_time".to_string());

    // The following are very close to working.
    // https://github.com/aws/aws-lambda-go/blob/master/events/ses.go
    blacklist.insert("ses".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/lex.go
    blacklist.insert("lex".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/kinesis.go
    blacklist.insert("kinesis".to_string());
    blacklist
}

fn overwrite_warning(path: &PathBuf, overwrite: bool) -> Option<()> {
    if path.exists() && !overwrite {
        error!(
            "File already exists and `--overwrite` not specified. Skipping: {}",
            path.to_string_lossy()
        );
        return Some(());
    }
    return None;
}

fn write_mod_index(
    mod_path: &PathBuf,
    parsed_files: &Vec<ParsedFile>,
    overwrite: bool,
) -> Result<()> {
    if overwrite_warning(&mod_path, overwrite).is_none() {
        let mut mod_content: Vec<String> = Vec::new();
        for parsed in parsed_files {
            mod_content.push(format!(
                "pub mod {};",
                parsed
                    .path
                    .file_stem()
                    .expect("file stem")
                    .to_string_lossy()
            ));
        }
        let mut f = File::create(mod_path)?;
        f.write_all(mod_content.join("\n").as_bytes())?;
        f.write_all("\n".as_bytes())?;
    }
    Ok(())
}

main!(|args: Cli, log_level: verbosity| {
    let mut parsed_files: Vec<ParsedFile> = Vec::new();

    // The glob pattern we are going to use to find the go files with event defs.
    let pattern = format!("{}/events/*.go", args.sdk_location.to_string_lossy());

    // Some files we don't properly handle yet.
    let blacklist = get_blacklist();

    // Loop over matched files.
    for path in glob(&pattern)? {
        let x = path.clone();
        let file_name = x.file_stem().expect("file stem").to_string_lossy();

        // Filter out tests and blacklisted files.
        if !file_name.contains("_test") && !blacklist.contains(&*file_name) {
            info!("Parsing: {}", x.to_string_lossy());
            let (go, rust) = go_to_rust::parse_go_file(&path)?;
            debug!("Go ------v\n{}", go);
            debug!("Rust-----v\n{}", rust);

            parsed_files.push(ParsedFile { path, go, rust });
        }
    }

    // Create the output location if needed.
    if !args.output_location.exists() {
        trace!("Creating directory: {:?}", args.output_location);
        create_dir(&args.output_location)?;
    }

    // Write the files.
    for parsed in &parsed_files {
        let out_dir = args.output_location.clone();
        let output_path = out_dir.join(
            parsed
                .path
                .with_extension("rs")
                .file_name()
                .expect("a file name exists"),
        );
        if overwrite_warning(&output_path, args.overwrite).is_none() {
            let mut f = File::create(output_path)?;
            f.write_all(parsed.rust.as_bytes())?;
            f.write_all("\n".as_bytes())?;
        }
    }

    // Write the crate index.
    let mod_path = args.output_location.clone().join("mod.rs");
    write_mod_index(&mod_path, &parsed_files, args.overwrite)?;
});
