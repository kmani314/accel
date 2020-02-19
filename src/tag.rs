#[derive(Debug, Clone)]
pub struct Tag {
    pub expr: String,
    pub actions: Vec<String>,
    pub pattern: String, 
    pub end: bool,
}

impl Tag {
    pub fn new() -> Tag {
        Tag {
            expr: String::from(""),
            pattern: String::from(""),
            actions: Vec::new(),
            end: false,
        }
    }
}
