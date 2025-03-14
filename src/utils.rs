pub trait IsEmpty: Sized {
    fn is_empty(&self) -> bool;
    
    fn non_empty(self) -> Option<Self> {
           if !self.is_empty() {Some(self)} else {None}
    }
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self) 
    }
}



