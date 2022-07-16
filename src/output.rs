pub struct RankingEntry {
    pub name: String,
    pub count: u64,
}

pub fn print_entries(mut entries: Vec<RankingEntry>) {
    entries.sort_by(|a, b| b.count.cmp(&a.count));

    println!("\nResult:\n");
    println!("{0: <16} | {1: <10}", "Username", "Count");
    println!("---------------- | ----------");
    entries.iter().for_each(|entry| print_entry(entry));
}

fn print_entry(entry: &RankingEntry) {
    println!("{0: <16} | {1: <10}", entry.name, entry.count);
}
