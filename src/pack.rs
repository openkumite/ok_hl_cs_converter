//! The Counter-Strike pack: your Counter-Strike game's levels converted, from the game folder alone ([`convert`]),
//! and their install beside a Tekken 3 pack ([`install`]): the CS LEVELS arenas (stage ids 40..) and two extra
//! Tekken Force stages built on Tekken 3's, written as the files they add to or change in the Tekken 3 pack (the
//! extras layer the game reads over it).
//!
//! A pack (any `Vfs`: a folder, or the browser's pack file):
//!
//! ```text
//! manifest.json        {"pack": "cs", "format": FORMAT, "levels": [name, ...]}  (in the order of their stage ids)
//! levels/NAME.glb      each level as a glb (bsp2glb's: Half-Life units x 0.0254 m, Y up; its WADs' textures)
//! ```

use crate::game;
use crate::install::{read_glb, write_glb};
use crate::pyjson;
use anyhow::{Context, Result, bail};
use ok_converter_common::io::{Ctx, Vfs};
use serde_json::{Value, json};

/// The pack's layout and contents; another one is converted again.
pub const FORMAT: i64 = 1;
/// What a pack's manifest names it.
pub const KIND: &str = "cs";

fn level_path(name: &str) -> String {
    format!("levels/{name}.glb")
}

/// Converts the levels of the Counter-Strike game folder `game` (the Half-Life folder: cstrike/ and valve/) into a
/// pack in `ctx.out`, emptied first; writes the manifest last and returns it.
pub fn convert(ctx: &Ctx, game: &dyn Vfs) -> Result<Value> {
    ctx.out.remove_all("")?;
    let mut names = Vec::new();
    for (name, (j, bin)) in game::convert_levels(game, ctx.log)? {
        ctx.log(&format!("[cs] {name}"));
        ctx.out.write(&level_path(&name), &write_glb(&j, &bin))?;
        names.push(name);
    }
    let manifest = json!({"pack": KIND, "format": FORMAT, "levels": names});
    ctx.out.write("manifest.json", pyjson::indent1(&manifest).as_bytes())?;
    ctx.out.checkpoint(&[])?;
    Ok(manifest)
}

/// The manifest of the pack in `pack`, checked: a Counter-Strike pack of this format.
pub fn manifest(pack: &dyn Vfs) -> Result<Value> {
    let m: Value = serde_json::from_slice(&pack.read("manifest.json")?).context("the pack's manifest.json")?;
    if m["pack"] != KIND {
        bail!("not a Counter-Strike pack");
    }
    if m["format"] != FORMAT {
        bail!("a Counter-Strike pack of another version: convert the Counter-Strike levels again");
    }
    Ok(m)
}

/// The level names of the pack, in the order of their stage ids.
pub fn levels(pack: &dyn Vfs) -> Result<Vec<String>> {
    let m = manifest(pack)?;
    Ok(m["levels"].as_array().into_iter().flatten().filter_map(|n| n.as_str().map(str::to_string)).collect())
}

/// Installs the pack `pack` into `ctx.out`: a view of the Tekken 3 pack whose writes go to the extras (an
/// `ok_converter_common::io::Layered` of the extras over it); the fingerprint is the caller's.
pub fn install(ctx: &Ctx, pack: &dyn Vfs) -> Result<()> {
    let names = levels(pack)?;
    game::install_levels(
        ctx,
        names.into_iter().map(|name| {
            let (j, mut bin) = read_glb(&pack.read(&level_path(&name))?)?;
            // (the glb's binary chunk is padded to 4 bytes; the level's buffer is its own length)
            let len = j["buffers"][0]["byteLength"].as_u64().context("the level's glb has no buffer")? as usize;
            bin.truncate(len);
            Ok((name, (j, bin)))
        }),
    )
}
