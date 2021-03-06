# Memory Map

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/memory_map/blob/master/COPYING)

Cross-platform Rust API for memory-mapped file IO.


|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/memory_map)](https://crates.io/crates/memory_map)|[![Build Status](https://travis-ci.org/maidsafe/memory_map.svg?branch=master)](https://travis-ci.org/maidsafe/memory_map)|[![Build status](https://ci.appveyor.com/api/projects/status/8d5pheadfx7ek0hd/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/memory-map/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/memory_map/badge.svg)](https://coveralls.io/r/maidsafe/memory_map)|[![Stories in Ready](https://badge.waffle.io/maidsafe/memory_map.png?label=ready&title=Ready)](https://waffle.io/maidsafe/memory_map)|

| [API Documentation - master branch](http://maidsafe.net/memory_map/master) | [SAFE Network System Documentation](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [SAFE Network Forum](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

## Features

- POSIX support.
- Windows support.
- File-backed memory maps.
- Anonymous memory maps.
- Synchronous and asynchrounous flushing.
- Copy-on-write memory maps.
- Read-only memory maps.
