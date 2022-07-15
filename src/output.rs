pub struct RankingEntry {
    pub name: String,
    pub count: u64,
}

pub fn print_entry(entry: &RankingEntry) {
    println!("{0: <16} | {1: <10}", entry.name, entry.count);
}
