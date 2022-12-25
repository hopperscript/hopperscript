fn Square(name){
  #{
    name: name,
    // rust doesn't support 'type' lol
    type: 101,
    filename: "fullSizeSquare.png",
    id: uuid()
  }
}

fn game_starts(o, par) {
  [
	  #{
		  datum: #{
			  type: 7000,
			  block_class: "operator",
			  description: "Game Starts"
		  },
		  default_value: "",
		  value: "",
		  type: 52,
		  key: ""
	  }
  ]
}

fn is_tapped(o, param) {
	[
		#{
			"default_value": "",
			"key": "",
			"datum": #{
				"block_class": "operator",
				"type": 7001,
				"description": "is Tapped",
				"params": [
					#{
						"default_value": "",
						"value": "",
						"key": "",
						"type": 50,
						"variable": param[0].value
					}
				]
			},
			"type": 52,
			"value": ""
		}
	]
}

fn destroy(par) {
	#{
		"block_class": "method",
		"type": 55,
		"description": "Destroy"
	}
}

fn move_forward(par) {
	#{
		"block_class": "method",
		"type": 23,
		"description": "Move Forward",
		"parameters": [
			paramset(par[0], #{
				"default_value": "",
				"key": "",
				"type": 57
			})
		]
	}
}

objects.push(Fn("Square"));
rules.push(Fn("game_starts"));
blocks.push(Fn("destroy"));
blocks.push(Fn("move_forward"));
rules.push(Fn("is_tapped"));