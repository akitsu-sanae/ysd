use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;

use util::Rgb;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub line_comment_mark: Option<String>,
    pub multi_comment_mark: Option<(String, String)>,
    pub color: Rgb,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub keywords: HashSet<String>,
    pub color: Rgb,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyntaxHighlight {
    pub comment: Comment,
    pub keyword: Keyword,
}

impl Default for SyntaxHighlight {
    fn default() -> Self {
        SyntaxHighlight {
            comment: Comment {
                line_comment_mark: None,
                multi_comment_mark: None,
                color: Rgb(0, 0, 0),
            },
            keyword: Keyword {
                keywords: HashSet::new(),
                color: Rgb(0, 0, 0),
            },
        }
    }
}
