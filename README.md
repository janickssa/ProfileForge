# ProfileForge

A small Rust CLI tool that spins up a fresh, disposable Chrome profile — with bookmarks pre-loaded — and launches it pointed at a given site. Useful for quickly opening a clean browser session (e.g. for admin/testing work) without polluting your main Chrome profile.

## What it does

On each run, ProfileForge:

1. **Ensures the profile directory exists** — creates `C:/ProfileMaker/Profiles` if it isn't there yet.
2. **Clears out old profiles** — deletes any previous profile folders in that directory, skipping any that are currently locked (i.e. still in use, detected via a `lockfile`).
3. **Generates a new profile** — creates a uniquely named folder (`Profile_<uuid>`) for the new Chrome profile.
4. **Imports bookmarks** — parses an HTML bookmarks export (`C:/ProfileMaker/bookmarks.html`), converts each link into Chrome's native `Bookmarks` JSON format, and writes it into the new profile's `Default` folder so they're available immediately.
5. **Launches Chrome** — starts `chrome.exe` with the new profile, maximized, in a new window, skipping the first-run and default-browser prompts, and navigates to the target site.

## Usage

```bash
ProfileForge.exe [URL]
```

- `URL` *(optional)* — the site to open on launch.
  Defaults to `https://admin.microsoft.com/` if not provided.

**Example:**

```bash
ProfileForge.exe https://portal.azure.com/
```

## Requirements

- Windows (paths are currently hardcoded to Windows-style locations)
- Google Chrome installed at:
  `C:/Program Files/Google/Chrome/Application/chrome.exe`
- A bookmarks export placed at:
  `C:/ProfileMaker/bookmarks.html`

## Dependencies

| Crate | Purpose |
|---|---|
| `uuid` | Generates unique profile folder names |
| `scraper` | Parses the bookmarks HTML file |
| `serde_json` | Builds and writes the Chrome `Bookmarks` JSON file |

## Project structure notes

- **Profile storage:** `C:/ProfileMaker/Profiles/Profile_<uuid>/`
- **Bookmarks source:** `C:/ProfileMaker/bookmarks.html`
- Old, unlocked profiles are wiped on every run — this tool is designed for *disposable* sessions, not persistent ones. If you need a profile to survive between runs, make sure it holds a `lockfile` (or adjust the cleanup logic).

## Known limitations / TODO

- Paths are hardcoded (`C:/ProfileMaker/...`, `C:/Program Files/Google/Chrome/...`) — not currently configurable via CLI flags or a config file.
- No error handling if Chrome isn't installed at the expected path (`.expect()` will panic).
- The `CreateLastRunFile` function and its logging directory (`C:/temp/ChromeProfiles_logs/`) are present in the code but currently commented out / unused.
- No tests included yet.

## License


## Todo
- Add GUI for customization of opened tabs in new instance, and setting folder locations
- Add installer GUI script with custom folders



_Not yet specified._


