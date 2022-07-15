use std::{collections::HashMap, process::Command};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

struct RankingEntry {
    name: String,
    count: u64,
}

fn main() {
    let opt = Opt::from_args();

    match opt.action.to_lowercase().as_str() {
        "openpr" => open_pr_command(opt),
        _ => {
            eprintln!("Invalid action: {}", opt.action);
            std::process::exit(1);
        }
    }
}

fn open_pr_command(opt: Opt) {
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

    let response: HashMap<String, IssueCount> = query_graphql(query);

    let mut vec = response
        .iter()
        .map(|(user, issue_count)| {
            return RankingEntry {
                name: user.to_string(),
                count: issue_count.issue_count,
            };
        })
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| b.count.cmp(&a.count));

    println!("{0: <16} | {1: <10}", "Username", "Count");
    println!("---------------- | ----------");
    vec.iter().for_each(|entry| print_entry(entry));
}

fn print_entry(entry: &RankingEntry) {
    println!("{0: <16} | {1: <10}", entry.name, entry.count);
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: T,
}

fn query_graphql<T>(query: String) -> T
where
    T: DeserializeOwned,
{
    let result = Command::new("gh")
        .args(["api", "graphql", "-f", format!("query={}", query).as_str()])
        .output()
        .expect("error");

    let response: GraphQLResponse<T> = serde_json::from_slice(&result.stdout).unwrap();

    return response.data;
}
