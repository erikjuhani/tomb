fn main() {
    println!("{}", get_version_string());
}

fn get_version_string() -> String {
    format!(
        "tomb {} ({} {})",
        env!("TOMB_VERSION"),
        env!("TOMB_COMMIT_SHORT_HASH"),
        env!("TOMB_COMMIT_DATE")
    )
}
