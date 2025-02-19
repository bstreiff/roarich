use std::env;
use std::path::PathBuf;

// Windows paths for this
const WINDOWS_PATHS: &[&str] = &[
    r"C:\SquareEnix\FINAL FANTASY XIV - A Realm Reborn",
    r"C:\Program Files (x86)\Steam\steamapps\common\FINAL FANTASY XIV Online",
    r"C:\Program Files (x86)\Steam\steamapps\common\FINAL FANTASY XIV - A Realm Reborn",
    r"C:\Program Files (x86)\FINAL FANTASY XIV - A Realm Reborn",
    r"C:\Program Files (x86)\SquareEnix\FINAL FANTASY XIV - A Realm Reborn",
];

fn unixify_windows_path(prefix: &PathBuf, original_path: &str) -> PathBuf {
    prefix
        .to_str()
        .iter()
        .copied()
        .chain(original_path.split('\\').skip(1))
        .collect::<PathBuf>()
}

#[cfg(target_os = "windows")]
fn find_install_platform() -> Option<PathBuf> {
    // On Windows we just look for all the paths.
    WINDOWS_PATHS
        .iter()
        .map(|path| PathBuf::from(path))
        .find(|path| path.exists())
}

#[cfg(not(target_os = "windows"))]
fn find_install_platform() -> Option<PathBuf> {
    // Linux XIVLauncher
    if let Ok(homedir) = env::var("HOME") {
        let mut xl_path = PathBuf::from(homedir);
        xl_path.push(".xlcore");
        xl_path.push("ffxiv");
        if xl_path.exists() {
            return Some(xl_path);
        }
    }

    let mut prefix_list = vec![];

    // Linux WSL
    prefix_list.push(PathBuf::from("/mnt/c"));

    // macOS XIV on Mac? (TODO: needs verification)
    #[cfg(target_os = "macos")]
    if let Ok(homedir) = env::var("HOME") {
        let mut xl_path = PathBuf::from(homedir);
        xl_path.push("Library");
        xl_path.push("Application Support");
        xl_path.push("XIV on Mac");
        xl_path.push("wineprefix");
        xl_path.push("drive_c");
        prefix_list.push(xl_path);
    }

    for win_path in WINDOWS_PATHS {
        for prefix in &prefix_list {
            let unix_path = unixify_windows_path(prefix, win_path);
            print!("{:?}\n", unix_path);
            if unix_path.exists() {
                return Some(unix_path);
            }
        }
    }

    None
}

pub fn find_install() -> Option<PathBuf> {
    if let Ok(ffxiv_path) = env::var("FFXIV_PATH") {
        Some(PathBuf::from(ffxiv_path))
    } else {
        find_install_platform()
    }
}
