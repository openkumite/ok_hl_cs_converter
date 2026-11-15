//! `cs_levels install LEVEL.glb ASSETS [STAGE_ID]`, `cs_levels force_stage ASSETS STAGE_ID BASE_STAGE [NAME]`:
//! an arena of the CS LEVELS menu, an extra Tekken Force stage; `cs_levels from_game GAME_DIR ASSETS`: all the
//! game's levels and the two Force stages. Each ends by refreshing ASSETS/manifest.json's fingerprint.
//! `cs_levels pack GAME_DIR PACK`, `cs_levels install_pack PACK T3_PACK EXTRAS`: what OpenKumite runs (pack.rs).
//! `cs_levels bsp2glb MAP.bsp OUT.glb [GAME_DIR]`: one level as a glb (to look at; the WADs from GAME_DIR, by default
//! the Half-Life folder the map is in).

use ok_converter_common::io::{Ctx, Layered, StdFs, print_line};
use ok_hl_cs_converter::{force_stage, game, install, pack};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let r = match args.first().map(String::as_str) {
        Some("install") => install::main(&args[1..]),
        Some("force_stage") => force_stage::main(&args[1..]),
        Some("from_game") => game::main(&args[1..]),
        Some("bsp2glb") => game::bsp2glb_main(&args[1..]),
        Some("pack") => pack_main(&args[1..]),
        Some("install_pack") => install_main(&args[1..]),
        _ => Err(anyhow::anyhow!(
            "usage: cs_levels install LEVEL.glb ASSETS [STAGE_ID]\n       cs_levels force_stage ASSETS STAGE_ID BASE_STAGE [NAME]\n       cs_levels from_game GAME_DIR ASSETS\n       cs_levels bsp2glb MAP.bsp OUT.glb [GAME_DIR]\n       cs_levels pack GAME_DIR PACK\n       cs_levels install_pack PACK T3_PACK EXTRAS"
        )),
    };
    if let Err(e) = r {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

/// `pack GAME_DIR PACK`: the Counter-Strike pack of the Half-Life folder GAME_DIR.
fn pack_main(args: &[String]) -> anyhow::Result<()> {
    let [game_dir, out] = args else { anyhow::bail!("usage: cs_levels pack GAME_DIR PACK") };
    pack::convert(
        &Ctx { out: &StdFs(out.into()), log: &print_line, jobs: 1 },
        &StdFs(game::game_root(game_dir.as_ref())?.0),
    )?;
    Ok(())
}

/// `install_pack PACK T3_PACK EXTRAS`: the pack installed beside a Tekken 3 pack, into the extras' folder.
fn install_main(args: &[String]) -> anyhow::Result<()> {
    let [pack_dir, t3, extras] = args else { anyhow::bail!("usage: cs_levels install_pack PACK T3_PACK EXTRAS") };
    let (top, base) = (StdFs(extras.into()), StdFs(t3.into()));
    let view = Layered { top: &top, below: vec![&base] };
    pack::install(&Ctx { out: &view, log: &print_line, jobs: 1 }, &StdFs(pack_dir.into()))
}
