mod graphql;
mod output;

use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use output::print_entry;
use output::RankingEntry;

#[derive(StructOpt)]
#[structopt(name = "gh-ranking")]
struct Opt {
    #[structopt(name = "ACTION")]
    action: String,
    #[structopt(short, long)]
    org: String,
    #[structopt(short, long)]
    team: Option<String>,
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

    println!("Fetching members");
    let members = match opt {
        Opt {
            action: _,
            ref org,
            team: Some(ref team),
        } => get_team_members(&org, &team),
        Opt {
            action: _,
            ref org,
            team: _,
        } => get_org_members(&org),
    };

    match opt.action.to_lowercase().as_str() {
        "openpr" => open_pr_command(opt, members),
        _ => {
            eprintln!("Invalid action: {}", opt.action);
            std::process::exit(1);
        }
    }
}

fn open_pr_command(opt: Opt, members: Vec<String>) {
    let mut vec = members
        .chunks(10)
        .flat_map(|users| {
            println!("Fetching data of {:?}", users);
            return get_open_pr_count(users, opt.org.as_str());
        })
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| b.count.cmp(&a.count));

    println!("\nResult:\n");
    println!("{0: <16} | {1: <10}", "Username", "Count");
    println!("---------------- | ----------");
    vec.iter().for_each(|entry| print_entry(entry));
}

fn get_open_pr_count(users: &[String], org: &str) -> Vec<RankingEntry> {
    let search_queries = users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let query = format!("author:{} type:pr org:{}", user, org);
            return format!(
                "{}: search(query: \"{}\", type: ISSUE, first: 0) {{ issueCount }}",
                format!("user_{}", i),
                query
            );
        })
        .collect::<Vec<String>>()
        .join("\n");

    let query = format!(
        "query {{
            {}
        }}",
        search_queries
    );

    let response: HashMap<String, IssueCount> = graphql::query(query);

    return response
        .iter()
        .map(|(key, issue_count)| {
            let i = key[5..].parse::<usize>().unwrap();
            return RankingEntry {
                name: users[i].to_string(),
                count: issue_count.issue_count,
            };
        })
        .collect::<Vec<_>>();
}

fn get_org_members(org: &str) -> Vec<String> {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        organization: Organization,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Organization {
        members_with_role: MembersConnection,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct MembersConnection {
        nodes: Vec<Member>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Member {
        login: String,
    }

    let query = format!(
        "query {{
            organization(login: \"{}\") {{
                membersWithRole(first: 100) {{
                    nodes {{
                        login
                    }}
                }}
            }}
        }}",
        org
    );

    let response: Response = graphql::query(query);

    return response
        .organization
        .members_with_role
        .nodes
        .iter()
        .map(|member| {
            return member.login.to_string();
        })
        .collect::<Vec<_>>();
}

fn get_team_members(org: &str, team: &str) -> Vec<String> {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        organization: Organization,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Organization {
        team: Team,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Team {
        members: MembersConnection,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct MembersConnection {
        nodes: Vec<Member>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Member {
        login: String,
    }

    let query = format!(
        "query {{
            organization(login: \"{}\") {{
                team(slug: \"{}\") {{
                    members(first: 100) {{
                        nodes {{
                            login
                        }}
                    }}
                }}
            }}
        }}",
        org, team
    );

    let response: Response = graphql::query(query);

    return response
        .organization
        .team
        .members
        .nodes
        .iter()
        .map(|member| {
            return member.login.to_string();
        })
        .collect::<Vec<_>>();
}
