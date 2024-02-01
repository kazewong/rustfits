# A pure rust fits file reader with built-in WASM support

This is a standalone lightweight fits reader written in pure rust. It is designed to be
the backend of [rustfits-web](https://github.com/kazewong/rustfits-web)

## Features/RoadMap

- Header
  - [x] Reading fitsblocks
  - [x] Checktype
- Image data
  - [x] Reading fitsblocks
  - [x] Converting data into desire precision
  - [ ] Add optional keyword detectors
  - [ ] Async read 
- ASCII table
  - [x] Reading fitsblocks
  - [ ] Formatting
  - [ ] Add optional keyword detectors
  - [ ] Async read
- Binary table
  - [x] Reading fitsblocks
  - [x] Formatting data into a table
  - [ ] Add optional keyword detectors
  - [ ] Async read
  - [ ] Variable length array
- Compressed Data
  - [ ] Reading fitsblocks
  - [ ] Decompressing data
  - [ ] Async read
  - [ ] Add optional keyword detectors
- World Coordinate System
  - [ ] Understanding the structure
- Random Groups
  - [ ] Understanding the structure
- Benchmarking
  - [ ] Read speed
  - [ ] Memory usage
  - [ ] 1D data test
  - [ ] 2D data test
  - [ ] 3D data test
  - [ ] Table data test