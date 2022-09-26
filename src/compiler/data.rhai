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
  [
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

fn destroy() {
	#{
		"block_class": "method",
		"typ": 55,
		"description": "Destroy"
	}
}

objects.push(Fn("Square"));
rules.push(Fn("game_starts"));
blocks.push(Fn("destroy"))