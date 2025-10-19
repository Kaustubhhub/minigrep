pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ans: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            ans.push(line);
        }
    }
    ans
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    let mut ans: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            ans.push(line);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
