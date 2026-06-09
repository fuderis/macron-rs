use macron_path::path;

#[test]
fn empty_path() {
    path!();
}

#[test]
fn path() {
    path!("foo/bar");
}

#[test]
fn execute_file_path() {
    path!("$");
}

#[test]
fn execute_dir_path() {
    path!("$/foo/bar");
}

#[test]
fn home_path() {
    path!("~/foo/bar");
}

#[test]
fn user_data_path() {
    path!("%/foo/bar");
}

#[test]
fn drive_path() {
    path!("C:/foo/bar");
}

#[test]
fn format_path() {
    path!("foo/{}", "bar");
}

#[test]
fn format_path2() {
    let bar = "bar";
    path!("foo/{bar}");
}

#[test]
fn from_path() {
    let path = std::path::Path::new("foo/bar");
    path!(path);
}
