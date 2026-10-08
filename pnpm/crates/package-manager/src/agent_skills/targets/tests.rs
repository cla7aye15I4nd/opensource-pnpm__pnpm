use super::agent_skills_dir_from_lookup;
use pretty_assertions::assert_eq;
use std::ffi::OsString;

#[test]
fn detects_claude_code() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "CLAUDECODE").then(|| OsString::from("1"))),
        Some(".claude/skills"),
    );
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "CLAUDE_CODE").then(|| OsString::from("1"))),
        Some(".claude/skills"),
    );
}

#[test]
fn detects_cursor() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "CURSOR_AGENT").then(|| OsString::from("1"))),
        Some(".cursor/skills"),
    );
}

#[test]
fn detects_gemini_cli() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "GEMINI_CLI").then(|| OsString::from("1"))),
        Some(".gemini/skills"),
    );
}

#[test]
fn detects_antigravity() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| {
            (var == "ANTIGRAVITY_AGENT").then(|| OsString::from("1"))
        }),
        Some(".agents/skills"),
    );
}

#[test]
fn detects_copilot() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "COPILOT_AGENT").then(|| OsString::from("1"))),
        Some(".github/skills"),
    );
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "COPILOT_CLI").then(|| OsString::from("1"))),
        Some(".github/skills"),
    );
}

#[test]
fn detects_codex() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| {
            (var == "CODEX_THREAD_ID").then(|| OsString::from("xyz"))
        }),
        Some(".agents/skills"),
    );
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "CODEX_SANDBOX").then(|| OsString::from("1"))),
        Some(".agents/skills"),
    );
}

#[test]
fn detects_generic_ai_agent() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| (var == "AI_AGENT").then(|| OsString::from("1"))),
        Some(".agents/skills"),
    );
}

#[test]
fn ignores_empty_environment_variables() {
    assert_eq!(agent_skills_dir_from_lookup(|var| (var == "CLAUDECODE").then(OsString::new)), None);
}

#[test]
fn prefers_specific_agent_over_generic_fallback() {
    assert_eq!(
        agent_skills_dir_from_lookup(|var| {
            match var {
                "AI_AGENT" | "CURSOR_AGENT" => Some(OsString::from("1")),
                _ => None,
            }
        }),
        Some(".cursor/skills"),
    );
}

#[test]
fn returns_none_when_no_agent_variable_is_set() {
    assert_eq!(agent_skills_dir_from_lookup(|_| None), None);
}
