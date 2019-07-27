use buffer::BufferId;

#[derive(Clone, Debug)]
pub struct Config {
    pub default_visible_line_number: bool,
    pub syntax_highlight: syntax_highlight::SyntaxHighlight,
}
