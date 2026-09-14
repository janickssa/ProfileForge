use std::fs;
use uuid::Uuid;
use std::env;
use std::time::SystemTime;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    install_directory: String,
    profiles_directory: String,
    bookmarks_file: String,
    chrome_path: String,
    sites: Vec<String>,
}





fn load_config(config_path: &str) -> Config {
    let config_content = fs::read_to_string(config_path)
        .expect("Failed to read config file");

    serde_json::from_str(&config_content)
        .expect("Failed to parse config file")
}



fn create_directory(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {}", e),
    }
}



fn clear_profiles(path: &str, skip_dir: &str) {
    match fs::read_dir(path) {
        Ok(dirs) => {
            println!("Clearing Chrome profiles in path: {}", path);

            for entry in dirs {
                match entry {
                    Ok(entry) => {
                        let entry_path = entry.path();

                        if entry_path == std::path::Path::new(skip_dir) {
                            continue;
                        }

                        let lockfile = entry_path.join("lockfile");

                        if !lockfile.exists() {
                            spawn_delete(&entry_path);
                        } else {
                            println!("Skipping locked profile: {:?}", entry_path);
                        }
                    },
                    Err(e) => eprintln!("Failed to read directory entry: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory '{}': {}", path, e);
        }
    }
}

#[cfg(target_os = "windows")]
fn spawn_delete(path: &std::path::Path) {
    // rmdir treats forward slashes as switches anywhere in the string, so
    // normalize to backslashes first.
    let windows_path = path.to_string_lossy().replace('/', "\\");

    let _ = Command::new("cmd")
        .args(["/C", "rmdir", "/s", "/q"])
        .arg(windows_path)
        .spawn(); // fire-and-forget, no .wait()
}

#[cfg(not(target_os = "windows"))]
fn spawn_delete(path: &std::path::Path) {
    let _ = Command::new("rm")
        .args(["-rf"])
        .arg(path)
        .spawn();
}




fn create_chrome_profile(pre_path: &str) -> String {
    let profileuuid = Uuid::new_v4();
    let profile = format!("Profile_{}", profileuuid);
    let path = format!("{}/{}", pre_path, profile);

    println!("Creating Chrome profile at path: {}", path);

    path
}



// fn create_last_run_file(path: &str) {
//     let sys_time = SystemTime::now();
//     let str_sys_time: String = format!("{:?}", sys_time);
//     println!("Last run file created successfully: {}", str_sys_time);
//     if !std::path::Path::new(path).exists() {
//         create_directory("C:/temp/ChromeProfiles_logs/");
//     } else {
//         println!("Directory already exists");
//     }
//     match fs::File::create(path) {
//         Ok(_) => println!("Last run file created successfully: {}", str_sys_time),
//         Err(e) => println!("Error creating last run file: {}", e),
//     }
// }



fn create_process(
    chrome_path: &str,
    profile_dir: String,
    sites: Vec<String>
) {
    let mut command = Command::new(chrome_path);

    command
        .arg(format!("--user-data-dir={}", profile_dir))
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--start-maximized")
        .arg("--new-window");

    for site in sites {
        command.arg(site);
    }

    command
        .spawn()
        .expect("Failed to start Chrome process");
}




use std::path::Path;
use scraper::{Html, Selector};
use serde_json::{json, Value};

fn import_bookmarks(html_path: &str, profile_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html = fs::read_to_string(html_path)?;
    let document = Html::parse_document(&html);

    let selector = Selector::parse("a")?;

    let mut children = Vec::new();

    for element in document.select(&selector) {
        let href = element.value().attr("href");

        if let Some(url) = href {
            let name = element.text().collect::<Vec<_>>().join("");

            children.push(json!({
                "date_added": "0",
                "guid": uuid::Uuid::new_v4().to_string(),
                "id": children.len() + 1,
                "name": name,
                "type": "url",
                "url": url
            }));
        }
    }

    let bookmarks = json!({
        "checksum": "",
        "roots": {
            "bookmark_bar": {
                "children": children,
                "name": "Bookmarks bar",
                "type": "folder"
            },
            "other": {
                "children": [],
                "name": "Other bookmarks",
                "type": "folder"
            },
            "synced": {
                "children": [],
                "name": "Mobile bookmarks",
                "type": "folder"
            }
        },
        "version": 1
    });

    let default_dir = Path::new(profile_path).join("Default");

    fs::create_dir_all(&default_dir)?;

    let bookmarks_path = default_dir.join("Bookmarks");

    fs::write(
        bookmarks_path,
        serde_json::to_string_pretty(&bookmarks)?
    )?;

    Ok(())
}




















fn main() {
    let configfile = "config.json";
    let config = load_config(configfile);

    println!("Install directory: {}", config.install_directory);
    println!("Profiles directory: {}", config.profiles_directory);
    println!("Chrome: {}", config.chrome_path);

    let path = &config.profiles_directory;

    if !std::path::Path::new(path).exists() {
        create_directory(path);
    } else {
        println!("Directory already exists");
    }

    let profile_path = create_chrome_profile(path);

    import_bookmarks(
        &config.bookmarks_file,
        &profile_path
    ).expect("Failed to import bookmarks");

    create_process(
        &config.chrome_path,
        profile_path.clone(),
        config.sites
    );

    // Fast: just lists dirs and fires off detached OS delete commands per folder.
    // No thread or sleep needed — spawn_delete's child processes survive on their own.
    clear_profiles(path, &profile_path);
}

