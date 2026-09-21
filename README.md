# ESP Busy Tag

Open-source ESP32-based alternative to Busy Tag, built for the Waveshare [ESP32-S3-Touch-AMOLED-1.8](https://www.waveshare.com/esp32-s3-touch-amoled-1.8.htm) (368×448 AMOLED). Switch your availability directly with the onboard button or control it from a computer over USB.

## Features

- Shows the current status full-screen:
  - `FREE` — green background
  - `BUSY` — red background
  - `MEETING` — blue background
- Orbitron 48px white text on the status-colored background
- Boots into `FREE` every time
- **BOOT** button (GPIO0) cycles `FREE → BUSY → MEETING → FREE` (60 ms debounce)
- Status set over USB serial, JSON Lines protocol

## TUI

```sh
cd tui
cargo run
```

Interactive terminal UI. Keys:

- `f` / `1` — Free
- `b` / `2` — Busy
- `m` / `3` — Meeting
- `r` — reconnect
- `u` — firmware update (prompts for merged `.bin` path)
- `q` / `Esc` — quit

The app finds a device by sending `{"cmd":"info"}` to serial ports. Firmware update uses a local merged `.bin` image and an installed `esptool` executable.

## Firmware

Install ESP-IDF 5.5+, then:

```sh
cd firmware
idf.py set-target esp32s3
idf.py build flash monitor
```

The first build downloads Waveshare's managed BSP component. The device appears as a USB serial port and accepts JSON Lines, e.g.:

```json
{"cmd":"status","value":"busy"}
{"cmd":"status","value":"free"}
{"cmd":"get_state"}
{"cmd":"info"}
```

For firmware update, create one merged image (not `build/desk_display.bin`, which starts at `0x10000`):

```sh
idf.py merge-bin -o desk-display.bin
```

Choose `firmware/build/desk-display.bin` in the TUI (`u`).
