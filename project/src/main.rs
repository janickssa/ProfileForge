use std::fs;
use uuid::Uuid;
use std::env;
use std::time::SystemTime;
use std::process::Command;

fn CreateDirectory(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {}", e),
    }
}



fn ClearProfiles(path: &str) {
    match fs::read_dir(path) {
        Ok(dirs) => {
            println!("Clearing Chrome profiles in path: {}", path);

            for entry in dirs {
                match entry {
                    Ok(entry) => {
                        let lockfile = entry.path().join("lockfile");
                                        
                        if !lockfile.exists() {
                            match fs::remove_dir_all(entry.path()) {
                                Ok(_) => println!("Deleted profile: {:?}", entry.path()),
                                Err(e) => eprintln!("Failed to delete profile {:?}: {}", entry.path(), e),
                            }
                        } else {
                            println!("Skipping locked profile: {:?}", entry.path());

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




fn CreateChromeProfile() -> String {
    let pre_path: &str = "C:/ProfileMaker/Profiles";
    let profileuuid = Uuid::new_v4();
    let profile: String = format!("Profile_{}", profileuuid);
    let path: String = format!("{}/{}", pre_path, profile);
    println!("Creating Chrome profile at path: {}", path);
    return path;
}


// fn CreateLastRunFile(path: &str) {
//     let sys_time = SystemTime::now();
//     let str_sys_time: String = format!("{:?}", sys_time);
//     println!("Last run file created successfully: {}", str_sys_time);
//     if !std::path::Path::new(path).exists() {
//         CreateDirectory("C:/temp/ChromeProfiles_logs/");
//     } else {
//         println!("Directory already exists");
//     }
//     match fs::File::create(path) {
//         Ok(_) => println!("Last run file created successfully: {}", str_sys_time),
//         Err(e) => println!("Error creating last run file: {}", e),
//     }
// }



fn CreateProcess(ProfileDir: String, ArgSite: &str) {
    let chrome: &str = "C:/Program Files/Google/Chrome/Application/chrome.exe";


    Command::new(chrome)
        .arg(format!("--user-data-dir={}", ProfileDir))
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--start-maximized")
        .arg("--new-window")
        .arg(ArgSite)
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
    // let Last_Run: &str = "C:/temp/ChromeProfiles_logs/last_run.txt";
    let ArgSite: String = std::env::args().nth(1).unwrap_or_else(|| "https://admin.microsoft.com/".to_string());
    
    let path: &str = "C:/ProfileMaker/Profiles";
    println!("Hello, world!");
    if !std::path::Path::new(path).exists() {
        CreateDirectory(path);
    } else {
        println!("Directory already exists");
    }
    ClearProfiles(path);


    // CreateLastRunFile(Last_Run);

    let profile_path: String = CreateChromeProfile();
    import_bookmarks(
        "C:/ProfileMaker/bookmarks.html",
        &profile_path
    ).expect("Failed to import bookmarks");

    CreateProcess(profile_path, &ArgSite);



    
    println!("Argument provided: {}", ArgSite);    



}
