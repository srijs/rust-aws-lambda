use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PathParameters(HashMap<String, String>);
impl PathParameters {
    pub fn get(&self, name: &str) -> Option<&str> {
        match self.0.get(name) {
            Some(v) => Some(v.as_str()),
            None => None,
        }
    }
}
impl From<HashMap<String, String>> for PathParameters {
    fn from(path_parameters: HashMap<String, String>) -> PathParameters {
        PathParameters(path_parameters)
    }
}

#[derive(Debug, PartialEq)]
pub struct QueryParameters(HashMap<String, String>);
impl QueryParameters {
    pub fn get(&self, name: &str) -> Option<&str> {
        match self.0.get(name) {
            Some(v) => Some(v.as_str()),
            None => None,
        }
    }
}
impl From<HashMap<String, String>> for QueryParameters {
    fn from(query_parameters: HashMap<String, String>) -> QueryParameters {
        QueryParameters(query_parameters)
    }
}
