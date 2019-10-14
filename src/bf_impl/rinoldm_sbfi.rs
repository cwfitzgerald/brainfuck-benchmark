use crate::*;

#[derive(Clone)]
pub struct RinoldmSbfiBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("rinoldm/sbfi");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/rinoldm/sbfi");
    /// Source URL
    static ref URL: String = String::from("https://github.com/rinoldm/sbfi.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/rinoldm/sbfi");
    /// Folder in the out folder for temporaries. Must be deleted after the preparation stage.
    static ref OUT_FOLDER: String = String::from("build/out/rinoldm/sbfi");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/out/rinoldm/sbfi/sbfi");
}

impl BFImpl for RinoldmSbfiBfImpl {
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

        create_cmake("sbfi", &*SRC_DIR, &format!("{}/*.c", &*SRC_DIR));
        build_cmake("sbfi", &*OUT_FOLDER, &*SRC_DIR);
    }

    fn prepare(&self, _file: PathBuf) {}

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} {}", &*RESULT_EXE, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", &*RESULT_EXE)).unwrap();
        regex
            .replace(&contents, format!("[`{}`]({})", &*NAME, &*WEBSITE).as_str())
            .into()
    }
}
