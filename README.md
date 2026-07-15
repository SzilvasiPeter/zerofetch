# minifetch

[![coverage](https://img.shields.io/endpoint?url=https://szilvasipeter.github.io/minifetch/coverage/badge.json)](https://szilvasipeter.github.io/minifetch/coverage/index.html)
[![crates.io](https://img.shields.io/crates/v/minifetch.svg)](https://crates.io/crates/minifetch)

A minimal, fast Linux system information tool written in Rust. Displays an ASCII logo alongside key system details like OS, hardware, desktop environment, memory, and more.

## Usage

```
$ minifetch

                   +*
                  *==*
                 #====#
                %======%
               %+=+====+%
              %%#++++++=+%
              *+***+++++++%
             *+++++++++++++%
            ****++++++++++++%
           *+++==============%
         %+=======+**+========#
        %========%    %+=======#
       #========%      %========#
      #========+        *=====+*+#
     #=========+        +======+**%
    *======+**#%        %#**+======*
   *-=++*%%                  %%#*+=-+
  *+#%                            %#++%
 %%                                  %%
OS: Arch Linux x86_64
Host: B550M K
Kernel: Linux 7.1.3-arch1-2
Uptime: 1 hours, 51 minutes
Packages: 722 (pacman)
Shell: bash 5.3.15
Display: ACR EK221Q H, 1920x1080 in 21", 60 Hz
DE: Sway
WM: wayland
WM Theme:
Theme: Adwaita
Icon: Adwaita
Font: Adwaita Sans 11
Cursor: Adwaita (24px)
Terminal: alacritty
CPU: AMD Ryzen 5 5600 6-Core Processor
GPU: Advanced Micro Devices, Inc. [AMD/ATI] Navi 23 [Radeon RX 6600/6600 XT/6600M]
Memory: 4.06 GiB / 15.53 GiB (26%)
Swap: 0.00 GiB / 7.77 GiB (0%)
Disk (/): 187.27 GiB / 456.39 GiB (41%) - ext4
Disk (/boot): 0.08 GiB / 1.00 GiB (7%) - vfat
Local IP (enp4s0): 192.168.1.74/24
Locale: en_US.UTF-8
```

## Installation

### From crates.io

```
cargo install minifetch
```

### Using cargo-binstall

```
cargo binstall minifetch
```

### From source

```
git clone https://github.com/SzilvasiPeter/minifetch.git
cd minifetch
cargo install --path .
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the [MIT License](LICENSE).
