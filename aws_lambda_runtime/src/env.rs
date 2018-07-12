//! Inspection of the lambda's environment.
//!
//! This module contains functions to inspect various aspects of the environment
//! and the currently running lambda function.

lazy_static! {
    static ref AWS_LAMBDA_FUNCTION_NAME: String = var("AWS_LAMBDA_FUNCTION_NAME");
    static ref AWS_LAMBDA_FUNCTION_VERSION: String = var("AWS_LAMBDA_FUNCTION_VERSION");
    static ref AWS_LAMBDA_FUNCTION_MEMORY_SIZE: usize = {
        let size_in_mb = var("AWS_LAMBDA_FUNCTION_MEMORY_SIZE")
            .parse::<usize>()
            .unwrap_or_else(|err| panic!("AWS_LAMBDA_FUNCTION_MEMORY_SIZE: {}", err));
        size_in_mb * 1024 * 1024
    };
}

fn var(key: &str) -> String {
    ::std::env::var(key).unwrap_or_else(|err| panic!("{}: {}", key, err))
}

/// Returns the name of the current function.
pub fn function_name() -> &'static str {
    &AWS_LAMBDA_FUNCTION_NAME
}

/// Returns the version of the current function.
pub fn function_version() -> &'static str {
    &AWS_LAMBDA_FUNCTION_VERSION
}

/// Returns the memory limit (in bytes) of the current function.
pub fn function_memory_size() -> usize {
    *AWS_LAMBDA_FUNCTION_MEMORY_SIZE
}
