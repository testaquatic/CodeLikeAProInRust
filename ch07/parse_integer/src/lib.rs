use regex::Regex;

pub fn parse_integer(s: &str) -> Option<i32> {
    let re = Regex::new(r"^-?\d{1,10}$").expect("Parsing regex failed");
    if re.is_match(s) {
        s.parse().ok()
    } else {
        None
    }
}
