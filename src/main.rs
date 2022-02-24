mod utils;
mod compiler;
use compiler::*;
use utils::*;
use std::io::Write;
use structopt::StructOpt;
use std::io::Error;
#[derive(StructOpt)]
#[structopt(about="MaTeX Compiler, a mate to LaTeX")]
pub enum CLI{
    #[structopt(about="Create a new MaTeX Project")]
    New{
        name: String
    },
    #[structopt(about="Build a MaTeX file to pdf using the file name")]
    Build{
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        compiler: Option<String>
    },
    #[structopt(about="Compiles a MaTeX project")]
    Compile,
}



fn main() -> Result<(), Error>{
    let cli: CLI = CLI::from_args();
    match cli{
        CLI::New { name } => {
            new_project(&name)?
        }
        CLI::Build {name, compiler } => {
            build(&name, &compiler)?
        }
        CLI::Compile => {
            compile()?
        }
    }
    Ok(())
}
