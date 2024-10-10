mod ast;
mod export;
mod generator;
mod getdata;
mod types;
mod utils;

// gosh this is such a mess
/// Main module for the compiler
pub mod compiler {
    use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
    use chumsky::prelude::*;

    use crate::ast::ast;
    pub use crate::export::to_json;
    use crate::generator::generator::gen_project;
    use crate::getdata::{self};
    use crate::types::*;

    pub type Span = std::ops::Range<usize>;

    /// The main compile fn
    ///
    /// Just throw a string that needs to be compiled
    ///
    /// I mean a `&str`
    pub fn compile(s: &str) -> Project {
        let (a, errs) = ast::generate_ast().parse_recovery(s);

        // Report::build(ReportKind::Error, (), 0)
        //.with_message("No such block")
        //.with_label(Label::new(ln..ln+name.len())
        //.with_message(format!("Block \"{}\" not found", name).fg(Color::Red))
        //.with_color(Color::Red))
        //.finish()
        //.print(Source::from(format!("{}{}","\n".repeat(ln),line))).unwrap();panic!();

        // very much copied code
        // also very experimental
        errs.into_iter().for_each(|v| {
            let msg = if let chumsky::error::SimpleReason::Custom(msg) = v.reason() {
                msg.clone()
            } else {
                format!(
                    "{}{}, expected {}",
                    if v.found().is_some() {
                        "Unexpected token"
                    } else {
                        "Unexpected end of input"
                    },
                    if let Some(label) = v.label() {
                        format!(" while parsing {}", label)
                    } else {
                        String::new()
                    },
                    if v.expected().len() == 0 {
                        "something else".to_string()
                    } else {
                        v.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.to_string(),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                )
            };

            Report::build(ReportKind::Error, (), v.span().start)
                .with_message(msg)
                .with_label(
                    Label::new(v.span())
                        .with_message(match v.reason() {
                            chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                            _ => format!(
                                "Unexpected {}",
                                v.found()
                                    .map(|c| format!("token {}", c.fg(Color::Red)))
                                    .unwrap_or_else(|| "end of input".to_string())
                            ),
                        })
                        .with_color(Color::Red),
                )
                .finish()
                .print(Source::from(&s))
                .unwrap();

            // panic?
        });

        // println!("{:#?}", a);

        gen_project(&a.unwrap(), getdata::generate_blocks())
    }
}
