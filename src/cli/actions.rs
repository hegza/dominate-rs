use super::super::phf;

#[derive(Debug, Clone)]
pub enum Keyword {
    Display, // Display currently loaded file
    Help, // Display help text
    Quit, // Exit program
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "display" => Keyword::Display,
    "help" => Keyword::Help,
    "quit" => Keyword::Quit,
};

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}
