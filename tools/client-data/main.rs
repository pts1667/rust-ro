//! Builds `config/client-data` from an installed kRO client: parses the GRF archives, extracts every
//! resource the client can ask for, drops the maps the server does not serve, and copies the loose
//! `BGM` folder. The result is served read-only under `/data` by `server/src/server/client_data`.
//!
//! The extraction is raw: entries keep their bytes, only the container is unpacked, and paths are the
//! archive names lowercased with the leading `data/` stripped, the layout the client's `data_dir`
//! option and `resnametable.txt` alias resolution already understand.
//!
//! Every compressible file is written only as `<name>.zst` so the endpoint can serve it with
//! `Content-Encoding: zstd`; already-compressed formats (mp3, png, jpeg) are stored as-is.

mod grf;
mod mixcrypt;
mod res_name_table;

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use grf::GrfArchive;

const USAGE: &str = "\
usage: client-data-setup [--client <kRO folder>] [options]

  --client <folder>   installed kRO client holding the GRF archives
                      (or set KRO_CLIENT_PATH)
  --out <dir>         where the extracted tree goes (default: config/client-data)
  --maps <dir|file>   .mcache directory or one map name per line; maps outside the
                      list are not extracted (default: config/maps/pre-re)
  --all-maps          extract every map in the archives
  --archive <name>    archive to read, relative to the client folder or absolute;
                      repeat in priority order, first wins (default: the client's
                      own data.grf; add rdata.grf, palettes.grf to match a
                      layered grf_paths)
  --zstd-level <n>    zstd level 0-22 for the `.zst` siblings (default: 3)
  --no-compress       do not write the `.zst` siblings
  --dry-run           read and hash everything, write nothing
  -h, --help          show this help
";

/// zstd level for the `.zst` siblings; 3 turns the multi-gigabyte tree around in a couple of minutes.
const DEFAULT_ZSTD_LEVEL: i32 = 3;

/// Per-file checksums served at `/data/check`.
const CHECK_FILE: &str = "check.jsonl";

struct Args {
    client: PathBuf,
    out: PathBuf,
    maps: Option<PathBuf>,
    all_maps: bool,
    archives: Vec<String>,
    zstd_level: Option<i32>,
    dry_run: bool,
}

#[derive(serde::Serialize)]
struct Manifest {
    source_client: String,
    archives: Vec<String>,
    maps: Option<usize>,
    generated_unix_secs: u64,
    files: Vec<ManifestFile>,
}

#[derive(serde::Serialize)]
struct ManifestFile {
    // Also one line of `check.jsonl`
    path: String,
    size: u64,
    md5: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };
    if let Err(error) = run(&args) {
        eprintln!("client-data-setup: {error}");
        std::process::exit(1);
    }
}

fn parse_args() -> Result<Args, String> {
    let mut client: Option<PathBuf> = std::env::var_os("KRO_CLIENT_PATH").map(PathBuf::from);
    let mut out = PathBuf::from("config/client-data");
    let mut maps = Some(PathBuf::from("config/maps/pre-re"));
    let mut all_maps = false;
    let mut archives: Vec<String> = Vec::new();
    let mut zstd_level = Some(DEFAULT_ZSTD_LEVEL);
    let mut dry_run = false;

    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = || it.next().ok_or_else(|| format!("{flag} needs a value\n\n{USAGE}"));
        match flag.as_str() {
            "--client" => client = Some(PathBuf::from(value()?)),
            "--out" => out = PathBuf::from(value()?),
            "--maps" => maps = Some(PathBuf::from(value()?)),
            "--all-maps" => {
                all_maps = true;
                maps = None;
            }
            "--archive" => archives.push(value()?),
            "--zstd-level" => {
                let raw = value()?;
                let level: i32 = raw
                    .parse()
                    .map_err(|_| format!("--zstd-level expects 0-22, got `{raw}`\n\n{USAGE}"))?;
                if !(0..=22).contains(&level) {
                    return Err(format!("--zstd-level expects 0-22, got `{raw}`\n\n{USAGE}"));
                }
                zstd_level = Some(level);
            }
            "--no-compress" => zstd_level = None,
            "--dry-run" => dry_run = true,
            "-h" | "--help" => return Err(USAGE.to_string()),
            other if other.starts_with('-') => return Err(format!("unknown flag `{other}`\n\n{USAGE}")),
            other => match client.take() {
                None => client = Some(PathBuf::from(other)),
                Some(_) => return Err(format!("unexpected argument `{other}`\n\n{USAGE}")),
            },
        }
    }

    let client = client.ok_or_else(|| format!("--client <folder> is required\n\n{USAGE}"))?;
    Ok(Args {
        client,
        out,
        maps,
        all_maps,
        archives,
        zstd_level,
        dry_run,
    })
}

fn run(args: &Args) -> Result<(), String> {
    if !args.client.is_dir() {
        return Err(format!("{} is not a directory", args.client.display()));
    }

    let archive_paths = resolve_archives(args)?;
    let data_dir = args.client.join("data");
    let data_dir = data_dir.is_dir().then_some(data_dir);
    let grf = GrfArchive::open_layered(&archive_paths, data_dir.as_deref())
        .map_err(|error| format!("cannot open {}: {error}", archive_paths[0]))?;

    let maps = if args.all_maps {
        None
    } else {
        let path = args
            .maps
            .as_ref()
            .ok_or_else(|| format!("no map list: pass --maps or --all-maps\n\n{USAGE}"))?;
        let mut maps = load_maps(path)?;
        expand_map_aliases(&grf, &mut maps);
        Some(maps)
    };

    let mut names: BTreeSet<String> = grf.layered_file_names().into_iter().collect();
    names.extend(grf.data_dir_file_names());

    if !args.dry_run {
        fs::create_dir_all(&args.out).map_err(|error| format!("{}: {error}", args.out.display()))?;
    }

    let mut output = Output::new(&args.out, args.dry_run, args.zstd_level);
    let mut skipped_maps = 0usize;
    let mut unreadable = 0usize;
    let mut written: HashSet<String> = HashSet::new();

    for name in &names {
        if let Some(maps) = &maps {
            if let Some(map) = map_geometry(name) {
                if !maps.contains(map) {
                    skipped_maps += 1;
                    continue;
                }
            }
        }
        let relative = output_path(name);
        // Some archives carry both `data/x` and `data//x`; keep the first spelling only.
        if relative.is_empty() || !written.insert(relative.clone()) {
            continue;
        }
        let data = match grf.read_file(name) {
            Ok(data) => data,
            Err(error) => {
                unreadable += 1;
                eprintln!("warning: cannot read {name}: {error}");
                continue;
            }
        };
        output.write(&relative, &data)?;
    }

    let bgm = args.client.join("BGM");
    if bgm.is_dir() {
        output.copy_tree(&bgm, Path::new("bgm"))?;
    }

    let file_count = output.files.len();
    if args.dry_run {
        println!(
            "would write {file_count} files, {:.1} MiB ({} maps skipped, {unreadable} unreadable)",
            output.total_bytes as f64 / (1024.0 * 1024.0),
            skipped_maps
        );
        return Ok(());
    }

    let mut files = output.files;
    files.sort_by(|a, b| a.path.cmp(&b.path));
    write_check_list(&args.out, &files, args.zstd_level)?;

    let manifest = Manifest {
        source_client: args.client.display().to_string(),
        archives: archive_paths,
        maps: maps.as_ref().map(|maps| maps.len()),
        generated_unix_secs: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0),
        files,
    };
    let json = serde_json::to_vec_pretty(&manifest).map_err(|error| format!("manifest: {error}"))?;
    let manifest_path = args.out.join("manifest.json");
    fs::write(&manifest_path, json).map_err(|error| format!("{}: {error}", manifest_path.display()))?;

    let compression = match args.zstd_level {
        Some(_) => format!(
            ", {} compressed to {:.1} MiB",
            output.compressed_files,
            output.compressed_bytes as f64 / (1024.0 * 1024.0)
        ),
        None => String::new(),
    };
    println!(
        "wrote {file_count} files, {:.1} MiB to {} ({} maps skipped, {unreadable} unreadable{compression})",
        output.total_bytes as f64 / (1024.0 * 1024.0),
        args.out.display(),
        skipped_maps
    );
    Ok(())
}

/// One JSON object per line, sorted by path, so a client can stream it and diff it against its own
/// files. `size` and `md5` describe the file as the client stores it, not the `.zst` the endpoint sends.
/// The `.zst` sibling is what `/data/check` serves to a client that accepts zstd.
fn write_check_list(out: &Path, files: &[ManifestFile], zstd_level: Option<i32>) -> Result<(), String> {
    let mut jsonl = Vec::new();
    for file in files {
        serde_json::to_writer(&mut jsonl, file).map_err(|error| format!("{CHECK_FILE}: {error}"))?;
        jsonl.push(b'\n');
    }
    let path = out.join(CHECK_FILE);
    fs::write(&path, &jsonl).map_err(|error| format!("{}: {error}", path.display()))?;
    if let Some(level) = zstd_level {
        let compressed = zstd::bulk::compress(&jsonl, level)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let path = out.join(format!("{CHECK_FILE}.zst"));
        fs::write(&path, compressed).map_err(|error| format!("{}: {error}", path.display()))?;
    }
    Ok(())
}

fn resolve_archives(args: &Args) -> Result<Vec<String>, String> {
    if !args.archives.is_empty() {
        let paths: Vec<PathBuf> = args
            .archives
            .iter()
            .map(|name| {
                let path = Path::new(name);
                if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    args.client.join(path)
                }
            })
            .collect();
        for path in &paths {
            if !path.is_file() {
                return Err(format!("{} is not a file", path.display()));
            }
        }
        return Ok(paths
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect());
    }

    let found: Vec<String> = ["data.grf"]
        .iter()
        .map(|name| args.client.join(name))
        .filter(|path| path.is_file())
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect();
    if found.is_empty() {
        return Err(format!(
            "no data.grf in {}: pass --archive",
            args.client.display()
        ));
    }
    Ok(found)
}

/// A directory of `{map}.mcache` files, or a text file of map names (one per line, `//` comments).
fn load_maps(path: &Path) -> Result<HashSet<String>, String> {
    if path.is_dir() {
        let entries = fs::read_dir(path).map_err(|error| format!("{}: {error}", path.display()))?;
        let mut maps = HashSet::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if let Some(stem) = name.strip_suffix(".mcache") {
                if !stem.is_empty() {
                    maps.insert(stem.to_string());
                }
            }
        }
        if maps.is_empty() {
            return Err(format!("{} has no .mcache files", path.display()));
        }
        return Ok(maps);
    }

    let text = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let maps: HashSet<String> = text
        .lines()
        .map(|line| line.split("//").next().unwrap_or("").trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_lowercase)
        .collect();
    if maps.is_empty() {
        return Err(format!("{} lists no maps", path.display()));
    }
    Ok(maps)
}

/// The client resolves `resnametable.txt` aliases at runtime, so a kept alias needs its target's
/// geometry too even when the server map list only names the alias.
fn expand_map_aliases(grf: &GrfArchive, maps: &mut HashSet<String>) {
    let Ok(data) = grf.read_file("data/resnametable.txt") else {
        return;
    };
    let (text, _, _) = encoding_rs::EUC_KR.decode(&data);
    for line in text.lines() {
        let mut fields = line.split('#');
        let (Some(alias), Some(target)) = (fields.next(), fields.next()) else {
            continue;
        };
        let (Some(alias), Some(target)) = (map_stem(alias), map_stem(target)) else {
            continue;
        };
        if maps.contains(&alias) {
            maps.insert(target);
        }
    }
}

fn map_stem(name: &str) -> Option<String> {
    let name = name.trim().to_lowercase();
    [".rsw", ".gnd", ".gat"]
        .iter()
        .find_map(|extension| name.strip_suffix(extension))
        .map(str::to_string)
}

/// `data/<map>.rsw`, `.gnd` or `.gat`, with no directory between; `None` for everything else.
fn map_geometry(name: &str) -> Option<&str> {
    let rest = name.strip_prefix(grf::DATA_PREFIX)?;
    if rest.contains('/') {
        return None;
    }
    rest.strip_suffix(".rsw")
        .or_else(|| rest.strip_suffix(".gnd"))
        .or_else(|| rest.strip_suffix(".gat"))
}

/// Archive name to extracted path: `data/sprite/foo.spr` becomes `sprite/foo.spr`, a `resnametable`
/// alias target keeps its own name, and entries outside `data/` (a GRF-embedded `bgm\…`, say) keep
/// their path. Redundant and parent components are dropped so an archive name can never escape the
/// output directory; some archives carry `data//x` beside `data/x`.
fn output_path(name: &str) -> String {
    let rest = name.strip_prefix(grf::DATA_PREFIX).unwrap_or(name);
    rest.split('/')
        .filter(|component| !component.is_empty() && *component != "." && *component != "..")
        .collect::<Vec<_>>()
        .join("/")
}

/// Collects the manifest while writing, and writes the `.zst` sibling the endpoint negotiates.
struct Output {
    root: PathBuf,
    dry_run: bool,
    zstd_level: Option<i32>,
    files: Vec<ManifestFile>,
    total_bytes: u64,
    compressed_files: usize,
    compressed_bytes: u64,
}

impl Output {
    fn new(root: &Path, dry_run: bool, zstd_level: Option<i32>) -> Self {
        Self {
            root: root.to_path_buf(),
            dry_run,
            zstd_level,
            files: Vec::new(),
            total_bytes: 0,
            compressed_files: 0,
            compressed_bytes: 0,
        }
    }

    fn write(&mut self, relative: &str, data: &[u8]) -> Result<(), String> {
        if !self.dry_run {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
            }
            match self.zstd_level.filter(|_| should_compress(relative)) {
                // Compressible files are stored only compressed: the endpoint serves them with
                // `Content-Encoding: zstd`, and a request that does not accept zstd gets a 404.
                Some(level) => {
                    let compressed = zstd::bulk::compress(data, level)
                        .map_err(|error| format!("{}: {error}", path.display()))?;
                    let mut name = path.as_os_str().to_os_string();
                    name.push(".zst");
                    let path = PathBuf::from(name);
                    fs::write(&path, &compressed)
                        .map_err(|error| format!("{}: {error}", path.display()))?;
                    self.compressed_files += 1;
                    self.compressed_bytes += compressed.len() as u64;
                }
                None => {
                    fs::write(&path, data).map_err(|error| format!("{}: {error}", path.display()))?;
                }
            }
        }
        self.total_bytes += data.len() as u64;
        self.files.push(ManifestFile {
            path: relative.to_string(),
            size: data.len() as u64,
            md5: format!("{:x}", md5::compute(data)),
        });
        Ok(())
    }

    fn copy_tree(&mut self, root: &Path, relative: &Path) -> Result<(), String> {
        let mut stack = vec![(root.to_path_buf(), relative.to_path_buf())];
        while let Some((dir, relative)) = stack.pop() {
            let entries = fs::read_dir(&dir).map_err(|error| format!("{}: {error}", dir.display()))?;
            for entry in entries {
                let entry = entry.map_err(|error| format!("{}: {error}", dir.display()))?;
                let path = entry.path();
                let child = relative.join(entry.file_name());
                if path.is_dir() {
                    stack.push((path, child));
                } else {
                    let data =
                        fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
                    let relative = child.to_string_lossy().replace('\\', "/");
                    self.write(&relative, &data)?;
                }
            }
        }
        Ok(())
    }
}

/// Formats that already carry compression: they are stored as-is, everything else only as
/// `<name>.zst`. `BGM` audio is mp3, so it stays identity and keeps range requests working.
fn should_compress(relative: &str) -> bool {
    let extension = relative.rsplit('.').next().unwrap_or("").to_lowercase();
    !matches!(extension.as_str(), "mp3" | "ogg" | "png" | "jpg" | "jpeg")
}
