fn Square(name){
  #{
    name: name,
    // rust doesn't support 'type' lol
    typ: 101,
    filename: "fullSizeSquare.png",
    id: uuid()
  }
}

fn game_starts() {
  #{
    rule_block_type: 6000,
    object_id: "",
    id: "",
    ability_id: "",
    parameters: []
  }
}

objects.push(Fn("Square"));
rules.push(Fn("game_starts"));