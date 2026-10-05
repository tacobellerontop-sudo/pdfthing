# PDFThing app icon

<img src="printcraft-small.svg" alt="PDFThing app icon: a pencil drawing a coral stroke on a violet tile" width="128">

**Motif:** a white pencil, tip down-left, drawing a coral squiggle: PDFThing is for freehand drawing.

**Palette:**

| Colour | Hex | Used for |
|---|---|---|
| Violet (app colour) | `#8A6CFF` to `#4E2FCB` | the full-bleed tile (diagonal gradient) |
| Coral | `#FF8A66` | the stroke |
| White, lilac, peach, ink | `#FFFFFF` `#D9CCFF` `#FFD3C4` `#2B2340` | the pencil |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`. Windows and Linux icons use the full-bleed
tile. macOS icons put it on Apple's grid (an 824 px body centred on a transparent 1024 px canvas).

**Provenance:** a hand-written SVG made for this project (no third-party material). The file names still say
`printcraft` because the build and packaging refer to them. Licence: [LICENSE.txt](LICENSE.txt)
(`MIT OR Apache-2.0`, like the repo).

## Files

| File | What it is |
|---|---|
| `printcraft.svg` | the master vector; every PNG, `.ico` and `.icns` is rendered from it |
| `printcraft-small.svg` | a lighter vector (traced at 1024 px) for places where size matters, such as this README |
| `printcraft-1024.png` | 1024 px on Apple's grid; also the runtime Dock icon on macOS |
| `printcraft.icns` | macOS icon (16–1024 px) |
| `printcraft.ico` | Windows icon (16–256 px), embedded in `printcraft.exe` by `apps/printcraft/build.rs` |
| `hicolor/<n>x<n>/apps/ai.storyteller.printcraft.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.printcraft.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/printcraft/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab, launcher) and the
Wayland app id `ai.storyteller.printcraft`; `packaging/linux/ai.storyteller.printcraft.desktop` names the
hicolor icon.

## Regenerate

```sh
packaging/icons.sh        # needs resvg and python3; iconutil (macOS) for the .icns
cargo xtask assets        # then update the sha256 values in ATTRIBUTION.toml and run with --write
```
