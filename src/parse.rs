use crate::entry::Entry;
use crate::tag::Tag;

// This would have been much better done with a proper parser
// That would have been overkill, since there is no nesting or any complicated features
// This is meant to work, not look nice

pub fn lex_string(input: &String) -> Vec<Entry>{
    let mut tokens = Vec::new();
    let mut this_str = String::from("");

    for i in input.chars() {
        match i {
            '<' | '>' | '/' => {
                if !this_str.is_empty() {
                    tokens.push(Entry::Item(this_str.clone()));
                    this_str.clear();
                }
                tokens.push(Entry::Bracket(i));
            },
            ' ' => {
                if !this_str.is_empty() {
                    tokens.push(Entry::Item(this_str.clone()));
                    this_str.clear();
                }
            }
            '\n' => continue,
            _ => this_str.push(i),
        }
    }
    if !this_str.is_empty() { tokens.push(Entry::Item(this_str)); }
    tokens
}

// I honestly don't care this literally just has to work
pub fn parse(input: &Vec<Entry>) -> Vec<Tag> {
    let mut tags = Vec::new();
    let mut this_tag: Tag = Tag::new();
    let mut is_tag = false;
    let mut has_expr = false;
    let mut this_pattern = String::from("");
    let mut has_pattern = false;

    for i in input {
        match i {
            Entry::Bracket(c) => {
                match c {
                    '<' => {
                        if is_tag { panic!("Unexpected bracket ({:?})", i) }
                        is_tag = true;
                        this_tag = Tag::new();
                    }
                    '/' => {
                        if !is_tag { panic!("This character cannot go here ({:?})", i) }
                        this_tag.end = true;
                    },
                    '>' => {
                        if !is_tag { panic!("Closing bracket without opening ({:?})", i) }
                        if !has_expr { panic!("No expression ({:?})", i) }
                        if !has_pattern { panic!("No pattern ({:?})", i) }
                        this_tag.pattern = this_pattern.clone();
                        this_pattern.clear();
                        tags.push(this_tag.clone());
                        has_pattern = false;
                        has_expr = false;
                        is_tag = false;
                    },
                    _ => panic!("Unknown entry ({:?})", c),
                }
            },
            Entry::Item(s) => {
                if is_tag && s.chars().nth(0).unwrap() == '!' {
                    if this_tag.end { panic!("End tags cannot have actions({:?})", i) }
                    this_tag.actions.push(s.to_string());
                } else if is_tag && !has_expr {
                    this_tag.expr = s.to_string();
                    has_expr = true;
                } else if is_tag {
                    if !has_pattern {
                        this_pattern = s.to_string();
                        has_pattern = true;
                    } else {
                        this_pattern = format!("{} {}", this_pattern, s.to_string());
                    }
                } else {
                    panic!("Unqualified-id ({:?})", i);
                }
            },
        }
    }
    
    if is_tag { panic!("Unexpected end of file") }
    tags
}
