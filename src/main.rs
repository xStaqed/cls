fn main() {
    if cfg!(windows)  {
        std::process::Command::new("cls").status().unwrap();
    } else if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("clear").status().unwrap();
    }
}


