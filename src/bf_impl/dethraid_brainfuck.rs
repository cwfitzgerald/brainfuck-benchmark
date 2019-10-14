use crate::*;

#[derive(Clone)]
pub struct DethraidBrainfuckBfImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("dethraid/brainfuck");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/dethraid/brainfuck");
    /// Source URL
    static ref URL: String = String::from("https://github.com/dethraid/brainfuck.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/dethraid/brainfuck");
    /// Folder in the out folder for temporaries. Must be deleted after the preparation stage.
    static ref OUT_FOLDER: String = String::from("build/out/dethraid/brainfuck");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/src/dethraid/brainfuck/bin/Release/bf");
}

impl BFImpl for DethraidBrainfuckBfImpl {
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
        #[cfg(not(windows))]
        {
            run_command(
                Command::new("premake5")
                    .args(&["gmake2"])
                    .current_dir(&*SRC_DIR),
            );
            run_command(Command::new("make").args(&[
                "config=release_linux",
                "all",
                "-C",
                &*SRC_DIR,
            ]));
        }
        #[cfg(windows)]
        {
            run_command(
                Command::new("premake5")
                    .args(&["vs2019"])
                    .current_dir(&*SRC_DIR),
            );
            run_command(
                Command::new("msbuild")
                    .args(&["/p:Configuration=Release", "/p:Platform=Win64"])
                    .current_dir(&*SRC_DIR),
            );
        }
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
