//! External links. The app ships with none of its own (no website, chat or source links): the
//! only URLs it ever opens are ones a document carries, and only after the check below.

/// Whether a URL that came from a document (a link annotation, a button's URI action or
/// `app.launchURL`) may be handed to the system. Only web (`http`, `https`) and email (`mailto`)
/// links pass. Anything else, such as `file:`, UNC paths, `javascript:` or an app's custom
/// protocol, could open local files, leak credentials to a network share or start another
/// program, so the viewer refuses it. Control characters and spaces are refused too, so the
/// URL can't smuggle extra arguments to the browser command line.
pub fn is_safe_document_url(url: &str) -> bool {
    if url.is_empty() || url.chars().any(|c| c.is_control() || c.is_whitespace()) {
        return false;
    }
    let Some((scheme, rest)) = url.split_once(':') else { return false };
    match scheme.to_ascii_lowercase().as_str() {
        "http" | "https" => rest.strip_prefix("//").is_some_and(|host| !host.is_empty() && !host.starts_with('/')),
        "mailto" => !rest.is_empty(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_web_and_email_links_leave_a_document() {
        for ok in ["https://example.org", "http://example.org/a?b=c#d", "HTTPS://Example.org", "mailto:someone@example.org"] {
            assert!(super::is_safe_document_url(ok), "{ok}");
        }
        for bad in [
            "",
            "file:///C:/Windows/System32/calc.exe",
            "file://attacker.example/share/x.html",
            "\\\\attacker.example\\share\\x.exe",
            "C:\\Windows\\System32\\calc.exe",
            "/etc/passwd",
            "javascript:alert(1)",
            "ms-msdt:/id PCWDiagnostic",
            "search-ms:query=x",
            "smb://attacker.example/share",
            "http:///etc/passwd",
            "https:example.org",
            "https://example.org\" --new-window",
            "https://example.org\nfile:///x",
            "mailto:",
            "example.org",
        ] {
            assert!(!super::is_safe_document_url(bad), "{bad:?}");
        }
    }
}
