# ronfmt - simple autoformatting tool for RON

# Important note:
The tool is in prealpha state. Bugs may occur, leading to loss of data. The tool creates automatic backups of files it works on, but take precautions when using it.
**The tool does not support comments at this time. When formatting, they will be discarded. Only use it if you don't have/care for comments in your files.**

## How to install
(requires nightly rustc)
`cargo install ronfmt`

## How to use
`ronfmt file_to_format.ron`

- On use, the tool will create a backup file called `<source_file_name>.bak` in the same directory. Only the latest backup is kept. Add `*.bak` to your `.gitignore` if you would like to keep your repo clean.
- Use `-d` flag to write the formatted output to the terminal instead of overwriting the source file
- Set tab size with `-t <size>` (4 by default)
- Set max line width with `-w <width>` (40 by default). This is a soft limit, so long or deeply-nested values may sometimes overrun it.