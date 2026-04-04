//! Linux display server string constants and predicates (Irony `LinuxDisplayServer`).

/// `"auto"`
pub const AUTO: &str = "auto";
/// `"wayland"`
pub const WAYLAND: &str = "wayland";
/// `"x11"`
pub const X11: &str = "x11";

/// Case-insensitive equality with [`AUTO`] (Irony `IsAuto`).
#[must_use]
pub fn is_auto(entry: &str) -> bool {
    entry.eq_ignore_ascii_case(AUTO)
}

/// Case-insensitive equality with [`WAYLAND`] (Irony `IsWayland`).
#[must_use]
pub fn is_wayland(entry: &str) -> bool {
    entry.eq_ignore_ascii_case(WAYLAND)
}

/// Case-insensitive equality with [`X11`] (Irony `IsX11`).
#[must_use]
pub fn is_x11(entry: &str) -> bool {
    entry.eq_ignore_ascii_case(X11)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicates_match_irony_ascii_case_rules() {
        assert!(is_auto("AUTO"));
        assert!(is_wayland("Wayland"));
        assert!(is_x11("X11"));
        assert!(!is_auto("wayland"));
    }
}
