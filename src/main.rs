use std::{
    collections::HashMap, env, fs::{self, File}, io::Write, process::exit
};
use serde_json as sj;
use sj::json;

struct AtomicState {
    file_path: String,
    fields: HashMap<String, String>
}

impl AtomicState {
    fn new(version: String) -> Self {
        let file_path = String::from("/opt/discord/resources/build_info.json");

        let mut fields = HashMap::new();
        fields.insert(String::from("releaseChannel"), String::from("stable"));
        fields.insert(String::from("version"), version);

        AtomicState { file_path, fields }
    }
}

fn overwrite_version(state: &AtomicState) {
    // GG if this function gets messed up
    fs::remove_file(&state.file_path).unwrap();

    let mut file = File::create(&state.file_path).unwrap();
    file.write(
        json!(state.fields)
            .to_string()
            .as_bytes()
    ).unwrap();
}

fn main() {
    // args[0] -> target, args[1] -> version number
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => {
            println!("The input should be a single version number!");
            exit(1);
        }
    }

    let state = AtomicState::new(args[1].clone());

    overwrite_version(&state);
}
