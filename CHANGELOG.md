# Changelog

## 0.0.25-dev — Development Release

> **Use with caution.** This is a development build from the `development` branch. It has not been fully tested and may corrupt save files. Always back up your saves before using it.

### New Features

- **Required auto-backup before save** — A backup folder must be selected in Settings before any save operation. The app creates a timestamped copy of the original file before writing changes.
- **`.dat` file support** — Open and save dialogs now accept both `.sl2` and `.dat` extensions. The save dialog pre-fills the source filename to preserve the original extension.
- **Current-game summoning pools (v1.12+)** — Added 213 summoning pool event flags with real names from The Grand Archives cheat table, covering base game and DLC1.
- **Softlock warnings for catacomb-like areas** — Regions and graces such as catacombs, hero's graves, gaols, and the Hidden Path to the Haligtree now show a yellow warning label when activated.

### Fixes

- **PlayStation save corruption on re-save** — Fixed `PlayerCoords` fields that were being dropped during read, causing adjacent bytes to be zeroed on write.
- **Summoning pool event flags** — Corrected placeholder offsets and added the missing pool (`670490`) using the canonical byte/bit formula.
- **DLC weapons with ashes of war** — Added all new DLC weapon types (Hand-to-Hand, Thrusting Shield, Throwing Weapon, Reverse Hand Sword, Light Greatsword, Great Katana, Beast Claw), infusion support, and affinity names.
- **Inventory single-add amount** — `add_single` now passes the correct amount instead of free-space count.
- **Hide unnamed placeholder rows** — Sentinel/cut placeholder rows shown as `[UNKNOWN_<id>]` are filtered out of single-add item lists.

### Internal

- Upgraded `eframe`/`egui` from 0.26.2 to 0.34.2 and refreshed all other dependencies.
- Migrated encryption code to `cipher` 0.5 API.

---

## 0.0.24

See tag `v0.0.24` for previous release details.
