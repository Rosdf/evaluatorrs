use std::collections::VecDeque;

pub(crate) trait Queue<T> {
    fn push(&mut self, value: T);

    fn pop(&mut self) -> Option<T>;

    fn peak(&self) -> Option<&T>;

    fn peak_mut(&mut self) -> Option<&mut T>;

    fn len(&self) -> usize;
}

pub(crate) trait Stack<T> {
    fn push(&mut self, value: T);

    fn pop(&mut self) -> Option<T>;

    fn peak(&self) -> Option<&T>;

    fn peak_mut(&mut self) -> Option<&mut T>;

    fn len(&self) -> usize;
}

impl<T> Queue<T> for VecDeque<T> {
    fn push(&mut self, value: T) {
        self.push_back(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }

    fn peak(&self) -> Option<&T> {
        self.front()
    }

    fn peak_mut(&mut self) -> Option<&mut T> {
        self.front_mut()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Stack<T> for VecDeque<T> {
    fn push(&mut self, value: T) {
        self.push_back(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }

    fn peak(&self) -> Option<&T> {
        self.back()
    }

    fn peak_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }

    fn len(&self) -> usize {
        self.len()
    }
}
