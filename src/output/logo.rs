use crate::output::color::RESET;

#[derive(Debug, Clone)]
pub struct Logo {
    pub name: &'static str,
    pub raw_lines: &'static [&'static str],
    pub primary_color: &'static str,
}

impl Logo {
    /// Returns the logo lines formatted with single-tone ANSI colors if enabled.
    pub fn render_lines(&self, enable_color: bool) -> Vec<String> {
        self.raw_lines
            .iter()
            .map(|line| {
                let clean = line
                    .replace("{p}", "")
                    .replace("{a}", "")
                    .replace("{0}", "");
                if enable_color && !clean.is_empty() {
                    format!("{}{}{}", self.primary_color, clean, RESET)
                } else {
                    clean
                }
            })
            .collect()
    }
}

pub const ALL_LOGOS: &[Logo] = &[
    Logo {
        name: "ferris",
        raw_lines: &[
            "        /^^^\\     /^^^\\",
            "       (  O  )   (  O  )",
            "      .-'---'-----'---'-.",
            "     /   _~^~^~^~^~_     \\",
            "    |   /  o     o  \\     |",
            "    |   |     -     |     |",
            "    |   \\  '---'  /     |",
            "     \\   '-------'     /",
            "      '-._'-------'_.-'",
            "          /       \\",
        ],
        primary_color: "\x1b[38;5;208m",
    },
    Logo {
        name: "debian",
        raw_lines: &[
            "       _,met$$$$$gg.",
            "    ,g$$$$$$$$$$$$$$$P.",
            "  ,g$$P\"        \"\"\"Y$$.\".",
            " ,$$P'              `$$$.",
            "',$$P       ,ggs.     `$$b:",
            "`d$$'     ,$P\"'   .    $$$",
            " $$P      d$'     ,    $$P",
            " $$:      $$.   -    ,d$$'",
            " $$;      Y$b._   _,d$P'",
            " Y$$.    `.`\"Y$$$$P\"'",
            " `$$b      \"-.__",
            "  `Y$$",
            "   `Y$$.",
            "     `$$b.",
            "       `Y$$b.",
            "          `\"Y$b._",
            "              `\"\"\"",
        ],
        primary_color: "\x1b[38;5;196m",
    },
    Logo {
        name: "ubuntu",
        raw_lines: &[
            "            .-/+oossssoo+\\-.",
            "        ´:+ssssssssssssssssss+:`",
            "      -+ssssssssssssssssssyyssss+-",
            "    .ossssssssssssssssssdMMMNysssso.",
            "   /ssssssssssshdmmNNmmyNMMMMhssssss\\",
            "  +ssssssssshmydMMMMMMMNddddyssssssss+",
            " /sssssssshNMMMyhhyyyyhmNMMMNhssssssss\\",
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss.",
            "+sssshhhyNMMNyssssssssssssyNMMMysssssss+",
            "ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
            "ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
            "+sssshhhyNMMNyssssssssssssyNMMMysssssss+",
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss.",
            " \\sssssssshNMMMyhhyyyyhdNMMMNhssssssss/",
            "  +sssssssssdmydMMMMMMMMddddyssssssss+",
            "   \\ssssssssssshdmNNNNmyNMMMMhssssss/",
            "    .ossssssssssssssssssdMMMNysssso.",
            "      -+sssssssssssssssssyyyssss+-",
            "        `:+ssssssssssssssssss+:`",
            "            .-\\+oossssoo+/-.",
        ],
        primary_color: "\x1b[38;5;208m",
    },
    Logo {
        name: "linuxmint",
        raw_lines: &[
            "             ...-:::::-...",
            "          .-MMMMMMMMMMMMMMM-.",
            "      .-MMMM`..-:::::::-..`MMMM-.",
            "    .:MMMM.:MMMMMMMMMMMMMMM:.MMMM:.",
            "   -MMM-M---MMMMMMMMMMMMMMMMMMM.MMM-",
            " `:MMM:MM`  :MMMM:....::-...-MMMM:MMM:`",
            " :MMM:MMM`  :MM:`  ``    ``  `:MMM:MMM:",
            ".MMM.MMMM`  :MM.  -MM.  .MM-  `MMMM.MMM.",
            ":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:",
            ":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM:MMM:",
            ":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:",
            ".MMM.MMMM`  :MM:--:MM:--:MM:  `MMMM.MMM.",
            " :MMM:MMM-  `-MMMMMMMMMMMM-`  -MMM-MMM:",
            "  :MMM:MMM:`                `:MMM:MMM:",
            "   .MMM.MMMM:--------------:MMMM.MMM.",
            "     '-MMMM.-MMMMMMMMMMMMMMM-.MMMM-'",
            "       '.-MMMM``--:::::--``MMMM-.'",
            "            '-MMMMMMMMMMMMM-'",
            "               ``-:::::-``",
        ],
        primary_color: "\x1b[38;5;46m",
    },
    Logo {
        name: "fedora",
        raw_lines: &[
            "             .',;::::;,'.",
            "         .';:cccccccccccc:;,.",
            "      .;cccccccccccccccccccccc;.",
            "    .:cccccccccccccccccccccccccc:.",
            "  .;ccccccccccccc;.:dddl:.;ccccccc;.",
            " .:ccccccccccccc;OWMKOOXMWd;ccccccc:.",
            ".:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.",
            ",cccccccccccccc;MMM.;cc;;WW:;cccccccc,",
            ":cccccccccccccc;MMM.;cccccccccccccccc:",
            ":ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:",
            "cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;",
            "ccccc;XM0';cccc;MMM.;cccccccccccccccc'",
            "ccccc;MMo;ccccc;MMW.;ccccccccccccccc;",
            "ccccc;0MNc.ccc.xMMd;ccccccccccccccc;",
            "cccccc;dNMWXXXWM0:;cccccccccccccc:,",
            "cccccccc;.:odl:.;cccccccccccccc:,.",
            ":cccccccccccccccccccccccccccc:'.",
            ".:cccccccccccccccccccccc:;,..",
            "  '::cccccccccccccc::;,.",
        ],
        primary_color: "\x1b[38;5;33m",
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
            "  `+sso+:-`                 `.-/+oso:",
            " `++:.                           `-/+/",
            " .`                                 `/",
        ],
        primary_color: "\x1b[38;5;67m",
    },
    Logo {
        name: "rhel",
        raw_lines: &[
            "             `.-..........`",
            "            `////////::.`-/.",
            "            -: ....-////////.",
            "            //:-::///////////`",
            "     `--::: `-://////////////:",
            "     //////-    ``.-:///////// .`",
            "     `://////:-.`    :///////::///:`",
            "       .-/////////:---/////////////:",
            "          .-://////////////////////.",
            "         yMN+`.-::///////////////-`",
            "      .-`:NMMNMs`  `..-------..`",
            "       MN+/mMMMMMhoooyysshsss",
            "MMM    MMMMMMMMMMMMMMyyddMMM+",
            " MMMM   MMMMMMMMMMMMMNdyNMMh`     hyhMMM",
            "  MMMMMMMMMMMMMMMMyoNNNMMM+.   MMMMMMMM",
            "   MMNMMMNNMMMMMNM+ mhsMNyyyyMNMMMMsMM",
        ],
        primary_color: "\x1b[38;5;196m",
    },
    Logo {
        name: "rocky",
        raw_lines: &[
            "    `-/+++++++++/-.`",
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
    },
    Logo {
        name: "almalinux",
        raw_lines: &[
            "         'c:.",
            "        lkkkx, ..       ..   ,cc,",
            "        okkkk:ckkx'  .lxkkx.okkkkd",
            "        .:llcokkx'  :kkkxkko:xkkd,",
            "      .xkkkkdood:  ;kx,  .lkxlll;",
            "       xkkx.       xk'     xkkkkk:",
            "       'xkx.       xd      .....,.",
            "      .. :xkl'     :c      ..''..",
            "    .dkx'  .:ldl:'. '  ':lollldkkxo;",
            "  .''lkkko'                     ckkkx.",
            "'xkkkd:kkd.       ..  ;'        :kkxo.",
            ",xkkkd;kk'      ,d;    ld.   ':dkd::cc,",
            " .,,.;xkko'.';lxo.      dx,  :kkk'xkkkkc",
            "     'dkkkkkxo:.        ;kx  .kkk:;xkkd.",
            "       .....   .;dk:.   lkk.  :;,",
            "             :kkkkkkkdoxkkx",
            "              ,c,,;;;:xkkd.",
            "                ;kkkkl...",
            "                ;kkkkl",
            "                 ,od;",
        ],
        primary_color: "\x1b[38;5;39m",
    },
    Logo {
        name: "endeavouros",
        raw_lines: &[
            "                     ./o.",
            "                   ./sssso-",
            "                 `:osssssss+-",
            "               `:+sssssssssso/.",
            "             `-/ossssssssssssso/.",
            "           `-/+sssssssssssssssso+:`",
            "         `-:/+sssssssssssssssssso+/.",
            "       `.://osssssssssssssssssssso++-",
            "      .://+ssssssssssssssssssssssso++:",
            "    .:///ossssssssssssssssssssssssso++:",
            "  `:////ssssssssssssssssssssssssssso+++.",
            "`-////+ssssssssssssssssssssssssssso++++-",
            " `..-+oosssssssssssssssssssssssso+++++/`",
            "   ./++++++++++++++++++++++++++++++/:.",
            "  `:::::::::::::::::::::::::------``",
        ],
        primary_color: "\x1b[38;5;127m",
    },
    Logo {
        name: "manjaro",
        raw_lines: &[
            "██████████████████  ████████",
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
    },
    Logo {
        name: "generic",
        raw_lines: &[
            "        #####",
            "       #######",
            "       ##O#O##",
            "       #######",
            "     ###########",
            "    #############",
            "   ###############",
            "   ################",
            "  #################",
            "#####################",
            "#####################",
            "  #################",
        ],
        primary_color: "\x1b[38;5;220m",
    },
    Logo {
        name: "opensuse",
        raw_lines: &[
            "           .;ldkO0000Okdl;.",
            "       .;d00xl:^''''''^:ok00d;.",
            "     .d00l'                'o00d.",
            "   .d0Kd'  Okxol:;,.          :O0d.",
            "  .OKKKK0kOKKKKKKKKKKOxo:,      lKO.",
            " ,0KKKKKKKKKKKKKKKK0P^,,,^dx:    ;00,",
            ".OKKKKKKKKKKKKKKKKk'.oOPPb.'0k.   cKO.",
            ":KKKKKKKKKKKKKKKKK: kKx..dd lKd   'OK:",
            "dKKKKKKKKKKKOx0KKKd ^0KKKO' kKKc   dKd",
            "dKKKKKKKKKKKK;.;oOKx,..^..;kKKK0.  dKd",
            ":KKKKKKKKKKKK0o;...^cdxxOK0O/^^'  .0K:",
            " kKKKKKKKKKKKKKKK0x;,,......,;od  lKk",
            " '0KKKKKKKKKKKKKKKKKKKKK00KKOo^  c00'",
            "  'kKKKOxddxkOO00000Okxoc;''   .dKk'",
            "    l0Ko.                    .c00l'",
            "     'l0Kk:.              .;xK0l'",
            "        'lkK0xl:;,,,,;:ldO0kl'",
            "            '^:ldxkkkkxdl:^'",
        ],
        primary_color: "\x1b[38;5;71m",
    },
    Logo {
        name: "alpine",
        raw_lines: &[
            "       .hddddddddddddddddddddddh.",
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
    },
    Logo {
        name: "gentoo",
        raw_lines: &[
            "         -/oyddmdhs+:.",
            "     -odNMMMMMMMMNNmhy+-`",
            "   -yNMMMMMMMMMMMNNNmmdhy+-",
            " `omMMMMMMMMMMMMNmdmmmmddhhy/`",
            " omMMMMMMMMMMMNhhyyyohmdddhhhdo`",
            ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`",
            " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.",
            "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh",
            "    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy",
            "       /mMMMMMMNNNmmmdddhhhhhmMNhs:",
            "    `oNMMMMMMMNNNmmmddddhhdmMNhs+`",
            "  `sNMMMMMMMMNNNmmmdddddmNMmhs/.",
            " /NMMMMMMMMNNNNmmmdddmNMNdso:`",
            "+MMMMMMMNNNNNmmmmdmNMNdso/-",
            "yMMNNNNNNNmmmmmNNMmhs+/-`",
            "/hMMNNNNNNNNMNdhs++/-`",
            "`/ohdmmddhys+++/:.`",
            "  `-//////:--.",
        ],
        primary_color: "\x1b[38;5;141m",
    },
    Logo {
        name: "void",
        raw_lines: &[
            "                __.;=====;.__",
            "            _.=+==++=++=+=+===;.",
            "             -=+++=+===+=+=+++++=_",
            "        .     -=:``     `--==+=++==.",
            "       _vi,    `            --+=++++:",
            "      .uvnvi.       _._       -==+==+.",
            "     .vvnvnI`    .;==|==;.     :|=||=|.",
            "+QmQQmpvvnv; _yYsyQQWUUQQQm #QmQ#:QQQWUV$QQm.",
            " -QQWQWpvvowZ?.wQQQE==<QWWQ/QWQW.QQWW(: jQWQE",
            "  -$QQQQmmU'  jQQQ@+=<QWQQ)mQQQ.mQQQC+;jWQQ@'",
            "   -$WQ8YnI:   QWQQwgQQWV`mWQQ.jQWQQgyyWW@!",
            "     -1vvnvv.     `~+++`        ++|+++",
            "      +vnvnnv,                 `-|===",
            "       +vnvnvns.           .      :=-",
            "        -Invnvvnsi..___..=sv=.     `",
            "          +Invnvnvnnnnnnnnvvnn;.",
            "            ~|Invnvnvvnvvvnnv}+`",
            "               -~|{*l}*|~",
        ],
        primary_color: "\x1b[38;5;35m",
    },
    Logo {
        name: "pop",
        raw_lines: &[
            "             `.-:::-.`",
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
    },
    Logo {
        name: "nixos",
        raw_lines: &[
            "          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖",
            "          ▜███▙       ▜███▙  ▟███▛",
            "           ▜███▙       ▜███▙▟███▛",
            "            ▜███▙       ▜██████▛",
            "     ▟█████████████████▙ ▜████▛     ▟▙",
            "    ▟███████████████████▙ ▜███▙    ▟██▙",
            "           ▄▄▄▄▖           ▜███▙  ▟███▛",
            "          ▟███▛             ▜██▛ ▟███▛",
            "         ▟███▛               ▜▛ ▟███▛",
            "▟███████████▛                  ▟██████████▙",
            "▜██████████▛                  ▟███████████▛",
            "      ▟███▛ ▟▙               ▟███▛",
            "     ▟███▛ ▟██▙             ▟███▛",
            "    ▟███▛  ▜███▙           ▝▀▀▀▀",
            "    ▜██▛    ▜███▙ ▜██████████████████▛",
            "     ▜▛     ▟████▙ ▜████████████████▛",
            "           ▟██████▙       ▜███▙",
            "          ▟███▛▜███▙       ▜███▙",
            "         ▟███▛  ▜███▙       ▜███▙",
            "         ▝▀▀▀    ▀▀▀▀▘       ▀▀▀▘",
        ],
        primary_color: "\x1b[38;5;75m",
    },
    Logo {
        name: "kali",
        raw_lines: &[
            "..............",
            "            ..,;:ccc,.",
            "          ......''';lxO.",
            ".....''''..........,:ld;",
            "           .';;;:::;,,.x,",
            "      ..'''.            0Xxoc:,.  ...",
            "  ....                ,ONkc;,;cokOdc',.",
            " .                   OMo           ':ddo.",
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
    },
    Logo {
        name: "freebsd",
        raw_lines: &[
            "   ```                        `",
            "  ` `.....---.......--.```   -/",
            "  +o   .--`         /y:`      +.",
            "   yo`:.            :o      `+-",
            "    y/               -/`   -o/",
            "   .-                  ::/sy+:.",
            "   /                     `--  /",
            "  `:                          :`",
            "  `:                          :`",
            "   /                          /",
            "   .-                        -.",
            "    --                      -.",
            "     `:`                  `:`",
            "       .--             `--.",
            "          .---.....----.",
        ],
        primary_color: "\x1b[38;5;196m",
    },
    Logo {
        name: "slackware",
        raw_lines: &[
            "                  :::::::",
            "            :::::::::::::::::::",
            "         :::::::::::::::::::::::::",
            "       ::::::::cllcccccllllllll::::::",
            "    :::::::::lc               dc:::::::",
            "   ::::::::cl   clllccllll    oc:::::::::",
            "  :::::::::o   lc::::::::co   oc::::::::::",
            " ::::::::::o    cccclc:::::clcc::::::::::::",
            " :::::::::::lc        cclccclc:::::::::::::",
            "::::::::::::::lcclcc          lc::::::::::::",
            "::::::::::cclcc:::::lccclc     oc:::::::::::",
            "::::::::::o    l::::::::::l    lc:::::::::::",
            " :::::cll:o     clcllcccll     o:::::::::::",
            " :::::occ:o                  clc:::::::::::",
            "  ::::ocl:ccslclccclclccclclc:::::::::::::",
            "   :::oclcccccccccccccllllllllllllll:::::",
            "    ::lcc1lcccccccccccccccccccccccco::::",
            "      ::::::::::::::::::::::::::::::::",
            "        ::::::::::::::::::::::::::::",
            "           ::::::::::::::::::::::",
            "                ::::::::::::",
        ],
        primary_color: "\x1b[38;5;61m",
    },
    Logo {
        name: "artix",
        raw_lines: &[
            "                   '",
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
    },
    Logo {
        name: "zorin",
        raw_lines: &[
            "        `osssssssssssssssssssso`",
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
    },
    Logo {
        name: "windows11",
        raw_lines: &[
            "                                ..,",
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
    },
    Logo {
        name: "windows10",
        raw_lines: &[
            "                                ..,",
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
    },
    Logo {
        name: "windows7",
        raw_lines: &[
            "        ,.=:!!t3Z3z.,",
            "       :tt:::tt333EE3",
            "       Et:::ztt33EEEL @Ee.,      ..,",
            "      ;tt:::tt333EE7 ;EEEEEEttttt33#",
            "     :Et:::zt333EEQ. $EEEEEttttt33QL",
            "     it::::tt333EEF @EEEEEEttttt33F",
            "    ;3=*^```\"*4EEV :EEEEEEttttt33@.",
            "    ,.=::::!t=., ` @EEEEEEtttz33QF",
            "   ;::::::::zt33)   \"4EEEtttji3P*",
            "  :t::::::::tt33.:Z3z..  `` ,..g.",
            "  i::::::::zt33F AEEEtttt::::ztF",
            " ;:::::::::t33V ;EEEttttt::::t3",
            " E::::::::zt33L @EEEtttt::::z3F",
            "{3=*^```\"*4E3) ;EEEtttt:::::tZ`",
            "             ` :EEEEtttt::::z7",
            "                 \"VEzjt:;;z>*`",
        ],
        primary_color: "\x1b[38;5;33m",
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
