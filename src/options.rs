//! Command-line options for iw.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Scan mode
    Scan(String),
    /// Connect mode
    Connect(String, String),
    /// Info mode
    Info(String),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Interface name
    pub interface: String,
    /// JSON output
    pub json: bool,
    /// Debug output
    pub debug: bool,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            interface: String::new(),
            json: false,
            debug: false,
            verbose: false,
            quiet: false,
        }
    }
}
