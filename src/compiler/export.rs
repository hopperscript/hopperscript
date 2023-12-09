//Convert `Project` struct into JSON
use crate::types::{Project};

// perform this function on the final result
// fn typ_to_type(obj: &mut Value) -> &mut Value{
//   obj.to_owned().as_object().unwrap().keys().for_each(|key|{
//     if key == "typ"{
//       obj["type"] = obj["typ"].clone();
//       let mut obj_as_map = obj.as_object().unwrap().to_owned();
//       obj_as_map.remove("typ");
//       *obj = Value::from(obj_as_map);
//     }else{
//       let value = &obj[key];

//       //TODO: deal with arrays
//       if value.is_object(){
//       obj[key] = typ_to_type(&mut obj[key]).clone();
//     }
//   }
//   });
//   return obj;
// }

pub fn to_json(prj: Project) -> String {
    // TODO should prettify till it's ready i guess
    serde_json::to_string_pretty(&prj).expect("Failed to export project JSON")
    // println!("{:#?}", prj.rules);
    // let mut json = json!({
    //   "playerVersion": "1.5.0",
    //   "baseObjectScale": 1,
    //   "customObjects": [],
    //   "variables": [],
    //   "eventParameters": [],
    //   "fontSize": 80,
    //   "scenes": [],
    //   "version": 32,
    //   "requires_beta_editor": false,
    //   "abilities": [],
    //   "rules": [],
    //   "customRules": [],
    //   "traits": [],
    //   "objects": [],
    //   "stageSize": {
    //     "width": 1024,
    //     "height": 768
    //   },
    //   "playerUpgrades": {},
    //   "uuid": prj.uuid,
    //   "author": "Hopperscript", // should maybe change
    //   "deleted_at": null,
    //   "edited_at": null,
    //   "filename": "",
    //   "text_object_label": null,
    //   "title": "", //defined in code, somewhere
    //   "has_been_removed": false,
    //   "in_moderation": false,
    //   "remote_asset_urls": []
    // });

    // fn g<'a>(e: &'a str, json: &'a mut Value) -> &'a mut Value {
    //     let ln = json[e].as_array().unwrap().len();
    //     json[e].as_array_mut().unwrap().push(json!({}));
    //     &mut json[e][ln]
    // }

    // //Objects
    // prj.objects.iter().for_each(|h| {
    //     let obj = &mut g("objects", &mut json);

    //     obj["name"] = h.name.clone().into();
    //     obj["filename"] = h.filename.clone().into();
    //     obj["objectID"] = h.id.clone().into();
    //     obj["type"] = h.typ.clone().into();
    //     obj["rules"] = h.rules.clone().into();
    // });
    // prj.rules.iter().for_each(|h| {
    //   let obj = &mut g("rules", &mut json);

    //   obj["ruleBlockType"] = h.rule_block_type.clone().into();
    //   obj["objectID"] = h.object_id.clone().into();
    //   obj["id"] = h.id.clone().into();
    //   obj["abilityID"] = h.ability_id.clone().into();
      
    //   let mut params: Vec<Value> = vec![];
    //   let mut i = 0;

    //   h.params.iter().for_each(|param|{
    //     let mut json_param = json!(param);
    //     //params.push(typ_to_type(&mut json_param).clone());
    //     i+=1;
    //   });

    //   obj["rules"] = json!(params);
    // });
    // return json;
}
