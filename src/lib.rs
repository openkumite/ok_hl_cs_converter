//! Counter-Strike levels as port arenas and Tekken Force stages: converted from the game folder (`game`: `bsp`,
//! `wad`, `level_glb`), installed into a Tekken 3 conversion (`install`, `force_stage`), the fingerprint again
//! (`fingerprint`). Files go through `ok_converter_common::io::Vfs`; the `main` functions are the command line's.

pub mod bsp;
pub mod fingerprint;
pub mod force_stage;
pub mod game;
pub mod install;
pub mod level_glb;
pub mod pack;
#[allow(dead_code)]
// a verbatim copy of ok_tekken3_converter's pyjson.rs (Tekken 3's converter is not a dependency)
mod pyjson;
pub mod wad;
