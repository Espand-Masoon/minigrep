pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result_lines.push(line);
        }
    }

    result_lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let content = "\
Rust:
Safe, Fast, Productive.
Pick three!";
        assert_eq!(search("ast", content), vec!["Safe, Fast, Productive."]);
    }
}
