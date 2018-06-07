extern crate go_to_rust;
#[macro_use]
extern crate quicli;
extern crate codegen;

use quicli::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct ParsedEventFile {
    service_name: String,
    path: PathBuf,
    go: go_to_rust::GoCode,
    rust: go_to_rust::RustCode,
    example_event: Option<String>,
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
    // https://github.com/aws/aws-lambda-go/blob/master/events/dynamodb.go
    blacklist.insert("dynamodb".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/epoch_time.go
    blacklist.insert("epoch_time".to_string());
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
    parsed_files: &Vec<ParsedEventFile>,
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

fn find_example_event(sdk_location: &PathBuf, service_name: &str) -> Result<Option<String>> {
    let location = match service_name.as_ref() {
        "code_commit" => "events/testdata/code-commit-event.json".to_string(),
        "codepipeline_job" => "events/testdata/codepipline-event.json".to_string(),
        "cloudwatch_logs" => "events/testdata/cloudwatch-logs-event.json".to_string(),
        "firehose" => "events/testdata/kinesis-firehose-event.json".to_string(),
        _ => format!("events/testdata/{}-event.json", service_name),
    };
    let p = sdk_location.join(location);
    read_example_event(&p, service_name)
}

fn read_example_event(test_fixture: &PathBuf, service_name: &str) -> Result<Option<String>> {
    trace!(
        "Looking for example event: {}",
        test_fixture.to_string_lossy()
    );
    if !test_fixture.exists() {
        info!("No example event for service: {}", service_name);
        return Ok(None);
    }
    info!("Found example event for service: {}", service_name);
    let mut f = File::open(test_fixture).expect("fixture not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the fixture");
    debug!("Example event content: {}", contents);
    Ok(Some(contents))
}

fn write_fixture(
    service_name: &str,
    example_event: &String,
    out_dir: &PathBuf,
    overwrite: &bool,
) -> Result<PathBuf> {
    let relative = PathBuf::from(format!("fixtures/example-{}-event.json", service_name));
    // Write the example event to the output location.
    let full = out_dir.join(relative.clone());
    {
        let parent = full.parent().expect("parent directory");
        if !parent.exists() {
            trace!("Creating fixture directory: {:?}", parent);
            create_dir(&parent)?;
        }
    }
    if overwrite_warning(&full, *overwrite).is_none() {
        let mut f = File::create(full)?;
        f.write_all(example_event.as_bytes())?;
        f.write_all("\n".as_bytes())?;
    }
    Ok(relative)
}

fn generate_test_module(scope: &codegen::Scope, relative: &PathBuf) -> Result<codegen::Module> {
    let mut toplevel_type = None;
    for item in scope.items() {
        match item {
            codegen::Item::Struct(s) => {
                if s.ty().name().ends_with("Event") {
                    toplevel_type = Some(s.ty().name());
                    break;
                }
            }
            _ => continue,
        }
    }
    let mut test_function = codegen::Function::new("deserializes_event");
    test_function.annotation(vec!["test"]);
    test_function.line(format!(
        r#"let data = include_bytes!("{}");"#,
        relative.to_string_lossy(),
    ));
    test_function.line(format!(
        r#"let _: {} = serde_json::from_slice(data).unwrap();"#,
        toplevel_type.expect("top-level type defined"),
    ));

    let mut test_module = codegen::Module::new("test");
    test_module.annotation(vec!["cfg(test)"]);
    test_module.import("super", "*");
    test_module.scope().raw("extern crate serde_json;");
    test_module.scope().push_fn(test_function);
    Ok(test_module)
}

main!(|args: Cli, log_level: verbosity| {
    let mut parsed_files: Vec<ParsedEventFile> = Vec::new();

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
            // Parse the code.
            info!("Parsing: {}", x.to_string_lossy());
            let (go, rust) = go_to_rust::parse_go_file(&path)?;
            debug!("Go ------v\n{}", go);
            debug!("Rust-----v\n{}", rust);

            // Check for an example event in their test data.
            let example_event = find_example_event(&args.sdk_location, &file_name)?;

            parsed_files.push(ParsedEventFile {
                service_name: file_name.into_owned(),
                path,
                go,
                rust,
                example_event,
            });
        }
    }

    // Create the output location if needed.
    if !args.output_location.exists() {
        trace!("Creating directory: {:?}", args.output_location);
        create_dir(&args.output_location)?;
    }

    // Write the files.
    for parsed in &mut parsed_files {
        let out_dir = args.output_location.clone();
        let output_path = out_dir.join(
            parsed
                .path
                .with_extension("rs")
                .file_name()
                .expect("a file name exists"),
        );

        if let Some(ref example_event) = parsed.example_event {
            // Write the example event to a test fixture.
            trace!("Writing fixure for: {:?}", parsed.service_name);
            let relative = write_fixture(
                &parsed.service_name,
                &example_event,
                &out_dir,
                &args.overwrite,
            )?;

            // Generate a test module with a test that deserializes the example
            // event.
            trace!("Generating test module for: {:?}", parsed.service_name);
            let test_module = generate_test_module(&parsed.rust.scope(), &relative)?;
            parsed.rust.push_module(test_module);
        }

        if overwrite_warning(&output_path, args.overwrite).is_none() {
            let mut f = File::create(output_path)?;
            f.write_all(parsed.rust.to_string().as_bytes())?;
            f.write_all("\n".as_bytes())?;
        }
    }

    // Write the crate index.
    let mod_path = args.output_location.clone().join("mod.rs");
    write_mod_index(&mod_path, &parsed_files, args.overwrite)?;
});
