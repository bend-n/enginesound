# enginesound

[![version](https://img.shields.io/badge/4.x-blue?logo=godot-engine&logoColor=white&label=godot&style=for-the-badge)](https://godotengine.org "Made with godot")
[![package](https://img.shields.io/npm/v/@bendn/enginesound?label=version&style=for-the-badge)](https://www.npmjs.com/package/@bendn/enginesound)

Extension for godot used to generate purely synthetic engine sounds in real-time.

Based on [DasEtwas/enginesound](https://github.com/DasEtwas/enginesound).

## installation

- use the [gpm](https://github.com/godot-package-manager/cli) to install
- make a `engine.gdextention` file: [^1]
```toml
[configuration]
entry_symbol = "gdext_rust_init"

[libraries]
linux.release.x86_64 = "res://addons/@bendn/enginesound/libenginesound.so"
linux.debug.x86_64 = "res://addons/@bendn/enginesound/libenginesound.so"
macos.release = "res://addons/@bendn/enginesound/libenginesound.dylib"
macos.debug = "res://addons/@bendn/enginesound/libenginesound.dylib"
windows.debug.x86_64 = "res://addons/@bendn/enginesound/libenginesound.dll"
windows.release.x86_64 = "res://addons/@bendn/enginesound/libenginesound.dll"
```

[^1]: i could include it in the addon but godot doesnt like non toplevel `.gdextension` files.

## usage

- make a `AudioStreamPlayer` (2d, 3d, or global)
- put a `EngineStream` in the `AudioStreamPlayer`
- call `play()` on the `AudioStreamPlayer`
- call `set_stream()` on the `EngineStream` with the `AudioStreamPlayers`'s stream playback (`stream.set_stream(get_stream_playback())`)
- make a `_process` function that calls `update()` on the `EngineStream`
