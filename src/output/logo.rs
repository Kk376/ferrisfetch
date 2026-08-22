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
                    let mut rendered = line
                        .replace("{p}", self.primary_color)
                        .replace("{a}", self.accent_color)
                        .replace("{0}", RESET);
                    if !line.contains("{p}") && !line.contains("{a}") && !line.is_empty() {
                        rendered = format!("{}{}{}", self.primary_color, line, RESET);
                    } else if !line.is_empty() {
                        rendered.push_str(RESET);
                    }
                    rendered
                } else {
                    line.replace("{p}", "")
                        .replace("{a}", "")
                        .replace("{0}", "")
                }
            })
            .collect()
    }
}

pub const ALL_LOGOS: &[Logo] = &[
    Logo {
        name: "ferris",
        raw_lines: &[
            "        {a}/^^^\\     /^^^\\",
            "       {a}(  O  )   (  O  )",
            "      {p}.-'---'-----'---'-.",
            "     {p}/   _~^~^~^~^~_     \\",
            "    {p}|   /  {a}o     o{p}  \\     |",
            "    {p}|   |     {a}-{p}     |     |",
            "    {p}|   \\  {a}'---'{p}  /     |",
            "     {p}\\   '-------'     /",
            "      {p}'-._'-------'_.-'",
            "          /       \\",
        ],
        primary_color: "\x1b[38;5;208m",
        accent_color: "\x1b[38;5;220m",
    },
    Logo {
        name: "debian",
        raw_lines: &[
            "{a}       _,met$$$$$gg.",
            "    ,g$$$$$$$$$$$$$$$P.",
            "  ,g$$P\"        \"\"\"Y$$.\".",
            " ,$$P'              `$$$.",
            "',$$P       ,ggs.     `$$b:",
            "`d$$'     ,$P\"'   {p}.{a}    $$$",
            " $$P      d$'     {p},{a}    $$P",
            " $$:      $$.   {p}-{a}    ,d$$'",
            " $$;      Y$b._   _,d$P'",
            " Y$$.    {p}`.{a}`\"Y$$$$P\"'",
            "{a} `$$b      {p}\"-.__",
            "{a}  `Y$$",
            "   `Y$$.",
            "     `$$b.",
            "       `Y$$b.",
            "          `\"Y$b._",
            "              `\"\"\"",
        ],
        primary_color: "\x1b[38;5;196m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "ubuntu",
        raw_lines: &[
            "{p}            .-/+oossssoo+\\-.",
            "        ´:+ssssssssssssssssss+:`",
            "      -+ssssssssssssssssssyyssss+-",
            "    .ossssssssssssssssss{a}dMMMNy{p}sssso.",
            "   /sssssssssss{a}hdmmNNmmyNMMMMh{p}ssssss\\",
            "  +sssssssss{a}hm{p}yd{a}MMMMMMMNddddy{p}ssssssss+",
            " /ssssssss{a}hNMMM{p}yh{a}hyyyyhmNMMMNh{p}ssssssss\\",
            ".ssssssss{a}dMMMNh{p}ssssssssss{a}hNMMMd{p}ssssssss.",
            "+ssss{a}hhhyNMMNy{p}ssssssssssss{a}yNMMMy{p}sssssss+",
            "oss{a}yNMMMNyMMh{p}ssssssssssssss{a}hmmmh{p}ssssssso",
            "oss{a}yNMMMNyMMh{p}sssssssssssssshmmmh{p}ssssssso",
            "+ssss{a}hhhyNMMNy{p}ssssssssssss{a}yNMMMy{p}sssssss+",
            ".ssssssss{a}dMMMNh{p}ssssssssss{a}hNMMMd{p}ssssssss.",
            " \\ssssssss{a}hNMMM{p}yh{a}hyyyyhdNMMMNh{p}ssssssss/",
            "  +sssssssss{a}dm{p}yd{a}MMMMMMMMddddy{p}ssssssss+",
            "   \\sssssssssss{a}hdmNNNNmyNMMMMh{p}ssssss/",
            "    .ossssssssssssssssss{a}dMMMNy{p}sssso.",
            "      -+sssssssssssssssss{a}yyy{p}ssss+-",
            "        `:+ssssssssssssssssss+:`",
            "            .-\\+oossssoo+/-.",
        ],
        primary_color: "\x1b[38;5;208m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "linuxmint",
        raw_lines: &[
            "{a}             ...-:::::-...",
            "{a}          .-MMMMMMMMMMMMMMM-.",
            "      .-MMMM{p}`..-:::::::-..`{a}MMMM-.",
            "    .:MMMM{p}.:MMMMMMMMMMMMMMM:.{a}MMMM:.",
            "   -MMM{p}-M---MMMMMMMMMMMMMMMMMMM.{a}MMM-",
            " `:MMM{p}:MM`  :MMMM:....::-...-MMMM:{a}MMM:`",
            " :MMM{p}:MMM`  :MM:`  ``    ``  `:MMM:{a}MMM:",
            ".MMM{p}.MMMM`  :MM.  -MM.  .MM-  `MMMM.{a}MMM.",
            ":MMM{p}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{a}MMM:",
            ":MMM{p}:MMMM`  :MM.  -MM-  .MM:  `MMMM:{a}MMM:",
            ":MMM{p}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{a}MMM:",
            ".MMM{p}.MMMM`  :MM:--:MM:--:MM:  `MMMM.{a}MMM.",
            " :MMM{p}:MMM-  `-MMMMMMMMMMMM-`  -MMM-{a}MMM:",
            "  :MMM{p}:MMM:`                `:MMM:{a}MMM:",
            "   .MMM{p}.MMMM:--------------:MMMM.{a}MMM.",
            "     '-MMMM{p}.-MMMMMMMMMMMMMMM-.{a}MMMM-'",
            "       '.-MMMM{p}``--:::::--``{a}MMMM-.'",
            "{a}            '-MMMMMMMMMMMMM-'",
            "{a}               ``-:::::-``",
        ],
        primary_color: "\x1b[38;5;46m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "fedora",
        raw_lines: &[
            "{p}             .',;::::;,'.",
            "         .';:cccccccccccc:;,.",
            "      .;cccccccccccccccccccccc;.",
            "    .:cccccccccccccccccccccccccc:.",
            "  .;ccccccccccccc;{a}.:dddl:.{p};ccccccc;.",
            " .:ccccccccccccc;{a}OWMKOOXMWd{p};ccccccc:.",
            ".:ccccccccccccc;{a}KMMc{p};cc;{a}xMMc{p};ccccccc:.",
            ",cccccccccccccc;{a}MMM.{p};cc;{a};WW:{p};cccccccc,",
            ":cccccccccccccc;{a}MMM.{p};cccccccccccccccc:",
            ":ccccccc;{a}oxOOOo{p};{a}MMM0OOk.{p};cccccccccccc:",
            "cccccc;{a}0MMKxdd:{p};{a}MMMkddc.{p};cccccccccccc;",
            "ccccc;{a}XM0'{p};cccc;{a}MMM.{p};cccccccccccccccc'",
            "ccccc;{a}MMo{p};ccccc;{a}MMW.{p};ccccccccccccccc;",
            "ccccc;{a}0MNc.{p}ccc{a}.xMMd{p};ccccccccccccccc;",
            "cccccc;{a}dNMWXXXWM0:{p};cccccccccccccc:,",
            "cccccccc;{a}.:odl:.{p};cccccccccccccc:,.",
            ":cccccccccccccccccccccccccccc:'.",
            ".:cccccccccccccccccccccc:;,..",
            "  '::cccccccccccccc::;,.",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "arch",
        raw_lines: &[
            "{p}                   -`",
            "                  .o+`",
            "                 `ooo/",
            "                `+oooo:",
            "               `+oooooo:",
            "               -+oooooo+:",
            "             `/:-:++oooo+:",
            "            `/++++/+++++++:",
            "           `/++++++++++++++:",
            "          `/+++o{a}oooooooo{p}oooo/`",
            "{a}         {p}./{a}ooosssso++osssssso{p}+`",
            "{a}        .oossssso-````/ossssss+`",
            "       -osssssso.      :ssssssso.",
            "      :osssssss/        osssso+++.",
            "     /ossssssss/        +ssssooo/-",
            "   `/ossssso+/:-        -:/+osssso+-",
            "  `+sso+:-`                 `.-/+oso:",
            " `++:.                           `-/+/",
            " .`                                 `/",
        ],
        primary_color: "\x1b[38;5;67m",
        accent_color: "\x1b[38;5;123m",
    },
    Logo {
        name: "rhel",
        raw_lines: &[
            "{p}             `.-..........`",
            "            `////////::.`-/.",
            "            -: ....-////////.",
            "            //:-::///////////`",
            "     `--::: `-://////////////:",
            "     //////-    ``.-:///////// .`",
            "     `://////:-.`    :///////::///:`",
            "       .-/////////:---/////////////:",
            "          .-://////////////////////.",
            "{a}         yMN+`.-{p}::///////////////-`",
            "{a}      .-`:NMMNMs`  `..-------..`",
            "       MN+/mMMMMMhoooyysshsss",
            "MMM    MMMMMMMMMMMMMMyyddMMM+",
            " MMMM   MMMMMMMMMMMMMNdyNMMh`     hyhMMM",
            "  MMMMMMMMMMMMMMMMyoNNNMMM+.   MMMMMMMM",
            "   MMNMMMNNMMMMMNM+ mhsMNyyyyMNMMMMsMM",
        ],
        primary_color: "\x1b[38;5;196m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "rocky",
        raw_lines: &[
            "{p}    `-/+++++++++/-.`",
            " `-+++++++++++++++++-`",
            ".+++++++++++++++++++++.",
            "-+++++++++++++++++++++++.",
            "+++++++++++++++/-/+++++++",
            "+++++++++++++/.   ./+++++",
            "+++++++++++:.       ./+++",
            "+++++++++:`   `:/:`   .:/",
            "-++++++:`   .:+++++:`",
            " .+++-`   ./+++++++++:`",
            "  `-`   ./+++++++++++-",
            "       -+++++++++:-.`",
        ],
        primary_color: "\x1b[38;5;35m",
        accent_color: "\x1b[38;5;121m",
    },
    Logo {
        name: "almalinux",
        raw_lines: &[
            "{p}         'c:.",
            "{p}        lkkkx, ..       {a}..   ,cc,",
            "{p}        okkkk:ckkx'  {a}.lxkkx.okkkkd",
            "{p}        .:llcokkx'  {a}:kkkxkko:xkkd,",
            "{p}      .xkkkkdood:  {a};kx,  .lkxlll;",
            "{p}       xkkx.       {a}xk'     xkkkkk:",
            "{p}       'xkx.       {a}xd      .....,.",
            "{a}      .. {p}:xkl'     {a}:c      ..''..",
            "{a}    .dkx'  {p}.:ldl:'. {a}'  {a}':lollldkkxo;",
            "{a}  .''lkkko'                     {a}ckkkx.",
            "{a}'xkkkd:kkd.       ..  {p};'        {a}:kkxo.",
            "{a},xkkkd;kk'      ,d;    {p}ld.   {a}':dkd::cc,",
            "{a} .,,.;xkko'.';lxo.      {p}dx,  {a}:kkk'xkkkkc",
            "{a}     'dkkkkkxo:.        {p};kx  {a}.kkk:;xkkd.",
            "{a}       .....   {p}.;dk:.   {p}lkk.  {a}:;,",
            "             {p}:kkkkkkkdoxkkx",
            "              ,c,,;;;:xkkd.",
            "                ;kkkkl...",
            "                ;kkkkl",
            "                 ,od;",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[38;5;220m",
    },
    Logo {
        name: "endeavouros",
        raw_lines: &[
            "{p}                     ./{a}o{a}.",
            "{p}                   ./{a}sssso{a}-",
            "{p}                 `:{a}osssssss+{a}-",
            "{p}               `:+{a}sssssssssso{a}/.",
            "{p}             `-/o{a}ssssssssssssso{a}/.",
            "{p}           `-/+{a}sssssssssssssssso{a}+:`",
            "{p}         `-:/+{a}sssssssssssssssssso{a}+/.",
            "{p}       `.://o{a}sssssssssssssssssssso{a}++-",
            "{p}      .://+{a}ssssssssssssssssssssssso{a}++:",
            "{p}    .:///o{a}ssssssssssssssssssssssssso{a}++:",
            "{p}  `:////{a}ssssssssssssssssssssssssssso{a}+++.",
            "{p}`-////+{a}ssssssssssssssssssssssssssso{a}++++-",
            "{p} `..-+{a}oosssssssssssssssssssssssso{a}+++++/`",
            "   ./++++++++++++++++++++++++++++++/:.",
            "  `:::::::::::::::::::::::::------``",
        ],
        primary_color: "\x1b[38;5;127m",
        accent_color: "\x1b[38;5;197m",
    },
    Logo {
        name: "manjaro",
        raw_lines: &[
            "{p}██████████████████  ████████",
            "██████████████████  ████████",
            "██████████████████  ████████",
            "██████████████████  ████████",
            "████████            ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
        ],
        primary_color: "\x1b[38;5;34m",
        accent_color: "\x1b[38;5;48m",
    },
    Logo {
        name: "generic",
        raw_lines: &[
            "{a}        #####",
            "{a}       #######",
            "{a}       ##{p}O{a}#{p}O{a}##",
            "{a}       #{a}#####{a}#",
            "{a}     ##{p}##{a}###{p}##{a}##",
            "{a}    #{p}##########{a}##",
            "{a}   #{p}############{a}##",
            "{a}   #{p}############{a}###",
            "{a}  ##{a}#{p}###########{a}##{a}#",
            "{a}######{a}#{p}#######{a}#{a}######",
            "{a}#######{a}#{p}#####{a}#{a}#######",
            "{a}  #####{a}#######{a}#####",
        ],
        primary_color: "\x1b[38;5;220m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "opensuse",
        raw_lines: &[
            "{a}           .;ldkO0000Okdl;.",
            "       .;d00xl:^''''''^:ok00d;.",
            "     .d00l'                'o00d.",
            "   .d0Kd'{p}  Okxol:;,.          {a}:O0d.",
            "  .OK{p}KKK0kOKKKKKKKKKKOxo:,      {a}lKO.",
            " ,0K{p}KKKKKKKKKKKKKKK0P^{a},,,{p}^dx:{a}    ;00,",
            ".OK{p}KKKKKKKKKKKKKKKk'{a}.oOPPb.{p}'0k.{a}   cKO.",
            ":KK{p}KKKKKKKKKKKKKKK: {a}kKx..dd {p}lKd{a}   'OK:",
            "dKK{p}KKKKKKKKKOx0KKKd {a}^0KKKO' {p}kKKc{a}   dKd",
            "dKK{p}KKKKKKKKKK;.;oOKx,..{a}^{p}..;kKKK0.{a}  dKd",
            ":KK{p}KKKKKKKKKK0o;...^cdxxOK0O/^^'  {a}.0K:",
            " kKK{p}KKKKKKKKKKKKK0x;,,......,;od  {a}lKk",
            " '0K{p}KKKKKKKKKKKKKKKKKKKK00KKOo^  {a}c00'",
            "  'kK{p}KKOxddxkOO00000Okxoc;''   {a}.dKk'",
            "    l0Ko.                    .c00l'",
            "     'l0Kk:.              .;xK0l'",
            "        'lkK0xl:;,,,,;:ldO0kl'",
            "            '^:ldxkkkkxdl:^'",
        ],
        primary_color: "\x1b[38;5;71m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "alpine",
        raw_lines: &[
            "{p}       .hddddddddddddddddddddddh.",
            "      :dddddddddddddddddddddddddd:",
            "     /dddddddddddddddddddddddddddd/",
            "    +dddddddddddddddddddddddddddddd+",
            "  `sdddddddddddddddddddddddddddddddds`",
            " `ydddddddddddd++hdddddddddddddddddddy`",
            ".hddddddddddd+`  `+ddddh:-sdddddddddddh.",
            "hdddddddddd+`      `+y:    .sddddddddddh",
            "ddddddddh+`   `//`   `.`     -sddddddddd",
            "ddddddh+`   `/hddh/`   `:s-    -sddddddd",
            "ddddh+`   `/+/dddddh/`   `+s-    -sddddd",
            "ddd+`   `/o` :dddddddh/`   `oy-    .yddd",
            "hdddyo+ohddyosdddddddddho+oydddy++ohdddh",
            ".hddddddddddddddddddddddddddddddddddddh.",
            " `yddddddddddddddddddddddddddddddddddy`",
            "  `sdddddddddddddddddddddddddddddddds`",
            "    +dddddddddddddddddddddddddddddd+",
            "     /dddddddddddddddddddddddddddd/",
            "      :dddddddddddddddddddddddddd:",
            "       .hddddddddddddddddddddddh.",
        ],
        primary_color: "\x1b[38;5;32m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "gentoo",
        raw_lines: &[
            "{p}         -/oyddmdhs+:.",
            "     -o{a}dNMMMMMMMMNNmhy+{p}-`",
            "   -y{a}NMMMMMMMMMMMNNNmmdhy{p}+-",
            " `o{a}mMMMMMMMMMMMMNmdmmmmddhhy{p}/`",
            " om{a}MMMMMMMMMMMN{p}hhyyyo{a}hmdddhhhd{p}o`",
            ".y{a}dMMMMMMMMMMd{p}hs++so/s{a}mdddhhhhdm{p}+`",
            " oy{a}hdmNMMMMMMMN{p}dyooy{a}dmddddhhhhyhN{p}d.",
            "  :o{a}yhhdNNMMMMMMMNNNmmdddhhhhhyym{p}Mh",
            "    .:{a}+sydNMMMMMNNNmmmdddhhhhhhmM{p}my",
            "       /m{a}MMMMMMNNNmmmdddhhhhhmMNh{p}s:",
            "    `o{a}NMMMMMMMNNNmmmddddhhdmMNhs{p}+`",
            "  `s{a}NMMMMMMMMNNNmmmdddddmNMmhs{p}/.",
            " /N{a}MMMMMMMMNNNNmmmdddmNMNdso{p}:`",
            "+M{a}MMMMMMNNNNNmmmmdmNMNdso{p}/-",
            "yM{a}MNNNNNNNmmmmmNNMmhs+/{p}-`",
            "/h{a}MMNNNNNNNNMNdhs++/{p}-`",
            "`/{a}ohdmmddhys+++/:{p}.`",
            "  `-//////:--.",
        ],
        primary_color: "\x1b[38;5;141m",
        accent_color: "\x1b[38;5;225m",
    },
    Logo {
        name: "void",
        raw_lines: &[
            "{p}                __.;=====;.__",
            "            _.=+==++=++=+=+===;.",
            "             -=+++=+===+=+=+++++=_",
            "        .     -=:``     `--==+=++==.",
            "       _vi,    `            --+=++++:",
            "      .uvnvi.       _._       -==+==+.",
            "     .vvnvnI`    .;==|==;.     :|=||=|.",
            "{a}+QmQQm{p}pvvnv; {a}_yYsyQQWUUQQQm #QmQ#{p}:{a}QQQWUV$QQm.",
            "{a} -QQWQW{p}pvvo{a}wZ?.wQQQE{p}==<{a}QWWQ/QWQW.QQWW{p}(: {a}jQWQE",
            "{a}  -$QQQQmmU'  jQQQ@{p}+=<{a}QWQQ)mQQQ.mQQQC{p}+;{a}jWQQ@'",
            "{a}   -$WQ8Y{p}nI:   {a}QWQQwgQQWV{p}`{a}mWQQ.jQWQQgyyWW@!",
            "{p}     -1vvnvv.     `~+++`        ++|+++",
            "      +vnvnnv,                 `-|===",
            "       +vnvnvns.           .      :=-",
            "        -Invnvvnsi..___..=sv=.     `",
            "          +Invnvnvnnnnnnnnvvnn;.",
            "            ~|Invnvnvvnvvvnnv}+`",
            "               -~|{*l}*|~",
        ],
        primary_color: "\x1b[38;5;35m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "pop",
        raw_lines: &[
            "{p}             `.-:::-.`",
            "         -+ydmNNNNNNNmdy+-",
            "      .+dNmdhs+//////+shdmdo.",
            "    .smmy+-`             ./sdy:",
            "  `omdo.    `.-/+osssso+/-` `+dy.",
            " `yms.   `:shmNmdhsoo++osyyo-``oh.",
            " hm/   .odNmds/.`    ``.....:::-+s",
            "/m:  `+dNmy:`   `./oyhhhhyyooo++so",
            "ys  `yNmy-    .+hmmho:-.`     ```",
            "s:  yNm+`   .smNd+.",
            "`` /Nm:    +dNd+`",
            "   yN+   `smNy.",
            "   dm    oNNy`",
            "   hy   -mNm.",
            "   +y   oNNo",
            "   `y`  sNN:",
            "    `:  +NN:",
            "     `  .mNo",
            "         /mm`",
            "          /my`",
            "           .sy`",
            "             .+:",
            "                `",
        ],
        primary_color: "\x1b[38;5;37m",
        accent_color: "\x1b[38;5;214m",
    },
    Logo {
        name: "nixos",
        raw_lines: &[
            "{p}          ▗▄▄▄       {a}▗▄▄▄▄    ▄▄▄▖",
            "{p}          ▜███▙       {a}▜███▙  ▟███▛",
            "{p}           ▜███▙       {a}▜███▙▟███▛",
            "{p}            ▜███▙       {a}▜██████▛",
            "{p}     ▟█████████████████▙ {a}▜████▛     {p}▟▙",
            "{p}    ▟███████████████████▙ {a}▜███▙    {p}▟██▙",
            "{a}           ▄▄▄▄▖           ▜███▙  {p}▟███▛",
            "{a}          ▟███▛             ▜██▛ {p}▟███▛",
            "{a}         ▟███▛               ▜▛ {p}▟███▛",
            "{a}▟███████████▛                  {p}▟██████████▙",
            "{a}▜██████████▛                  {p}▟███████████▛",
            "{a}      ▟███▛ {p}▟▙               ▟███▛",
            "{a}     ▟███▛ {p}▟██▙             ▟███▛",
            "{a}    ▟███▛  {p}▜███▙           ▝▀▀▀▀",
            "{a}    ▜██▛    {p}▜███▙ {a}▜██████████████████▛",
            "{a}     ▜▛     {p}▟████▙ {a}▜████████████████▛",
            "{p}           ▟██████▙       {a}▜███▙",
            "{p}          ▟███▛▜███▙       {a}▜███▙",
            "{p}         ▟███▛  ▜███▙       {a}▜███▙",
            "{p}         ▝▀▀▀    ▀▀▀▀▘       {a}▀▀▀▘",
        ],
        primary_color: "\x1b[38;5;75m",
        accent_color: "\x1b[38;5;117m",
    },
    Logo {
        name: "kali",
        raw_lines: &[
            "{p}..............",
            "            ..,;:ccc,.",
            "          ......''';lxO.",
            ".....''''..........,:ld;",
            "           .';;;:::;,,.x,",
            "      ..'''.            0Xxoc:,.  ...",
            "  ....                ,ONkc;,;cokOdc',.",
            " .                   OMo           ':{a}dd{p}o.",
            "                    dMc               :OO;",
            "                    0M.                 .:o.",
            "                    ;Wd",
            "                     ;XO,",
            "                       ,d0Odlc;,..",
            "                           ..',;:cdOOd::,.",
            "                                    .:d;.':;.",
            "                                       'd,  .'",
            "                                         ;l   ..",
            "                                          .o",
            "                                            c",
            "                                            .'",
            "                                             .",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;196m",
    },
    Logo {
        name: "freebsd",
        raw_lines: &[
            "   {a}```                        {p}`",
            "  {a}` `.....---...{p}....--.```   -/",
            "  {a}+o   .--`         {p}/y:`      +.",
            "  {a} yo`:.            {p}:o      `+-",
            "    {a}y/               {p}-/`   -o/",
            "   {a}.-                  {p}::/sy+:.",
            "   {a}/                     {p}`--  /",
            "  {a}`:                          {p}:`",
            "  {a}`:                          {p}:`",
            "   {a}/                          {p}/",
            "   {a}.-                        {p}-.",
            "    {a}--                      {p}-.",
            "     {a}`:`                  {p}`:`",
            "       .--             `--.",
            "          .---.....----.",
        ],
        primary_color: "\x1b[38;5;196m",
        accent_color: "\x1b[38;5;208m",
    },
    Logo {
        name: "slackware",
        raw_lines: &[
            "{p}                  :::::::",
            "            :::::::::::::::::::",
            "         :::::::::::::::::::::::::",
            "       ::::::::{a}cllcccccllllllll{p}::::::",
            "    :::::::::{a}lc               dc{p}:::::::",
            "   ::::::::{a}cl   clllccllll    oc{p}:::::::::",
            "  :::::::::{a}o   lc{p}::::::::{a}co   oc{p}::::::::::",
            " ::::::::::{a}o    cccclc{p}:::::{a}clcc{p}::::::::::::",
            " :::::::::::{a}lc        cclccclc{p}:::::::::::::",
            "::::::::::::::{a}lcclcc          lc{p}::::::::::::",
            "::::::::::{a}cclcc{p}:::::{a}lccclc     oc{p}:::::::::::",
            "::::::::::{a}o    l{p}::::::::::{a}l    lc{p}:::::::::::",
            " :::::{a}cll{p}:{a}o     clcllcccll     o{p}:::::::::::",
            " :::::{a}occ{p}:{a}o                  clc{p}:::::::::::",
            "  ::::{a}ocl{p}:{a}ccslclccclclccclclc{p}:::::::::::::",
            "   :::{a}oclcccccccccccccllllllllllllll{p}:::::",
            "    ::{a}lcc1lcccccccccccccccccccccccco{p}::::",
            "      ::::::::::::::::::::::::::::::::",
            "        ::::::::::::::::::::::::::::",
            "           ::::::::::::::::::::::",
            "                ::::::::::::",
        ],
        primary_color: "\x1b[38;5;61m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "artix",
        raw_lines: &[
            "{p}                   '",
            "                  'o'",
            "                 'ooo'",
            "                'ooxoo'",
            "               'ooxxxoo'",
            "              'oookkxxoo'",
            "             'oiioxkkxxoo'",
            "            ':;:iiiioxxxoo'",
            "               `'.;::ioxxoo'",
            "          '-.      `':;jiooo'",
            "         'oooio-..     `'i:io'",
            "        'ooooxxxxoio:,.   `'-;'",
            "       'ooooxxxxxkkxoooIi:-.  `'",
            "      'ooooxxxxxkkkkxoiiiiiji'",
            "     'ooooxxxxxkxxoiiii:'`     .i'",
            "    'ooooxxxxxoi:::'`       .;ioxo'",
            "   'ooooxooi::'`         .:iiixkxxo'",
            "  'ooooi:'`                `'';ioxxo'",
            " 'i:'`                          '':io'",
            "'`                                   `'",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[38;5;123m",
    },
    Logo {
        name: "zorin",
        raw_lines: &[
            "{p}        `osssssssssssssssssssso`",
            "       .osssssssssssssssssssssso.",
            "      .+oooooooooooooooooooooooo+.",
            "",
            "",
            "  `::::::::::::::::::::::.         .:`",
            " `+ssssssssssssssssss+:.`     `.:+ssso`",
            ".ossssssssssssssso/.       `-+ossssssso.",
            "ssssssssssssso/-`      `-/osssssssssssss",
            ".ossssssso/-`      .-/ossssssssssssssso.",
            " `+sss+:.      `.:+ssssssssssssssssss+`",
            "  `:.         .::::::::::::::::::::::`",
            "",
            "",
            "      .+oooooooooooooooooooooooo+.",
            "       -osssssssssssssssssssssso-",
            "        `osssssssssssssssssssso`",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[38;5;231m",
    },
    Logo {
        name: "windows11",
        raw_lines: &[
            "{p}                                ..,",
            "                    ....,,:;+ccllll",
            "      ...,,+:;  cllllllllllllllllll",
            ",cclllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "`'ccllllllllll  lllllllllllllllllll",
            "       `' \\*::  :ccllllllllllllllll",
            "                       ````''*::cll",
            "                                 ``",
        ],
        primary_color: "\x1b[38;5;39m",
        accent_color: "\x1b[38;5;45m",
    },
    Logo {
        name: "windows10",
        raw_lines: &[
            "{p}                                ..,",
            "                    ....,,:;+ccllll",
            "      ...,,+:;  cllllllllllllllllll",
            ",cclllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "llllllllllllll  lllllllllllllllllll",
            "`'ccllllllllll  lllllllllllllllllll",
            "       `' \\*::  :ccllllllllllllllll",
            "                       ````''*::cll",
            "                                 ``",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;39m",
    },
    Logo {
        name: "windows7",
        raw_lines: &[
            "{p}        ,.=:!!t3Z3z.,",
            "       :tt:::tt333EE3",
            "{p}       Et:::ztt33EEEL{a} @Ee.,      ..,",
            "{p}      ;tt:::tt333EE7{a} ;EEEEEEttttt33#",
            "{p}     :Et:::zt333EEQ.{a} $EEEEEttttt33QL",
            "{p}     it::::tt333EEF{a} @EEEEEEttttt33F",
            "{p}    ;3=*^```\"*4EEV{a} :EEEEEEttttt33@.",
            "{a}    ,.=::::!t=., {p}`{a} @EEEEEEtttz33QF",
            "{a}   ;::::::::zt33){a}   \"4EEEtttji3P*",
            "{a}  :t::::::::tt33.{a}:Z3z..{a}  ``{a} ,..g.",
            "{a}  i::::::::zt33F{a} AEEEtttt::::ztF",
            "{a} ;:::::::::t33V{a} ;EEEttttt::::t3",
            "{a} E::::::::zt33L{a} @EEEtttt::::z3F",
            "{a}{3=*^```\"*4E3){a} ;EEEtttt:::::tZ`",
            "{a}             `{a} :EEEEtttt::::z7",
            "                 \"VEzjt:;;z>*`",
        ],
        primary_color: "\x1b[38;5;33m",
        accent_color: "\x1b[38;5;220m",
    },
];

/// Resolves a matching `Logo` based on the detected OS string or user override.
pub fn match_logo(
    logo_override: Option<&str>,
    distro_id: &str,
    distro_like: &[String],
) -> Option<&'static Logo> {
    if let Some(name) = logo_override {
        let name_lower = name.to_lowercase();
        if name_lower == "none" || name_lower == "off" {
            return None;
        }
        for logo in ALL_LOGOS {
            if logo.name.eq_ignore_ascii_case(&name_lower) {
                return Some(logo);
            }
        }
        // Aliases
        if name_lower == "mint" {
            return ALL_LOGOS.iter().find(|l| l.name == "linuxmint");
        }
        if name_lower == "tux" || name_lower == "linux" {
            return ALL_LOGOS.iter().find(|l| l.name == "generic");
        }
        if name_lower == "win11" || name_lower == "windows11" || name_lower == "windows" {
            return ALL_LOGOS.iter().find(|l| l.name == "windows11");
        }
        if name_lower == "win10" || name_lower == "windows10" {
            return ALL_LOGOS.iter().find(|l| l.name == "windows10");
        }
        if name_lower == "win7" || name_lower == "windows7" {
            return ALL_LOGOS.iter().find(|l| l.name == "windows7");
        }
        return ALL_LOGOS.iter().find(|l| l.name == "ferris");
    }

    let distro_id_lower = distro_id.to_lowercase();

    for logo in ALL_LOGOS {
        if logo.name.eq_ignore_ascii_case(&distro_id_lower) {
            return Some(logo);
        }
    }

    // Heuristics for distro_id
    if distro_id_lower.contains("ubuntu") {
        return ALL_LOGOS.iter().find(|l| l.name == "ubuntu");
    }
    if distro_id_lower.contains("arch") {
        return ALL_LOGOS.iter().find(|l| l.name == "arch");
    }
    if distro_id_lower.contains("fedora") {
        return ALL_LOGOS.iter().find(|l| l.name == "fedora");
    }
    if distro_id_lower.contains("debian") {
        return ALL_LOGOS.iter().find(|l| l.name == "debian");
    }
    if distro_id_lower.contains("mint") {
        return ALL_LOGOS.iter().find(|l| l.name == "linuxmint");
    }
    if distro_id_lower.contains("suse") {
        return ALL_LOGOS.iter().find(|l| l.name == "opensuse");
    }
    if distro_id_lower.contains("gentoo") {
        return ALL_LOGOS.iter().find(|l| l.name == "gentoo");
    }
    if distro_id_lower.contains("void") {
        return ALL_LOGOS.iter().find(|l| l.name == "void");
    }
    if distro_id_lower.contains("pop") {
        return ALL_LOGOS.iter().find(|l| l.name == "pop");
    }
    if distro_id_lower.contains("nix") {
        return ALL_LOGOS.iter().find(|l| l.name == "nixos");
    }
    if distro_id_lower.contains("manjaro") {
        return ALL_LOGOS.iter().find(|l| l.name == "manjaro");
    }
    if distro_id_lower.contains("alpine") {
        return ALL_LOGOS.iter().find(|l| l.name == "alpine");
    }
    if distro_id_lower.contains("kali") {
        return ALL_LOGOS.iter().find(|l| l.name == "kali");
    }
    if distro_id_lower.contains("freebsd") {
        return ALL_LOGOS.iter().find(|l| l.name == "freebsd");
    }
    if distro_id_lower.contains("rhel") || distro_id_lower.contains("redhat") {
        return ALL_LOGOS.iter().find(|l| l.name == "rhel");
    }
    if distro_id_lower.contains("rocky") {
        return ALL_LOGOS.iter().find(|l| l.name == "rocky");
    }
    if distro_id_lower.contains("alma") {
        return ALL_LOGOS.iter().find(|l| l.name == "almalinux");
    }
    if distro_id_lower.contains("endeavour") {
        return ALL_LOGOS.iter().find(|l| l.name == "endeavouros");
    }
    if distro_id_lower.contains("slackware") {
        return ALL_LOGOS.iter().find(|l| l.name == "slackware");
    }
    if distro_id_lower.contains("artix") {
        return ALL_LOGOS.iter().find(|l| l.name == "artix");
    }
    if distro_id_lower.contains("zorin") {
        return ALL_LOGOS.iter().find(|l| l.name == "zorin");
    }
    if distro_id_lower.contains("windows 11") {
        return ALL_LOGOS.iter().find(|l| l.name == "windows11");
    }
    if distro_id_lower.contains("windows 10") {
        return ALL_LOGOS.iter().find(|l| l.name == "windows10");
    }
    if distro_id_lower.contains("windows 7") {
        return ALL_LOGOS.iter().find(|l| l.name == "windows7");
    }
    if distro_id_lower.contains("windows") {
        return ALL_LOGOS.iter().find(|l| l.name == "windows11");
    }

    // Try distro_like fallbacks
    for like in distro_like {
        let like_lower = like.to_lowercase();
        for logo in ALL_LOGOS {
            if logo.name.eq_ignore_ascii_case(&like_lower) {
                return Some(logo);
            }
        }
        if like_lower.contains("ubuntu") {
            return ALL_LOGOS.iter().find(|l| l.name == "ubuntu");
        }
        if like_lower.contains("arch") {
            return ALL_LOGOS.iter().find(|l| l.name == "arch");
        }
        if like_lower.contains("fedora") {
            return ALL_LOGOS.iter().find(|l| l.name == "fedora");
        }
        if like_lower.contains("debian") {
            return ALL_LOGOS.iter().find(|l| l.name == "debian");
        }
        if like_lower.contains("rhel") {
            return ALL_LOGOS.iter().find(|l| l.name == "rhel");
        }
        if like_lower.contains("suse") {
            return ALL_LOGOS.iter().find(|l| l.name == "opensuse");
        }
    }

    // Default fallback to generic Linux or ferris crab
    ALL_LOGOS
        .iter()
        .find(|l| l.name == "generic")
        .or_else(|| ALL_LOGOS.iter().find(|l| l.name == "ferris"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_match_logo_direct() {
        let logo = match_logo(Some("ubuntu"), "ubuntu", &[]).unwrap();
        assert_eq!(logo.name, "ubuntu");
    }

    #[test]
    fn test_match_logo_like_fallback() {
        let logo = match_logo(None, "my_custom_distro", &["ubuntu".to_string()]).unwrap();
        assert_eq!(logo.name, "ubuntu");
    }

    #[test]
    fn test_match_logo_none_override() {
        assert!(match_logo(Some("none"), "ubuntu", &[]).is_none());
        assert!(match_logo(Some("off"), "ubuntu", &[]).is_none());
    }

    #[test]
    fn test_match_logo_override() {
        let logo = match_logo(Some("arch"), "ubuntu", &[]).unwrap();
        assert_eq!(logo.name, "arch");
    }

    #[test]
    fn test_match_logo_unknown_fallback_to_ferris() {
        let logo = match_logo(None, "unknown_distro", &[]).unwrap();
        assert_eq!(logo.name, "generic");
    }

    #[test]
    fn test_match_logo_windows() {
        let logo = match_logo(None, "windows 11", &[]).unwrap();
        assert_eq!(logo.name, "windows11");
    }
}
