# Fido
E2E Solution for your distributed file storage

## Real-World use case

1. you need to download a file, many files, a bunch of file from your scraper
2. one disk is not enough, you always need more
3. then you decide to use Fido with Systemd
4. Fido will automatically distribute your files into selected external disk
5. When your files getting bigger again, just add external disk again to your server

## Prerequisite

If you only use --cmd 0, you can pass this section

1. Make sure you have running mongodb server, if not, [install mongodb](https://docs.mongodb.com/manual/tutorial/install-mongodb-on-ubuntu/)
2. Setup mongodb url in [here](https://github.com/codenoid/Fido/blob/master/src/main.rs#L49)

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