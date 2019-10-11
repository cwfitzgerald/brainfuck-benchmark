use bf_impl::*;
use glob::{glob_with, MatchOptions, Paths};
use std::fs::{canonicalize, copy, create_dir_all, read_dir, read_to_string, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

mod bf_impl;

/// Master trait for all implementations
trait BFImpl {
    /// Returns the name of the implementation
    fn name(&self) -> &'static str;

    /// Returns if it is an interpreter
    fn interpreted(&self) -> bool;

    /// Stage for fetching the source/binary from the sky
    fn get(&self);

    /// Stage for building the program itself
    fn build(&self);

    /// Stage for building any binaries with the program (for compilers, empty for interpreters)
    fn prepare(&self, file: PathBuf);

    /// Run the brainfuck!
    fn get_invoke_command(&self, file: PathBuf) -> String;

    /// Filter the output md file to use the proper program name
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

        run_command(
            Command::new("git")
                .args(&["pull", "--ff", "origin", "master"])
                .current_dir(folder),
        );
    } else {
        run_command(Command::new("git").args(&["clone", &url, &folder]));
    }
}

fn compile_c_code(output: &str, out_dir: &str, glob: String, options: &[String], cpp: bool) {
    let res = glob::glob(&glob).unwrap().map(|x| x.unwrap());

    let mut b = cc::Build::new()
        .files(res)
        .cpp(cpp)
        .debug(false)
        .opt_level(3)
        .out_dir(out_dir)
        .clone();
    for op in options {
        b.flag(op);
        c
    }
    b.compile(output);
}

fn main() {
    create_dir_all("build/src").unwrap();
    create_dir_all("build/out").unwrap();
    create_dir_all("results").unwrap();

    let mut bf: Vec<Box<dyn BFImpl + Send + Sync>> = vec![
        Box::new(WilfredBfcBfImpl),
        Box::new(CwfitzgeraldBfccBfImpl),
        Box::new(CwfitzgeraldBfccOldBfImpl),
        Box::new(DethraidBrainfuckBfImpl),
    ];
    bf.sort_unstable_by_key(|v| v.name());

    let mut benches: Vec<_> = read_dir("benches").unwrap().into_iter().collect();
    benches.sort_unstable_by_key(|v| v.as_ref().unwrap().file_name());

    for b in &bf {
        println!("Fetching {}", b.name());

        b.get();
    }

    for b in &bf {
        println!("Building {}", b.name());

        b.build();
    }

    let mut full_output = String::new();

    for bench in benches {
        let bench = bench.unwrap();
        let rel_path = bench.path().to_string_lossy().to_string();
        let full_path = canonicalize(bench.path()).unwrap();
        let file_name = full_path.file_name().unwrap().to_string_lossy().to_string();
        let file_stem = full_path.file_stem().unwrap().to_string_lossy().to_string();

        println!("==========================================");
        println!("Starting benchmark {}\n", rel_path);

        for b in &bf {
            if !b.interpreted() {
                println!("Compiling {} using {}", rel_path, b.name());
            }
            b.prepare(full_path.clone());
        }

        let result_md = format!("results/{}.md", file_stem);
        let extra = vec![
            "-s".into(),
            "full".into(),
            "-m".into(),
            "3".into(),
            "--export-markdown".into(),
            result_md.clone(),
        ];
        let v: Vec<String> = bf
            .iter()
            .map(|b| b.get_invoke_command(full_path.clone()))
            .chain(extra.into_iter())
            .collect();

        println!("Benchmarking...");

        run_command(Command::new("hyperfine").args(&v));

        let mut output_file = read_to_string(result_md).unwrap();

        for b in &bf {
            println!("Filtering output for {}", b.name());
            output_file = b.filter_output(output_file);
        }

        full_output += &format!("# {}\n{}", file_name, output_file);

        remove_dir_all("build/out").unwrap();

        println!("\nBenchmark finished!");
    }

    File::create("results/full.md")
        .unwrap()
        .write_all(full_output.as_bytes())
        .unwrap();
}
