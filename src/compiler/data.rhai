fn Square(name){
  #{
    name: name,
    // rust doesn't support 'type' lol
    typ: 101,
    filename: "fullSizeSquare.png",
    id: uuid(),
    rules: []
  }
}

fn game_starts(obj) {
  #{
    rule_block_type: 6000,
    object_id: obj,
    id: uuid(),
    ability_id: "",
    parameters: []
  }
}

objects.push(Fn("Square"));
rules.push(Fn("game_starts"));