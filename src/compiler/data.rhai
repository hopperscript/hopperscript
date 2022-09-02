fn Square(name){
  #{
    name: name,
    // rust doesn't support 'type' lol
    typ: 101,
    filename: "fullSizeSquare.png",
    id: uuid()
  }
}

fn game_starts(obj) {
  #{
    rule_block_type: 6000,
    object_id: obj,
    id: uuid(),
    ability_id: "",
    parameters: [
		  #{
			  datum: #{
				  typ: 7000,
				  block_class: "operator",
				  description: "Game Starts"
			  },
			  default_value: "",
			  value: "",
			  typ: 52,
			  key: ""
		  }
    ]
  }
}

objects.push(Fn("Square"));
rules.push(Fn("game_starts"));