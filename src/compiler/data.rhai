fn Square(name){
  #{
    name: name,
    // rust doesn't support 'type' lol
    type: 101,
    filename: "fullSizeSquare.png",
    objectID: uuid(),
		xPosition: 1,
		yPosition: 1, //TODO
  }
}

fn game_starts(o, par, proj) {
  [
	  #{
		  datum: #{
			  type: 7000,
			  block_class: "operator",
			  description: "Game Starts"
		  },
		  defaultValue: "",
		  value: "",
		  type: 52,
		  key: ""
	  }
  ]
}

fn is_tapped(o, param, proj) {
	[
		#{
			"defaultValue": "",
			"key": "",
			"datum": #{
				"block_class": "operator",
				"type": 7001,
				"description": "is Tapped",
				"params": [
					#{
						"defaultValue": "",
						"value": "",
						"key": "",
						"type": 50,
						"variable": proj.eventParameters.drain(|v| v.objectID == param[0].value)[0].id
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
				"defaultValue": "",
				"key": "",
				"type": 57
			})
		]
	}
}

fn set_variable(par) {
	#{
		"block_class": "method",
		"type": 45,
		"description": "Set",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "",
				"type": 47
			}),
			paramset(par[1], #{
				"defaultValue": "",
				"key": "to",
				"type": 48
			})
		]
	}
}

objects.push(Fn("Square"));
rules.push(Fn("game_starts"));
blocks.push(Fn("destroy"));
blocks.push(Fn("move_forward"));
blocks.push(Fn("set_variable"));
rules.push(Fn("is_tapped"));