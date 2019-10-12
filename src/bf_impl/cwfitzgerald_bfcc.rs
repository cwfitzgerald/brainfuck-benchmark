use crate::*;

#[derive(Clone)]
pub struct CwfitzgeraldBfccBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("cwfitzgerald/bfcc");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/cwfitzgerald/bfcc");
    /// Source URL
    static ref URL: String = String::from("https://github.com/cwfitzgerald/bfcc.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/cwfitzgerald/bfcc");
    /// Folder in the out folder for temporaries. Must be deleted after the preparation stage.
    static ref OUT_FOLDER: String = String::from("build/out/cwfitzgerald/bfcc");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/src/cwfitzgerald/bfcc/bfcc");
}

impl BFImpl for CwfitzgeraldBfccBfImpl {
    fn name(&self) -> &'static str {
        &NAME
    }

    fn interpreted(&self) -> bool {
        true
    }

    fn get(&self) {
        git_repo(URL.clone(), SRC_DIR.clone());
    }

    fn build(&self) {
        create_dir_all(&*OUT_FOLDER).unwrap();

        create_cmake("bfcc", &*SRC_DIR, &format!("{}/**/*.cpp", &*SRC_DIR));
        build_cmake("bfcc", &*OUT_FOLDER, &*SRC_DIR);

        remove_dir_all(&*OUT_FOLDER).unwrap();
    }

    fn prepare(&self, _file: PathBuf) {}

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
