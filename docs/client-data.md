# Client data endpoint

The server can publish the kRO client data that `../nostalro-client` reads over HTTP under `/data`,
so a client install can be replaced by the server plus a downloaded data folder.

## Setup

The tree is built once from an installed kRO client (its GRF archives and loose `BGM` folder):

```bash
# from the repository root
cargo run --package tools --bin client-data-setup --release -- \
  --client "/c/Users/Thoma/Downloads/kRO_FullRE_2012-02-09"
```

The tool parses the archive the client opens by default, `data.grf`, and applies the loose
`data/` folder on top. Pass `--archive data.grf --archive rdata.grf --archive palettes.grf` to match
a layered client `grf_paths`. It writes every entry to `config/client-data/` under the archive name
lowercased with the leading `data/` stripped (`sprite/…`, `texture/…`, `prontera.rsw`, `bgm/…`).
That is the same layout as the client's `data_dir` option, which resolves a resource name such as
`data/sprite/foo.spr` by dropping `data/`.

Compressible files are written only as `<name>.zst` (zstd level 3 by default); already-compressed
formats (the mp3 `BGM` folder, PNG, JPEG) are written as-is. The endpoint sends the `.zst` with
`Content-Encoding: zstd` when the request accepts it, so a client must send `Accept-Encoding: zstd`
to fetch those assets; the files stored as-is stay available to every client and keep range request
support for audio.

Only the maps the server serves are extracted: `--maps` (default `config/maps/pre-re`, read as the
names of its `.mcache` files) narrows `data/*.rsw`, `.gnd` and `.gat` to that list, plus the targets
of any kept `resnametable.txt` alias. Everything else the client can name — UI textures, sprites,
palettes, models, sounds, tables and BGM — is kept, so the extraction never drops a resource that
only a server-sent string can name.

Useful options:

| Flag | Meaning |
| --- | --- |
| `--out <dir>` | Output directory (default `config/client-data`). |
| `--maps <dir\|file>` | `.mcache` directory, or a text file of map names, one per line. |
| `--all-maps` | Extract every map in the archives. |
| `--archive <name>` | Read this archive instead of the default set; repeat in priority order, first wins. |
| `--zstd-level <0-22>` | zstd level for the compressed files (default `3`). |
| `--no-compress` | Store every file as-is instead of writing `.zst`. |
| `--dry-run` | Read and hash everything, write nothing. |
| `KRO_CLIENT_PATH` | Environment fallback for `--client`. |

The output is gitignored. Re-running the setup overwrites the files it writes; delete the output
directory first when you change `--no-compress` or `--zstd-level` so files from the previous mode
do not linger. `manifest.json` describes the result (`path`, `size`, `md5` per file) so a client can
sync only what changed. The same entries are written as `check.jsonl`, one JSON object per line sorted
by path (plus `check.jsonl.zst` unless `--no-compress`), which is what `/data/check` serves:

```jsonl
{"path":"prontera.rsw","size":123456,"md5":"d41d8cd98f00b204e9800998ecf8427e"}
{"path":"sprite/몬스터/poring.spr","size":9876,"md5":"…"}
```

`size` and `md5` describe the file as the client keeps it (the decompressed content), not the `.zst`
the endpoint sends. A client re-fetches every path whose local file is missing or differs.

## Endpoint

`server/src/server/client_data` serves the directory read-only with `tower-http`'s `ServeDir`:

- `GET /data/<path>` maps to `<root>/<path>`, for example `GET /data/sprite/몬스터/poring.spr`.
- Range requests, `Last-Modified` and percent-decoded UTF-8 paths (Korean folder names) are handled
  by `ServeDir`; paths are refused outside the root.
- Compressible files exist only as `<path>.zst` and are sent as-is with `Content-Encoding: zstd`
  when the request accepts it; without `Accept-Encoding: zstd` they return 404, while the files
  stored as-is (audio, images) are served to every client.
- `GET /data/manifest.json` returns the setup manifest.
- `GET /data/check` returns the per-file checksum list (`application/x-ndjson`, zstd-encoded when the
  request accepts it). It answers 404 when the tree was built before the list existed: run the setup
  again.

Configuration in `config.json` (defaults shown, also in `config.template.json`):

```json
"client_data": {
  "enabled": true,
  "host": "127.0.0.1",
  "port": 6903,
  "root": "config/client-data"
}
```

The endpoint runs on its own thread and port, independent of the bot API (6902). When the root is
missing the server logs how to build it and does not listen.
