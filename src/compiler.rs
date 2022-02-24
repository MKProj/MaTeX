use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};



#[derive(Debug, Clone)]
pub enum Token{
    Element(Element),
    Literal(Literal),
    BeginEnv(BeginEnv),
    EndEnv(EndEnv),
    Comment(Comment)
}

impl Token{
    pub fn get_token(pattern: String) -> Result<Token, String>{
        let pattern = remove_whitespace(&pattern);
        if pattern.contains(":"){
            let p = pattern.split_at(pattern.find(":").unwrap());
            let name = p.0;
            let mut value = p.1.to_owned();
            let _c = value.remove(value.find(":").unwrap());
            return Ok(Token::Element(Element(name.to_owned(), value)));
        }
        else if pattern.contains(";") {
            let mut pattern = pattern;
            let _p = pattern.remove(pattern.find(";").unwrap());
            return Ok(Token::Literal(Literal(pattern)));
        }
        else if pattern.starts_with("%")  || pattern.is_empty(){
            return Ok(Token::Comment(Comment(pattern)));
        }
        else if pattern.contains(">"){
            let p = pattern.split_at(pattern.find(">").unwrap());
            let mut pos = p.1.to_owned();
            let _c = pos.remove(pos.find(">").unwrap());
            match pos.as_str(){
                "begin" => {
                    return Ok(Token::BeginEnv(BeginEnv(p.0.to_owned())))
                }
                "end" => {
                    return Ok(Token::EndEnv(EndEnv(p.0.to_owned())))
                }
                _ => return Err(format!("Invalid envrionment {}", &pattern))
            }
        }
        else {
            Err(format!("Invalid token: {}", &pattern))
        }
    }
}

pub type LineResult = io::Result<io::Lines<io::BufReader<File>>>;

#[derive(Debug, Clone)]
pub struct Element(String, String);
#[derive(Debug, Clone)]
pub struct Literal(String);
#[derive(Debug, Clone)]
pub struct BeginEnv(String);
#[derive(Debug, Clone)]
pub struct EndEnv(String);
#[derive(Debug, Clone)]
pub struct Comment(String);

pub fn get_input(t:&Token) -> Option<LineResult>{
    let mut result = None;
    match t{
        Token::Element(e) => {
            if &e.0 == "input"{
                let path = format!("{}.matex", &e.1);
                result = Some(read_lines(&path));
            }
        }
        _ => result = None
    }
    result
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn read_lines<P>(filename: P) -> LineResult
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub trait Latex{
    fn to_latex(&self) -> String;
}
// Implementations of Latex
impl Latex for Element{
    fn to_latex(&self) -> String {
        let name = if &self.0 == "import"{
            "usepackage"
        } else {
            &self.0
        };
        let value = &self.1;
        format!("\\{name}{{{value}}}")
    }
}
impl Latex for Literal{
    fn to_latex(&self) -> String {
        let name = &self.0;
        format!("\\{name}")
    }
}

impl Latex for Comment{
    fn to_latex(&self) -> String {
        self.0.to_owned()
    }
}

impl Latex for BeginEnv{
    fn to_latex(&self) -> String {
        let name = &self.0;
        format!("\\begin{{{name}}}")
    }
}

impl Latex for EndEnv{
    fn to_latex(&self) -> String {
        let name = &self.0;
        format!("\\end{{{name}}}")
    }
}

impl Latex for Token{
    fn to_latex(&self) -> String {
        match &self{
            Token::Element(e) => {e.to_latex()}
            Token::Literal(l) => {l.to_latex()}
            Token::BeginEnv(b) => {b.to_latex()}
            Token::EndEnv(e) => {e.to_latex()}
            Token::Comment(c) => {c.to_latex()}
        }
    }
}