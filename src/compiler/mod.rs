mod types;

/// Main module for the compiler
pub mod compiler {
    use chumsky::error::Cheap;
    use chumsky::prelude::*;
    use chumsky::text::whitespace;
    use uuid::Uuid;

    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    pub type Span = std::ops::Range<usize>;

    #[derive(Clone, Debug)]
    pub enum Script {
        Define {
            typ: String,
            name: String,
            //val: Option<String>,
        },
        Str(String),
        Loop(Vec<Self>),
    }

    /// The main compile fn
    ///
    /// Just throw a string that needs to be compiled
    ///
    /// I mean a `str`
    pub fn compile() -> impl Parser<char, Vec<(Script, Span)>, Error = Cheap<char>> {
        let stri = just::<_, _, Cheap<char>>('"')
            .ignore_then(filter(|c| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>();

        let def = just("define")
            .ignore_then(whitespace())
            .ignore_then(text::ident())
            .then_ignore(whitespace())
            .then(stri)
            .map(|(a, b)| Script::Define { typ: a, name: b });

        def.recover_with(skip_then_retry_until([]))
            .map_with_span(|tok, span| (tok, span))
            .padded()
            .repeated()
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
