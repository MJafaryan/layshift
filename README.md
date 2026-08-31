# layshift
A small Linux clipboard tool for converting text between keyboard layouts.
![preview](preview.gif)

## How it works
It reads the current text from clipboard, maps it to the target layout, and writes the result back to the clipboard.
```text
text -copy/cut-> clipboard -(wl-clipboard/xclip)-> layshift -(wl-clipboard/xclip)-> clipboard -paste-> result
```

## Installation
### Install from source code
Clone the repository or download the latest release then go to the project directory and run this `just` command:
```bash
just install
```
- You might need `Rust compiler` and `just` installed.
- This method may require superuser privileges.
- Depending on your display server, make sure the corresponding clipboard utility is installed:
  1. **Wayland:** `wl-clipboard`
  2. **X11:** `xclip`

## Layouts
Layouts can be specified in two ways:
### Built-in layouts
```text
<language>:<variant>
```

### Custom layouts
You can also provide a path to a layout JSON file.

- If your layout is not supported, just add it as a JSON file😁️.
```json
{
    "normal": ["`", "1", ...],
    "shift": ["~", "!", ...]
}
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
- The layout configuration will be saved in `~/.config/layshift/config.toml`

### list
Lists all available languages:
```bash
layshift list
```
List layouts available for a language:
```bash
layshift list <language>
```
- Languages can be specified by both **name** and **symbol**.
- When using the language **name**, you should enter it using lowercase letters only.
