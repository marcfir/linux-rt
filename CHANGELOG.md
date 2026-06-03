# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2026-06-03

### ✨ Features

* Support Windows

### ♻️ Refactor

* Refactor internal structure to support multiple platforms

### ⚙️ Miscellaneous Tasks

* 🚨 Rename crate to **rt_tunex**

## [0.4.1] - 2026-02-21

### ✨ Features

* Improve `TimeSpec` implementation
* 
### ⚙️ Miscellaneous Tasks

* Update dependencies

## [0.4.0] - 2025-12-17

### ✨ Features

* Improve `CpuSet` implementation

## [0.3.2] - 2025-12-10

### ✨ Features

* Add mman functions for Linux

### ⚙️ Miscellaneous Tasks

* Update dependencies
* Improve tests in CI

## [0.3.1] - 2025-09-22

### ✨ Features

* Improve `CpuSet` implementation

## [0.3.0] - 2025-09-19

### 🐛 Fixed

* Fixed `CpuSet` implementation

### ✨ Features

* Add clock calls for Linux like clock_gettime and clock_nanosleep

### ⚙️ Miscellaneous Tasks

* Add CI
  
## [0.2.1] - 2025-09-04

### ✨ Features

* Improve `Pid` by adding common derives

### ⚙️ Miscellaneous Tasks

* Bump dependencies

## [0.2.0] - 2025-05-13

### ✨ Features

* Initial release with support for scheduling calls on linux
