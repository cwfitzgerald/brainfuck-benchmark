use crate::*;
use std::path::PathBuf;

pub struct BfcBfImpl;

impl BFImpl for BfcBfImpl {
    fn get(&self) {
        println!("Fetching bfc");

        git_repo(
            String::from("https://github.com/Wilfred/bfc.git"),
            String::from("build/src/bfc/"),
        );
    }

    fn build(&self) {
        println!("Building bfc");

        run_command(Command::new("cargo").args(&["build", "--release"]).current_dir("build/src/bfc"));
    }

    fn prepare(&self, file: PathBuf) {
        let file_name = PathBuf::from(&file).file_stem().map(|v| v.to_os_string()).unwrap();
        println!("Building {} with bfc", file.to_string_lossy());

        create_dir_all("build/out/bfc").unwrap();

        run_command(Command::new("../../../build/src/bfc/target/release/bfc").args(&[&file]).current_dir("build/out/bfc"));

        let loc: PathBuf = path_dsl::path!("build" | "out" | "bfc" | file_name).into();
        copy(loc, "build/out/bfc.out");

        remove_dir_all("build/out/bfc").unwrap();
    }

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("build/out/bfc.out")
    }

    fn filter_output(&self, contents: String) -> String {
        contents.replace("`build/out/bfc.out`", "[`bfc`](https://github.com/Wilfred/bfc)")
    }
}