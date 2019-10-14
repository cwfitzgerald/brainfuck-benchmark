use crate::*;

#[derive(Clone)]
pub struct KotayBffsreeBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("kotay/bffsree");
    /// Source Website
    static ref WEBSITE: String = String::from("http://sree.kotay.com/2013/02/implementing-brainfuck.html");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/kotay/bffsree");
    /// Folder in the out folder for temporaries.
    static ref OUT_FOLDER: String = String::from("build/out/kotay/bffsree");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/out/kotay/bffsree/bffsree");
}

impl BFImpl for KotayBffsreeBfImpl {
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
        create_dir_all(&*OUT_FOLDER).unwrap();

        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                curl_file("http://www.kotay.com/sree/bf/bffsree_gcc.exe", &*RESULT_EXE);
            } else {
                curl_file("http://www.kotay.com/sree/bf/bffsree", &*RESULT_EXE);
            }
        }
    }

    fn build(&self) {}

    fn prepare(&self, _file: PathBuf) {}

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} {} || true", &*RESULT_EXE, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", &*RESULT_EXE)).unwrap();
        regex
            .replace(&contents, format!("[`{}`]({})", &*NAME, &*WEBSITE).as_str())
            .into()
    }
}
