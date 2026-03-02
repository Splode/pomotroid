use std::process::Command;

fn main() {
    // Rerun when git HEAD changes (new commit, checkout) or when tags move.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
    // Rerun if the canonical version source changes.
    println!("cargo:rerun-if-changed=tauri.conf.json");

    let base_version = read_base_version();

    // Full 40-char SHA for the log line.
    let full_sha = run_git(&["rev-parse", "HEAD"])
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Build version string from git describe.
    let build_version = match run_git(&["describe", "--tags", "--long", "--always", "--dirty"]) {
        Some(raw) => {
            let raw = raw.trim();
            match parse_describe(raw) {
                Some(info) => format_version(&info),
                // --always fallback: no tags, describe returned just a raw SHA.
                None => format_fallback_from_sha(&base_version, raw),
            }
        }
        None => format!("{base_version}+unknown"),
    };

    println!("cargo:rustc-env=APP_BUILD_VERSION={build_version}");
    println!("cargo:rustc-env=APP_BUILD_SHA={full_sha}");

    tauri_build::build()
}

// ---------------------------------------------------------------------------
// Git helpers
// ---------------------------------------------------------------------------

fn run_git(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
}

// ---------------------------------------------------------------------------
// Version parsing
// ---------------------------------------------------------------------------

struct DescribeInfo {
    tag: String,
    count: u32,
    sha: String,
    dirty: bool,
}

/// Parse `git describe --tags --long --always --dirty` output.
///
/// Expected form: `v1.0.0-80-g20b2d87[-dirty]`
/// Uses rsplitn so tag names containing hyphens are handled correctly.
fn parse_describe(s: &str) -> Option<DescribeInfo> {
    let (s, dirty) = s
        .strip_suffix("-dirty")
        .map(|t| (t, true))
        .unwrap_or((s, false));

    // rsplitn(3) from the right: ["g20b2d87", "80", "v1.0.0"]
    let parts: Vec<&str> = s.rsplitn(3, '-').collect();
    if parts.len() != 3 {
        return None;
    }

    let sha_part = parts[0]; // "g20b2d87"
    let count_part = parts[1]; // "80"
    let tag_part = parts[2]; // "v1.0.0"

    let count = count_part.parse::<u32>().ok()?;
    let sha = sha_part.strip_prefix('g')?;
    let tag = tag_part.trim_start_matches('v');

    Some(DescribeInfo {
        tag: tag.to_string(),
        count,
        sha: sha.to_string(),
        dirty,
    })
}

/// Format a parsed describe into a semver string.
///
/// | count | dirty | result                          |
/// |-------|-------|---------------------------------|
/// | 0     | no    | `1.0.0+20b2d87`                 |
/// | 0     | yes   | `1.0.0+20b2d87.dirty`           |
/// | N > 0 | no    | `1.0.0-dev.N+20b2d87`           |
/// | N > 0 | yes   | `1.0.0-dev.N+20b2d87.dirty`     |
fn format_version(info: &DescribeInfo) -> String {
    let base = if info.count == 0 {
        format!("{}+{}", info.tag, info.sha)
    } else {
        format!("{}-dev.{}+{}", info.tag, info.count, info.sha)
    };
    if info.dirty {
        format!("{base}.dirty")
    } else {
        base
    }
}

/// Fallback when no tags exist: describe returned a bare SHA (from --always).
/// Input looks like `20b2d87` or `20b2d87-dirty`.
fn format_fallback_from_sha(base_version: &str, raw: &str) -> String {
    let (sha, dirty) = raw
        .strip_suffix("-dirty")
        .map(|s| (s, true))
        .unwrap_or((raw, false));
    // Strip 'g' prefix if somehow present.
    let sha = sha.strip_prefix('g').unwrap_or(sha);
    if dirty {
        format!("{base_version}+{sha}.dirty")
    } else {
        format!("{base_version}+{sha}")
    }
}

// ---------------------------------------------------------------------------
// Base version source of truth
// ---------------------------------------------------------------------------

/// Read "version" from tauri.conf.json (same directory as build.rs = src-tauri/).
/// Falls back to "1.0.0" if the file is unreadable or the field is missing.
fn read_base_version() -> String {
    std::fs::read_to_string("tauri.conf.json")
        .ok()
        .and_then(|content| extract_json_str_field(&content, "version"))
        .unwrap_or_else(|| "1.0.0".to_string())
}

/// Minimal JSON string-field extractor — avoids a serde_json build-dependency.
fn extract_json_str_field(json: &str, field: &str) -> Option<String> {
    let key = format!("\"{}\"", field);
    let pos = json.find(&key)?;
    let rest = json[pos + key.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
