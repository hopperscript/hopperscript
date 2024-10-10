pub mod generator {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::types::*;
    use crate::utils::*;

    /// Generate the `Project` data from compiled AST
    pub fn gen_project(p: &Vec<Script>, bd: CompiledData) -> Project {
        use radix_fmt::radix;

        let uuid = radix(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Can't generate UUID for some reason")
                .as_millis()
                / 1000
                * 65536,
            36,
        )
        .to_string();

        let mut proj = Project {
            variables: vec![],
            uuid,
            objects: vec![],
            rules: vec![],
            abilities: vec![],
            scenes: vec![Scene {
                name: "My Scene".to_string(),
                objects: vec![],
            }],
            event_params: vec![],
        };

        for v in p.to_owned() {
            fn convert_to_block(
                c: &BlockAST,
                bd: &CompiledData,
                ability_json: &mut Ability,
                proj: &Project,
            ) {
                if c.typ == AstTypes::Block {
                    let ptr = bd
                        .blocks
                        .to_owned()
                        .into_iter()
                        .find(|v| v.name == c.name)
                        .expect("Block not found");

                    let transformed = transform_vals(c.params.clone(), &proj);
                    let params = transformed
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| Param {
                            datum: v.datum,
                            default_value: "".to_string(),
                            key: "".to_string(),
                            typ: match ptr.parameters[i].typ.as_str() {
                                "num" => 57,
                                "evt" => 50,
                                &_ => 0,
                            },
                            value: v.value,
                            variable: None,
                        })
                        .collect::<Vec<Param>>();

                    let block = Block {
                        block_class: "method".to_string(),
                        typ: ptr.id,
                        description: ptr.label,
                        parameters: Some(params),
                        control_script: None,
                    };

                    ability_json.blocks.push(block);
                } else {
                    let ability = if &c.name == ability_json.name.get_or_insert("".to_string()) {
                        ability_json.ability_id.clone()
                    } else {
                        proj.abilities
                            .clone()
                            .into_iter()
                            .find(|v| v.name.as_ref().expect("Rule not found?") == &c.name)
                            .expect("Ability not found")
                            .ability_id
                    };

                    ability_json.blocks.push(Block {
                        typ: 123,
                        description: c.name.clone(),
                        control_script: Some(ControlScript {
                            ability_id: ability,
                        }),
                        block_class: "control".to_string(),
                        parameters: None,
                    });
                }
            }

            match v {
                Script::Define { typ, name } => {
                    match typ {
                        DefineTypes::Variable(code) => proj.variables.push(Variable {
                            name: name.to_string(),
                            typ: 8000 + code,
                            object_id_string: giv_me_uuid(),
                        }),

                        DefineTypes::Object(val) => {
                            // TODO: use ariadne

                            let f = bd
                                .obj
                                .to_owned()
                                .into_iter()
                                .find(|v| v.name == val)
                                .expect("Object not found");

                            let act_res = Object {
                                filename: "".to_string(),
                                typ: f.id,
                                name,
                                id: giv_me_uuid(),
                                rules: vec![] as Vec<String>,
                                x: 10, //dont forget!!
                                y: 10,
                            };

                            let actbor = &act_res;

                            proj.scenes[0].objects.push(actbor.id.to_owned());
                            proj.event_params.push(EventParam {
                                description: actbor.name.to_owned(),
                                block_type: 8000,
                                id: giv_me_uuid(),
                                object_id: Some(actbor.id.to_owned()),
                            });
                            proj.objects.push(act_res)
                        }

                        DefineTypes::Ability(mut blocks) => {
                            let id = giv_me_uuid();

                            let mut ability_json = Ability {
                                ability_id: id,
                                blocks: vec![],
                                created_at: 0,
                                name: Some(name),
                            };

                            for c in blocks.get_or_insert(vec![]) {
                                convert_to_block(&c, &bd, &mut ability_json, &proj);
                            }

                            proj.abilities.push(ability_json)
                        } //_ => todo!(),
                    }
                }

                Script::On { obj, con } => {
                    for v in con {
                        match v {
                            Script::Rule { name, con, params } => {
                                let ob = proj
                                    .objects
                                    .iter()
                                    .position(|p| p.name == obj)
                                    .expect("No object with that name");
                                let object = proj.objects[ob].to_owned();

                                let transformed = transform_vals(params, &proj);

                                //make this a func for reuse with the block part
                                let datum = if name != "cond" {
                                    let f = bd
                                        .rules
                                        .to_owned()
                                        .into_iter()
                                        .find(|v| v.name == name)
                                        .expect("Rule not found");

                                    let paramets = transformed
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, v)| Param {
                                            datum: v.datum,
                                            default_value: "".to_string(),
                                            key: "".to_string(),
                                            typ: match f.parameters[i].typ.as_str() {
                                                "num" => 57,
                                                "evt" => 50,
                                                &_ => 0,
                                            },
                                            value: v.value.to_owned(),
                                            variable: if f.parameters[i].typ.as_str() == "evt" {
                                                Some(
                                                    proj.event_params
                                                        .to_owned()
                                                        .into_iter()
                                                        .find(|ev| {
                                                            ev.object_id.as_ref().unwrap()
                                                                == &v.value
                                                        })
                                                        .expect("Object not found")
                                                        .id,
                                                )
                                            } else {
                                                None
                                            },
                                        })
                                        .collect::<Vec<Param>>();

                                    Datum {
                                        block_class: Some("operator".to_string()),
                                        typ: f.id,
                                        object: None,
                                        variable: None,
                                        params: Some(paramets),
                                    }
                                } else {
                                    value_to_param(&transformed[0], 52, None).datum.unwrap()
                                };

                                // let res = f
                                //     .call(
                                //         &bd.eng,
                                //         &bd.ast,
                                //         (
                                //             object.to_owned().id,
                                //             transformed,
                                //             to_dynamic(&proj).unwrap(),
                                //         ),
                                //     )
                                //     .expect("Failed to get rule");

                                let ability = giv_me_uuid();

                                let rule = Rule {
                                    rule_block_type: 6000,
                                    object_id: object.id,
                                    id: giv_me_uuid(),
                                    ability_id: ability.to_owned(),
                                    params: vec![Param {
                                        default_value: "".to_string(),
                                        key: "".to_string(),
                                        datum: Some(datum),
                                        typ: 52,
                                        value: "".to_string(),
                                        variable: None,
                                    }],
                                };

                                let mut ability_json = Ability {
                                    ability_id: ability,
                                    blocks: vec![],
                                    created_at: 0,
                                    name: None,
                                };

                                for c in con {
                                    convert_to_block(&c, &bd, &mut ability_json, &proj);
                                }

                                proj.abilities.push(ability_json);

                                proj.objects[ob].rules.push(rule.to_owned().id);

                                proj.rules.push(rule)
                            }

                            _ => {}
                        }
                    }
                }

                _ => {}
            }
        }

        proj
    }
}
