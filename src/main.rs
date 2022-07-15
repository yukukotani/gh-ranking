use std::{collections::HashMap, process::Command};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "gh-ranking")]
struct Opt {
    #[structopt(name = "ACTION")]
    action: String,
    #[structopt(short, long)]
    org: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: HashMap<String, IssueCount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct IssueCount {
    issue_count: u64,
}

fn main() {
    let opt = Opt::from_args();

    let search_queries = ["yukukotani", "sosukesuzuki"]
        .map(|user| {
            let query = format!("author:{} type:pr org:{}", user, opt.org);
            return format!(
                "{}: search(query: \"{}\", type: ISSUE, first: 0) {{ issueCount }}",
                user, query
            );
        })
        .join("\n");

    let query = format!(
        "query {{
            {}
        }}",
        search_queries
    );

    let result = Command::new("gh")
        .args(["api", "graphql", "-f", format!("query={}", query).as_str()])
        .output()
        .expect("error");

    let response: Response = serde_json::from_slice(&result.stdout).unwrap();

    let mut vec = response
        .data
        .iter()
        .map(|(user, issue_count)| (user, issue_count.issue_count))
        .collect::<Vec<_>>();

    vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{vec:?}");
}
