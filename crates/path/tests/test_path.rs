use macron_path::path;
use std::path::PathBuf;

pub const APP_NAME: &str = "ovsy";

#[test]
fn test_bin_path() {
    let exe = std::env::current_exe().unwrap();
    assert_eq!(path!("$"), exe);

    let mut expected_dir = exe.parent().unwrap().to_path_buf();
    expected_dir.push("assets/config.toml");
    assert_eq!(path!("$/assets/config.toml"), expected_dir);
}

#[test]
fn test_data_path() {
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA").unwrap();
        let expected = PathBuf::from(local_app_data)
            .join(APP_NAME)
            .join("data")
            .join("storage.db");
        assert_eq!(path!("$share$/storage.db"), expected);
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap();
        let expected = PathBuf::from(home)
            .join("Library/Application Support")
            .join(APP_NAME)
            .join("storage.db");
        assert_eq!(path!("$share$/storage.db"), expected);
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let home = std::env::var("HOME").unwrap();
        let xdg_data = std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(home).join(".local/share"));
        let expected = xdg_data.join(APP_NAME).join("storage.db");

        assert_eq!(path!("$share$/storage.db"), expected);
    }
}

#[test]
fn test_userdata_path() {
    let home = if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").unwrap()
    } else {
        std::env::var("HOME").unwrap()
    };

    let expected_docs = PathBuf::from(&home)
        .join("Documents")
        .join(APP_NAME)
        .join("report.pdf");
    assert_eq!(path!("$documents$/report.pdf"), expected_docs);

    let id = 42;
    let expected_download = PathBuf::from(&home)
        .join("Downloads")
        .join(APP_NAME)
        .join("track_42.mp3");
    assert_eq!(path!("$downloads$/track_{}.mp3", id), expected_download);
}

#[test]
fn test_from_let() {
    let dynamic_target = String::from("$temp$/session.lock");
    let expected = std::env::temp_dir().join(APP_NAME).join("session.lock");

    assert_eq!(path!(dynamic_target), expected);
}

#[test]
fn test_from_path() {
    let dynamic_target = PathBuf::from("$temp$/session.lock");
    let expected = std::env::temp_dir().join(APP_NAME).join("session.lock");

    assert_eq!(path!(dynamic_target.to_string_lossy()), expected);
}

#[test]
fn test_path_formatting() {
    let home = if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").unwrap()
    } else {
        std::env::var("HOME").unwrap()
    };

    let folder = "projects";
    let file = "main.rs";
    let expected_home = PathBuf::from(&home).join(folder).join(file);

    assert_eq!(path!("~/{}/{}", folder, file), expected_home);

    let service = "database";
    let ext = "yaml";

    let mut expected_config = {
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(std::env::var("APPDATA").unwrap())
                .join(APP_NAME)
                .join("config")
        }
        #[cfg(target_os = "macos")]
        {
            PathBuf::from(&home)
                .join("Library/Application Support")
                .join(APP_NAME)
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(&home).join(".config"))
                .join(APP_NAME)
        }
    };
    expected_config = expected_config.join("v1").join("database.yaml");

    assert_eq!(
        path!(
            "$config$/v1/{name}.{extension}",
            name = service,
            extension = ext
        ),
        expected_config
    );

    let user_id = 1337;
    let mut expected_cache = {
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(std::env::var("LOCALAPPDATA").unwrap())
                .join(APP_NAME)
                .join("cache")
        }
        #[cfg(target_os = "macos")]
        {
            PathBuf::from(&home).join("Library/Caches").join(APP_NAME)
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(&home).join(".cache"))
                .join(APP_NAME)
        }
    };
    expected_cache = expected_cache.join("user_1337").join("avatar.png");

    assert_eq!(path!("$cache$/user_{user_id}/avatar.png"), expected_cache);
}
