use std::env;
use copy_to_output::copy_to_output;

fn main() {
    let files_to_copy = vec!["./sprites", "icon.ico", "./openal32.dll", "./sfml-audio-2.dll", "./sfml-audio-d-2.dll", "./sfml-graphics-2.dll", "./sfml-graphics-d-2.dll", "./sfml-network-2.dll", "./sfml-network-d-2.dll", "./sfml-system-2.dll", "./sfml-system-d-2.dll", "./sfml-window-2.dll", "./sfml-window-d-2.dll"];

    for file in &files_to_copy {
        copy_to_output(file, &env::var("PROFILE").unwrap()).expect("Could not copy files");
    }

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./icon.ico");
        res.compile().expect("Failed to compile resources");
    }
}
