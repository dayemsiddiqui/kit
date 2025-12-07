use console::style;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

struct ProcessManager {
    children: Vec<Child>,
    shutdown: Arc<AtomicBool>,
}

impl ProcessManager {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    fn spawn_with_prefix(
        &mut self,
        command: &str,
        args: &[&str],
        cwd: Option<&Path>,
        prefix: &str,
        color: console::Color,
    ) -> Result<(), String> {
        let mut cmd = Command::new(command);
        cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", command, e))?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let shutdown_stdout = self.shutdown.clone();
        let shutdown_stderr = self.shutdown.clone();

        let prefix_out = prefix.to_string();
        let prefix_err = prefix.to_string();

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if shutdown_stdout.load(Ordering::SeqCst) {
                    break;
                }
                if let Ok(line) = line {
                    println!("{} {}", style(&prefix_out).fg(color).bold(), line);
                }
            }
        });

        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if shutdown_stderr.load(Ordering::SeqCst) {
                    break;
                }
                if let Ok(line) = line {
                    eprintln!("{} {}", style(&prefix_err).fg(color).bold(), line);
                }
            }
        });

        self.children.push(child);
        Ok(())
    }

    fn shutdown_all(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        for child in &mut self.children {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    fn any_exited(&mut self) -> bool {
        for child in &mut self.children {
            if let Ok(Some(_)) = child.try_wait() {
                return true;
            }
        }
        false
    }
}

fn validate_kit_project(backend_only: bool, frontend_only: bool) -> Result<(), String> {
    let cargo_toml = Path::new("Cargo.toml");
    let frontend_dir = Path::new("frontend");

    if !frontend_only && !cargo_toml.exists() {
        return Err("No Cargo.toml found. Are you in a Kit project directory?".into());
    }

    if !backend_only && !frontend_dir.exists() {
        return Err("No frontend directory found. Are you in a Kit project directory?".into());
    }

    Ok(())
}

fn ensure_cargo_watch() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["watch", "--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => {
            println!(
                "{}",
                style("cargo-watch not found. Installing...").yellow()
            );
            let install = Command::new("cargo")
                .args(["install", "cargo-watch"])
                .status()
                .map_err(|e| format!("Failed to install cargo-watch: {}", e))?;

            if !install.success() {
                return Err("Failed to install cargo-watch".into());
            }
            println!("{}", style("cargo-watch installed successfully.").green());
            Ok(())
        }
    }
}

fn ensure_npm_dependencies() -> Result<(), String> {
    let frontend_path = Path::new("frontend");
    let node_modules = frontend_path.join("node_modules");

    if !node_modules.exists() {
        println!(
            "{}",
            style("Installing frontend dependencies...").yellow()
        );
        let npm_install = Command::new("npm")
            .args(["install"])
            .current_dir(frontend_path)
            .status()
            .map_err(|e| format!("Failed to run npm install: {}", e))?;

        if !npm_install.success() {
            return Err("Failed to install npm dependencies".into());
        }
        println!(
            "{}",
            style("Frontend dependencies installed successfully.").green()
        );
    }

    Ok(())
}

pub fn run(port: u16, frontend_port: u16, backend_only: bool, frontend_only: bool) {
    println!();
    println!(
        "{}",
        style("Starting Kit development servers...").cyan().bold()
    );
    println!();

    // Validate project
    if let Err(e) = validate_kit_project(backend_only, frontend_only) {
        eprintln!("{} {}", style("Error:").red().bold(), e);
        std::process::exit(1);
    }

    // Ensure cargo-watch is installed (only if running backend)
    if !frontend_only {
        if let Err(e) = ensure_cargo_watch() {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            std::process::exit(1);
        }
    }

    // Ensure npm dependencies are installed (only if running frontend)
    if !backend_only {
        if let Err(e) = ensure_npm_dependencies() {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            std::process::exit(1);
        }
    }

    let mut manager = ProcessManager::new();
    let shutdown = manager.shutdown.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        println!();
        println!("{}", style("Shutting down servers...").yellow());
        shutdown.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Start backend with cargo-watch
    if !frontend_only {
        println!(
            "{} Backend server on http://127.0.0.1:{}",
            style("[backend]").magenta().bold(),
            port
        );

        // TODO: Pass port via KIT_PORT environment variable when supported
        let _ = port; // Port configuration to be implemented
        if let Err(e) = manager.spawn_with_prefix(
            "cargo",
            &["watch", "-x", "run"],
            None,
            "[backend] ",
            console::Color::Magenta,
        ) {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            std::process::exit(1);
        }
    }

    // Start frontend with npm/vite
    if !backend_only {
        println!(
            "{} Frontend server on http://127.0.0.1:{}",
            style("[frontend]").cyan().bold(),
            frontend_port
        );

        // TODO: Pass frontend_port via VITE_PORT when supported
        let _ = frontend_port; // Frontend port configuration to be implemented
        let frontend_path = Path::new("frontend");

        if let Err(e) = manager.spawn_with_prefix(
            "npm",
            &["run", "dev"],
            Some(frontend_path),
            "[frontend]",
            console::Color::Cyan,
        ) {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            manager.shutdown_all();
            std::process::exit(1);
        }
    }

    println!();
    println!("{}", style("Press Ctrl+C to stop all servers").dim());
    println!();

    // Wait for shutdown signal or process exit
    while !manager.shutdown.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_millis(100));

        // Check if any child process has exited
        if manager.any_exited() {
            manager.shutdown.store(true, Ordering::SeqCst);
            break;
        }
    }

    manager.shutdown_all();
    println!("{}", style("Servers stopped.").green());
}
