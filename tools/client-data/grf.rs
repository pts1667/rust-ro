//! Read-only GRF reader, copied from the client's `ragnarok-formats` crate so the setup tool carries
//! no dependency on the client repository. Only what extraction needs is kept: no writing, no repacking.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use flate2::read::ZlibDecoder;

use super::mixcrypt;
use super::res_name_table::ResNameTable;

/// Prefix of the archive's data tree. The setup tool strips it when writing, matching the client's
/// `data_dir` layout, where a resource name resolves to the archive path without this prefix.
pub const DATA_PREFIX: &str = "data/";
const RES_NAME_TABLE: &str = "data/resnametable.txt";

const HEADER_SIZE: usize = 46;
const FILE_OFFSET: usize = 7;

#[derive(Debug)]
pub enum GrfError {
    Io(std::io::Error),
    InvalidMagic,
    UnsupportedVersion(u8, u8),
    DecompressionFailed(String),
    UnexpectedEof,
    NotFound(String),
}

impl std::fmt::Display for GrfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrfError::Io(error) => write!(f, "{error}"),
            GrfError::InvalidMagic => write!(f, "not a GRF archive (bad magic)"),
            GrfError::UnsupportedVersion(major, minor) => write!(f, "unsupported GRF version {major}.{minor}"),
            GrfError::DecompressionFailed(error) => write!(f, "decompression failed: {error}"),
            GrfError::UnexpectedEof => write!(f, "unexpected end of archive"),
            GrfError::NotFound(name) => write!(f, "file not found in archive: {name}"),
        }
    }
}

impl std::error::Error for GrfError {}

impl From<std::io::Error> for GrfError {
    fn from(error: std::io::Error) -> Self {
        GrfError::Io(error)
    }
}

struct GrfEntry {
    compressed_size: u32,
    compressed_size_aligned: u32,
    uncompressed_size: u32,
    flags: u8,
    offset: u32,
}

pub struct GrfArchive {
    entries: HashMap<String, GrfEntry>,
    file: Mutex<File>,
    data_dir: Option<DataDirIndex>,
    overlays: Vec<GrfArchive>,
    res_name_table: OnceLock<ResNameTable>,
}

/// Extracted files on disk, keyed by their normalized path relative to the directory. A resource
/// name resolves against this by dropping its leading `data/` component, so a directory holding
/// `sprite/foo.spr` overrides the archive entry `data/sprite/foo.spr`.
struct DataDirIndex {
    files: HashMap<String, PathBuf>,
}

impl DataDirIndex {
    fn build(dir: &Path) -> DataDirIndex {
        let mut files = HashMap::new();
        index_dir(dir, dir, &mut files);
        DataDirIndex { files }
    }

    fn lookup(&self, name: &str) -> Option<&PathBuf> {
        self.files.get(&data_dir_key(name))
    }
}

fn data_dir_key(name: &str) -> String {
    let normalized = name.to_lowercase().replace('\\', "/");
    normalized
        .strip_prefix(DATA_PREFIX)
        .unwrap_or(&normalized)
        .to_string()
}

fn index_dir(root: &Path, dir: &Path, out: &mut HashMap<String, PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            index_dir(root, &path, out);
        } else if let Ok(relative) = path.strip_prefix(root) {
            let key = relative.to_string_lossy().to_lowercase().replace('\\', "/");
            out.insert(key, path);
        }
    }
}

fn parse_grf(file: &mut File) -> Result<HashMap<String, GrfEntry>, GrfError> {
    let mut header_buf = [0u8; HEADER_SIZE];
    file.read_exact(&mut header_buf)?;

    if &header_buf[..16] != b"Master of Magic\0" {
        return Err(GrfError::InvalidMagic);
    }

    let file_table_offset = u32::from_le_bytes(header_buf[30..34].try_into().unwrap());
    let reserved_files = u32::from_le_bytes(header_buf[34..38].try_into().unwrap());
    let file_count = u32::from_le_bytes(header_buf[38..42].try_into().unwrap());
    let version = u32::from_le_bytes(header_buf[42..46].try_into().unwrap());

    if !(0x100..=0x200).contains(&version) {
        return Err(GrfError::UnsupportedVersion((version >> 8) as u8, version as u8));
    }

    let file_data_end_offset = HEADER_SIZE as u64 + file_table_offset as u64;
    let actual_file_count = (file_count - reserved_files) as usize - FILE_OFFSET;

    file.seek(SeekFrom::Start(file_data_end_offset))?;

    if version == 0x200 {
        parse_v2_entries(file, actual_file_count)
    } else {
        parse_v1_entries(file, actual_file_count)
    }
}

impl GrfArchive {
    pub fn open(path: &Path) -> Result<Self, GrfError> {
        let mut file = File::open(path)?;
        let entries = parse_grf(&mut file)?;

        Ok(GrfArchive {
            entries,
            file: Mutex::new(file),
            data_dir: None,
            overlays: Vec::new(),
            res_name_table: OnceLock::new(),
        })
    }

    /// Open a priority-ordered stack of archives plus an optional directory of extracted files.
    /// `grf_paths[0]` is the primary archive; later paths only provide files the earlier ones lack.
    /// `data_dir`, when present, overrides every archive.
    pub fn open_layered(grf_paths: &[String], data_dir: Option<&Path>) -> Result<Self, GrfError> {
        let (first, rest) = grf_paths.split_first().ok_or_else(|| {
            GrfError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "grf_paths is empty"))
        })?;

        let mut archive = GrfArchive::open(Path::new(first))?;

        for path in rest {
            match GrfArchive::open(Path::new(path)) {
                Ok(overlay) => archive.overlays.push(overlay),
                Err(error) => eprintln!("warning: cannot open overlay GRF {path}: {error}"),
            }
        }

        if let Some(dir) = data_dir {
            if dir.is_dir() {
                archive.data_dir = Some(DataDirIndex::build(dir));
            } else {
                eprintln!("warning: data_dir {} is not a directory, ignoring", dir.display());
            }
        }

        Ok(archive)
    }

    /// Reads `name`, falling back to the `resnametable.txt` alias when nothing carries that name.
    pub fn read_file(&self, name: &str) -> Result<Vec<u8>, GrfError> {
        match self.read_file_direct(name) {
            Ok(data) => Ok(data),
            Err(error) => {
                let Some(alias) = self.alias_table().resolve(name) else {
                    return Err(error);
                };
                self.read_file_direct(&alias)
            }
        }
    }

    /// Every layer contributes its own table; the higher-priority layer wins a key both define.
    fn alias_table(&self) -> &ResNameTable {
        self.res_name_table.get_or_init(|| {
            let mut table = ResNameTable::default();
            if let Some(disk) = self.data_dir.as_ref().and_then(|d| d.lookup(RES_NAME_TABLE)) {
                if let Ok(data) = std::fs::read(disk) {
                    table.extend_from(ResNameTable::parse(&data));
                }
            }
            for layer in std::iter::once(self).chain(self.overlays.iter()) {
                if let Ok(data) = layer.read_archive_entry(RES_NAME_TABLE) {
                    table.extend_from(ResNameTable::parse(&data));
                }
            }
            table
        })
    }

    fn read_file_direct(&self, name: &str) -> Result<Vec<u8>, GrfError> {
        if let Some(path) = self.data_dir.as_ref().and_then(|d| d.lookup(name)) {
            return std::fs::read(path).map_err(GrfError::Io);
        }

        let name_lower = name.to_lowercase().replace('\\', "/");
        if !self.entries.contains_key(&name_lower) {
            for overlay in &self.overlays {
                if overlay.entries.contains_key(&name_lower) {
                    return overlay.read_archive_entry(name);
                }
            }
        }
        self.read_archive_entry(name)
    }

    /// This archive's own entries, ignoring `data_dir` and overlays.
    fn read_archive_entry(&self, name: &str) -> Result<Vec<u8>, GrfError> {
        let name_lower = name.to_lowercase().replace('\\', "/");
        let Some(entry) = self.entries.get(&name_lower) else {
            return Err(GrfError::NotFound(name.to_string()));
        };

        let position = entry.offset as u64 + HEADER_SIZE as u64;
        let mut compressed = vec![0u8; entry.compressed_size_aligned as usize];

        {
            let mut file = self.file.lock().unwrap();
            file.seek(SeekFrom::Start(position))?;
            file.read_exact(&mut compressed)?;
        }

        mixcrypt::decrypt_file(entry.flags, entry.compressed_size, &mut compressed);

        let mut decompressed = Vec::with_capacity(entry.uncompressed_size as usize);
        let mut decoder = ZlibDecoder::new(compressed.as_slice());
        if let Err(error) = decoder.read_to_end(&mut decompressed) {
            // The client loads assets with miniz_oxide 0.8, which returns the bytes it already decoded
            // when the stream stops at the end of the entry instead of failing. Keep that data: a couple
            // of kRO entries are stored a few bytes short and the client still plays them. Genuinely
            // corrupt streams report another kind and stay errors.
            if error.kind() != std::io::ErrorKind::UnexpectedEof {
                return Err(GrfError::DecompressionFailed(error.to_string()));
            }
        }

        Ok(decompressed)
    }

    /// Every name in the layer stack, listed once and resolved the way `read_file` resolves it:
    /// a name several archives carry is reported once. Ordered by layer, then name.
    pub fn layered_file_names(&self) -> Vec<String> {
        let mut seen: HashSet<&str> = HashSet::new();
        let mut names = Vec::new();
        for layer in std::iter::once(self).chain(self.overlays.iter()) {
            let mut layer_names: Vec<&String> = layer.entries.keys().collect();
            layer_names.sort();
            for name in layer_names {
                if seen.insert(name.as_str()) {
                    names.push(name.clone());
                }
            }
        }
        names
    }

    /// Files of the loose data directory, named the way an archive would name them.
    pub fn data_dir_file_names(&self) -> Vec<String> {
        let Some(data_dir) = &self.data_dir else {
            return Vec::new();
        };
        let mut names: Vec<String> = data_dir
            .files
            .keys()
            .map(|key| format!("{DATA_PREFIX}{key}"))
            .collect();
        names.sort();
        names
    }
}

fn read_u32_le(file: &mut File) -> Result<u32, GrfError> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn parse_v2_entries(file: &mut File, file_count: usize) -> Result<HashMap<String, GrfEntry>, GrfError> {
    let table_compressed_size = read_u32_le(file)?;
    let table_uncompressed_size = read_u32_le(file)?;

    let mut compressed_table = vec![0u8; table_compressed_size as usize];
    file.read_exact(&mut compressed_table)?;

    let mut table = Vec::with_capacity(table_uncompressed_size as usize);
    let mut decoder = ZlibDecoder::new(compressed_table.as_slice());
    decoder
        .read_to_end(&mut table)
        .map_err(|e| GrfError::DecompressionFailed(e.to_string()))?;

    let mut entries = HashMap::with_capacity(file_count);
    let mut pos = 0;
    for _ in 0..file_count {
        let name_end = table[pos..].iter().position(|&b| b == 0).ok_or(GrfError::UnexpectedEof)?;
        let name_bytes = &table[pos..pos + name_end];
        let (name_decoded, _, _) = encoding_rs::EUC_KR.decode(name_bytes);
        let name = name_decoded.into_owned().to_lowercase().replace('\\', "/");
        pos += name_end + 1;

        if pos + 17 > table.len() {
            return Err(GrfError::UnexpectedEof);
        }

        let compressed_size = u32::from_le_bytes(table[pos..pos + 4].try_into().unwrap());
        let compressed_size_aligned = u32::from_le_bytes(table[pos + 4..pos + 8].try_into().unwrap());
        let uncompressed_size = u32::from_le_bytes(table[pos + 8..pos + 12].try_into().unwrap());
        let flags = table[pos + 12];
        let offset = u32::from_le_bytes(table[pos + 13..pos + 17].try_into().unwrap());
        pos += 17;

        entries.insert(
            name,
            GrfEntry {
                compressed_size,
                compressed_size_aligned,
                uncompressed_size,
                flags,
                offset,
            },
        );
    }
    Ok(entries)
}

fn parse_v1_entries(file: &mut File, file_count: usize) -> Result<HashMap<String, GrfEntry>, GrfError> {
    let mut table = Vec::new();
    file.read_to_end(&mut table)?;

    let mut entries = HashMap::with_capacity(file_count);
    let mut ofs = 0;
    for _ in 0..file_count {
        if ofs + 6 > table.len() {
            return Err(GrfError::UnexpectedEof);
        }
        let entry_len = u32::from_le_bytes(table[ofs..ofs + 4].try_into().unwrap()) as usize;
        let name_len = (entry_len as u8).wrapping_sub(6) as usize;
        if ofs + 6 + name_len > table.len() {
            return Err(GrfError::UnexpectedEof);
        }
        let decrypt_len = (name_len + 7) & !7;
        let mut name_buf = vec![0u8; decrypt_len];
        let copy_len = name_len.min(table.len() - (ofs + 6));
        name_buf[..copy_len].copy_from_slice(&table[ofs + 6..ofs + 6 + copy_len]);
        mixcrypt::decrypt_filename(&mut name_buf);

        let name_end = name_buf.iter().position(|&b| b == 0).unwrap_or(name_len);
        let (name_decoded, _, _) = encoding_rs::EUC_KR.decode(&name_buf[..name_end]);
        let name = name_decoded.into_owned().to_lowercase().replace('\\', "/");

        let ofs2 = ofs + entry_len + 4;
        if ofs2 + 17 > table.len() {
            return Err(GrfError::UnexpectedEof);
        }

        let raw_compressed = u32::from_le_bytes(table[ofs2..ofs2 + 4].try_into().unwrap());
        let raw_aligned = u32::from_le_bytes(table[ofs2 + 4..ofs2 + 8].try_into().unwrap());
        let uncompressed_size = u32::from_le_bytes(table[ofs2 + 8..ofs2 + 12].try_into().unwrap());
        let entry_type = table[ofs2 + 12];
        let offset = u32::from_le_bytes(table[ofs2 + 13..ofs2 + 17].try_into().unwrap());

        let compressed_size = raw_compressed.wrapping_sub(uncompressed_size).wrapping_sub(715);
        let compressed_size_aligned = raw_aligned.wrapping_sub(37579);
        let flags = if entry_type & 0x01 != 0 {
            encryption_flags_for_extension(&name) | 0x01
        } else {
            entry_type
        };

        ofs = ofs2 + 17;

        if entry_type & 0x01 == 0 {
            continue;
        }

        entries.insert(
            name,
            GrfEntry {
                compressed_size,
                compressed_size_aligned,
                uncompressed_size,
                flags,
                offset,
            },
        );
    }
    Ok(entries)
}

fn encryption_flags_for_extension(filename: &str) -> u8 {
    match filename.rsplit('.').next() {
        Some("gnd" | "gat" | "act" | "str") => mixcrypt::GRF_FLAG_HEADER_DES_CRYPT,
        _ => mixcrypt::GRF_FLAG_FULL_MIX_CRYPT,
    }
}
