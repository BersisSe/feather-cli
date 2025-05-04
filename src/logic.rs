use notify::{recommended_watcher, Event, EventKind, Result, Watcher};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;

use crate::constants;

pub fn create_new_project(name: &str) {
    let project_path = Path::new(name);
    if project_path.exists() {
        println!("Project directory already exists. Please choose a different name.");
        return;
    }
    let output = Command::new("cargo")
        .args(&[&"new", "--bin", name])
        .output()
        .expect("Failed to run cargo new");
    if !output.status.success() {
        eprintln!("❌ Failed to create project.");
        return;
    }
    let deps = Command::new("cargo")
        .current_dir(&project_path)
        .args(&["add", "feather"])
        .output()
        .expect("Failed to add Dependencies");

    if !deps.status.success() {
        eprintln!("❌ Failed to add feather dependency.");
        println!("{}", String::from_utf8_lossy(&deps.stderr));
        return;
    }
    // 4. Replace main.rs
    let main_rs = constants::MAIN;
    fs::write(project_path.join("src").join("main.rs"), main_rs).unwrap();
    println!("✅ Project '{}' created successfully!", name);
    println!("Run `cd {}` to navigate to your project directory.", name);
    println!("Run `feather-cli dev` to start hot-reloading dev server.");
    println!("Run `feather-cli build` to build a optimized executable.");
}

pub fn start_dev_server() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new("./src"), notify::RecursiveMode::Recursive)?;

    let mut c = Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("Error While Running Cargo Run");

    for res in rx {
        match res {
            Ok(event) => {
                if let EventKind::Modify(_) = event.kind {
                    // Kill the current process
                    c.kill().expect("Error While Killing Cargo Run");
                    c.wait().expect("Error While Waiting for Cargo Run to Terminate");

                    // Restart the process
                    c = Command::new("cargo")
                        .arg("run")
                        .spawn()
                        .expect("Error While Running Cargo Run");
                }
            }
            Err(e) => {
                eprintln!("Error watching files: {:?}", e);
            }
        }
    }

    Ok(())
}


pub fn build_project() -> Result<()> {
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(".")
        .output()
        .expect("Failed to run cargo build");
    if !output.status.success() {
        eprintln!("❌ Failed to build project.");
        return Ok(());
    }
    println!("✅ Project built successfully!");
    Ok(())
}

