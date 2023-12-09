// OBJECTS
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

objects.push(Fn("Square"));

// RULES
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

rules.push(Fn("game_starts"));
rules.push(Fn("is_tapped"));

// MOVEMENT
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

fn turn(par) {
	#{
		"block_class": "method",
		"type": 24,
		"description": "Turn",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "degrees",
				"type": 57
			})
		]
	}
}

fn set_position(par) {
	#{
		"block_class": "method",
		"type": 41,
		"description": "Set Position",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "to x",
				"type": 57
			}),
			paramset(par[1], #{
				"defaultValue": "",
				"key": "y",
				"type": 57
			})
		]
	}
}

fn flip(par) {
	#{
		"block_class": "method",
		"type": 50,
		"description": "Flip"
	}
}

fn change_x(par) {
	#{
		"block_class": "method",
		"type": 27,
		"description": "Change X by",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "",
				"type": 57
			})
		]
	}
}

fn change_y(par) {
	#{
		"block_class": "method",
		"type": 28,
		"description": "Change Y by",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "",
				"type": 57
			})
		]
	}
}

fn set_speed(par) {
	#{
		"block_class": "method",
		"type": 34,
		"description": "Set Speed",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "to",
				"type": 57
			})
		]
	}
}

fn set_angle(par) {
	#{
		"block_class": "method",
		"type": 39,
		"description": "Set Angle",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "",
				"type": 57
			})
		]
	}
}

fn set_origin(par) {
	#{
		"block_class": "method",
		"type": 59,
		"description": "Set Origin",
		"parameters": [
			paramset(par[0], #{
				"defaultValue": "",
				"key": "to x",
				"type": 57
			}),
			paramset(par[1], #{
				"defaultValue": "",
				"key": "y",
				"type": 57
			})
		]
	}
}

blocks.push(Fn("move_forward"));
blocks.push(Fn("turn"));
blocks.push(Fn("set_position"));
blocks.push(Fn("flip"));
blocks.push(Fn("change_x"));
blocks.push(Fn("change_y"));
blocks.push(Fn("set_speed"));
blocks.push(Fn("set_angle"));
blocks.push(Fn("set_origin"));


// VARIABLES
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

blocks.push(Fn("set_variable"));

// CONTROL
fn destroy(par) {
	#{
		"block_class": "method",
		"type": 55,
		"description": "Destroy"
	}
}

blocks.push(Fn("destroy"));
