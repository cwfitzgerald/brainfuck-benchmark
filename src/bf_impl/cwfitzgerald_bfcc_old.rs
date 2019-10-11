use crate::*;

#[derive(Clone)]
pub struct CwfitzgeraldBfccOldBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("cwfitzgerald/bfcc-old");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/cwfitzgerald/bfcc-old");
    /// Source URL
    static ref URL: String = String::from("https://github.com/cwfitzgerald/bfcc-old.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/cwfitzgerald/bfcc-old");
    /// Folder in the out folder for temporaries. Must be deleted after the preparation stage.
    static ref OUT_FOLDER: String = String::from("build/out/cwfitzgerald/bfcc-old");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/src/cwfitzgerald/bfcc-old/bfcc");
}

impl BFImpl for CwfitzgeraldBfccOldBfImpl {
    fn name(&self) -> &'static str {
        &NAME
    }

    fn interpreted(&self) -> bool {
        true
    }

    fn get(&self) {
        git_repo(
            URL.clone(),
            SRC_DIR.clone(),
        );
    }

    fn build(&self) {
        create_dir_all(path_dsl::path!((&*SRC_DIR) | "bin").as_path()).unwrap();

        run_command(Command::new("make").args(&["-C", &*SRC_DIR]));
    }

    fn prepare(&self, _file: PathBuf) {
    }

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} -i {}", &*RESULT_EXE, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", &*RESULT_EXE)).unwrap();
        regex.replace(&contents, format!("[`{}`]({})", &*NAME, &*WEBSITE).as_str()).into()
    }
}