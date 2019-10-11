use crate::*;
use std::path::PathBuf;

#[derive(Clone)]
pub struct WilfredBfcBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("wilfried/bfc");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/Wilfred/bfc");
    /// Source URL
    static ref URL: String = String::from("https://github.com/Wilfred/bfc.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/wilfred/bfc");
    /// Folder in the out folder for temporaries. Must be deleted after the preparation stage.
    static ref OUT_FOLDER: String = String::from("build/out/wilfred/bfc");
    /// Actual EXE produced.
    static ref RESULT_EXE: String = String::from("build/out/wilfred_bfc.out");
}

impl BFImpl for WilfredBfcBfImpl {
    fn name(&self) -> &'static str {
        &*NAME
    }

    fn interpreted(&self) -> bool {
        false
    }

    fn get(&self) {
        git_repo(
            URL.clone(),
            SRC_DIR.clone(),
        );
    }

    fn build(&self) {
        run_command(Command::new("cargo").args(&["build", "--release"]).current_dir(&*SRC_DIR));
    }

    fn prepare(&self, file: PathBuf) {
        let file_name = PathBuf::from(&file).file_stem().unwrap().to_os_string();

        create_dir_all(&*OUT_FOLDER).unwrap();

        run_command(Command::new("../../../../build/src/wilfred/bfc/target/release/bfc").args(&[&file]).current_dir(&*OUT_FOLDER));

        let exe_location: PathBuf = path_dsl::path!((&*OUT_FOLDER) | file_name).into();
        copy(exe_location, &*RESULT_EXE).unwrap();

        remove_dir_all(&*OUT_FOLDER).unwrap();
    }

    fn get_invoke_command(&self, _file: PathBuf) -> String {
        RESULT_EXE.clone()
    }

    fn filter_output(&self, contents: String) -> String {
        contents.replace(&format!("`{}`", &*RESULT_EXE), &format!("[`{}`]({})", &*NAME, &*WEBSITE))
    }
}