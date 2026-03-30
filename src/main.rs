use std::process::Command;

fn usage() {
    eprintln!("Usage: ds-algo <problem> [args...]");
    eprintln!("Examples:");
    eprintln!("  ds-algo jump-game");
    eprintln!("  ds-algo jump-game.rs");
    eprintln!("  ds-algo leetcode/jump-game.rs");
    eprintln!("  ds-algo crates/leetcode/src/bin/jump-game.rs");
}

fn normalize_problem_name(input: &str) -> Option<String> {
    let mut name = input.trim();

    for prefix in ["crates/leetcode/src/bin/"] {
        if let Some(stripped) = name.strip_prefix(prefix) {
            name = stripped;
        }
    }

    if let Some(stripped) = name.strip_suffix(".rs") {
        name = stripped;
    }

    if name.is_empty() || name.contains('/') {
        return None;
    }

    Some(name.to_string())
}

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(problem_input) = args.next() else {
        usage();
        std::process::exit(1);
    };

    let Some(problem) = normalize_problem_name(&problem_input) else {
        eprintln!("Invalid problem name: {problem_input}");
        usage();
        std::process::exit(1);
    };

    let run_status = Command::new("cargo")
        .arg("run")
        .arg("-q")
        .arg("-p")
        .arg("leetcode")
        .arg("--bin")
        .arg(problem)
        .arg("--")
        .args(args)
        .status()
        .unwrap_or_else(|err| {
            eprintln!("Failed to invoke cargo: {err}");
            std::process::exit(1);
        });

    std::process::exit(run_status.code().unwrap_or(1));
}
