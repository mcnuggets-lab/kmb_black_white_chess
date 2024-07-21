# KMB Black-White-Chess Solution Finder

Search-based solution finder for KMB's black-white-chess minigame.

## How to compile

This program is written in Rust, so you will need it to compile. A `cargo build` should suffice. **If you want the program to run faster, build it in release mode** using a `--release` flag.

If you are using Windows 11 (and if you trust me that the binary does not do harm to your machine), you can download a built binary in the [release](https://github.com/mcnuggets-lab/kmb_black_white_chess/releases) page.

## How to run

The program should prompt you to enter the number of rows and number of columns, as well as a "board string". This is a left-to-right, up-to-down description of the board using

- 0 = white
- 1 = black
- x = hole

For example, if the board looks like

![image](https://github.com/user-attachments/assets/712e3cff-1fb4-4419-9148-ef8ee84fe350)

Then the board string should be `"1100010x00x0x001"`.

### Note

This solution finder may not be fast enough to guarantee a win in the 7x7 (highest difficulty) board. It should be able to find a solution for any 6x6 board or smaller in a matter of seconds.
