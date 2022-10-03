//Convert `Project` struct into JSON
use crate::{types::{Project}};
use serde_json::{json, Value};

pub fn to_json(prj: Project) -> Value{
    let mut json = json!({
        "playerVersion": "1.5.0",
        "baseObjectScale": 1,
        "customObjects": [],
        "variables": [],
        "eventParameters": [],
        "fontSize": 80,
        "scenes": [],
        "version": 32,
        "requires_beta_editor": false,
        "abilities": [],
        "rules": [],
        "customRules": [],
        "traits": [],
        "objects": [],
        "stageSize": {
          "width": 1024,
          "height": 768
        },
        "playerUpgrades": {},
        "uuid": prj.uuid,
        "author": "Hopperscript", // should maybe change
        "deleted_at": null,
        "edited_at": null,
        "filename": "",
        "text_object_label": null,
        "title": "", //defined in code, somewhere
        "has_been_removed": false,
        "in_moderation": false,
        "remote_asset_urls": []
      });

      //Objects
      prj.objects.iter().for_each(|h|{
        let ln = json["objects"].as_array().unwrap().len();
        json["objects"].as_array_mut().unwrap().push(json!({}));
        let obj = &mut json["objects"][ln];

        obj["name"] = h.name.clone().into();
        obj["filename"] = h.filename.clone().into();
        obj["objectID"] = h.id.clone().into();
        obj["type"] = h.typ.clone().into();
        obj["rules"] = h.rules.clone().into();
      });
      return json;
}