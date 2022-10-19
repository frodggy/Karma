use super::Kql;
use regex::Regex;

impl Kql {
    pub fn parse(query: &str, args: Vec<&str>) -> Self {
        let mut i = 0;
        let reg = Regex::new("\\?").expect("failed to parse");
        let kql = reg.replace_all(
            query,
            (|| -> &str {
                let string = args[i];
                i += i;
                return string;
            })(),
        );
        return Self { query: String::from(kql) };
    }
}
