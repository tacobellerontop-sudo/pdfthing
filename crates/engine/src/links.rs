//! Where PrintCraft and the ArtCraft community live on the web. One table, so the Help menu, the
//! About dialog, the home screen, the CLI and the README agree.

/// The app's name in ArtCraft URLs (`getartcraft.com/apps/{APP}`, `github.com/storytold/{APP}`).
pub const APP: &str = "printcraft";

pub const DISCORD: &str = "https://discord.gg/artcraft";
pub const WEBSITE: &str = "https://getartcraft.com";
pub const APP_PAGE: &str = "https://getartcraft.com/apps/printcraft";
pub const GITHUB: &str = "https://github.com/storytold/printcraft";

/// A link and the registry command that opens it.
#[derive(Clone, Copy, Debug)]
pub struct Link {
    pub command: &'static str,
    pub label: &'static str,
    pub url: &'static str,
    /// Lucide icon name.
    pub icon: &'static str,
}

/// In the order they are shown. Discord comes first: it is where people get help fastest.
pub const LINKS: &[Link] = &[
    Link { command: "help.discord", label: "Join the ArtCraft Discord", url: DISCORD, icon: "messages-square" },
    Link { command: "help.app_page", label: "PrintCraft web page", url: APP_PAGE, icon: "globe" },
    Link { command: "help.github", label: "PrintCraft on GitHub", url: GITHUB, icon: "code-xml" },
    Link { command: "help.website", label: "ArtCraft website", url: WEBSITE, icon: "external-link" },
];

pub fn for_command(id: &str) -> Option<&'static Link> {
    LINKS.iter().find(|l| l.command == id)
}

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
    fn urls_follow_the_artcraft_scheme() {
        assert_eq!(super::APP_PAGE, format!("{}/apps/{}", super::WEBSITE, super::APP));
        assert_eq!(super::GITHUB, format!("https://github.com/storytold/{}", super::APP));
        for l in super::LINKS {
            assert!(l.url.starts_with("https://"), "{}", l.url);
            assert!(crate::commands::command(l.command).is_some(), "{} is a registered command", l.command);
            assert!(super::is_safe_document_url(l.url), "{}", l.url);
        }
    }

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
