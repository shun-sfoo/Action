use clap::Parser;
use std::{fs, process::Command};

static NVIM_PACK_PATH: &str = "pack/plugins/opt";
static COMMON_PACK_PATH: &str = ".";
static GITHUB_URL: &str = "https://github.com";
static NVIM_MODULES: &str = "modules";
static NVIM_SUFFIX: &str = ".nvim";

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Install,
    Update,
    List,
    Add(AddArgs),
    Remove(RemoveArgs),
}

#[derive(clap::Args)]
struct AddArgs {
    #[arg(short)]
    url: String,
    #[arg(short)]
    pack: Option<String>,
    #[arg(short)]
    branch: Option<String>,
}

#[derive(clap::Args)]
struct RemoveArgs {
    #[arg(short)]
    name: String,
    #[arg(short)]
    pack: Option<String>,
}

struct PlugInfo {
    name: String,
    plug: String,
}

impl PlugInfo {
    fn new(name: String) -> Self {
        let plug = name.clone();
        let info: Vec<&str> = name.split('/').collect();
        assert!(info.len() == 2, "input corrent format 'auther/plug'");
        let plug_name;
        if info[1].ends_with(NVIM_SUFFIX) {
            plug_name = &info[1][0..(info[1].len() - 5)];
        } else {
            plug_name = &info[1]
        };

        PlugInfo {
            name: String::from(plug_name),
            plug,
        }
    }
}

fn execute_message(err_vec: &Vec<u8>, info_vec: &Vec<u8>) {
    let err = String::from_utf8_lossy(err_vec);
    let err_mes = err.trim();
    println!("\x1b[31mERROR:\x1b[0m");
    let err_list: Vec<&str> = err_mes.split('\n').collect();
    for e in err_list.iter() {
        if e.is_empty() {
            continue;
        }
        println!("\x1b[31m{}\x1b[0m", e);
    }

    let info = String::from_utf8_lossy(info_vec);
    let info_mes = info.trim();
    println!("\x1b[32mINFO:\x1b[0m");
    let info_list: Vec<&str> = info_mes.split('\n').collect();
    for e in info_list.iter() {
        if e.is_empty() {
            continue;
        }
        println!("\x1b[32m{}\x1b[0m", e);
    }
}

impl Action {
    fn execute(self) {
        match self {
            Action::Install => {
                let mut cmd = Command::new("git");
                cmd.args([
                    "submodule",
                    "update",
                    "--init",
                    "--depth",
                    "1",
                    "--recursive",
                ]);
                println!("{:?}", cmd);
                let out = cmd.output().expect("install submodule failed");
                execute_message(&out.stderr, &out.stdout);
            }
            Action::Update => {
                let mut cmd = Command::new("git");
                cmd.args([
                    "submodule",
                    "update",
                    "--remote",
                    "--init",
                    "--depth",
                    "1",
                    "--recursive",
                ]);
                println!("{:?}", cmd);
                let out = cmd.output().expect("update submodule failed");
                execute_message(&out.stderr, &out.stdout);
            }
            Action::Add(args) => {
                let pack_path = match args.pack {
                    Some(p) => p,
                    None if is_nvim() => NVIM_PACK_PATH.to_string(),
                    None => COMMON_PACK_PATH.to_string(),
                };

                let info = PlugInfo::new(args.url);
                let mut cmd = Command::new("git");
                cmd.args([
                    "submodule",
                    "add",
                    "--name",
                    &String::from(info.name.clone()),
                ]);
                if let Some(b) = args.branch {
                    cmd.args(["-b", &b]);
                }
                cmd.args([
                    "--depth",
                    "1",
                    &format!("{}/{}", GITHUB_URL, info.plug),
                    &format!("{}/{}", &pack_path, info.name),
                ]);

                println!("{:?}", cmd);
                let out = cmd.output().expect("add submodule failed");
                execute_message(&out.stderr, &out.stdout);
            }
            Action::Remove(args) => {
                let pack_path = match args.pack {
                    Some(p) => p,
                    None if is_nvim() => NVIM_PACK_PATH.to_string(),
                    None => COMMON_PACK_PATH.to_string(),
                };

                // Unregister submodule (this also empties plugin's directory)
                // git submodule deinit -f $submodule_path
                let path = format!("{}/{}", &pack_path, args.name);
                let mut unregister_cmd = Command::new("git");
                unregister_cmd.args(["submodule", "deinit", "-f", &path]);
                println!("{:?}", unregister_cmd);

                let out = unregister_cmd.output().expect("Unregister failed");
                execute_message(&out.stderr, &out.stdout);

                // Remove the working tree of the submodule
                //git rm --cached $submodule_path
                let mut tree_remove_cmd = Command::new("git");
                tree_remove_cmd.args(["rm", "--cached", &path]);
                println!("{:?}", tree_remove_cmd);
                let out = tree_remove_cmd
                    .output()
                    .expect("remove working tree failed");
                execute_message(&out.stderr, &out.stdout);

                // git config -f .gitmodules --remove-section "submodule.$submodule_name"
                let mut section_remove_cmd = Command::new("git");
                section_remove_cmd.args([
                    "config",
                    "-f",
                    ".gitmodules",
                    "--remove-section",
                    &format!("submodule.{}", &args.name),
                ]);

                println!("{:?}", section_remove_cmd);
                let out = section_remove_cmd.output().expect("remove-section failed");
                execute_message(&out.stderr, &out.stdout);

                // Remove submodule's (which should be empty) directory from file system
                let mut dir_remove_cmd = Command::new("rm");
                dir_remove_cmd.args(["-r", &path]);

                println!("{:?}", dir_remove_cmd);
                let out = dir_remove_cmd.output().expect("remove directory failed");
                execute_message(&out.stderr, &out.stdout);

                // Remove associated submodule directory in '.git/modules'.
                // rm -rf $git_dir/modules/$submodule_path
                let mut dir_cmd = Command::new("git");
                dir_cmd.args(["rev-parse", "--git-dir"]);
                let mut git_path = String::from_utf8_lossy(
                    &dir_cmd
                        .output()
                        .expect("can't transform git rev-parse into utf8 string")
                        .stdout,
                )
                .to_string();
                git_path.truncate(git_path.len() - 1);

                let mut associated_remove_cmd = Command::new("rm");
                associated_remove_cmd.args([
                    "-rf",
                    &format!("{}/{}/{}", git_path, NVIM_MODULES, &args.name),
                ]);

                println!("{:?}", associated_remove_cmd);
                let out = associated_remove_cmd
                    .output()
                    .expect("associated remove failed");
                execute_message(&out.stderr, &out.stdout);
            }
            Action::List => {
                //git config --file .gitmodules --name-only --get-regexp path
                let mut cmd = Command::new("git");
                cmd.args([
                    "config",
                    "--file",
                    ".gitmodules",
                    "--name-only",
                    "--get-regexp",
                    "path",
                ]);
                println!("{:?}", cmd);
                let out = cmd.output().expect("list submodule failed").stdout;
                let out = String::from_utf8_lossy(&out).to_string();
                let out = out.trim();
                let outs: Vec<&str> = out.split("\n").collect();
                for m in outs {
                    let v: Vec<&str> = m.split(".").collect();
                    println!("\x1b[31m{}\x1b[0m", v.get(1).unwrap());
                }
            }
        }
    }
}

fn check_git() {
    let mut is_git = false;
    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        if dir.path().to_string_lossy().ends_with(".git") {
            is_git = true;
        }
    }

    if !is_git {
        panic!("it's not a git directory");
    }
}

fn is_nvim() -> bool {
    let mut is_git = false;
    let mut is_nvim = false;
    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        if dir.path().to_string_lossy().ends_with(".git") {
            is_git = true;
        }
        if dir.path().to_string_lossy().ends_with("init.lua") {
            is_nvim = true;
        }
    }
    is_git && is_nvim
}

fn main() {
    check_git();
    let args = Args::parse();
    args.action.execute();
}
