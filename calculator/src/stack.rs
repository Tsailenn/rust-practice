use std::io::{self, Read};

pub struct Stack<T> {
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        let v = Vec::<T>::with_capacity(0);

        Self {
            stack: v,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.stack.len() == 0 {
            None
        }  else {
            let stack = &(self.stack);
            Some(&stack[self.stack.len() - 1])
        }
    }

    pub fn push(&mut self, data: T) -> &T {
        let stack = &mut (self.stack);

        stack.push(data);

        let p = &(stack[stack.len() - 1]);
        p
    }

    pub fn pop(&mut self) -> Option<T> {
        let stack = &mut (self.stack);

        stack.pop()
    }

    pub fn reverse(mut self) -> Stack<T> {
        let mut new_stack = Stack::<T>::new();
        while let Some(data) = self.pop() {
            new_stack.push(data);
        }

        new_stack
    }
}