pub fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut result_lines: Vec<&str> = Vec::new();


    if case_sensitive {
        for line in content.lines() {
            if line.contains(query) {
                result_lines.push(line);
            }
        }
    } else {
        let query = query.to_lowercase();
        for line in content.lines() {
            if line.to_lowercase().contains(&query) {
                result_lines.push(line);
            }
        }
    }

    result_lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let content = "\
Rust:
Safe, Fast, Productive.
Pick three!
there is no PAST";
        assert_eq!(search("ast", content, true), vec!["Safe, Fast, Productive."]);
    }

    #[test]
    fn case_insensitive() {
        let content = "\
Rust:
Safe, Fast, Productive.
Pick three!
There is no PAST";
        assert_eq!(search("ast", content, false), vec!["Safe, Fast, Productive.", "There is no PAST"]);
    }
}
