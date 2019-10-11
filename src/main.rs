use std::path::PathBuf;
use std::process::{Command, exit};
use std::fs::{create_dir_all, copy, remove_dir_all, read_dir, canonicalize, read_to_string};

mod bf_impl;

/// Master trait for all implementations
trait BFImpl {
    fn get(&self);

    fn build(&self);

    fn prepare(&self, file: PathBuf);

    fn get_invoke_command(&self, file: PathBuf) -> String;

    fn filter_output(&self, contents: String) -> String;
}

fn run_outputted_command(c: &mut Command) {
    if !c.status().unwrap().success() {
        exit(1);
    }
}

fn run_command(c: &mut Command) {
    let command_output = c.output().unwrap();

    if !command_output.status.success() {
        println!(
            "Command {:#?} output {}:\n\nSTDOUT:\n{}\n\nSTDERR:\n{}",
            c,
            command_output.status.code().unwrap(),
            String::from_utf8(command_output.stdout).unwrap(),
            String::from_utf8(command_output.stderr).unwrap()
        );
        exit(1);
    }
}

fn git_repo(url: String, folder: String) {
    let folder_path = PathBuf::from(&folder);
    if folder_path.exists() {
        assert_eq!(folder_path.is_dir(), true);

        run_command(Command::new("git")
            .args(&["pull", "--ff", "origin", "master"])
            .current_dir(folder));
    } else {
        run_command(Command::new("git").args(&["clone", &url, &folder]));
    }
}

fn main() {
    create_dir_all("build/src").unwrap();
    create_dir_all("build/out").unwrap();
    create_dir_all("results").unwrap();

    let bf: Vec<Box<dyn BFImpl>> = vec![Box::new(bf_impl::bfc::BfcBfImpl)];
    let mut benches: Vec<_> = read_dir("benches").unwrap().into_iter().collect();
    benches.sort_unstable_by_key(|v| v.as_ref().unwrap().file_name());

    for b in &bf {
        b.get();
    }

    for b in &bf {
        b.build();
    }

    let mut full_output = String::new();

    for bench in benches {
        let bench = bench.unwrap();
        let full_path = canonicalize(bench.path()).unwrap();
        let file_stem = full_path.file_name().unwrap().to_string_lossy().to_string();

        println!("==========================================");
        println!("Starting benchmark {}", bench.file_name().to_string_lossy());

        for b in &bf {
            b.prepare(full_path.clone());
        }

        let result_md = format!("results/{}.md", file_stem);
        let extra = vec!["-s".into(), "full".into(), "--export-markdown".into(), result_md.clone()];
        let v: Vec<String> = bf.iter().map(|b| b.get_invoke_command(full_path.clone())).chain(extra.into_iter()).collect();

        run_outputted_command(Command::new("hyperfine").args(&v));

        let mut output_file =  read_to_string(result_md).unwrap();

        for b in &bf {
            output_file = b.filter_output(output_file);
        }

        full_output += &format!("# {}\n{}", file_stem, output_file);

        remove_dir_all("build/out");

        println!("Benchmark finished!");
    }
}
