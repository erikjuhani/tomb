use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VersionInfo {
    pub version: &'static str,
    pub short_hash: &'static str,
    pub date: &'static str,
}

impl VersionInfo {
    pub const fn from_env() -> Self {
        Self {
            version: env!("TOMB_VERSION"),
            short_hash: env!("TOMB_COMMIT_SHORT_HASH"),
            date: env!("TOMB_COMMIT_DATE"),
        }
    }
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} {})", self.version, self.short_hash, self.date)
    }
}

#[test]
fn version_format() {
    let info = VersionInfo {
        version: "0.1.0",
        short_hash: "abc123def",
        date: "2026-03-15",
    };
    assert_eq!(info.to_string(), "0.1.0 (abc123def 2026-03-15)");
}
