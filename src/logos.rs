pub fn print(os: &str) {
    print!("{}", render(os));
}

pub fn render(os: &str) -> String {
    let logo = match os.to_lowercase().as_str() {
        "ubuntu" => UBUNTU,
        "arch linux" => ARCH,
        _ => &[""],
    };
    logo.join("\n")
}

const ARCH: &[&str] = &[
    "\x1b[32m                                        ",
    "                   +*                   ",
    "                  *==*                  ",
    "                 #====#                 ",
    "                %======%                ",
    "               %+=+====+%               ",
    "              %%#++++++=+%              ",
    "              *+***+++++++%             ",
    "             *+++++++++++++%            ",
    "            ****++++++++++++%           ",
    "           *+++==============%          ",
    "         %+=======+**+========#         ",
    "        %========%    %+=======#        ",
    "       #========%      %========#       ",
    "      #========+        *=====+*+#      ",
    "     #=========+        +======+**%     ",
    "    *======+**#%        %#**+======*    ",
    "   *-=++*%%                  %%#*+=-+   ",
    "  *+#%                            %#++% ",
    " %%                                  %% \x1b[0m",
];

const UBUNTU: &[&str] = &[
    "\x1b[32m                            %=-::-*     ",
    "                %%#***++**%%:::::::*    ",
    "              %+=========-* :::::::*    ",
    "           #**%%===========#%+---=#     ",
    "         %*****%%+=++****++=+#####++%   ",
    "        #******+% %        %*========%  ",
    "       #*******%              #=======% ",
    "    %  #******                 %======+ ",
    " #+===+%%****                   #======#",
    "#=======%%***                    ######%",
    "#======= %***                   %======*",
    " %*+++*%%****%                  +::::::#",
    "       #+*****%                *::::::= ",
    "       %*******#%            %=::::::-  ",
    "         ******+# ##%%  %%#+-:::::::=   ",
    "          %****% +::::--:::::+***+=*    ",
    "            %*%%-::::::::::+%##**#%     ",
    "               *=-::::::::- *+****+#    ",
    "                  %%##**##% ******+#    ",
    "                             #****#     \x1b[0m",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_arch_linux() {
        let result = render("arch linux");
        assert!(result.contains("+*"));
        assert!(result.contains("*==*"));
        assert!(result.contains("#====#"));
        assert!(result.starts_with("\x1b[32m"));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn render_arch_case_insensitive() {
        let result = render("Arch Linux");
        assert!(result.contains("+*"));
    }

    #[test]
    fn render_ubuntu() {
        let result = render("ubuntu");
        assert!(result.contains("%=-::-*"));
        assert!(result.contains("%%#***++**%%"));
        assert!(result.starts_with("\x1b[32m"));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn render_unknown_os_returns_empty() {
        let result = render("fedora");
        assert_eq!(result, "");
    }

    #[test]
    fn render_empty_string_returns_empty() {
        let result = render("");
        assert_eq!(result, "");
    }
}
