# sued

> ⚠️ sued is in early stages of development.
> It does *work*, but it's not really very stable.

A text editor that works and is an editor.

sued is a line editor, similar to the venerable and standard ed text editor,
just simpler and not nearly as powerful.

Syntax highlighting? Code analysis? *Modal editing?* ***Cursor positioning??***

Who the hell cares? Just shut up and edit.

Written in Rust, because frick you, that's why.

## Questions

+ Why a line editor?
    1. I like ed. I find it extremely pleasant to use, despite its terseness. I
      wanted to try my hand at writing a similar text editor, just a lot less
      complex.
    2. I don't want to go insane trying to develop a Vim-like editor. Especially
      not when projects like [Helix](https://github.com/helix-editor/helix)
      exist.
+ If it's ed-like, why not copy ed's syntax?
  + sued is actually more modelled after my earlier attempt at a line editor,
    [Streakline](https://github.com/That1M8Head/Streakline).
  + In Streakline, editing is straightforward. You open the editor and start
    typing. It used commands prefixed with `~`, so sued does too.
+ What does sued stand for?
  + It stands for "shut up editor". This can be read as "shut up and edit", "the shut up editor"
    or "Shut up, editor!"
+ How do you pronounce sued?
  + "soo-ed". Don't pronounce it "sood". Otherwise people are going to think
    a text editor's going to go to court(!)
+ Now that you're working on [QVSED](https://github.com/That1M8Head/QVSED),
  will sued be forgotten about?
  + Pfft. No. QVSED is a *graphical* editor with a completely different paradigm,
    and it isn't a sued replacement.

## Commands

All commands start with `~`. Run `~` by itself to see a list of commands.

+ `~save [filename]` - Write the buffer contents to the provided file name.
+ `~open [filename]` - Find or create a file and open its contents up in sued.
+ `~show` - Display the buffer contents, complete with line numbers.
+ `~replace [line]` - Interactively replace the chosen line's contents in the buffer.
+ `~delete [line]` - Immediately remove a line from the buffer.
+ `~run [command]` - Run the provided executable or shell builtin. Real executables will be prioritised over shell builtins.
+ `~exit` - Quit sued, discarding the buffer contents.
+ `~help` - Display some information about sued.
