use crate::*;
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone)]
pub enum RdebathTritiumMode {
    ArrayInterpreter,
    LightningJIT,
    DynASM,
}
impl Display for RdebathTritiumMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let v = match self {
            RdebathTritiumMode::ArrayInterpreter => "-r",
            RdebathTritiumMode::DynASM => "-q",
            RdebathTritiumMode::LightningJIT => "-j",
        };
        write!(f, "{}", v)
    }
}

#[derive(Clone)]
pub struct RdebathTritiumBfImpl(pub RdebathTritiumMode);

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("rdebath/Brainfuck/tritium");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/rdebath/Brainfuck/tree/master/tritium");
    /// Source URL
    static ref URL: String = String::from("https://github.com/rdebath/Brainfuck.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/rdebath/brainfuck");
    /// Folder in the out folder for temporaries.
    static ref OUT_FOLDER: String = String::from("build/out/rdebath/brainfuck");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/src/rdebath/brainfuck/tritium/bfi.out");
}

impl BFImpl for RdebathTritiumBfImpl {
    fn name(&self) -> String {
        let kind = match self.0 {
            RdebathTritiumMode::ArrayInterpreter => "Array Interpreter",
            RdebathTritiumMode::DynASM => "DynASM JIT",
            RdebathTritiumMode::LightningJIT => "GNU Lightning JIT",
        };
        format!("{} ({})", *NAME, kind)
    }

    fn interpreted(&self) -> bool {
        true
    }

    fn enabled(&self) -> bool {
        !windows()
    }

    fn get(&self) {
        git_repo(&URL, &SRC_DIR);
    }

    fn build(&self) {
        create_dir_all(&*OUT_FOLDER).unwrap();

        run_command(Command::new("make").args(&["-C", "build/src/rdebath/brainfuck/tritium"]));
    }

    fn prepare(&self, _file: PathBuf) {}

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} {} {}", &*RESULT_EXE, self.0, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}\\s*{}.*?`", &*RESULT_EXE, self.0)).unwrap();
        regex
            .replace(&contents, format!("[`{}`]({})", self.name(), &*WEBSITE).as_str())
            .into()
    }
}
