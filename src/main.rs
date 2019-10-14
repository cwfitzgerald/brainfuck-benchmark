#![feature(proc_macro_hygiene, stmt_expr_attributes)]

use bf_impl::*;
use indoc::indoc;
use itertools::Itertools;
use regex::Regex;
use std::env::current_dir;
use std::fs::{copy, create_dir_all, read_dir, read_to_string, remove_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use structopt::StructOpt;

mod bf_impl;

/// Master trait for all implementations
trait BFImpl {
    /// Returns the name of the implementation
    fn name(&self) -> String;

    /// Returns if it is an interpreter
    fn interpreted(&self) -> bool;

    /// Enabled on current platform
    fn enabled(&self) -> bool;

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
            String::from_utf8_lossy(&command_output.stdout),
            String::from_utf8_lossy(&command_output.stderr),
        );
        exit(1);
    }
}

fn run_command_with_pipe(c: &mut Command, output: &str) {
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
    } else {
        File::create(output)
            .unwrap()
            .write_all(&command_output.stdout)
            .unwrap();
    }
}

fn windows() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            return true;
        } else {
            return false;
        }
    }
}

fn git_repo(url: &str, folder: &str) {
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

fn git_repo_branch(url: &str, folder: &str, branch: &str) {
    let folder_path = PathBuf::from(&folder);
    if folder_path.exists() {
        assert_eq!(folder_path.is_dir(), true);

        run_command(
            Command::new("git")
                .args(&["checkout", branch])
                .current_dir(folder),
        );
    } else {
        run_command(Command::new("git").args(&["clone", url, folder, "--branch", branch]));
    }
}

fn curl_file(url: &str, dest: &str) {
    if !Path::new(dest).exists() {
        run_command(Command::new("curl").args(&["-L", url, "--output", dest]));
        std::fs::metadata(&dest).unwrap().permissions();
        cfg_if::cfg_if! {
            if #[cfg(not(windows))] {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&dest).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(dest, perms).unwrap();
            }
        };
    }
}

fn create_cmake(name: &str, src_dir: &str, glob: &str) {
    let files = glob::glob(glob).unwrap();
    let p = PathBuf::new();
    p.as_os_str().to_string_lossy().to_string();
    let files_str = files
        .into_iter()
        .map(|f| {
            let p = path_dsl::path!((current_dir().unwrap()) | (f.unwrap()))
                .as_os_str()
                .to_string_lossy()
                .to_string();

            format!("\"{}\"", p)
        })
        .join("\n    ");
    let cmake = format!(
        indoc!(
            "
        cmake_minimum_required(VERSION 3.12)
        project({0:} LANGUAGES C CXX)
        set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_BINARY_DIR}})
        add_executable(
            {0:}
            {1:}
        )"
        ),
        name, files_str
    );
    let cmake = cmake.replace("\\", "/");
    let path = path_dsl::path!(src_dir | "CMakeLists.txt");
    if !path.exists() {
        File::create(&path_dsl::path!(src_dir | "CMakeLists.txt"))
            .unwrap()
            .write_all(cmake.as_bytes())
            .unwrap();
    }
}

fn build_cmake(_name: &str, output_dir: &str, src_dir: &str) {
    run_command(Command::new("cmake").args(&[
        "-S",
        src_dir,
        "-B",
        output_dir,
        "-DCMAKE_BUILD_TYPE=Release",
    ]));
    run_command(Command::new("cmake").args(&["--build", &output_dir, "--config", "release"]));
    #[cfg(target_os = "windows")]
    {
        let exe = format!("{}.exe", _name);
        copy(
            &path_dsl::path!(output_dir | "Release" | &exe),
            &path_dsl::path!(output_dir | exe),
        )
        .unwrap();
    }
}

/// A program to benchmark various different brainfuck implementations,
#[derive(StructOpt)]
#[structopt(name = "brainfuck-benchmark")]
struct Options {
    /// Regex to select which implementations to run
    #[structopt(short, long = "impls")]
    impl_regex: Option<String>,

    /// Regex to select which implementations not to run
    #[structopt(long = "ignore-impls", alias = "ni")]
    negative_impl_regex: Option<String>,

    /// Regex to select which benchmarks to run
    #[structopt(short, long = "benches")]
    bench_regex: Option<String>,

    /// Regex to select which benchmarks to not run
    #[structopt(long = "ignore-benches", alias = "nb")]
    negative_bench_regex: Option<String>,

    /// Don't run interpreters
    #[structopt(long)]
    no_interpreters: bool,

    /// Don't run compilers
    #[structopt(long)]
    no_compilers: bool,

    /// Clean all temporary data and quit
    #[structopt(long)]
    clean: bool,
}

fn main() {
    let opt: Options = Options::from_args();

    if opt.clean {
        let _ = remove_dir_all("build");
        let _ = remove_dir_all("results");
        exit(0);
    }

    let impl_regex = opt.impl_regex.as_ref().map(|s| Regex::new(&s).unwrap());
    let negative_impl_regex = opt
        .negative_impl_regex
        .as_ref()
        .map(|s| Regex::new(&s).unwrap());
    let bench_regex = opt.bench_regex.as_ref().map(|s| Regex::new(&s).unwrap());
    let negative_bench_regex = opt
        .negative_bench_regex
        .as_ref()
        .map(|s| Regex::new(&s).unwrap());

    create_dir_all("build/src").unwrap();
    create_dir_all("build/out").unwrap();
    create_dir_all("results").unwrap();

    let mut bf: Vec<Box<dyn BFImpl + Send + Sync>> = vec![
        Box::new(ApankratBffBfImpl),
        Box::new(AsumagicAshbfBfImpl),
        Box::new(CwfitzgeraldBfccBfImpl),
        Box::new(CwfitzgeraldBfccOldBfImpl),
        Box::new(DethraidBrainfuckBfImpl),
        Box::new(GardrekStvmBfImpl),
        Box::new(KotayBffsreeBfImpl),
        Box::new(LifthrasiirEsotopeBfImpl),
        Box::new(RdebathTritiumBfImpl(RdebathTritiumMode::ArrayInterpreter)),
        Box::new(RdebathTritiumBfImpl(RdebathTritiumMode::DynASM)),
        Box::new(RdebathTritiumBfImpl(RdebathTritiumMode::LightningJIT)),
        Box::new(RinoldmSbfiBfImpl),
        Box::new(WilfredBfcBfImpl),
    ];
    bf.sort_unstable_by_key(|v| v.name());
    bf.retain(|v| {
        let enabled = v.enabled();
        let regex_enabled = match &impl_regex {
            Some(r) => r.is_match(&v.name()),
            None => true,
        };
        let negative_regex_enabled = match &negative_impl_regex {
            Some(r) => !r.is_match(&v.name()),
            None => true,
        };
        let type_enabled = match v.interpreted() {
            true => !opt.no_interpreters,
            false => !opt.no_compilers,
        };
        enabled && regex_enabled && negative_regex_enabled && type_enabled
    });
    bf.iter()
        .for_each(|v| println!("Implemenation: {}", v.name()));

    let mut benches: Vec<_> = read_dir("benches")
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect();
    benches.sort_unstable_by_key(|v| v.file_name());
    benches.retain(|b| {
        let regex_enabled = match &bench_regex {
            Some(r) => r.is_match(&b.file_name().to_string_lossy()),
            None => true,
        };
        let negative_regex_enabled = match &negative_bench_regex {
            Some(r) => !r.is_match(&b.file_name().to_string_lossy()),
            None => true,
        };
        regex_enabled && negative_regex_enabled
    });

    benches
        .iter()
        .for_each(|b| println!("Benchmark: {}", b.file_name().to_string_lossy()));

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
        let rel_path = bench.path().to_string_lossy().to_string();
        let full_path: PathBuf = path_dsl::path!((current_dir().unwrap()) | (bench.path())).into();
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
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                let extra = vec![
                    "--show-output".into(),
                    "-m".into(),
                    "3".into(),
                    "--export-markdown".into(),
                    result_md.clone(),
                    "--shell".into(),
                    "powershell".into()
                ];
             }
             else {
                let extra = vec![
                    "--show-output".into(),
                    "-m".into(),
                    "3".into(),
                    "--export-markdown".into(),
                    result_md.clone(),
                ];
             }
        }
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

        println!("\nBenchmark finished!");
    }

    File::create("results/full.md")
        .unwrap()
        .write_all(full_output.as_bytes())
        .unwrap();
}
