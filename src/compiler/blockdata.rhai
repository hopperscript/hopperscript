fn MoveForward(arg){
  let h = #{
    "block_class": "method",
    "type": 23,
    "description": "Move Forward",
    "parameters": []
  };
  h.parameters = #{
    "value": arg[0],
    "defaultValue": "",
    "key": "",
    "type": 57
  }; h
}
fn main(function, args){
  Fn(function).call(args)
}
print("h")