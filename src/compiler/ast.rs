pub mod ast {
    use crate::types::*;
    use crate::utils::is_hex;

    use chumsky::prelude::*;
    use chumsky::text::ident;

    /// Generate the "AST" or whatever for the generator
    pub fn generate_ast() -> impl Parser<char, Vec<Script>, Error = Simple<char>> {
        let escape = just('\\').ignore_then(
            just('\\')
                .or(just('/'))
                .or(just('"'))
                .or(just('n').to('\n'))
                .or(just('t').to('\t')),
        );
        let stri = just('"')
            .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
            .then_ignore(just('"'))
            .collect::<String>();

        let hex_color_short = just::<char, char, Simple<char>>('#')
            .chain(filter(|c: &char| is_hex(*c)).repeated().exactly(3))
            .collect::<String>();
        let hex_color_long = just::<char, char, Simple<char>>('#')
            .chain(filter(|c: &char| is_hex(*c)).repeated().exactly(6))
            .collect::<String>();

        let hex_color = hex_color_long.or(hex_color_short);

        let number = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
            .repeated()
            .at_least(1)
            .collect::<String>();

        let abil = just("ability!")
            .ignore_then(stri.delimited_by(just('('), just(')')))
            .padded()
            .map(|v| BlockAST {
                name: v,
                params: vec![],
                typ: AstTypes::Ability,
            });

        let var = just("var")
            .padded()
            .ignore_then(stri.padded())
            .map(|b| Script::Define {
                typ: DefineTypes::Variable(3),
                name: b,
            });

        let obj_ty = just::<_, _, Simple<char>>("objects")
            .ignore_then(just('.'))
            .ignore_then(ident());

        let obj = just("object")
            .padded()
            .ignore_then(stri.padded())
            .then_ignore(just('=').padded())
            .then(obj_ty.padded())
            .map(|(a, c)| Script::Define {
                typ: DefineTypes::Object(c),
                name: a,
            });

        let obj_ref = just('o').ignore_then(stri).map(Values::Object);
        let var_ref = just('v').ignore_then(stri).map(|a| Values::Variable(a, 3));
        let objvar_ref = just('v')
            .ignore_then(stri)
            .then_ignore(just('.'))
            .then(stri)
            .map(|(obj, var)| Values::ObjectVariable(obj, var));
        let selfvar_ref = just("v Self.")
            .ignore_then(stri)
            .map(|a| Values::Variable(a, 4));
        let value = stri
            .map(Values::Str)
            .or(hex_color.map(Values::Str))
            .or(number.map(Values::Str))
            .or(obj_ref)
            .or(objvar_ref)
            .or(var_ref)
            .or(selfvar_ref);

        let block = ident()
            .then(
                value
                    .padded()
                    .separated_by(just(','))
                    .delimited_by(just('('), just(')')),
            )
            .padded()
            .map(|(a, b)| BlockAST {
                name: a,
                params: b,
                typ: AstTypes::Block,
            });

        let block_or_abil = block.or(abil);

        let ability_def = just("ability")
            .padded()
            .ignore_then(stri.padded())
            .then(
                block_or_abil
                    .repeated()
                    .or_not()
                    .delimited_by(just('{'), just('}'))
                    .padded(),
            )
            .map(|(a, c)| Script::Define {
                typ: DefineTypes::Ability(c),
                name: a,
            });

        let def = just("define").ignore_then(var.or(obj.or(ability_def)));

        let conditional = value
            .then(just("==").or(just("!=")).padded())
            .then(value)
            .map(|((a, b), c)| Values::Conditional(Box::new(a), b.to_string(), Box::new(c)));
        let actualconditional = conditional
            .clone()
            .delimited_by(just('('), just(')'))
            .or(conditional);
        let rule = just("when")
            .ignore_then(just("cond").map(|s| s.to_string()).or(ident()).padded())
            .then(
                obj_ref
                    .or(actualconditional)
                    .padded()
                    .separated_by(just(','))
                    .delimited_by(just('('), just(')')),
            )
            .then(
                block_or_abil
                    .repeated()
                    .or_not()
                    .delimited_by(just('{'), just('}'))
                    .padded(),
            )
            .map(|((a, c), mut b)| Script::Rule {
                name: a,
                con: b.get_or_insert(vec![]).to_vec(),
                params: c,
            })
            .padded();

        let on = just("for")
            .ignore_then(stri.padded())
            .then(
                def.or(rule)
                    .padded()
                    .repeated()
                    .delimited_by(just('{'), just('}'))
                    .padded()
                    .or_not(),
            )
            .map(|(a, mut b)| Script::On {
                obj: a,
                con: b.get_or_insert(vec![]).to_vec(),
            });

        def.or(on)
            .recover_with(skip_then_retry_until([]))
            .padded()
            .repeated()
    }
}
