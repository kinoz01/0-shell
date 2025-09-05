// Unix-only ls with -l, -a, -F plus:
// total (st_blocks/2), "." and ".." with -a, case-insensitive sort ignoring leading '.'
// long format with aligned columns, device major/minor, xattrs '+', colors (incl. broken links),
// symlink targets shown. No globbing, columns, or locale sort.

use std::ffi::{ CStr, CString };
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{ FileTypeExt, MetadataExt };
use std::path::{ Path, PathBuf };
use std::time::{ Duration, SystemTime, UNIX_EPOCH };

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

pub fn run(args: &[String]) {
    let Flags { a, l, f } = parse_flags(args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        Flags::default()
    });
    let mut paths = collect_operands(args);

    if paths.is_empty() {
        paths.push(".".into());
    }

    // split into files/dirs using lstat (symlink_metadata)
    let mut files: Vec<(String, fs::Metadata)> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();

    for p in &paths {
        match fs::symlink_metadata(p) {
            Ok(md) if md.file_type().is_dir() => dirs.push(p.clone()),
            Ok(md) => files.push((p.clone(), md)),
            Err(e) => eprintln!("ls: cannot access '{}': {}", p, e),
        }
    }

    // print files first
    if !files.is_empty() {
        if l {
            let items: Vec<_> = files
                .iter()
                .map(|(p, md)| (Path::new(p), md))
                .collect();
            let widths = compute_widths(&items);
            for (p, md) in &files {
                print_long(Path::new(p), md, f, &widths);
            }
        } else {
            for (p, md) in &files {
                let name = Path::new(p)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| p.clone());
                println!("{}", render_name(&name, Path::new(p), md, f));
            }
        }
        if !dirs.is_empty() {
            println!();
        }
    }

    // print directories
    let need_headers = dirs.len() + (!files.is_empty() as usize) + 0 > 1;
    for (i, d) in dirs.iter().enumerate() {
        if need_headers {
            println!("{}:", d);
        }
        if let Err(e) = list_dir(Path::new(d), a, l, f) {
            eprintln!("ls: cannot open directory '{}': {}", d, e);
        }
        if i + 1 < dirs.len() {
            println!();
        }
    }
}

/* ---------------- flags and operands ---------------- */
#[derive(Default, Copy, Clone)]
struct Flags {
    a: bool,
    l: bool,
    f: bool,
}

fn parse_flags(args: &[String]) -> Result<Flags, String> {
    let mut after_ddash = false;
    let mut flags = Flags::default();
    for a in args {
        if !after_ddash && a == "--" {
            after_ddash = true;
            continue;
        }
        if !after_ddash && a.starts_with('-') && a != "-" {
            for ch in a.chars().skip(1) {
                match ch {
                    'a' => flags.a = true,
                    'l' => flags.l = true,
                    'F' => flags.f = true,
                    _ => return Err(format!("ls: invalid option -- '{}'", ch)), 
                }
            }
        }
    }
    Ok(flags)
}

fn collect_operands(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut after_ddash = false;
    for a in args {
        if !after_ddash && a == "--" {
            after_ddash = true;
            continue;
        }
        if !after_ddash && a.starts_with('-') && a != "-" {
            continue;
        }
        if fs::symlink_metadata(a).is_err() {
            eprintln!("ls: cannot access '{}': No such file or directory", a);
        } else {
            out.push(a.clone());
        }
    }
    out
}

/* ---------------- directory listing ---------------- */
fn list_dir(dir: &Path, show_all: bool, long: bool, classify: bool) -> io::Result<()> {
    let mut entries: Vec<(String, PathBuf, fs::Metadata)> = Vec::new();

    // “.” and “..” for -a
    if show_all {
        if let Ok(md) = fs::symlink_metadata(dir.join(".")) {
            entries.push((".".into(), dir.join("."), md));
        }
        if let Ok(md) = fs::symlink_metadata(dir.join("..")) {
            entries.push(("..".into(), dir.join(".."), md));
        }
    }

    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let name = ent.file_name().to_string_lossy().into_owned();
        if !show_all && name.starts_with('.') {
            continue;
        }
        match fs::symlink_metadata(ent.path()) {
            Ok(md) => entries.push((name, ent.path(), md)),
            Err(e) => eprintln!("ls: cannot access '{}': {}", ent.path().to_string_lossy(), e),
        }
    }

    // case-insensitive, ignore leading dots
    entries.sort_by(|a, b| normalize(&a.0).cmp(&normalize(&b.0)));

    if long {
        println!("total {}", total_blocks(dir, show_all).unwrap_or(0));
        let items: Vec<_> = entries
            .iter()
            .map(|(_, p, md)| (p.as_path(), md))
            .collect();
        let widths = compute_widths(&items);
        for (name, path, md) in entries {
            print_long_named(&name, &path, &md, classify, &widths);
        }
    } else {
        for (name, path, md) in entries {
            println!("{}", render_name(&name, &path, &md, classify));
        }
    }
    Ok(())
}

fn normalize(name: &str) -> String {
    name.trim_start_matches('.').to_lowercase()
}

fn total_blocks(dir: &Path, show_all: bool) -> io::Result<u64> {
    let mut total = 0u64;

    // Only count "." and ".." when -a is on (to match the listing)
    if show_all {
        if let Ok(md) = fs::symlink_metadata(dir.join(".")) {
            total = total.saturating_add(md.blocks());
        }
        if let Ok(md) = fs::symlink_metadata(dir.join("..")) {
            total = total.saturating_add(md.blocks());
        }
    }

    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let name = ent.file_name();
        let name = name.to_string_lossy();
        // Skip hidden entries unless -a
        if !show_all && name.starts_with('.') {
            continue;
        }
        // Use symlink_metadata so we count the link itself if present
        let md = fs::symlink_metadata(ent.path())?;
        total = total.saturating_add(md.blocks());
    }

    // st_blocks are 512-byte units; GNU ls "total" shows 1K units by default
    Ok(total / 2)
}

/* ---------------- long format and widths ---------------- */
struct Widths {
    links: usize,
    user: usize,
    group: usize,
    size: usize,
    major: usize,
    minor: usize,
}

fn compute_widths(items: &[(&Path, &fs::Metadata)]) -> Widths {
    let mut w = Widths { links: 0, user: 0, group: 0, size: 0, major: 0, minor: 0 };
    for (_p, md) in items {
        w.links = w.links.max(md.nlink().to_string().len());

        let (user, group) = uid_gid(md.uid(), md.gid());
        w.user = w.user.max(user.len());
        w.group = w.group.max(group.len());

        if is_dev(md) {
            let (maj, min) = major_minor(md.rdev());
            w.major = w.major.max(maj.to_string().len());
            w.minor = w.minor.max(min.to_string().len());
        } else {
            w.size = w.size.max(md.size().to_string().len());
        }
    }
    w
}

fn print_long(path: &Path, md: &fs::Metadata, classify: bool, w: &Widths) {
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    print_long_named(&name, path, md, classify, w);
}

fn print_long_named(name: &str, path: &Path, md: &fs::Metadata, classify: bool, w: &Widths) {
    let perms = mode_string(path, md);
    let (user, group) = uid_gid(md.uid(), md.gid());
    let nlink = md.nlink();

    if is_dev(md) {
        let (maj, min) = major_minor(md.rdev());
        print!(
            "{:>11} {:>l$} {:>u$} {:>g$} {:>M$}, {:>m$} {} ",
            perms,
            nlink,
            user,
            group,
            maj,
            min,
            mtime(md),
            l = w.links,
            u = w.user,
            g = w.group,
            M = w.major,
            m = w.minor
        );
    } else {
        let size = md.size();
        print!(
            "{:>11} {:>l$} {:>u$} {:>g$} {:>s$} {} ",
            perms,
            nlink,
            user,
            group,
            size,
            mtime(md),
            l = w.links,
            u = w.user,
            g = w.group,
            s = w.size
        );
    }

    // name + classify + symlink target
    print!("{}", render_name(name, path, md, classify));
    if md.file_type().is_symlink() {
        if let Ok(target) = fs::read_link(path) {
            let tstr = target.to_string_lossy().into_owned();
            let tpath = if target.is_absolute() {
                target
            } else {
                path.parent().unwrap_or(Path::new("")).join(target)
            };
            let tmd = fs::symlink_metadata(&tpath).ok();
            let (pref, _) = match tmd {
                Some(ref m) => color_for(&tpath, m),
                None => (format!("{}{}", BOLD, RED), String::new()),
            };
            print!(" -> {}{}{}", pref, tstr, RESET);
        }
    }
    println!();
}

/* ---------------- helpers: mode/attrs, ids, time, color ---------------- */
fn is_dev(md: &fs::Metadata) -> bool {
    let ft = md.file_type();
    ft.is_block_device() || ft.is_char_device()
}

fn major_minor(rdev: u64) -> (u32, u32) {
    let major = ((rdev >> 8) & 0xff) as u32;
    let minor = ((rdev & 0xff) | ((rdev >> 12) & 0xfff00)) as u32;
    (major, minor)
}

fn mode_string(path: &Path, md: &fs::Metadata) -> String {
    let m = md.mode();
    let ft = md.file_type();
    let ftype = if ft.is_dir() {
        'd'
    } else if ft.is_symlink() {
        'l'
    } else if ft.is_fifo() {
        'p'
    } else if ft.is_block_device() {
        'b'
    } else if ft.is_char_device() {
        'c'
    } else if ft.is_socket() {
        's'
    } else {
        '-'
    };

    let suid = (m & 0o4000) != 0;
    let sgid = (m & 0o2000) != 0;
    let sticky = (m & 0o1000) != 0;

    let ur = if (m & 0o400) != 0 { 'r' } else { '-' };
    let uw = if (m & 0o200) != 0 { 'w' } else { '-' };
    let ux = if (m & 0o100) != 0 {
        if suid { 's' } else { 'x' }
    } else {
        if suid { 'S' } else { '-' }
    };

    let gr = if (m & 0o040) != 0 { 'r' } else { '-' };
    let gw = if (m & 0o020) != 0 { 'w' } else { '-' };
    let gx = if (m & 0o010) != 0 {
        if sgid { 's' } else { 'x' }
    } else {
        if sgid { 'S' } else { '-' }
    };

    let or_ = if (m & 0o004) != 0 { 'r' } else { '-' };
    let ow = if (m & 0o002) != 0 { 'w' } else { '-' };
    let ox = if (m & 0o001) != 0 {
        if sticky { 't' } else { 'x' }
    } else {
        if sticky { 'T' } else { '-' }
    };

    let mut s = String::with_capacity(11);
    s.push(ftype);
    for c in [ur, uw, ux, gr, gw, gx, or_, ow, ox] {
        s.push(c);
    }

    if has_xattrs(path) {
        s.push('+');
    }
    s
}

fn has_xattrs(path: &Path) -> bool {
    // libc::listxattr(path, NULL, 0) -> size; >0 means there are xattrs
    let c = match CString::new(path.as_os_str().as_bytes()) {
        Ok(v) => v,
        Err(_) => {
            return false;
        }
    };
    unsafe { libc::listxattr(c.as_ptr(), std::ptr::null_mut(), 0) > 0 }
}

fn uid_gid(uid: u32, gid: u32) -> (String, String) {
    let user = unsafe {
        let pw = libc::getpwuid(uid);
        if pw.is_null() {
            uid.to_string()
        } else {
            CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned()
        }
    };
    let group = unsafe {
        let gr = libc::getgrgid(gid);
        if gr.is_null() {
            gid.to_string()
        } else {
            CStr::from_ptr((*gr).gr_name).to_string_lossy().into_owned()
        }
    };
    (user, group)
}

fn mtime(md: &fs::Metadata) -> String {
    // classic: if older/newer than ~6 months, show year, else HH:MM
    let secs = md.mtime();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs() as i64;
    let six_months = 183 * 24 * 3600;
    let fmt = if (now - secs).abs() > six_months { "%b %e  %Y" } else { "%b %e %H:%M" };

    unsafe {
        let t = secs as libc::time_t;
        let tm = libc::localtime(&t);
        if tm.is_null() {
            return String::new();
        }
        let mut buf = [0u8; 64];
        let fmt_c = CString::new(fmt).unwrap();
        let n = libc::strftime(
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len(),
            fmt_c.as_ptr(),
            tm
        );
        if n == 0 {
            String::new()
        } else {
            CStr::from_ptr(buf.as_ptr() as *const libc::c_char)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/* ---------------- coloring and -F ---------------- */
fn is_exec_file(md: &fs::Metadata) -> bool {
    md.file_type().is_file() && (md.mode() & 0o111) != 0
}

fn class_suffix(md: &fs::Metadata) -> Option<char> {
    let ft = md.file_type();
    if ft.is_dir() {
        Some('/')
    } else if ft.is_symlink() {
        Some('@')
    } else if ft.is_fifo() {
        Some('|')
    } else if ft.is_socket() {
        Some('=')
    } else if is_exec_file(md) {
        Some('*')
    } else {
        None
    }
}

fn broken_link(path: &Path, md: &fs::Metadata) -> bool {
    md.file_type().is_symlink() && !path.exists()
}

fn color_for(path: &Path, md: &fs::Metadata) -> (String, String) {
    let ft = md.file_type();
    if ft.is_symlink() {
        if broken_link(path, md) {
            (format!("{}{}", BOLD, RED), String::new())
        } else {
            (format!("{}{}", BOLD, CYAN), String::new())
        }
    } else if ft.is_dir() {
        (format!("{}{}", BOLD, BLUE), "/".to_string())
    } else if ft.is_socket() {
        (format!("{}{}", BOLD, MAGENTA), "=".to_string())
    } else if ft.is_fifo() {
        (format!("{}", YELLOW), "|".to_string())
    } else if is_dev(md) {
        (format!("{}{}", BOLD, YELLOW), String::new())
    } else if is_exec_file(md) {
        (format!("{}{}", BOLD, GREEN), "*".to_string())
    } else {
        (String::new(), String::new())
    }
}

fn render_name(name: &str, path: &Path, md: &fs::Metadata, classify: bool) -> String {
    let (pref, fallback_suffix) = color_for(path, md);
    let mut out = String::with_capacity(name.len() + 8);
    out.push_str(&pref);
    out.push_str(name);
    out.push_str(RESET);
    if classify {
        if let Some(c) = class_suffix(md) {
            out.push(c);
        } else if !fallback_suffix.is_empty() {
            out.push_str(&fallback_suffix);
        }
    }
    out
}
