mod graphql;
mod output;

use std::{collections::HashMap, fmt::Debug};

use output::print_entries;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

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
    #[structopt(short, long)]
    query: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    println!("Fetching members");
    let members = match opt {
        Opt {
            action: _,
            ref org,
            team: Some(ref team),
            query: _,
        } => get_team_members(&org, &team),
        Opt {
            action: _,
            ref org,
            team: _,
            query: _,
        } => get_org_members(&org),
    };

    let get_count = match opt.action.to_lowercase().as_str() {
        "open-pr" => get_open_pr_count,
        "review-pr" => get_review_pr_count,
        _ => {
            eprintln!("Invalid action: {}", opt.action);
            std::process::exit(1);
        }
    };

    let entries = members
        .chunks(10)
        .flat_map(|users| {
            println!("Fetching data of {:?}", users);
            return get_count(users, &opt);
        })
        .collect::<Vec<_>>();

    print_entries(entries);
}

fn get_open_pr_count(users: &[String], opt: &Opt) -> Vec<RankingEntry> {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct IssueCount {
        issue_count: u64,
    }

    let search_queries = users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let query = format!(
                "type:pr author:{} org:{} {}",
                user,
                opt.org,
                opt.query.as_ref().unwrap_or(&"".to_string())
            );
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

fn get_review_pr_count(users: &[String], opt: &Opt) -> Vec<RankingEntry> {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct IssueCount {
        issue_count: u64,
    }

    let search_queries = users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let query = format!(
                "type:pr reviewed-by:{0} -author:{0} org:{1} {2}",
                user,
                opt.org,
                opt.query.as_ref().unwrap_or(&"".to_string())
            );
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
