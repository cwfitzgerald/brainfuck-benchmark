use crate::*;

#[derive(Clone)]
pub struct AsumagicAshbfBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("asumagic/ashbf");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/AsuMagic/AshBF");
    /// Source URL
    static ref URL: String = String::from("https://github.com/AsuMagic/AshBF.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/asumagic/ashbf");
    /// Folder in the out folder for temporaries.
    static ref OUT_FOLDER: String = String::from("build/out/asumagic/ashbf");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/out/asumagic/ashbf/ashbf");
}

impl BFImpl for AsumagicAshbfBfImpl {
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
        git_repo_branch(&URL, &SRC_DIR, "5b77debb34e81ad40904dac9b848fbf288a0fdd0");
    }

    fn build(&self) {
        create_dir_all(&*OUT_FOLDER).unwrap();

        build_cmake("ashbf", &*OUT_FOLDER, &*SRC_DIR);
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
