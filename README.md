# layshift
A small linux clipboard tool for converting text between keyboard layouts.

![preview](preview.gif)

## How it works
It read the current text from clipboard, maps it to the target layout, and writes the result back to the clipboard.

```text
text -copy/cut-> clipboard -wl-paste-> layshift -wl-copy-> clipboard -paste-> result
```

## Commands
### map
Map text between two layouts

```bash
layshift map <source> <target>
```

- By setting default layouts, you can use `map` command without specifying them:

```bash
layshift map
```

- For faster and easier use, it is recommended to bind layshift to a keyboard shortcut.

### set-default
Set the default source and target layouts:

```bash
layshift set-default <source> <target>
```

- The layout configuration will save in `~/.config/layshift/config.toml`

### list
Lists all available languages:

```bash
laysift list
```

List layouts available for a language:

```bash
laysift list <language>
```

- Languages can be specified by both **name** and **symbol**.
- When using the language **name**, you should enter it using lowercase letters only.

## Layouts
Layouts are identified using:

```text
<language>:<variant>
```

- If your layout is not supported, just add it as a json file😁️.

```json
{
    "normal": ["`", "1", ...],
    "shift": ["~", "!", ...]
}
```

## Installation
### Install from source code
Clone the repository or download the latest release then Go to the project directory and run this `just` command:

```
just install
```

- You might need `Rust compiler` and `just` installed.
- This method may require superuser privileges.
