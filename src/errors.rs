pub struct OverlayConfigParseError(pub String);

impl std::fmt::Display for OverlayConfigParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse the configuration file {}", self.0)
    }
}
