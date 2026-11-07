//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Dev,
    Station,
    Link,
    Scan,
    Connect,
    Disconnect,
}

#[derive(Debug, Clone)]
pub struct IwOptions {
    pub mode: Mode,
    pub interface: Option<String>,
}

impl Default for IwOptions {
    fn default() -> Self { Self { mode: Mode::Default, interface: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(IwOptions::default().mode, Mode::Default); }
}
