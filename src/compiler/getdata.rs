use crate::types::*;

/// Generate block and object data
pub fn generate_blocks() -> CompiledData {
    // TODO: SUPPORT CUSTOM PATHS
    let contents = include_str!("data/blocks.json");
    let objcontents = include_str!("data/objects.json");
    let blockjson: Vec<BlockData> = serde_json::from_str(contents).expect("oops");
    let objectjson: Vec<ObjectData> = serde_json::from_str(objcontents).expect("oops");

    // rename every block n object
    let betterblockjson = blockjson.into_iter().map(|v| {
        let mut y = v;
        y.name = y.name.replace(" ", "_").to_lowercase();
        y
    });
    let betterobjectjson: Vec<ObjectData> = objectjson
        .into_iter()
        .map(|v| {
            let mut y = v;
            y.name = y.name.replace(" ", "_").to_lowercase();
            y
        })
        .collect();

    //filter out rules
    let rulejson: Vec<BlockData> = betterblockjson
        .to_owned()
        .into_iter()
        .filter(|v: &BlockData| v.typ == "rule")
        .collect();
    let actualblockjson: Vec<BlockData> = betterblockjson
        .into_iter()
        .filter(|v| !rulejson.contains(v))
        .collect();

    CompiledData {
        obj: betterobjectjson,
        rules: rulejson,
        blocks: actualblockjson,
    }
}
