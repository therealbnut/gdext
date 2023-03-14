use std::path::{Path, PathBuf};

pub fn write_formatted_if_needed(
    source: proc_macro2::TokenStream,
    gen_path: &Path,
    filename: &str,
    out_files: &mut Vec<PathBuf>,
) {
    let _ = std::fs::create_dir_all(gen_path);
    let out_path = gen_path.join(filename);

    let file: syn::File = syn::parse2(source).unwrap();
    let formatted = prettyplease::unparse(&file);

    let mut need_emit = true;
    if let Ok(old_code) = std::fs::read_to_string(&out_path) {
        if old_code == formatted {
            eprintln!("skipping '{}' as it is unchanged", filename);
            need_emit = false;
        }
    }

    if need_emit {
        std::fs::write(&out_path, formatted).unwrap_or_else(|e| {
            panic!(
                "failed to write code file to {};\n\t{}",
                out_path.display(),
                e
            )
        });
    }

    out_files.push(out_path);
}
