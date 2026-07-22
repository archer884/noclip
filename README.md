# noclip

```shell
❯ noclip --help
Usage: noclip [COMMAND]

Commands:
  copy    copy text
  paste   past text
  x-copy  copy text but trim by default
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Yes, it's named for a cheatcode. That's probably its best feature.

Actual usage:

```shell
cat foo.txt | noclip # copy to clipboard
noclip v | some-command-here # paste from clipboard
```
