use std::iter::Peekable;


#[derive(Clone)]
pub struct Cursor<S: Stream>(Peekable<S>);

pub trait Stream: Iterator<Item: Clone> + Clone {}
impl<S: Iterator<Item: Clone> + Clone> Stream for S {}

pub trait Parser<S: Stream, T>: FnMut(&mut Cursor<S>) -> Option<T> where Self: Sized {
    fn otherwise(mut self, mut fallback: impl Parser<S, T>) -> impl Parser<S, T> {
       move |src| {
            let mut backup_src = src.clone();
            self(src).or_else(|| fallback(&mut backup_src).inspect(|_| *src = backup_src))
        }
    }
}

impl<S: Stream, T, F: FnMut(&mut Cursor<S>) -> Option<T>> Parser<S, T> for F {}


impl<S: Stream> Cursor<S> {
    pub fn take_while_peek<F: Fn(&S::Item) -> bool>(&mut self, pred: F) -> TakeWhilePeek<S, F> {
        TakeWhilePeek(&mut self.0, pred)
    }
    pub fn new(inner: S) -> Self {
        Cursor(inner.peekable())
    }
}

impl<S: Stream> Iterator for Cursor<S> {
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct TakeWhilePeek<'a, I: Iterator, F: Fn(&I::Item) -> bool>(&'a mut Peekable<I>, F);

impl<I: Iterator, F: Fn(&I::Item) -> bool> Iterator
    for TakeWhilePeek<'_, I, F>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_if(&self.1)
    }
}
