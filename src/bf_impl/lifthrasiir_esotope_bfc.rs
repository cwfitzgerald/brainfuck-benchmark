use crate::*;

#[derive(Clone)]
pub struct LifthrasiirEsotopeBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("lifthrasiir/esotope-bfc");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/lifthrasiir/esotope-bfc");
    /// Source URL
    static ref URL: String = String::from("https://github.com/lifthrasiir/esotope-bfc.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/lifthrasiir/esotope-bfc");
    /// Folder in the out folder for temporaries. Must be deleted after the preparation stage.
    static ref OUT_FOLDER: String = String::from("build/out/lifthrasiir/esotope-bfc");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/out/lifthrasiir/esotope-bfc/esotope_bfc");
}

impl BFImpl for LifthrasiirEsotopeBfImpl {
    fn name(&self) -> String {
        NAME.clone()
    }

    fn interpreted(&self) -> bool {
        false
    }

    fn enabled(&self) -> bool {
        true
    }

    fn get(&self) {
        git_repo(&URL, &SRC_DIR);
    }

    fn build(&self) {}

    fn prepare(&self, file: PathBuf) {
        create_dir_all(&*OUT_FOLDER).unwrap();

        let esotope: PathBuf = path_dsl::path!((&*SRC_DIR) | "esotope-bfc").into();
        let esotope_str = esotope.to_string_lossy().to_string();

        let file_str = file.to_string_lossy().to_string();

        let output_str: String = path_dsl::path!((&*OUT_FOLDER) | "esotope-bfc.c")
            .to_string_lossy()
            .to_string();

        cfg_if::cfg_if!(
            if #[cfg(windows)] {
                let mut command = Command::new("py");
                command.args(&["-2", &esotope_str, &file_str]);
            } else {
                let mut command = Command::new("python");
                command.args(&[&esotope_str, &file_str]);
            }
        );

        run_command_with_pipe(&mut command, &output_str);

        create_cmake("esotope_bfc", &*OUT_FOLDER, &output_str);
        build_cmake("esotope_bfc", &*OUT_FOLDER, &*OUT_FOLDER);
    }

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} -i {}", &*RESULT_EXE, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", &*RESULT_EXE)).unwrap();
        regex
            .replace(&contents, format!("[`{}`]({})", &*NAME, &*WEBSITE).as_str())
            .into()
    }
}
