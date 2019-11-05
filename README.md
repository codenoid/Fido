# [WIP]Fido
E2E Solution for your distributed file storage

## Installation

```
git clone 
cd Fido
cargo build --release
```

## Usage

```bash
# generate symbolic link for any data that already in brick/slave/disk
./target/release/Fido --cmd 0 --path /path/to/test-fido/ --bricks /mnt/disks/disk-1

# move temporary file to brick/slave/disk, then create ln from disk to temporary location
./target/release/Fido --cmd 1 --path /path/to/scrapped-data --bricks /mnt/disks/
```