use serde_json::Value;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DisplayRow {
    pub col1: String,
    pub col2: String,
    pub col3: String,
    pub col4: String,
    pub col5: String,
}

fn run_nft_json() -> Result<Value, String> {
    let output = Command::new("pkexec")
        .arg("nft")
        .args(["-j", "list", "ruleset"])
        .output()
        .map_err(|e| format!("Errore esecuzione nft: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("nft ha restituito errore: {}", stderr));
    }

    serde_json::from_slice::<Value>(&output.stdout)
        .map_err(|e| format!("Errore parsing JSON nft: {}", e))
}

pub fn restart_nftables_service() -> Result<(), String> {
    let output = Command::new("pkexec")
        .arg("systemctl")
        .args(["restart", "nftables"])
        .output()
        .map_err(|e| format!("Errore restart nftables: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("systemctl ha restituito errore: {}", stderr));
    }

    Ok(())
}

pub fn load_nat_rows() -> Result<Vec<DisplayRow>, String> {
    let json = run_nft_json()?;
    Ok(parse_nat_rows(&json))
}

pub fn load_filter_rows() -> Result<Vec<DisplayRow>, String> {
    let json = run_nft_json()?;
    Ok(parse_filter_rows(&json))
}

fn parse_nat_rows(json: &Value) -> Vec<DisplayRow> {
    let mut rows = Vec::new();
    let Some(items) = json.get("nftables").and_then(|v| v.as_array()) else {
        return rows;
    };

    let mut current_table_family = String::new();
    let mut current_table_name = String::new();
    let mut current_chain_name = String::new();

    for item in items {
        if let Some(table) = item.get("table") {
            current_table_family = table
                .get("family")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            current_table_name = table
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
        }

        if let Some(chain) = item.get("chain") {
            current_chain_name = chain
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
        }

        if let Some(rule) = item.get("rule") {
            if !is_nat_table(&current_table_name, &current_chain_name) {
                continue;
            }

            let expr = rule.get("expr").cloned().unwrap_or(Value::Null);
            let raw = serde_json::to_string(&expr).unwrap_or_default();

            let rule_type = detect_nat_type(&raw);
            let protocol = detect_protocol(&raw);
            let source = extract_source(&raw);
            let destination = extract_destination(&raw);
            let translation = extract_translation(&raw);

            rows.push(DisplayRow {
                col1: rule_type,
                col2: protocol,
                col3: source,
                col4: destination,
                col5: translation,
            });
        }
    }

    rows
}

fn parse_filter_rows(json: &Value) -> Vec<DisplayRow> {
    let mut rows = Vec::new();
    let Some(items) = json.get("nftables").and_then(|v| v.as_array()) else {
        return rows;
    };

    let mut current_table_family = String::new();
    let mut current_table_name = String::new();
    let mut current_chain_name = String::new();

    for item in items {
        if let Some(table) = item.get("table") {
            current_table_family = table
                .get("family")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            current_table_name = table
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
        }

        if let Some(chain) = item.get("chain") {
            current_chain_name = chain
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
        }

        if let Some(rule) = item.get("rule") {
            if !is_filter_table(&current_table_name, &current_chain_name) {
                continue;
            }

            let expr = rule.get("expr").cloned().unwrap_or(Value::Null);
            let raw = serde_json::to_string(&expr).unwrap_or_default();

            let verdict = detect_verdict(&raw);
            let protocol = detect_protocol(&raw);
            let source = extract_source(&raw);
            let destination = extract_destination(&raw);
            let details = compact_details(&raw);

            rows.push(DisplayRow {
                col1: verdict,
                col2: protocol,
                col3: source,
                col4: destination,
                col5: details,
            });
        }
    }

    rows
}

fn is_nat_table(table_name: &str, chain_name: &str) -> bool {
    table_name.contains("nat")
        || matches!(
            chain_name,
            "prerouting" | "postrouting" | "output" | "input"
        )
}

fn is_filter_table(table_name: &str, chain_name: &str) -> bool {
    table_name.contains("filter")
        || matches!(chain_name, "input" | "forward" | "output")
}

fn detect_nat_type(raw: &str) -> String {
    if raw.contains("\"masquerade\"") {
        "MASQUERADE".to_string()
    } else if raw.contains("\"snat\"") {
        "SNAT".to_string()
    } else if raw.contains("\"dnat\"") {
        "DNAT".to_string()
    } else if raw.contains("\"redirect\"") {
        "REDIRECT".to_string()
    } else {
        "NAT".to_string()
    }
}

fn detect_verdict(raw: &str) -> String {
    if raw.contains("\"accept\"") {
        "ACCEPT".to_string()
    } else if raw.contains("\"drop\"") {
        "DROP".to_string()
    } else if raw.contains("\"reject\"") {
        "REJECT".to_string()
    } else if raw.contains("\"jump\"") {
        "JUMP".to_string()
    } else {
        "RULE".to_string()
    }
}

fn detect_protocol(raw: &str) -> String {
    if raw.contains("\"tcp\"") {
        "tcp".to_string()
    } else if raw.contains("\"udp\"") {
        "udp".to_string()
    } else if raw.contains("\"icmp\"") {
        "icmp".to_string()
    } else if raw.contains("\"ct\"") {
        "ct".to_string()
    } else {
        "all".to_string()
    }
}

fn extract_source(raw: &str) -> String {
    if let Some(v) = extract_right_after_field(raw, "saddr") {
        v
    } else if raw.contains("\"iifname\"") {
        format!("iif {}", extract_right_after_key(raw, "right").unwrap_or("-".into()))
    } else {
        "-".to_string()
    }
}

fn extract_destination(raw: &str) -> String {
    if let Some(v) = extract_right_after_field(raw, "daddr") {
        v
    } else if let Some(v) = extract_right_after_field(raw, "dport") {
        v
    } else if raw.contains("\"oifname\"") {
        format!("oif {}", extract_right_after_key(raw, "right").unwrap_or("-".into()))
    } else {
        "-".to_string()
    }
}

fn extract_translation(raw: &str) -> String {
    if raw.contains("\"masquerade\"") {
        "masquerade".to_string()
    } else if let Some(v) = extract_after_keyword(raw, "\"snat\":") {
        format!("snat {}", v)
    } else if let Some(v) = extract_after_keyword(raw, "\"dnat\":") {
        format!("dnat {}", v)
    } else if let Some(v) = extract_after_keyword(raw, "\"redirect\":") {
        format!("redirect {}", v)
    } else {
        compact_details(raw)
    }
}

fn compact_details(raw: &str) -> String {
    let mut out = raw
        .replace("\\\"", "\"")
        .replace('{', "")
        .replace('}', "")
        .replace('[', "")
        .replace(']', "");

    if out.len() > 80 {
        out.truncate(80);
        out.push_str("...");
    }

    if out.is_empty() {
        "-".to_string()
    } else {
        out
    }
}

fn extract_right_after_field(raw: &str, field: &str) -> Option<String> {
    let needle = format!("\"field\":\"{}\"", field);
    let pos = raw.find(&needle)?;
    let tail = &raw[pos..];
    let right_pos = tail.find("\"right\":")?;
    let mut value = tail[right_pos + 8..].trim_start();

    if value.starts_with('"') {
        value = &value[1..];
        let end = value.find('"')?;
        Some(value[..end].to_string())
    } else {
        let end = value.find([',', '}']).unwrap_or(value.len());
        Some(value[..end].trim().to_string())
    }
}

fn extract_right_after_key(raw: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let pos = raw.find(&needle)?;
    let mut value = raw[pos + needle.len()..].trim_start();

    if value.starts_with('"') {
        value = &value[1..];
        let end = value.find('"')?;
        Some(value[..end].to_string())
    } else {
        let end = value.find([',', '}']).unwrap_or(value.len());
        Some(value[..end].trim().to_string())
    }
}

fn extract_after_keyword(raw: &str, keyword: &str) -> Option<String> {
    let pos = raw.find(keyword)?;
    let tail = raw[pos + keyword.len()..].trim_start();
    let end = tail.find('}').unwrap_or(tail.len());
    Some(tail[..end].trim_matches('"').trim().to_string())
}
