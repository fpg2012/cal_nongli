use std::process::Command;

fn run_git(args: &[&str]) -> Option<String> {
    let o = Command::new("git")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .ok()?;
    if o.status.success() {
        String::from_utf8(o.stdout)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    } else {
        None
    }
}

fn main() {
    // 不声明 rerun-if-changed：让 build.rs 每次构建都重跑，以便版本号实时反映 git 状态。
    // 版本号：优先 git 标签；无标签则用 commit 短 hash(可带 -dirty)。不在 git 里则退回 Cargo 版本号。
    let ver = run_git(&["describe", "--tags", "--always", "--dirty"])
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());
    let long = format!("{ver} (nth233 <mrnothing233@gmail.com>)");
    println!("cargo:rustc-env=LONG_VERSION={long}");
}
