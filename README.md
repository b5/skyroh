# Skyroh: bluesky-p2p-pds

This is a one off proof-of-concept that I'm in no way claiming I'll finish or support. Just exploring what's possible.

### Intended Stack:
* [tauri](https://tauri.app/) as desktop/iOS/Android application host
* [ouranos](https://github.com/pdelfan/ouranos) for the frontend (replacing next.js with vite)
* [rsky](https://github.com/blacksky-algorithms/rsky) for backend
* [iroh](https://github.com/n0-computer/iroh) added to rsky-pds 

nothing "works", but at least it compiles.


### Tauri Parts:
```
cd skyroh
npm i
npm run tauri dev
```

### rky parts:
requires rust
```
cd rsky/rsky-pds
cargo run
```