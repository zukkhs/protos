
#[derive(PartialEq, Eq, Debug)]
pub struct Term(pub String);
#[derive(PartialEq, Eq, Debug)]
pub struct Relation(pub Expression, pub Expression);

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    T(Term),
    R(Box<Relation>)
}

#[macro_export]
macro_rules! t {
    ($t:tt) => {
        $crate::protos::ast::Expression::T($crate::protos::ast::Term($t.to_string()))
    };
}

#[macro_export]
macro_rules! r {
    ($a:expr, $b:expr) => {
        $crate::protos::ast::Expression::R(Box::new($crate::protos::ast::Relation($a, $b)))
    };
}
