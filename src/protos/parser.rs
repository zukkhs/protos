use utils::{Cursor, Stream};
use super::ast::{self};
use crate::utils::IsEmpty;

pub mod utils;

pub mod expression {
    use super::ast::{self, Expression};
    use super::utils::{Cursor, Parser, Stream};
    use super::*;

    pub fn parse_expression<S: Stream<Item = char>>(
        src: &mut Cursor<S>,
    ) -> Option<ast::Expression> {
        let a = (parse_term).otherwise(_parse_expression_paren)(src)?;
        Some(_parse_expression_tail(a, src))
    }

    fn _parse_expression_tail<S: Stream<Item = char>>(
        a: Expression,
        src: &mut Cursor<S>,
    ) -> ast::Expression {
        let mut backup_src = src.clone();

        match parse_whitespace(&mut backup_src).and_then(|_| _parse_expression_arg(&mut backup_src))
        {
            Some(b) => {
                *src = backup_src;
                _parse_expression_tail(ast::Expression::R(Box::new(ast::Relation(a, b))), src)
            }
            None => a,
        }
    }

    fn _parse_expression_arg<S: Stream<Item = char>>(
        src: &mut Cursor<S>,
    ) -> Option<ast::Expression> {
        parse_term.otherwise(_parse_expression_paren)(src)
    }

    fn _parse_expression_paren<S: Stream<Item = char>>(
        src: &mut Cursor<S>,
    ) -> Option<ast::Expression> {
        if src.next() != Some('(') {
            return None;
        }

        let expr = parse_expression(src)?;

        if src.next() != Some(')') {
            return None;
        }

        Some(expr)
    }
}

fn parse_term<S: Stream<Item = char>>(src: &mut Cursor<S>) -> Option<ast::Expression> {
    src.take_while_peek(is_term_char)
        .collect::<String>()
        .non_empty()
        .map(|x| ast::Expression::T(ast::Term(x)))
}

fn parse_whitespace<S: Stream<Item = char>>(src: &mut Cursor<S>) -> Option<()> {
    let mut res = None;

    for _ in src.take_while_peek(|x| x.is_whitespace()) {
        res = Some(())
    }

    res
}

fn is_term_char(c: &char) -> bool {
    !(c.is_control() || c.is_whitespace() || *c == '(' || *c == ')')
}

#[cfg(test)]
mod tests {
    use super::expression::parse_expression;
    use super::*;
    use crate::{r, t};

    #[test]
    fn can_parse_term() {
        let mut src = Cursor::new("term".chars().into_iter());
        assert_eq!(parse_term(&mut src), Some(t!("term")))
    }

    #[test]
    fn can_parse_relation() {
        let mut src = Cursor::new("a b".chars().into_iter());
        assert_eq!(parse_expression(&mut src), Some(r!(t!("a"), t!("b"))))
    }

    #[test]
    fn can_parse_parenthesized_relation() {
        let mut src = Cursor::new("(a b c)".chars().into_iter());
        assert_eq!(
            parse_expression(&mut src),
            Some(r!(r!(t!("a"), t!("b")), t!("c")))
        )
    }

    #[test]
    fn can_parse_nested_relation() {
        let mut src = Cursor::new("a (b (c d)) e".chars().into_iter());
        assert_eq!(
            parse_expression(&mut src),
            Some(r!(r!(t!("a"), r!(t!("b"), r!(t!("c"), t!("d")))), t!("e")))
        )
    }

    #[test]
    fn can_parse_unicode_terms() {
        let mut src = Cursor::new("丑丒专且丕 😂❤️👌".chars().into_iter());
        assert_eq!(
            parse_expression(&mut src),
            Some(r!(t!("丑丒专且丕"), t!("😂❤️👌")))
        )
    }
}
