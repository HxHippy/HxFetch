use crate::logo_config::LogoConfig;

pub struct AsciiArt;

impl AsciiArt {
    pub fn get_logo(os_name: &str) -> Vec<String> {
        let os_lower = os_name.to_lowercase();

        // Strategy 1: Check for user-defined logos in TOML config
        let config = LogoConfig::load();
        if let Some(custom_logo) = config.get_logo(&os_lower) {
            return custom_logo.lines.clone();
        }

        // Strategy 2: Fall back to built-in logos
        // Try to find a logo function that matches the OS name
        // This allows for flexible matching - if someone creates a "my_custom_os_logo" function
        // and their OS contains "my_custom_os", it will automatically match

        // Clean the OS name for function matching (remove spaces, special chars)
        let clean_os = os_lower
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();

        // Try multiple matching strategies for maximum flexibility

        // Built-in logo matching
        if os_lower.contains("macos") || os_lower.contains("mac os") || os_lower.contains("darwin")
        {
            return Self::macos_logo();
        }
        if os_lower.contains("ubuntu") {
            return Self::ubuntu_logo();
        }
        if os_lower.contains("debian") {
            return Self::windows_subsystem_for_linux_apt_arch_linux_debian_linux_github_installation_spiral_theme_logos_logo();
        }
        if os_lower.contains("arch") {
            return Self::arch_logo();
        }
        if os_lower.contains("fedora") {
            return Self::fedora_logo();
        }
        if os_lower.contains("mint") {
            return Self::mint_logo();
        }
        if os_lower.contains("manjaro") {
            return Self::manjaro_logo();
        }
        if os_lower.contains("opensuse") {
            return Self::opensuse_logo();
        }
        if os_lower.contains("centos") {
            return Self::centos_logo();
        }
        if os_lower.contains("alpine") {
            return Self::alpine_logo();
        }
        if os_lower.contains("kali") {
            return Self::kali_logo();
        }
        if os_lower.contains("gentoo") {
            return Self::gentoo_logo();
        }
        if os_lower.contains("nixos") {
            return Self::nixos_logo();
        }
        if os_lower.contains("void") {
            return Self::void_logo();
        }
        if os_lower.contains("endeavour") {
            return Self::endeavour_logo();
        }

        // Strategy 3: If no custom or built-in match found, use generic logo
        // Provide helpful guidance for creating custom logos

        Self::generic_logo_with_hint(&clean_os)
    }

    fn ubuntu_logo() -> Vec<String> {
        vec![
            "            .-/+oossssoo+\\-.".to_string(),
            "        ´:+ssssssssssssssssss+:`".to_string(),
            "      -+ssssssssssssssssssyyssss+-".to_string(),
            "    .ossssssssssssssssssdMMMNysssso.".to_string(),
            "   /ssssssssssshdmmNNmmyNMMMMhssssss\\".to_string(),
            "  +ssssssssshmydMMMMMMMNddddyssssssss+".to_string(),
            " /sssssssshNMMMyhhhhhhmNMMMNhssssssss\\".to_string(),
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss.".to_string(),
            "+sssshhhyNMMNysssssssssssyNMMMysssssss+".to_string(),
            "ossyNMMMNyMMhsssssssssssssshmmmhssssssso".to_string(),
            "ossyNMMMNyMMhsssssssssssssshmmmhssssssso".to_string(),
            "+sssshhhyNMMNysssssssssssyNMMMysssssss+".to_string(),
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss.".to_string(),
            " \\sssssssshNMMMyhhhhhhdNMMMNhssssssss/".to_string(),
            "  +sssssssssdmydMMMMMMMMddddyssssssss+".to_string(),
            "   \\ssssssssssshdmNNNNmyNMMMMhssssss/".to_string(),
            "    .ossssssssssssssssssdMMMNysssso.".to_string(),
            "      -+sssssssssssssssssyyssss+-".to_string(),
            "        ´:+ssssssssssssssssss+:`".to_string(),
            "            .-\\+oossssoo+/-.".to_string(),
        ]
    }

    fn arch_logo() -> Vec<String> {
        vec![
            "                   -`".to_string(),
            "                  .o+`".to_string(),
            "                 `ooo/".to_string(),
            "                `+oooo:".to_string(),
            "               `+oooooo:".to_string(),
            "               -+oooooo+:".to_string(),
            "             `/:-:++oooo+:".to_string(),
            "            `/++++/+++++++:".to_string(),
            "           `/++++++++++++++:".to_string(),
            "          `/+++oooooooooooo/`".to_string(),
            "         ./ooosssso++osssssso+`".to_string(),
            "        .oossssso-````/ossssss+`".to_string(),
            "       -osssssso.      :ssssssso.".to_string(),
            "      :osssssss/        osssso+++.".to_string(),
            "     /ossssssss/        +ssssooo/-".to_string(),
            "   `/ossssso+/:-        -:/+osssso+-".to_string(),
            "  `+sso+:-`                 `.-/+oso:".to_string(),
            " `++:.                           `-/+/".to_string(),
            " .`                                 `/".to_string(),
        ]
    }

    fn fedora_logo() -> Vec<String> {
        vec![
            "        ,'''''.".to_string(),
            "       |   ,.  |".to_string(),
            "       |  |  '_'".to_string(),
            "   ,...|  |..".to_string(),
            " .'  ,_;|   ..'".to_string(),
            "|  |   |  |".to_string(),
            "|  ',_,'  |".to_string(),
            " '.     .'".to_string(),
            "   '''''".to_string(),
        ]
    }

    fn mint_logo() -> Vec<String> {
        vec![
            "             ...-:::::-...".to_string(),
            "          .-MMMMMMMMMMMMMMM-.".to_string(),
            "      .-MMMM`.:-::-.`MMMM-.".to_string(),
            "    .:MMMM.:MMMMMMMMM.`MMMM:.".to_string(),
            "   -MMM-M---MMMMMMMMM---M-MMM-".to_string(),
            "  `:MMM:MM  `MMMMMMM`  MM:MMM:`".to_string(),
            "   :MMM:MMM  -MM  MM-  MMM:MMM:".to_string(),
            "    .MMM.MMM-     -MMM.MMM.".to_string(),
            "     `MMM:MMMM._.MMMM:MMM`".to_string(),
            "      .MMMM:`MMMMM`:MMMM.".to_string(),
            "       -MMM-:MMMMMMM:-MMM-".to_string(),
            "        .`-MMMMMMMMMM-`.".to_string(),
            "           `MMMMMMM`".to_string(),
            "            .MMMMM.".to_string(),
            "             `--`".to_string(),
        ]
    }

    fn manjaro_logo() -> Vec<String> {
        vec![
            "██████████████████  ████████".to_string(),
            "██████████████████  ████████".to_string(),
            "██████████████████  ████████".to_string(),
            "██████████████████  ████████".to_string(),
            "████████            ████████".to_string(),
            "████████  ████████  ████████".to_string(),
            "████████  ████████  ████████".to_string(),
            "████████  ████████  ████████".to_string(),
            "████████  ████████  ████████".to_string(),
            "████████  ████████  ████████".to_string(),
            "████████  ████████  ████████".to_string(),
            "████████  ████████  ████████".to_string(),
        ]
    }

    fn opensuse_logo() -> Vec<String> {
        vec![
            "           .;ldkO0000Okdl;.".to_string(),
            "       .;d00xl:^^^^^^:ok00d;.".to_string(),
            "     .d00l                 o00d.".to_string(),
            "   .d0Kd   Okxol:;,.          :O0d.".to_string(),
            "  .OK0yc  ,l.                   d0O.".to_string(),
            "  ,0K.                           0K,".to_string(),
            "  dKd    .,oKO.                .dKd".to_string(),
            "  dKd    ;KOOOOkkOkd:.           .dKd".to_string(),
            "   Ok    ;kOkk0OOOOl.            .xO".to_string(),
            "   ;00    cdOOOOkkOkd:       .d0;".to_string(),
            "    kOx        :ldO0NNK;     .lOk".to_string(),
            "     ;00d;.        .:dONK: ..;d00;".to_string(),
            "      lO00xl:;,.  .;ldko:;:ldO0l".to_string(),
            "        .;lxO00OOOOkkxolc:oo:..".to_string(),
            "             .;ldooollc;..".to_string(),
        ]
    }

    fn centos_logo() -> Vec<String> {
        vec![
            "      .----.".to_string(),
            "    .' ,-.  '.".to_string(),
            "   /  /   \\   \\".to_string(),
            "  |  |     |   |".to_string(),
            "  |   \\   /   |".to_string(),
            "   \\   '-'   /".to_string(),
            "    '.     .'".to_string(),
            "      '---'".to_string(),
        ]
    }

    fn generic_logo() -> Vec<String> {
        vec![
            "        #####".to_string(),
            "       #######".to_string(),
            "       ##O#O##".to_string(),
            "       #VVVVV#".to_string(),
            "     ##  VVV  ##".to_string(),
            "    #          ##".to_string(),
            "   #            ##".to_string(),
            "  #              ##".to_string(),
            "  #               ##".to_string(),
            "  ##             ###".to_string(),
            "  ###           ####".to_string(),
            "   ###         #####".to_string(),
            "   ####      ######".to_string(),
            "    #####  #######".to_string(),
            "     ############".to_string(),
            "      ##########".to_string(),
            "        ######".to_string(),
            "         ###".to_string(),
        ]
    }

    fn generic_logo_with_hint(clean_os_name: &str) -> Vec<String> {
        let mut logo = Self::generic_logo();

        // If we have a clean OS name, provide a hint for creating a custom logo
        if !clean_os_name.is_empty() && clean_os_name != "unknown" {
            // Add a helpful comment at the end
            logo.push(String::new());
            logo.push("# Create a custom logo with:".to_string());
            logo.push("# hxfetch --ascii-maker".to_string());
            logo.push(format!("# Use '{}' as OS name", clean_os_name));
        }

        logo
    }

    fn macos_logo() -> Vec<String> {
        vec![
            "                    'c.".to_string(),
            "                 ,xNMM.".to_string(),
            "               .OMMMMo".to_string(),
            "               OMMM0,".to_string(),
            "     .;loddo:' loolloddol;.".to_string(),
            "   cKMMMMMMMMMMNWMMMMMMMMMM0:".to_string(),
            " .KMMMMMMMMMMMMMMMMMMMMMMMWd.".to_string(),
            " XMMMMMMMMMMMMMMMMMMMMMMMX.".to_string(),
            ";MMMMMMMMMMMMMMMMMMMMMMMM:".to_string(),
            ":MMMMMMMMMMMMMMMMMMMMMMMM:".to_string(),
            ".MMMMMMMMMMMMMMMMMMMMMMMMX.".to_string(),
            " kMMMMMMMMMMMMMMMMMMMMMMMMWd.".to_string(),
            " .XMMMMMMMMMMMMMMMMMMMMMMMMMMk".to_string(),
            "  .XMMMMMMMMMMMMMMMMMMMMMMMMK.".to_string(),
            "    kMMMMMMMMMMMMMMMMMMMMMMd".to_string(),
            "     ;KMMMMMMMWXXWMMMMMMMk.".to_string(),
            "       .cooc,.    .,coo:.".to_string(),
        ]
    }

    fn alpine_logo() -> Vec<String> {
        vec![
            "       .hddddddddddddddddddddddh.".to_string(),
            "      :dddddddddddddddddddddddddd:".to_string(),
            "     /dddddddddddddddddddddddddddd/".to_string(),
            "    +dddddddddddddddddddddddddddddd+".to_string(),
            "  `sdddddddddddddddddddddddddddddddds`".to_string(),
            " `ydddddddddddd++hdddddddddddddddddddy`".to_string(),
            ".hddddddddddd+`  `+ddddh:-sdddddddddddh.".to_string(),
            "hdddddddddd+`      `+y:    .sddddddddddh".to_string(),
            "ddddddddh+`   `//`   `.`     `+dddddddddd".to_string(),
            "ddddddh+`    `/hddh/`   `:s-    `+hddddddd".to_string(),
            "ddddh+`      `/ddddh/`    +s`     `+hdddd".to_string(),
            "ddd+`        `/ddddh/`     `        `+ddd".to_string(),
            "h+`          `/ddddh/`               `+h".to_string(),
            "`            `/ddddh/`                 `".to_string(),
            "              /ddddh/".to_string(),
            "               /ddh/".to_string(),
            "                //".to_string(),
        ]
    }

    fn kali_logo() -> Vec<String> {
        vec![
            "..............".to_string(),
            "            ..,;:ccc,.".to_string(),
            "          ......''';lxO.".to_string(),
            ".....''''..........,:ld;".to_string(),
            "           .';;;:::;,,.x,".to_string(),
            "      ..'''.            0Xxoc:,.  ...".to_string(),
            "  ....                ,ONkc;,;cokOdc',.".to_string(),
            " .                   OMo           ':ddo.".to_string(),
            "                    dMc               :OO;".to_string(),
            "                    0M.                 .:o.".to_string(),
            "                    ;Wd".to_string(),
            "                     ;XO,".to_string(),
            "                       ,d0Odlc;,..".to_string(),
            "                           ..',;:cdOOd::,.".to_string(),
            "                                    .:d;.':;.".to_string(),
            "                                       'd,  .'".to_string(),
            "                                         ;l   ..".to_string(),
            "                                          .o".to_string(),
            "                                            c".to_string(),
            "                                            .'".to_string(),
            "                                             .".to_string(),
        ]
    }

    fn gentoo_logo() -> Vec<String> {
        vec![
            "         -/oyddmdhs+:.".to_string(),
            "     -odNMMMMMMMMNNmhy+-`".to_string(),
            "   -yNMMMMMMMMMMMNNNmmdhy+-".to_string(),
            " `omMMMMMMMMMMMMNmdmmmmddhhy/`".to_string(),
            " omMMMMMMMMMMMNhhyyyohmdddhhhdo`".to_string(),
            ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`".to_string(),
            " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.".to_string(),
            "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh".to_string(),
            "    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy".to_string(),
            "       /mMMMMMMNNNmmmdddhhhhhmMNhs:".to_string(),
            "    `oNMMMMMMMNNNmmmddddhhdmMNhs+`".to_string(),
            "  `sNMMMMMMMMNNNmmmdddddmNMmhs/.".to_string(),
            " /NMMMMMMMMNNNNmmmdddmNMNdso:`".to_string(),
            "+MMMMMMMNNNNNmmmmdmNMNdso/-".to_string(),
            "yMMNNNNNNNmmmmmNNMmhs+/-`".to_string(),
            "/hMMNNNNNNNNMNdhs++/-`".to_string(),
            "`/ohdmmddhys+++/:.`".to_string(),
            "  `-//////:--.".to_string(),
        ]
    }

    fn nixos_logo() -> Vec<String> {
        vec![
            "          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖".to_string(),
            "          ▜███▙       ▜███▙  ▟███▛".to_string(),
            "           ▜███▙       ▜███▙▟███▛".to_string(),
            "            ▜███▙       ▜██████▛".to_string(),
            "     ▟█████████████████▙ ▜████▛     ▟▙".to_string(),
            "    ▟███████████████████▙ ▜███▙    ▟██▙".to_string(),
            "           ▄▄▄▄▖           ▜███▙  ▟███▛".to_string(),
            "          ▟███▛             ▜██▛ ▟███▛".to_string(),
            "         ▟███▛               ▜▛ ▟███▛".to_string(),
            "▟███████████▛                  ▟██████████▙".to_string(),
            "▜██████████▛                  ▟███████████▛".to_string(),
            "      ▟███▛ ▟▙               ▟███▛".to_string(),
            "     ▟███▛ ▟██▙             ▟███▛".to_string(),
            "    ▟███▛  ▜███▙           ▝▀▀▀▀".to_string(),
            "    ▜██▛    ▜███▙ ▜██████████████████▛".to_string(),
            "     ▜▛     ▟████▙ ▜████████████████▛".to_string(),
            "           ▟██████▙       ▜███▙".to_string(),
            "          ▟███▛▜███▙       ▜███▙".to_string(),
            "         ▟███▛  ▜███▙       ▜███▙".to_string(),
            "         ▝▀▀▀    ▀▀▀▀▘       ▀▀▀▘".to_string(),
        ]
    }

    fn void_logo() -> Vec<String> {
        vec![
            "                __.;=====;.__".to_string(),
            "            _.=+==++=++=+=+===;.".to_string(),
            "             -=+++=+===+=+=+++++=_".to_string(),
            "        .     -=:``     `--==+=++==.".to_string(),
            "       _vi,    `            --+=++++:".to_string(),
            "      .uvnvi.       _._       -==+==+.".to_string(),
            "     .vvnvnI`    .;==|==;.     :|=||=|.".to_string(),
            "chel. iMU2\".   .;:==|==;.:    ;==+==+{".to_string(),
            "     xMMu    .:+++=+=+=++:       .:-.".to_string(),
            "     mNM;  .:=::+====+====:       `:`".to_string(),
            "     dM'   .:;=++++++++++:         .'".to_string(),
            "     d\"    ':++=+==+++=.".to_string(),
            "           .'-+=+=:.".to_string(),
        ]
    }

    fn endeavour_logo() -> Vec<String> {
        vec![
            "                     .o+`".to_string(),
            "                    `ooo/".to_string(),
            "                   `+oooo:".to_string(),
            "                  `+oooooo:".to_string(),
            "                  -+oooooo+:".to_string(),
            "                `/:-:++oooo+:".to_string(),
            "               `/++++/+++++++:".to_string(),
            "              `/++++++++++++++:".to_string(),
            "             `/+++ooooooooo+++/`".to_string(),
            "            ./ooosssso++osssssso+`".to_string(),
            "           .oossssso-````/ossssss+`".to_string(),
            "          -osssssso.      :ssssssso.".to_string(),
            "         :osssssss/        osssso+++.".to_string(),
            "        /ossssssss/        +ssssooo/-".to_string(),
            "      `/ossssso+/:-        -:/+osssso+-".to_string(),
            "     `+sso+:-`                 `.-/+oso:".to_string(),
            "    `++:.                           `-/+/".to_string(),
            "    .`   ████████ ████████   ████████".to_string(),
            "         ██                 ██      ██".to_string(),
            "         ████████           ██      ██".to_string(),
            "         ██                 ██      ██".to_string(),
            "         ████████           ████████".to_string(),
        ]
    }

    fn windows_subsystem_for_linux_apt_arch_linux_debian_linux_github_installation_spiral_theme_logos_logo(
    ) -> Vec<String> {
        vec![
            "[38;2;230;230;230mMMMMMM[38;2;230;227;228mM[38;2;227;171;190mw[38;2;222;110;150mX[38;2;219;64;119mf[38;2;217;32;98m)[38;2;216;13;85m}[38;2;215;7;82m[[38;2;216;14;86m}[38;2;217;33;99m([38;2;219;66;120mj[38;2;223;112;151mX[38;2;227;174;192mq[38;2;230;228;228mM[38;2;230;230;230mMMMMMM[0m".to_string(),
            "[38;2;230;230;230mMMMM[38;2;230;221;224m#[38;2;224;125;160mJ[38;2;217;29;96m)[38;2;215;6;81m[[[[[[[[[[[[38;2;217;32;98m)[38;2;224;130;163mC[38;2;230;223;225m#[38;2;230;230;230mMMMM[0m".to_string(),
            "[38;2;230;230;230mMMM[38;2;227;170;190mw[38;2;217;33;98m([38;2;215;6;81m[[[[[[[38;2;215;10;83m[[38;2;215;9;83m[[38;2;215;6;81m[[[[[[[[38;2;217;37;101m([38;2;227;176;194mq[38;2;230;230;230mMMM[0m".to_string(),
            "[38;2;230;230;230mMM[38;2;224;136;167mL[38;2;215;8;82m[[38;2;215;6;81m[[[[[38;2;216;14;87m}[38;2;230;104;149mX[38;2;245;196;213ma[38;2;253;252;253m@[38;2;250;232;238m8[38;2;250;228;236m&[38;2;245;198;214ma[38;2;236;139;173mQ[38;2;226;77;130mx[38;2;215;8;82m[[38;2;215;6;81m[[[[38;2;215;9;83m[[38;2;225;146;173m0[38;2;230;230;230mMM[0m".to_string(),
            "[38;2;230;230;230mM[38;2;226;154;179mO[38;2;215;7;82m[[38;2;215;6;81m[[[[[38;2;224;66;122mj[38;2;249;226;234mWW[38;2;236;141;174m0[38;2;222;53;113mt[38;2;217;21;91m{[38;2;217;20;90m{[38;2;222;52;112m/[38;2;234;132;168mL[38;2;252;244;247mB[38;2;246;202;217mo[38;2;219;35;101m([38;2;215;6;81m[[[[38;2;215;9;83m[[38;2;226;163;185mm[38;2;230;230;230mM[0m".to_string(),
            "[38;2;229;210;216mo[38;2;216;18;89m{[38;2;215;6;81m[[[[[38;2;221;45;107m\\[38;2;251;237;242m%[38;2;233;120;160mJ[38;2;217;20;90m{[38;2;215;6;81m[[[[[[[38;2;223;56;115mt[38;2;252;244;247mB[38;2;241;178;200md[38;2;215;6;81m[[[[[38;2;217;23;92m1[38;2;229;214;219mo[0m".to_string(),
            "[38;2;223;114;152mY[38;2;215;6;81m[[[[[38;2;215;9;83m[[38;2;248;216;227mM[38;2;227;86;136mu[38;2;215;6;81m[[[[38;2;220;37;102m|[38;2;227;87;136mu[38;2;224;65;121mj[38;2;220;39;103m|[38;2;215;8;82m[[38;2;215;6;81m[[38;2;235;136;171mQ[38;2;237;153;182mZ[38;2;215;11;84m[[38;2;215;6;81m[[[[[38;2;223;119;156mU[0m".to_string(),
            "[38;2;218;45;107m\\[38;2;215;6;81m[[[[[38;2;223;59;117mf[38;2;245;200;216ma[38;2;215;6;81m[[[[38;2;217;18;89m{[38;2;230;102;146mz[38;2;215;6;81m[[[[[[38;2;229;93;141mc[38;2;243;185;205mb[38;2;215;6;81m[[[[[[38;2;218;47;108m\\[0m".to_string(),
            "[38;2;216;12;85m}[38;2;215;6;81m[[[[[38;2;224;65;121mj[38;2;239;158;186mm[38;2;215;6;81m[[[[38;2;222;54;113mt[38;2;227;86;136mu[38;2;215;6;81m[[[[[[38;2;233;121;161mJ[38;2;229;99;144mz[38;2;215;6;81m[[[[[[38;2;216;13;85m}[0m".to_string(),
            "[38;2;216;12;85m}[38;2;215;6;81m[[[[[38;2;223;57;116mt[38;2;241;171;196mp[38;2;215;6;81m[[[[38;2;215;9;83m[[38;2;236;142;174m0[38;2;219;30;97m)[38;2;215;6;81m[[[38;2;215;8;82m[[38;2;226;80;132mn[38;2;236;141;174m0[38;2;215;8;82m[[38;2;215;6;81m[[[[[[38;2;216;13;85m}[0m".to_string(),
            "[38;2;218;47;108m\\[38;2;215;6;81m[[[[[38;2;216;13;86m}[38;2;248;219;229mM[38;2;218;27;95m)[38;2;215;6;81m[[[[38;2;218;25;93m1[38;2;234;131;166mL[38;2;232;117;157mU[38;2;230;106;149mX[38;2;232;118;158mU[38;2;224;62;119mf[38;2;215;6;81m[[[[[[[[38;2;219;49;109m/[0m".to_string(),
            "[38;2;223;117;154mY[38;2;215;6;81m[[[[[[38;2;234;128;165mC[38;2;248;219;229mM[38;2;215;8;82m[[38;2;215;6;81m[[[[[38;2;215;8;82m[[38;2;215;10;83m[[38;2;215;6;81m[[[[[[[[[[38;2;224;123;158mU[0m".to_string(),
            "[38;2;229;211;218mo[38;2;216;20;90m{[38;2;215;6;81m[[[[[38;2;216;10;83m[[38;2;242;180;202md[38;2;231;110;153mY[38;2;215;6;81m[[[[[[[[[[[[[[[38;2;217;26;94m1[38;2;229;216;221m*[0m".to_string(),
            "[38;2;230;230;230mM[38;2;226;159;182mZ[38;2;215;8;82m[[38;2;215;6;81m[[[[[38;2;216;10;84m[[38;2;236;141;175m0[38;2;234;132;168mL[38;2;217;17;89m{[38;2;215;6;81m[[[[[[[[[[[[38;2;215;10;83m[[38;2;227;169;189mw[38;2;230;230;230mM[0m".to_string(),
            "[38;2;230;230;230mMM[38;2;225;143;172mQ[38;2;215;9;83m[[38;2;215;6;81m[[[[[[38;2;221;45;107m\\[38;2;231;112;154mY[38;2;227;85;135mu[38;2;218;29;96m)[38;2;215;6;81m[[[[[[[[[38;2;215;11;84m[[38;2;225;152;178mO[38;2;230;230;230mMM[0m".to_string(),
            "[38;2;230;230;230mMMM[38;2;227;176;194mq[38;2;217;38;102m|[38;2;215;6;81m[[[[[[[[38;2;216;11;84m}[38;2;215;10;83m[[38;2;215;6;81m[[[[[[[38;2;218;43;105m|[38;2;228;183;198md[38;2;230;230;230mMMM[0m".to_string(),
            "[38;2;230;230;230mMMMM[38;2;230;223;225m#[38;2;224;132;165mC[38;2;217;35;100m([38;2;215;6;81m[[[[[[[[[[[[38;2;218;38;102m|[38;2;224;138;169mL[38;2;230;225;227m#[38;2;230;230;230mMMMM[0m".to_string(),
            "[38;2;230;230;230mMMMMMM[38;2;230;228;229mM[38;2;227;176;194mq[38;2;223;114;152mY[38;2;219;66;120mj[38;2;217;33;98m([38;2;216;13;85m}[38;2;215;7;82m[[38;2;216;14;86m}[38;2;217;34;99m([38;2;220;68;122mj[38;2;223;116;154mY[38;2;227;180;196mp[38;2;230;229;229mM[38;2;230;230;230mMMMMMM[0m".to_string(),
        ]
    }
}
