use std::fs::{self, write};

pub fn create(name: &str) {
    let _ = fs::create_dir_all(format!("{}/src", name));
    let _ = fs::create_dir_all(format!("{}/build", name));

    let _ = write(
        format!("{}/src/main.th", name),
        "fn main() {\n\n  // The code starts running here. \n\n}\n",
    );

    let _ = write(
        format!("{}/Project.toml", name),
        format!(
            "[project]
name = \"{}\"

[compiler]
target = \"\"

[dependencies]
",
            name,
        ),
    );
}
