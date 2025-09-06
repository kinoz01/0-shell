use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn run(args: &[String]) {
    if args.len() < 2 {
        eprintln!(
            "cp: missing destination file operand after '{}'\nUsage: cp SOURCE DEST",
            args[0]
        );
    }

    let dst = &args[args.len() - 1];
    for i in 0..args.len() - 1 {
        let src = &args[i];
        match cp_like(src, dst) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("cp: error copying: {}", e);
            }
        }
    }
}

/// Copy `src` to `dst`, similar to `cp`:
/// - If `src` is a file:
///     - If `dst` is a directory, places the file inside it.
///     - Otherwise writes exactly to `dst`.
/// - If `src` is a directory, copies its contents recursively.
/// - Creates destination parents as needed.
/// - Always overwrites existing files.
pub fn cp_like<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    let meta = fs::symlink_metadata(src)?;
    if meta.is_dir() {
        copy_dir_recursive(src, dst)
    } else {
        copy_file(src, dst)
    }
}

fn copy_file(src: &Path, dst: &Path) -> io::Result<()> {
    let src_meta = fs::metadata(src)?;

    let dest_path = if dst.exists() && dst.is_dir() {
        dst.join(file_name(src)?)
    } else {
        dst.to_path_buf()
    };

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(src, &dest_path)?;
    // Preserve permissions best effort
    let _ = fs::set_permissions(&dest_path, src_meta.permissions());
    Ok(())
}

fn copy_dir_recursive(src_dir: &Path, dst: &Path) -> io::Result<()> {
    let dest_root: PathBuf = if dst.exists() && dst.is_dir() {
        dst.join(file_name(src_dir)?)
    } else {
        dst.to_path_buf()
    };

    fs::create_dir_all(&dest_root)?;

    if let Ok(src_meta) = fs::metadata(src_dir) {
        let _ = fs::set_permissions(&dest_root, src_meta.permissions());
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let child_src = entry.path();
        let child_name = file_name(&child_src)?;
        let child_dst = dest_root.join(child_name);

        let child_meta = fs::symlink_metadata(&child_src)?;
        if child_meta.is_dir() {
            copy_dir_recursive(&child_src, &child_dst)?;
        } else {
            copy_file(&child_src, &child_dst)?;
        }
    }
    Ok(())
}

fn file_name(p: &Path) -> io::Result<&str> {
    p.file_name().and_then(|s| s.to_str()).ok_or_else(|| {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("Bad path: {}", p.display()),
        )
    })
}
