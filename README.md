## Features: 
- Create New Album
- List Albums
- Upload all files/directories/sub-directories files to bunkr (individually or to an album, both supported)
- Upload progress bar

## Usage: 
### Requirements
- You must have an bunkr account to be able to upload.
- `https://dash.bunkr.cr/dashboard` visit your dashboard to get the account token.

### Installation
```
cargo install bunkr-uploader
```

```
Usage: bunkr-uploader [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  path to files or directory

Options:
  -f          force upload without skipping(for special case)
  -h, --help  Print help
```
