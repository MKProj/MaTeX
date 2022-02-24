use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use crate::compiler::*;
use std::io::{Error, ErrorKind, Write};
use serde_derive::{Serialize, Deserialize};
use toml::{from_str, to_string_pretty};

const README: &str = r#"# Using Matex!!
 Matex is a simpler version of latex, and compiles down to latex, and uses a latex compiler to
 generate a pdf file. To set your latex compiler, edit the `Matex.toml` file. To find the Matex files,
 they are found inside the `src` directory while latex and pdf files are outputted inside the `out` directory.

 When ready to build run `matex build` :)

 Visit ... for documentation on matex.
"#;

const MAIN: &str = r#"documentclass: article
author: Author
title:  Title
date:   Today

document > begin
    maketitle;
    pagenumbering: arabic
    newpage;
    tableofcontents;
    newpage;
document > end
"#;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config{
    pub project_name: String,
    pub compiler: String,
}

impl Config {
    pub fn new(name: &str, compiler: Option<&str>) -> Self {
        let c = match compiler {
            Some(c) => c,
            None => "pdflatex"
        };
        Self {
            project_name: name.to_owned(),
            compiler: c.to_owned()
        }
    }
}

pub fn build(name: &str, compiler: &Option<String>) -> Result<(), Error>{
    let compiler = match compiler {
        Some(c) => c,
        None => "pdflatex"
    };
    let lines = read_lines(&format!("{}.matex", name))?;
    let mut latex = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let token = match Token::get_token(line) {
            Ok(t) => t,
            Err(m) => panic!("{}", m)
        };
        latex.push(token.to_latex());
    }
    let latex_str = latex.join("\n");
    let latex_path = format!("{}.tex", &name);
    let mut latex_file = std::fs::File::create(&latex_path)?;
    latex_file.write_all(latex_str.as_bytes())?;
    let cmd = std::process::Command::new(compiler).arg(&latex_path).spawn().unwrap();
    Ok(())
}

pub fn compile() -> Result<(), Error>{
    let config: Config = from_str(read_to_string("Matex.toml")?.as_str())?;
    let lines = read_lines("src/main.matex")?;
    let mut latex = Vec::new();
    for line in lines{
        let line = line.unwrap();
        let token = match Token::get_token(line){
            Ok(t) => t,
            Err(m) => panic!("{}", m)
        };
        latex.push(token.to_latex());
    }
    let latex_string = latex.join("\n\t");
    let file_path = format!("out/tex/{}.matex", config.project_name);
    let mut latex_file = std::fs::File::create(&file_path)?;
    latex_file.write_all(latex_string.as_bytes())?;
    let pdf_path = "out/pdf/";
    let cmd = std::process::Command::new("pdflatex").arg(&format!("--output-directory={}", pdf_path)).arg(file_path).spawn().unwrap();
    Ok(())
}


pub fn new_project(name: &str) -> Result<(),Error >{
    let path = name;
    if Path::new(path).exists(){
        return Err(Error::from(ErrorKind::AlreadyExists));
    }
    std::fs::create_dir(path)?;
    let src = format!("{}/src", path);
    let out = format!("{}/out", path);
    let tex = format!("{}/tex", &out);
    let pdf = format!("{}/pdf", &out);
    std::fs::create_dir(&src)?;
    std::fs::create_dir(&out)?;
    std::fs::create_dir(&tex)?;
    std::fs::create_dir(&pdf)?;

    let mut readme = std::fs::File::create(&format!("{}/README.md", path))?;
    let config = Config::new(name, None);
    let mut matex_toml = std::fs::File::create(&format!("{}/Matex.toml", path))?;
    readme.write_all(README.as_bytes())?;
    matex_toml.write_all(to_string_pretty(&config).unwrap().as_bytes())?;
    let mut main = std::fs::File::create(&format!("{}/main.matex", src))?;
    main.write_all(MAIN.as_bytes())?;
    Ok(())
}