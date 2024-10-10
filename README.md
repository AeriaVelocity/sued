# sued - shut up editor

sued is a stateless vector-oriented command-based text editor written in Rust,
with focus on speed, simplicity, ease of use and staying the hell out of your
way. It's inspired by more contemporary editors, such as the ed family (ed, em,
ex, vi, Vim, Neovim, etc.).

No, it's not "super user editor", or an editor that will sue you. It's "soo-ed",
"shut up editor".

To be clear, it's a text editor that works on Rust's `Vec` type. To be clearer,
it's a text editor that works on lines and isn't graphical, like you're probably
used to.

~~Yes, I know ed doesn't use vectors, shut up~~

```sued
sued v0.13.2 - put that mouse AWAY
type ~ for commands, otherwise just start typing
sued can even run inside a readme!
jk this is just a copied-and-pasted terminal session
~show
1│sued can even run inside a readme!
2│jk this is just a copied-and-pasted terminal session
~sub 2 jk/just kidding
~show
1│sued can even run inside a readme!
2│just kidding this is just a copied-and-pasted terminal session
~clear
print("sued is fricking awesome " * 5)
~runhere python
running C:\Users\sonic\AppData\Local\Programs\Python\Python311\python.exe
sued is fricking awesome sued is fricking awesome sued is fricking awesome sued is fricking awesome sued is fricking awesome 
finished running python
# I didn't even need to save the file to do that
~show
1│print("sued is fricking awesome " * 5)
2│# I didn't even need to save the file to do that
~search save
line 2: # I didn't even need to save the file to do that
~save damn-thats-cool.py
saved to damn-thats-cool.py
~exit
```

Written in Rust, because frick you, that's why.

## Obtaining

First of all, you need Cargo. If you don't have Cargo, [install Rust.](https://www.rust-lang.org/learn/get-started) If you don't want to install Rust, frick off and use a different editor.

Run

```bash
cargo install sued
```

and make sure that `~/.cargo/bin/` is in your PATH or you won't be able to run it until it is.

## Documentation

sued's documentation is present [online](https://aeriavelocity.github.io/sued) in the form of a man page-like website.

You can also just read the source code yourself.

## Accelerated

sued is written in pure, idiomatic Rust.

Its speed, as a compiled line editor, is pretty damn fast. The time between opening it up and getting some text edited is negligently small.

You can even run a compiler or an interpreter with a simple `~runhere`, without even the need to save your file.

And it doesn't even take a second. As it should be.

## Uncomplicated

sued is much more user-friendly than ed - it's a modeless editor, so there's no complicated mode switching to keep in your head.

It also uses a simple command syntax, with whole words, like `~save`, `~show` and `~open`, not single letters.

Its error messages, despite being communicated concisely, are still informative and use colloquial language, so they're pretty easy to understand.

## Efficient

sued looks basic on the offset, but under the hood, it has support for regex replacements, file searching, pretty-looking line numbers, and some other stuff.

sued's command set consists of stuff like `delete`, `indent`, `insert`, `open`, `run`, `runhere`, `save`, `search`, `show`, `substitute`, `swap`, and so on.

For the rest of the commands, as well as information on what these commands do, run `~help` inside sued or check out the [documentation](https://aeriavelocity.github.io/sued).

And sued leverages [linefeed](https://github.com/murarth/linefeed) for its command line input. Meaning it supports GNU Readline commands and functionality. You can do some Emacs-style line navigation, like `C-a`, `C-e`, `C-f`, `C-b`, whatever you want, it'll work. Probably.

Remember how in the example above, I used `~runhere` to run Python without even needing to specify the file name, let alone save it? The `~runhere` command will have sued create a temporary file and run the chosen command with that temporary file as an argument. That's pretty damn efficient.

## No-nonsense

sued is designed to not get in your way. Error messages are useful but brief.

They're not ed levels of brief, but they're still pretty brief.

And there's no auto-save feature - your file is only saved when you enter the `~save` command.

## Ubiquitous

Being a CLI app, you can run sued anywhere!

In your terminal, in VS Code, in Emacs, even inside Vim! And sued doesn't care what OS it runs under. Windows, macOS, GNU/Linux, BusyBox/Linux, other kinds of Linux, BSD, ChromeOS, Android, probably Xbox if you really wanted.

If it has a terminal, it can run sued.

It's been tested on Windows, Fedora and Android, and works great under all three (except that Android doesn't support `~copy`, but that's not sued's fault)

## Free/Libre

sued is free. You can get it for free, but it's also free.

It's released under the GNU General Public License version 3 or later, because
free software, hell yeah!
