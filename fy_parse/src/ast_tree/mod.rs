pub mod template;
pub mod script;

#[derive(Debug)]
pub struct Block<'a> {
    content: &'a str,
}

impl<'a> From<&'a str> for Block<'a> {
    fn from(content: &'a str) -> Self {
        Self {
            content,
        }
    }
}