use std::{fmt::Debug, process::Command};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<Value>>,
}

pub fn query<T>(query: String) -> T
where
    T: DeserializeOwned + Debug,
{
    let result = Command::new("gh")
        .args(["api", "graphql", "-f", format!("query={}", query).as_str()])
        .output()
        .expect("error");

    let response: GraphQLResponse<T> = serde_json::from_slice(&result.stdout).unwrap();

    match response.data {
        Some(data) => data,
        None => {
            eprintln!("{:#?}", response.errors);
            std::process::exit(1);
        }
    }
}
