use regex::Regex;
use std::fmt::Display;

pub struct Repo {
    pub owner: String,
    pub repo: String,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

const PATTERN1: &str = r"^(?:https://[\w\.\-]+/)|(?:git@[\w\.\-]+:)";
const PATTERN2: &str = r"^([\w-]+)/([\w\.-]+)";

pub fn parse_url(url: &str) -> Option<Repo> {
    let mut url = url;

    // here use unwrap because it depends on the developer
    let ptn1 = Regex::new(PATTERN1).unwrap();
    if let Some(m) = ptn1.find(url) {
        url = &url[m.end()..];
    }

    if let Some(u) = url.strip_prefix("/") {
        url = u
    };

    // use unwrap like above
    let ptn2 = Regex::new(PATTERN2).unwrap();
    let Some(c) = ptn2.captures(url) else {
        return None;
    };

    let Some(owner) = c.get(1) else {
        return None;
    };
    let owner = owner.as_str();
    let Some(repo) = c.get(2) else {
        return None;
    };
    let mut repo = repo.as_str();

    let Some(m) = c.get(0) else {
        return None;
    };
    if m.len() == url.len() || url.as_bytes()[m.len()] != b'/' {
        if let Some(r) = repo.strip_suffix(".git") {
            repo = r;
        }
    }

    Some(Repo {
        owner: owner.to_string(),
        repo: repo.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::parse_url;

    fn f(url: &str) -> String {
        match parse_url(url) {
            Some(r) => r.to_string(),
            None => "".to_string(),
        }
    }

    #[test]
    fn test_1() {
        assert_eq!(f("x/y"), "x/y");
        assert_eq!(f("x/y.git"), "x/y");
        assert_eq!(f("/x/y"), "x/y");
        assert_eq!(f("/x/y.git"), "x/y");
        assert_eq!(f("https://github.com/x/y"), "x/y");
        assert_eq!(f("https://github.com/x/y.git"), "x/y");
        assert_eq!(f("git@github.com:x/y"), "x/y");
        assert_eq!(f("git@github.com:x/y.git"), "x/y");
        assert_eq!(f("https://github.mirror/x/y"), "x/y");
        assert_eq!(f("https://github.mirror/x/y.git"), "x/y");
        assert_eq!(f("https://a.b.c/x/y"), "x/y");
        assert_eq!(f("https://a.b.c/x/y.git"), "x/y");
    }

    #[test]
    fn test_2() {
        assert_eq!(f("a-b/c_d"), "a-b/c_d");
        assert_eq!(f("a-b/c_d.git"), "a-b/c_d");
        assert_eq!(f("a-b/c_d.git.git"), "a-b/c_d.git");
        assert_eq!(f("a-b/c_d.github.io"), "a-b/c_d.github.io");
        assert_eq!(f("a-b/c_d.github.io.git"), "a-b/c_d.github.io");
        assert_eq!(f("a-b/c_d.github.io.git.git"), "a-b/c_d.github.io.git");
    }

    #[test]
    fn test_3() {
        assert!(parse_url("xy").is_none());
        assert!(parse_url("https://github.com/xy").is_none());
    }

    #[test]
    fn test_4() {
        assert_eq!(f("a/b/c/x/y"), "a/b");
        assert_eq!(f("a/b.git/c/x/y"), "a/b.git");
        assert_eq!(f("a/b/c/x/y.git"), "a/b");
        assert_eq!(f("a/b.git/c/x/y.git"), "a/b.git");
        assert_eq!(f("a/b.git/c.git/x.git/y.git"), "a/b.git");
        assert_eq!(f("/a/b.git/c.git/x.git/y.git"), "a/b.git");

        assert_eq!(f("https://github.com/x/y/issues"), "x/y");
        assert_eq!(f("https://github.com/x/y.git/issues"), "x/y.git");
        assert_eq!(f("https://github.com/x/y/issues.git"), "x/y");
        assert_eq!(f("https://github.com/x/y.git/issues.git"), "x/y.git");

        assert_eq!(f("https://github.com/x/y/releases/tag/v1.0"), "x/y");
        assert_eq!(f("https://github.com/x/y.git/releases/tag/v1.0"), "x/y.git");

        assert_eq!(f("https://github.com/x/y/releases/tag/v1.0.git"), "x/y");
    }

    #[test]
    fn test_5() {
        assert!(parse_url("https://github.com/[]/{}").is_none());
        assert!(parse_url("git@github.com:[]/{}").is_none());
        assert!(parse_url("git@github.com:/[]/{}").is_none());
    }

    #[test]
    fn test_6() {
        assert!(parse_url("https:///x/y").is_none());
        assert!(parse_url("git@:/x/y").is_none());
    }
}
