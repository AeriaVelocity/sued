# sued - shut up editor

sued is a vector-oriented line editor, kind of similar to the venerable and standard ed text editor, just simpler and not nearly as powerful.

~~Yes, I know ed doesn't use vectors, shut up~~

![Screenshot](screenshot.png)

Syntax highlighting? Code analysis? *Modal editing?* ***Cursor positioning??***

Who the hell cares? Just shut up and edit.

Written in Rust, because frick you, that's why.

## Obtaining

```bash
cargo install sued
```

and make sure that `~/.cargo/bin/` is in your PATH or you won't be able to run it until it is.

## Questions

+ Why a line editor?
    1. I like ed. It's really nice, and the minimalism is really what sells it.
       I wanted to try my hand at writing a similar text editor, just a lot less
       complex.
    2. I don't want to go insane trying to develop a Vim-like editor. Especially
       not when projects like [Helix](https://github.com/helix-editor/helix)
       exist.
+ If it's ed-like, why not copy ed's syntax?
  + ed has a lot of commands, and like I said, I want sued to be more user-friendly.
  + Furthermore, sued inherits its command prefix (~) from my earlier project,
    [Streakline](https://github.com/That1M8Head/Streakline).
+ What does sued stand for?
  + It stands for "shut up editor," which alludes to how not-in-your-way it is.
+ How do you pronounce sued?
  + "soo-ed". Don't pronounce it "sood," it's not a law thing, and has nothing
    to do with GitHub Copilot. (The context behind that statement is weird.)
+ Now that you're working on [QVSED](https://github.com/That1M8Head/QVSED),
  will sued be forgotten about?
  + Pfft. No. QVSED is a *graphical* editor with a completely different paradigm,
    and it isn't a sued replacement. It's just another project.

## Commands

All commands start with `~`. Run `~` by itself to see a list of commands.

+ `~clear` - Clear the file buffer.
+ `~save [filename]` - Write the buffer contents to the provided file name.
+ `~open [filename]` - Find or create a file and open its contents up in sued.
+ `~show [start] [end]` - Display the buffer contents, complete with line numbers. You can specify the start and end point if you wish.
+ `~insert [line]` - Interactively insert text into the chosen position in the buffer.
+ `~replace [line]` - Interactively replace the chosen line's contents in the buffer.
+ `~substitute [line] [pattern]/[replacement]` - Perform a regex substitution on the chosen line number, with the pattern and replacement.
+ `~search [term]` - Searches for the given term in the buffer and prints matching lines.
+ `~indent [line] [level]` - Indent the specified line by a number of spaces. A negative level will outdent.
+ `~swap [source] [target]` - Swap two lines with each other in the buffer.
+ `~delete [line]` - Immediately remove a line from the buffer.
+ `~run [command]` - Run the provided executable or shell builtin. Real executables will be prioritised over shell builtins.
+ `~exit` - Quit sued, discarding the buffer contents.
+ `~help` - Display the list of commands with descriptions.
+ `~about` - Display some information about sued.
