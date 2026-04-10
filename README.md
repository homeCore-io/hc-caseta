# hc-caseta

Bridges Lutron Caseta Smart Bridge Pro devices into HomeCore via the Lutron Integration Protocol (LIP) over telnet.

Requires the **Caseta Smart Bridge Pro** (L-BDGPRO2-WH). The standard Caseta bridge does not support telnet integration.

## Supported device types

| Kind | HomeCore device_type | Notes |
|---|---|---|
| `dimmer` | `light` | Brightness 0-100, configurable fade time |
| `switch` | `switch` | On/off relay |
| `shade` | `cover` | Motorized shade with position control |
| `fan_control` | `fan` | Fan speed levels |
| `pico` | `button` | Button press/release/hold events (read-only) |
| `occupancy_sensor` | `occupancy_sensor` | Occupied/vacant state |

## Setup

1. Copy `config/config.toml.example` to `config/config.toml`
2. Set the bridge IP and device integration IDs (find IDs at `http://{bridge_ip}/DbXmlInfo.xml`)
3. Add a `[[plugins]]` entry in `homecore.toml`

## Configuration

- `host` — Caseta Pro bridge IP
- `default_fade_secs` — global fade time for dimmers (per-device override with `fade_secs`)
- `[[devices]]` — each device needs `integration_id`, `name`, `kind`, and `area`
