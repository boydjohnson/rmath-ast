pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub math);

pub use self::math::ExprParser;

#[cfg(test)]
mod tests {
    use super::math::{ExprParser, FloatParser, IntParser, NumParser, PosIntParser, TermParser};
    use crate::parser::ast::{BinaryOp, Expr, Num, ProbGenerator, Term};
    use yajlish::ndjson_handler::Selector;

    #[test]
    fn test_float_parser() {
        assert_eq!(FloatParser::new().parse("4.5"), Ok(4.5));

        assert_eq!(FloatParser::new().parse("0.55546"), Ok(0.55546));

        assert_eq!(FloatParser::new().parse("45e2"), Ok(4500.0));

        assert_eq!(FloatParser::new().parse("4676e-2"), Ok(46.76));
        assert!(FloatParser::new().parse("345-443").is_err());
    }

    #[test]
    fn test_posint_parser() {
        assert_eq!(PosIntParser::new().parse("45643948"), Ok(45643948));

        assert!(PosIntParser::new()
            .parse("284849585850000000000000")
            .is_err());

        assert!(PosIntParser::new().parse("44858---00000").is_err());
    }

    #[test]
    fn test_int_parser() {
        assert_eq!(IntParser::new().parse("-345454"), Ok(-345454));
        assert_eq!(IntParser::new().parse("-10"), Ok(-10));

        assert!(IntParser::new().parse("-10.0").is_err());
        assert!(IntParser::new().parse("--10.4").is_err());
    }

    #[test]
    fn test_num_parser() {
        assert_eq!(NumParser::new().parse("4564345"), Ok(Num::PosInt(4564345)));
        assert_eq!(NumParser::new().parse("0.0456400"), Ok(Num::Float(0.04564)));
        assert_eq!(NumParser::new().parse("345e-3"), Ok(Num::Float(0.345)));
        assert_eq!(NumParser::new().parse("-345"), Ok(Num::Int(-345)));

        assert!(NumParser::new().parse("-345-").is_err());
    }

    #[test]
    fn test_term_parser() {
        assert_eq!(
            TermParser::new().parse("d.manager.salary"),
            Ok(Box::new(Expr::Term(Term::Value(vec![
                Selector::Identifier("manager".into()),
                Selector::Identifier("salary".into())
            ])))),
        );

        assert_eq!(
            TermParser::new().parse("rbern( seed = 645, prob = 0.5)"),
            Ok(Box::new(Expr::Term(Term::ProbGenerator(
                ProbGenerator::RBern {
                    seed: 645,
                    prob: 0.5
                }
            )))),
        );
    }

    #[test]
    fn test_expr_parser() {
        assert_eq!(
            ExprParser::new().parse("7 + 5 * 9"),
            Ok(Box::new(Expr::Op(
                Box::new(Expr::Term(Term::Num(Num::PosInt(7)))),
                BinaryOp::Add,
                Box::new(Expr::Op(
                    Box::new(Expr::Term(Term::Num(Num::PosInt(5)))),
                    BinaryOp::Mul,
                    Box::new(Expr::Term(Term::Num(Num::PosInt(9))))
                ))
            ))),
        );

        assert_eq!(
            ExprParser::new().parse("(d.TOTAL_SALES + 5) / rbern(prob = 0.2, seed=5)"),
            Ok(Box::new(Expr::Op(
                Box::new(Expr::Op(
                    Box::new(Expr::Term(Term::Value(vec![Selector::Identifier(
                        "TOTAL_SALES".into()
                    )]))),
                    BinaryOp::Add,
                    Box::new(Expr::Term(Term::Num(Num::PosInt(5))))
                )),
                BinaryOp::Div,
                Box::new(Expr::Term(Term::ProbGenerator(ProbGenerator::RBern {
                    seed: 5,
                    prob: 0.2
                })))
            ))),
        );

        assert!(ExprParser::new()
            .parse("(d.TOTAL_SALES + ) / rbern(prob = 0.2, seed = 5)")
            .is_err());
    }
}