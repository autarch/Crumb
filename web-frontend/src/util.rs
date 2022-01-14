pub(crate) fn maybe_plural(count: u32, noun: &'static str) -> String {
    match count {
        1 => format!("1 {}", noun),
        _ => format!("{} {}s", count, noun),
    }
}
