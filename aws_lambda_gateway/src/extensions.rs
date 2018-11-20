use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PathParameters(pub HashMap<String, String>);

#[derive(Debug, PartialEq)]
pub struct QueryParameters(pub HashMap<String, String> );