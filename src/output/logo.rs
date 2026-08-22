use crate::output::color::RESET;

#[derive(Debug, Clone)]
pub struct Logo {
    pub name: &'static str,
    pub raw_lines: &'static [&'static str],
    pub primary_color: &'static str,
    pub accent_color: &'static str,
}

impl Logo {
    /// Returns the logo lines formatted with ANSI colors if enabled.
    pub fn render_lines(&self, enable_color: bool) -> Vec<String> {
        self.raw_lines
            .iter()
            .map(|line| {
                if enable_color {
                    format!("{}{}{}", self.primary_color, line, RESET)
                } else {
                    line.to_string()
                }
            })
            .collect()
    }
}

pub const ALL_LOGOS: &[Logo] = &[
    Logo {
        name: "ferris",
        raw_lines: &[
            "      _~^~^~_",
            "  \\) /  o o  \\ (/",
            "    '_   -   _'",
            "    / '-----' \\",
            "  <|  /     \\  |>",
            "   | (  \\ /  ) |",
            "    \\ \\  V  / /",
            "     \\ \\---/ /",
            "      \\     /",
            "       '---'",
        ],
        primary_color: "\x1b[38;5;208m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "debian",
        raw_lines: &[
            "         _,met$$$$$gg.",
            "      ,g$$$$$$$$$$$$$$$P.",
            "    ,g$$P\"        \"\"\"Y$$\".\"",
            "   ,$$P'              `$$$.",
            "  ',$$P       ,ggs.     `$$b:",
            "  `d$$'     ,$P\"'   .    $$$",
            "   $$P      d$'     ,    $$$P",
            "   $$:      $$.   -    ,d$$'",
            "   $$;      Y$b._   _,d$P'",
            "   Y$$.    `.`\"Y$$$$P\"'",
            "   `$$b      \"-.__",
            "    `Y$$b",
            "     `\"Y$$.",
            "         `\"\"",
        ],
        primary_color: "\x1b[38;5;196m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "ubuntu",
        raw_lines: &[
            "                             ....",
            "              .',:clooo:  .:looooo:.",
            "           .;looooooooc  .oooooooooo'",
            "        .;looooool:,''.  :ooooooooooc",
            "       ;looool;.         'oooooooooo,",
            "      ;clool'             .cooooooc.  ,,",
            "         ...                ......  .:oo,",
            "  .;clol:,.                        .loooo'",
            " :ooooooooo,                        'ooool",
            "'ooooooooooo.                        loooo.",
            "'ooooooooool                         coooo.",
            " ,loooooooc.                        .loooo.",
            "   .,;;;'.                          ;ooooc",
            "       ...                         ,ooool.",
            "    .cooooc.              ..',,'.  .cooo.",
            "      ;ooooo:.           ;oooooooc.  :l.",
            "       .coooooc,..      coooooooooo.",
            "         .:ooooooolc:. .ooooooooooo'",
            "           .':loooooo;  ,oooooooooc",
            "               ..';::c'  .;loooo:'",
        ],
        primary_color: "\x1b[38;5;208m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "linuxmint",
        raw_lines: &[
            "  ___________",
            " |_          \\",
            "   |  ______  |",
            "   | | _____| |",
            "   | | | | |  |",
            "   | | | | |  |",
            "   | | | | |  |",
            "   | \\_____/  |",
            "   \\_________/",
        ],
        primary_color: "\x1b[38;5;46m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "fedora",
        raw_lines: &[
            "             .',;::::;,'.",
            "         .';cccccccccccc:;.",
            "      .;cccccccccccccccccccc;.",
            "    .:cccccccccccccccccccccccc:.",
            "  .:ccccccccccccc;::dddl:;ccccccc:.",
            "  :ccccccccccccc;OWMKOOXMWd;ccccccc:",
            " :cccccccccccc;KMMc;cc;xMMc;ccccccc:",
            ",ccccccccccccc;MMM.;cc;WW;;cccccccc,",
            ":ccccccccccccc;MMM.;cccccccccccccccc:",
            ":ccccccc;oxOOo;MMM0OOK.;cccccccccccc:",
            "cccccc;0MMKxdd:MMMkddc;cccccccccccc:",
            "ccccc;XMO';cccc;MMW.;cccccccccccccc'",
            "ccccc;MMo;ccccc;MMW.;cccccccccccccc",
            "ccccc;0MNc.ccc.xMMd;cccccc;",
            "cccccc;dNMWXXXWM0:;cccccc:",
            " :cccccccccccccccccccccc:",
            "   '::cccccccccccccc::;'",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;81m",
    },
    Logo {
        name: "arch",
        raw_lines: &[
            "                   -`",
            "                  .o+`",
            "                 `ooo/",
            "                `+oooo:",
            "               `+oooooo:",
            "               -+oooooo+:",
            "             `/:-:++oooo+:",
            "            `/++++/+++++++:",
            "           `/++++++++++++++:",
            "          `/+++ooooooooooooo/`",
            "         ./ooosssso++osssssso+`",
            "        .oossssso-````/ossssss+`",
            "       -osssssso.      :ssssssso.",
            "      :osssssss/        osssso+++.",
            "     /ossssssss/        +ssssooo/-",
            "   `/ossssso+/:-        -:/+osssso+-",
            "  `+sso+:-`                 .-/+oso:",
            " `++:.                           `-/+/",
            " .`                                 `/",
        ],
        primary_color: "\x1b[38;5;67m",
        accent_color: "\x1b[38;5;123m",
    },
    Logo {
        name: "rhel",
        raw_lines: &[
            "           .---.",
            "          / /\"\\ \\",
            "         | |   | |",
            "      .-.-\\ \\_/ /.-.-.",
            "     /  ___\"\"\"\"\"___  \\",
            "     | |   | |   | | |",
            "     | |   | |   | | |",
            "     \\__(_________)__/",
        ],
        primary_color: "\x1b[38;5;196m",
        accent_color: "\x1b[90m",
    },
    Logo {
        name: "rocky",
        raw_lines: &[
            "          .---.",
            "        /       \\",
            "       |   /\\    |",
            "       |  /  \\   |",
            "       |  \\  /   |",
            "       |   \\/    |",
            "        \\       /",
            "          '---'",
        ],
        primary_color: "\x1b[38;5;35m",
        accent_color: "\x1b[38;5;84m",
    },
    Logo {
        name: "almalinux",
        raw_lines: &[
            "         __o__",
            "       /       \\",
            "      |    O    |",
            "      |   /|\\   |",
            "       \\  / \\  /",
            "        '-o-o-'",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[38;5;220m",
    },
    Logo {
        name: "endeavouros",
        raw_lines: &[
            "            / \\",
            "           /   \\",
            "          /  /\\ \\",
            "         /  /  \\ \\",
            "        /  /    \\ \\",
            "       /  /  __  \\ \\",
            "      /__/  /__\\  \\_\\",
        ],
        primary_color: "\x1b[38;5;127m",
        accent_color: "\x1b[38;5;197m",
    },
    Logo {
        name: "manjaro",
        raw_lines: &[
            "|||||||||||||||| ||||||||",
            "|||||||||||||||| ||||||||",
            "||||||           ||||||||",
            "||||||  |||||||| ||||||||",
            "||||||  |||||||| ||||||||",
            "||||||  |||||||| ||||||||",
            "||||||  |||||||| ||||||||",
            "||||||  |||||||| ||||||||",
        ],
        primary_color: "\x1b[38;5;34m",
        accent_color: "\x1b[38;5;47m",
    },
    Logo {
        name: "generic",
        raw_lines: &[
            "           .---.",
            "          /     \\",
            "         | ()_() |",
            "          \\  _  /",
            "   __   __| '-' |__   __",
            "  /  '-'   \\___/   '-'  \\",
            " |  \\                 /  |",
            "  \\  '.             .'  /",
            "   \\   '---.   .---'   /",
            "    '-.__  |   |  __.-'",
            "         | |   | |",
            "         \\ |   | /",
            "          \\_\\ /_/",
        ],
        primary_color: "\x1b[38;5;220m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "opensuse",
        raw_lines: &[
            "          _______",
            "        /  _____/",
            "       (  (_____",
            "        \\_____  \\",
            "         _____)  )",
            "        /_______/",
            "       (________)",
        ],
        primary_color: "\x1b[38;5;71m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "alpine",
        raw_lines: &[
            "          /\\",
            "         // \\  /\\",
            "        //   \\// \\",
            "       ///    /   \\",
            "      //     /     \\",
        ],
        primary_color: "\x1b[38;5;32m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "gentoo",
        raw_lines: &[
            "        _-----_",
            "       (  ---  )",
            "      | (  _  ) |",
            "      |  (   )  |",
            "       \\  ---  /",
            "        \\_____/",
        ],
        primary_color: "\x1b[38;5;141m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "void",
        raw_lines: &[
            "        _______",
            "       /\\ ____ \\",
            "      /  \\__  \\ \\",
            "      \\ \\ \\ \\  \\ \\",
            "       \\ \\ \\ \\__\\ \\",
            "        \\ \\ \\_____/",
            "         \\/_______/",
        ],
        primary_color: "\x1b[38;5;35m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "pop",
        raw_lines: &[
            "       ______",
            "      \\  __ \\",
            "       \\ \\_\\ \\  __",
            "        \\ ___/ /_/",
            "         \\ \\",
            "          \\/",
        ],
        primary_color: "\x1b[38;5;37m",
        accent_color: "\x1b[38;5;214m",
    },
    Logo {
        name: "nixos",
        raw_lines: &[
            "          \\\\  //",
            "        ==\\\\//==",
            "          //\\\\",
            "         //  \\\\",
            "       ==//  \\\\==",
            "        //    \\\\",
        ],
        primary_color: "\x1b[38;5;75m",
        accent_color: "\x1b[38;5;117m",
    },
    Logo {
        name: "kali",
        raw_lines: &[
            "       .:::::::::.",
            "      :;;;;;;;;;;;:",
            "     :;;;;;;;;;;;;;:",
            "      :;;;;;;;;;;;:",
            "       ':::::::::'",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "freebsd",
        raw_lines: &[
            "       /\\   /\\",
            "      (  ) (  )",
            "       \\ \\_/ /",
            "        \\___/",
        ],
        primary_color: "\x1b[38;5;196m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "slackware",
        raw_lines: &[
            "        ________",
            "       / ____  /",
            "      / /   / /",
            "     / /___/ /",
            "    /_______/",
        ],
        primary_color: "\x1b[38;5;61m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "artix",
        raw_lines: &[
            "           /\\",
            "          /  \\",
            "         /`'.,\\",
            "        /     ',",
            "       /      ,`\\",
            "      /__,.'`'___\\",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "zorin",
        raw_lines: &[
            "       _______",
            "      / _____ \\",
            "       / ___/ /",
            "      / /____",
            "     /_______/",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[37m",
    },
    Logo {
        name: "windows11",
        raw_lines: &[
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
            "################  ################",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[38;5;45m",
    },
    Logo {
        name: "windows10",
        raw_lines: &[
            "                   ...,:-==+??***",
            "            ..::--===++???*******",
            "      ..--===+++????*************",
            "..-==+++????*********************",
            "################  ###############",
            "################  ###############",
            "################  ###############",
            "################  ###############",
            "################  ###############",
            "..-==+++????*********************",
            "      ..--===+++????*************",
            "            ..::--===++???*******",
            "                   ...,:-==+??***",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;39m",
    },
    Logo {
        name: "windows7",
        raw_lines: &[
            "       _.-;;-._",
            " '-..-'|   ||   |",
            " '-..-'|_.-''-._|",
            " '-..-'|   ||   |",
            " '-..-'|_.-''-._|",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;220m",
    },
];

pub fn get_all_logos() -> &'static [Logo] {
    ALL_LOGOS
}

/// Matches a logo by name, distro_id, or distro_like chain.
/// Resolution precedence:
/// 1. Explicit CLI `--logo` override (supports "none" to disable logo rendering).
/// 2. Primary `distro_id` matching `ID` from `/etc/os-release`.
/// 3. Upstream distribution inheritance chain from `ID_LIKE` (e.g. Pop!_OS -> Ubuntu -> Debian).
/// 4. Default fallback to Ferris the Rust crab mascot.
pub fn match_logo(
    logo_override: Option<&str>,
    distro_id: &str,
    distro_like: &[String],
) -> Option<&'static Logo> {
    let all = get_all_logos();

    if let Some(name) = logo_override {
        let clean = name.trim().to_lowercase();
        if clean == "none" {
            return None;
        }
        if let Some(logo) = find_logo_by_key(&clean, all) {
            return Some(logo);
        }
    }

    if let Some(logo) = find_logo_by_key(distro_id, all) {
        return Some(logo);
    }

    for like in distro_like {
        if let Some(logo) = find_logo_by_key(like, all) {
            return Some(logo);
        }
    }

    // Default mascot fallback
    find_logo_by_key("ferris", all)
}

/// Normalizes common distribution aliases, derivative names, and shorthand keys.
fn find_logo_by_key(key: &str, logos: &'static [Logo]) -> Option<&'static Logo> {
    let normalized = key.trim().to_lowercase();
    match normalized.as_str() {
        "ferris" | "rust" => logos.iter().find(|l| l.name == "ferris"),
        "debian" => logos.iter().find(|l| l.name == "debian"),
        "ubuntu" => logos.iter().find(|l| l.name == "ubuntu"),
        "mint" | "linuxmint" => logos.iter().find(|l| l.name == "linuxmint"),
        "fedora" => logos.iter().find(|l| l.name == "fedora"),
        "arch" | "archlinux" => logos.iter().find(|l| l.name == "arch"),
        "rhel" | "redhat" | "centos" => logos.iter().find(|l| l.name == "rhel"),
        "rocky" | "rockylinux" => logos.iter().find(|l| l.name == "rocky"),
        "alma" | "almalinux" => logos.iter().find(|l| l.name == "almalinux"),
        "endeavour" | "endeavouros" => logos.iter().find(|l| l.name == "endeavouros"),
        "manjaro" => logos.iter().find(|l| l.name == "manjaro"),
        "tux" | "generic" | "linux" => logos.iter().find(|l| l.name == "generic"),
        "opensuse" | "suse" | "opensuse-leap" | "opensuse-tumbleweed" => {
            logos.iter().find(|l| l.name == "opensuse")
        }
        "alpine" => logos.iter().find(|l| l.name == "alpine"),
        "gentoo" => logos.iter().find(|l| l.name == "gentoo"),
        "void" => logos.iter().find(|l| l.name == "void"),
        "pop" | "popos" | "pop_os" => logos.iter().find(|l| l.name == "pop"),
        "nixos" | "nix" => logos.iter().find(|l| l.name == "nixos"),
        "kali" | "kalilinux" => logos.iter().find(|l| l.name == "kali"),
        "freebsd" | "bsd" => logos.iter().find(|l| l.name == "freebsd"),
        "slackware" | "slack" => logos.iter().find(|l| l.name == "slackware"),
        "artix" => logos.iter().find(|l| l.name == "artix"),
        "zorin" | "zorinos" => logos.iter().find(|l| l.name == "zorin"),
        "windows11"
        | "win11"
        | "windows 11"
        | "microsoft windows 11"
        | "mswindows 11"
        | "windows"
        | "microsoft windows"
        | "mswindows" => logos.iter().find(|l| l.name == "windows11"),
        "windows10" | "win10" | "windows 10" | "microsoft windows 10" | "mswindows 10" => {
            logos.iter().find(|l| l.name == "windows10")
        }
        "windows7"
        | "win7"
        | "windows 7"
        | "classic_windows"
        | "classic windows"
        | "microsoft windows 7"
        | "mswindows 7"
        | "windows vista"
        | "windows xp" => logos.iter().find(|l| l.name == "windows7"),
        _ => {
            if normalized.contains("win11") || normalized.contains("windows 11") {
                logos.iter().find(|l| l.name == "windows11")
            } else if normalized.contains("win10") || normalized.contains("windows 10") {
                logos.iter().find(|l| l.name == "windows10")
            } else if normalized.contains("win7")
                || normalized.contains("windows 7")
                || normalized.contains("classic_windows")
                || normalized.contains("classic windows")
            {
                logos.iter().find(|l| l.name == "windows7")
            } else if normalized.contains("windows") {
                logos.iter().find(|l| l.name == "windows11")
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_logo_direct() {
        let debian = match_logo(None, "debian", &[]);
        assert!(debian.is_some());
        assert_eq!(debian.unwrap().name, "debian");
    }

    #[test]
    fn test_match_logo_override() {
        let arch = match_logo(Some("arch"), "debian", &[]);
        assert!(arch.is_some());
        assert_eq!(arch.unwrap().name, "arch");
    }

    #[test]
    fn test_match_logo_none_override() {
        let none = match_logo(Some("none"), "debian", &[]);
        assert!(none.is_none());
    }

    #[test]
    fn test_match_logo_like_fallback() {
        let ubuntu = match_logo(
            None,
            "elementary",
            &["ubuntu".to_string(), "debian".to_string()],
        );
        assert!(ubuntu.is_some());
        assert_eq!(ubuntu.unwrap().name, "ubuntu");
    }

    #[test]
    fn test_match_logo_unknown_fallback_to_ferris() {
        let logo = match_logo(None, "unknowndistro", &[]);
        assert!(logo.is_some());
        assert_eq!(logo.unwrap().name, "ferris");
    }

    #[test]
    fn test_match_logo_windows() {
        // Windows 11 matching
        for key in &[
            "windows11",
            "win11",
            "windows 11",
            "windows",
            "microsoft windows",
            "microsoft windows 11",
            "Windows 11 Pro",
        ] {
            let logo = match_logo(Some(key), "unknown", &[]);
            assert!(logo.is_some(), "Failed to match key '{}'", key);
            assert_eq!(
                logo.unwrap().name,
                "windows11",
                "Key '{}' matched wrong logo",
                key
            );
        }

        // Windows 10 matching
        for key in &[
            "windows10",
            "win10",
            "windows 10",
            "microsoft windows 10",
            "Windows 10 Enterprise",
        ] {
            let logo = match_logo(Some(key), "unknown", &[]);
            assert!(logo.is_some(), "Failed to match key '{}'", key);
            assert_eq!(
                logo.unwrap().name,
                "windows10",
                "Key '{}' matched wrong logo",
                key
            );
        }

        // Windows 7 matching
        for key in &[
            "windows7",
            "win7",
            "windows 7",
            "classic_windows",
            "classic windows",
            "microsoft windows 7",
            "windows xp",
        ] {
            let logo = match_logo(Some(key), "unknown", &[]);
            assert!(logo.is_some(), "Failed to match key '{}'", key);
            assert_eq!(
                logo.unwrap().name,
                "windows7",
                "Key '{}' matched wrong logo",
                key
            );
        }
    }
}
