use regex::Regex;
use serde::Serialize;

pub enum Verdict {
    Good,
    Bad,
    Maybe,
}

impl Serialize for Verdict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Good => serializer.serialize_unit_variant("Verdict", 0, "Good"),
            Self::Bad => serializer.serialize_unit_variant("Verdict", 1, "Bad"),
            Self::Maybe => serializer.serialize_unit_variant("Verdict", 2, "Maybe"),
        }
    }
}

impl TryFrom<&str> for Verdict {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "ok" => Ok(Self::Good),
            "bad" => Ok(Self::Bad),
            "maybe" => Ok(Self::Maybe),
            _ => Err(()),
        }
    }
}

#[derive(Serialize)]
pub struct Entry {
    pub name: String,
    pub verdict: Verdict,
    pub description: String,
    pub sources: Vec<String>,
    pub review: Option<String>,
}

struct EntryBuild<'a> {
    pub name: Option<&'a str>,
    pub verdict: Option<Verdict>,
    pub description: Vec<&'a str>,
    pub sources: Vec<&'a str>,
    pub review: Option<&'a str>,
}

impl<'a> EntryBuild<'a> {
    pub fn new() -> Self {
        Self {
            name: None,
            verdict: None,
            description: Vec::new(),
            sources: Vec::new(),
            review: None,
        }
    }
}

pub fn get_entries() -> Vec<Entry> {
    let file = include_str!("info.md");

    let mut entries = Vec::new();
    let mut entry_build = EntryBuild::new();

    macro_rules! push_entry {
        () => {
            entries.push(Entry {
                name: entry_build.name.expect("No name given!").to_string(),
                verdict: entry_build
                    .verdict
                    .expect("No verdict given, or is invalid!"),
                description: entry_build.description.join("\n"),
                sources: entry_build.sources.iter().map(|x| x.to_string()).collect(),
                review: entry_build.review.map(|x| x.to_string()),
            });
        };
    }

    let mut first_heading_occurred = false;
    for line in file.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (token, rest) = split_token(line);

        match token {
            "#" => {
                if first_heading_occurred {
                    push_entry!();
                }
                first_heading_occurred = true;
                entry_build = EntryBuild::new();
                entry_build.name = Some(rest);
            }

            "-" => {
                if entry_build.verdict.is_some() {
                    panic!("Verdict already given!");
                }
                entry_build.verdict = rest.try_into().ok();
            }

            token if Regex::new(r"^\d+\.$").unwrap().is_match(token) => {
                entry_build.sources.push(rest);
            }

            ">" => {
                if entry_build.review.is_some() {
                    panic!("Review already given!");
                }
                entry_build.review = Some(rest);
            }

            _ => entry_build.description.push(line),
        }
    }

    push_entry!();
    entries
}

fn split_token(line: &str) -> (&str, &str) {
    match line.find(' ') {
        Some(pos) => {
            let (a, b) = line.split_at(pos);
            (a.trim(), b.trim())
        }
        None => ("", line),
    }
}
