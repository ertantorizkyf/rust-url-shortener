use regex::Regex;

pub fn url_validator(url: String) -> bool {
    let url_regex_pattern = Regex::new(r"(https?:\/\/)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#()?&//=]*)").unwrap(); 
    let mut _is_pattern_matched = true;
    match url_regex_pattern.captures(&url) {
        Some(_caps) => _is_pattern_matched = true,
        None => _is_pattern_matched = false
    };
    
    _is_pattern_matched
}
