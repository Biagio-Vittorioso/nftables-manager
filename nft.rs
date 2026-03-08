use std::process::Command;
use serde_json::Value;

pub fn get_ruleset() -> Result<Value, String> {
    let output = Command::new("nft")
        .arg("-j")
        .arg("list")
        .arg("ruleset")
        .output()
        .map_err(|e| e.to_string())?;

    serde_json::from_slice(&output.stdout)
        .map_err(|e| e.to_string())
}

pub fn add_rule(proto: &str, src: &str, dst: &str, port: &str, action: &str) {
    let mut cmd = Command::new("pkexec");
    cmd.arg("nft")
       .arg("add")
       .arg("rule")
       .arg("inet")
       .arg("filter")
       .arg("input");

    if src != "any" { cmd.arg("ip").arg("saddr").arg(src); }
    if dst != "any" { cmd.arg("ip").arg("daddr").arg(dst); }
    if proto != "any" { cmd.arg(proto); }
    if port != "any" { cmd.arg("dport").arg(port); }
    cmd.arg(action);

    let _ = cmd.spawn().expect("failed to add nft rule");
}
