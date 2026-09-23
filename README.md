# ESP Busy Tag

Open-source ESP32-based alternative to Busy Tag, built for the Waveshare [ESP32-S3-Touch-AMOLED-1.8](https://www.waveshare.com/esp32-s3-touch-amoled-1.8.htm) (368×448 AMOLED). Switch your availability directly with the onboard button or control it from a computer over USB.

## Demo

https://github.com/user-attachments/assets/9c50b3d7-ad3b-48d7-b5e5-555797bc96bd

## Features

- Shows the current status full-screen:
  - `FREE` — green background
  - `BUSY` — red background
  - `MEETING` — blue background
- Orbitron 48px white text on the status-colored background
- Boots into `FREE` every time
- **BOOT** button (GPIO0) cycles `FREE → BUSY → MEETING → FREE` (60 ms debounce)
- Status set over USB serial, JSON Lines protocol

## Enclosure

The [ESP32 table dock](https://github.com/khlebobul/build123d_models/tree/main/esp32_table_dock) is included as a git submodule at `enclosure/esp32_table_dock/`. It contains the ready-to-print STL and the build123d source.

After cloning, keep only this model in `enclosure/`:

```sh
git submodule update --init --recursive
git -C enclosure sparse-checkout set esp32_table_dock
```

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
- `q` / `Esc` — quit

The app finds a device by sending `{"cmd":"info"}` to serial ports.

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
