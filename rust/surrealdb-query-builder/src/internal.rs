use regex::Regex;
pub fn replace_params(query: &str) -> String {
    let mut count = 0;
    let re = regex::Regex::new(r"_param_[[:xdigit:]]+").unwrap();
    re.replace_all(query, |caps: &regex::Captures<'_>| {
        count += 1;
        format!("_param_{:08}", count)
    })
    .to_string()
}
