use crate::*;

#[derive(Clone)]
pub struct GardrekStvmBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("gardrek/stvm");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/gardrek/stvm");
    /// Source URL
    static ref URL: String = String::from("https://github.com/gardrek/stvm.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/gardrek/stvm");
    /// Folder in the out folder for temporaries.
    static ref OUT_FOLDER: String = String::from("build/out/gardrek/stvm");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/src/gardrek/stvm/target/release/main");
}

impl BFImpl for GardrekStvmBfImpl {
    fn name(&self) -> String {
        NAME.clone()
    }

    fn interpreted(&self) -> bool {
        true
    }

    fn enabled(&self) -> bool {
        true
    }

    fn get(&self) {
        git_repo(&URL, &SRC_DIR);
    }

    fn build(&self) {
        create_dir_all(&*OUT_FOLDER).unwrap();

        run_command(Command::new("cargo").args(&["build", "--release"]).current_dir(&*SRC_DIR));
    }

    fn prepare(&self, _file: PathBuf) {}

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} --bf {}", &*RESULT_EXE, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", &*RESULT_EXE)).unwrap();
        regex
            .replace(&contents, format!("[`{}`]({})", &*NAME, &*WEBSITE).as_str())
            .into()
    }
}
