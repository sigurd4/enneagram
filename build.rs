use std::path::Path;

macro_rules! expect {
    ($result:expr, $pat:pat => $error:expr $(, $retval:expr)?) => {
        match $result
        {
            $pat => {
                println!("cargo::error={}", $error);
                return $($retval)?
            },
            result => result.unwrap()
        }
    };
}

fn main()
{
    println!("cargo::rerun-if-changed=presets");

    let manifest_dir_var = expect!(std::env::var("CARGO_MANIFEST_DIR"), Err(_) => "Variable '$CARGO_MANIFEST_DIR' not defined!");
    let manifest_dir = Path::new(&manifest_dir_var);
    let presets_dir = manifest_dir.join(Path::new("presets"));
    let presets_rs = manifest_dir.join(Path::new("src/config/presets.rs"));

    let yamls = expect!(std::fs::read_dir(presets_dir), Err(_) => "Unable to read contents of presets directory.")
        .filter_map(|dir_entry| {
            let dir_entry = expect!(dir_entry, Err(_) => "Unable to read directory entry of presets directory.", None);
            let file_name = dir_entry.file_name();
            let file_name = file_name.to_string_lossy();
            let file_type = expect!(dir_entry.file_type(), Err(_) => &format!("Unable to verify file-type of preset {}", file_name), None);
            if file_type.is_file() && file_name.ends_with(".yaml")
            {
                println!("cargo::rerun-if-changed=presets/{}", file_name);
                Some(file_name.into_owned())
            }
            else
            {
                None
            }
        }).map(|file_name| {
            let file_name_no_ext = &file_name[..file_name.len() - ".yaml".len()];
            format!("\t(\"{file_name_no_ext}\", include_str!(\"../../presets/{file_name}\")),\n")
        }).collect::<Vec<_>>()
        .concat();
    
    let presets_contents = format!("pub static PRESETS: &[(&str, &str)] = &[\n{yamls}];");

    expect!(std::fs::write(presets_rs, presets_contents), Err(_) => "Failed to write presets.rs")
}