use std::process::Command;

fn main() {
    // TODO: generated from `../zola` skipping `public`
    println!("cargo:rerun-if-changed=../zola/content");
    println!("cargo:rerun-if-changed=../zola/themes");
    println!("cargo:rerun-if-changed=../zola/config.toml");

    // need to fix CI somehow, maybe this?
    std::env::remove_var("RUSTFLAGS");
    for (k, v) in std::env::vars() {
        println!("{k}={v}");
    }

    run_command_in_dir(
        "cargo",
        &[
            "install",
            "--git",
            "https://github.com/getzola/zola.git",
            "--rev",
            "18245b323c041220851047f95eee2396b942abc9",
        ],
        ".",
    );
    run_command_in_dir("zola", &["build"], "../zola");
}

fn run_command_in_dir(command: &str, args: &[&str], dir: &str) -> String {
    let output = Command::new(command)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        panic!("command {command} {args:?} failed:\nstdout:\n{stdout}\nstderr:\n{stderr}")
    }
    stdout
}
