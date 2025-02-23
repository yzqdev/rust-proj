use std::error::Error;

use crate::html;
use crate::parser::Ast;

#[derive(Debug)]
pub struct Markdown<'markdown> {
    ast: Ast,
    path: Option<&'markdown str>,
    text: Option<&'markdown str>,
}

impl<'markdown> Default for Markdown<'markdown> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'markdown> Markdown<'markdown> {
    pub fn new() -> Self {
        Markdown {
            ast: Ast::new(),
            path: None,
            text: None,
        }
    }

    // Specify the path of a markdown file, then read the file and parse it
    pub fn path(&mut self, path: &'markdown str) -> &mut Self {
        self.path = Some(path);
        self
    }

    // Provide the content of a markdown file, then parse it directly
    pub fn text(&mut self, text: &'markdown str) -> &mut Self {
        self.text = Some(text);
        self
    }

    // Use 'f' function to convert markdown ast into a string, .e.g html document
    pub fn map_mut<F>(&mut self, f: F) -> Result<Vec<String>, Box<dyn Error>>
    where
        F: Fn(&Ast) -> Result<Vec<String>, Box<dyn Error>>,
    {
        self.parse()?;
        let s = f(&self.ast)?;
        Ok(s)
    }

    fn parse(&mut self) -> Result<&Self, Box<dyn Error>> {
        match self.text {
            Some(s) => self.ast.parse_string(s)?,
            None => match self.path {
                Some(p) => self.ast.parse_file(p)?,
                None => return Err("not found path or text to parse".into()),
            },
        }
        Ok(self)
    }
}

// Convert markdown ast into body part of the html and it contains toc
pub fn to_body_toc(ast: &Ast) -> Result<Vec<String>, Box<dyn Error>> {
    let body = ast.generate_content(&html::Generator::new(ast.ref_link_tags())?);
    let toc = ast.generate_toc(&html::Generator::new(ast.ref_link_tags())?);
    let v = vec![toc, body];
    Ok(v)
}

// Convert markdown ast into body part of the html
pub fn to_body(ast: &Ast) -> Result<Vec<String>, Box<dyn Error>> {
    let body = ast.generate_content(&html::Generator::new(ast.ref_link_tags())?);
    let v = vec![body];
    Ok(v)
}

// Generate the toc part of the html from markdown ast
pub fn to_toc(ast: &Ast) -> Result<Vec<String>, Box<dyn Error>> {
    let toc = ast.generate_toc(&html::Generator::new(ast.ref_link_tags())?);
    let v = vec![toc];
    Ok(v)
}

// Generate the slice of markdown
pub fn to_slice(ast: &Ast) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(ast.generate_slice(&html::Generator::new(ast.ref_link_tags())?))
}
