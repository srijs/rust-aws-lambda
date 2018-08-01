extern crate env_logger;
extern crate glob;
extern crate go_to_rust;
extern crate rustc_test as test;

use glob::glob;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use test::{DynTestFn, DynTestName, TestDesc, TestDescAndFn};

fn mk_test(desc: &str, input: String, expect: String, expected_path: PathBuf) -> TestDescAndFn {
    TestDescAndFn {
        desc: TestDesc::new(DynTestName(desc.to_string())),
        testfn: DynTestFn(Box::new(move || {
            let (_, output) = go_to_rust::parse_go_string(input.clone()).expect("parser parses");

            // Add support for recording fixtures.
            if let Ok(_) = env::var("GO_TO_RUST_FIXTURE_RECORD") {
                let mut f = File::create(expected_path.clone()).expect("fixture to be opened");
                f.write_all(output.to_string().as_bytes())
                    .expect("fixture to be written");
            }

            if output != go_to_rust::RustCode::new(expect.clone()) {
                panic!(
                    "\n- input -\n{}\n- got -\n{}\n- expected -\n{}\n",
                    input, output, expect
                );
            }
        })),
    }
}

fn tests(src_dir: &Path) -> Vec<TestDescAndFn> {
    let mut tests = vec![];

    let entries = glob(&format!(
        "{}/{}",
        src_dir.to_string_lossy(),
        "tests/fixtures/*"
    )).expect("some fixtures");

    // Loop over matched fixtures.
    for entry in entries {
        let fixture_path = entry.unwrap();
        let input_path = fixture_path.clone().join("input.txt");
        let expected_path = fixture_path.clone().join("expected.txt");

        let data: Vec<String> = vec![input_path.clone(), expected_path.clone()]
            .iter()
            .map(|p| {
                let file = File::open(&p).unwrap();
                let mut reader = BufReader::new(file);
                let mut text = String::new();
                reader.read_to_string(&mut text).unwrap();
                text
            }).collect();

        let test_name = fixture_path
            .file_stem()
            .expect("fixture to have a file stem")
            .to_string_lossy();

        tests.push(mk_test(
            &test_name,
            data[0].clone().trim().to_string(),
            data[1].clone().trim().to_string(),
            expected_path.clone(),
        ));
    }

    tests
}

fn main() {
    let args: Vec<_> = env::args().collect();
    test::test_main(&args, tests(Path::new(env!("CARGO_MANIFEST_DIR"))));
}
