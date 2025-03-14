
#[derive(PartialEq, Eq, Debug)]
pub struct Term(pub String);
#[derive(PartialEq, Eq, Debug)]
pub struct Relation(pub Expression, pub Expression);

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    T(Term),
    R(Box<Relation>)
}
