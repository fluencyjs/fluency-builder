use super::Block;

impl Block<'_> {
    pub fn to_html_ast(&self) {
        println!("{}", self.content)
    }
}