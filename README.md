# Chrome Profile Launcher

A small Rust utility that spins up a fresh, disposable Chrome profile on every run — imports bookmarks from an HTML export, launches Chrome with a predefined set of sites, and cleans up old (unlocked) profiles from previous runs.

## What it does

1. Reads settings from `config.json`.
2. Creates the profiles directory if it doesn't already exist.
3. Generates a new Chrome profile folder with a random UUID-based name.
4. Imports bookmarks from an HTML bookmarks export into the new profile's `Default/Bookmarks` file.
5. Launches Chrome with the new profile, maximized, opening the configured list of sites.
6. Deletes old profile folders from previous runs (skipping the one just created, and skipping any profile that still has an active `lockfile`, meaning Chrome is currently using it).

Old profiles are deleted via a detached OS command (`rmdir /s /q` on Windows, `rm -rf` on Linux/macOS) so the app doesn't have to wait around for the cleanup to finish.

## Requirements

- Rust (stable toolchain) and Cargo
- Google Chrome installed
- Dependencies (see `Cargo.toml`): `uuid`, `serde`, `serde_json`, `scraper`

## Configuration

Create a `config.json` file in the same directory as the executable:

```json
{
  "install_directory": "C:/ChromeProfileLauncher",
  "profiles_directory": "C:/ChromeProfileLauncher/profiles",
  "bookmarks_file": "C:/ChromeProfileLauncher/bookmarks.html",
  "chrome_path": "C:/Program Files/Google/Chrome/Application/chrome.exe",
  "sites": [
    "https://example.com",
    "https://another-site.com"
  ]
}
```

| Field | Description |
|---|---|
| `install_directory` | Base install location (currently just logged; not yet used for logic). |
| `profiles_directory` | Where Chrome profile folders are created and cleaned up. |
| `bookmarks_file` | Path to an HTML bookmarks export to import into every new profile. |
| `chrome_path` | Full path to the Chrome executable. |
| `sites` | List of URLs to open as tabs on launch. |

## Building

```bash
cargo build --release
```

The compiled binary will be in `target/release/`.

## Running

Place `config.json` (and your bookmarks HTML export) next to the executable, then run:

```bash
cargo run --release
```

or run the compiled binary directly.

## Notes / Known limitations

- No validation is done on `config.json` beyond basic JSON parsing — malformed paths or a missing Chrome executable will cause a panic or a failed process spawn.
- Old profile cleanup happens *after* Chrome is already launched with the new profile, so it doesn't block startup.
- Locked profiles (i.e. ones with an active `lockfile`, meaning Chrome still has them open) are skipped during cleanup rather than force-deleted.

## Roadmap

- [ ] A GUI for editing settings (`config.json`) instead of hand-editing the file.
- [ ] An installer for easier setup/distribution.

## License

TBD