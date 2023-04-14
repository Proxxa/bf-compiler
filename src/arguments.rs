#![allow(dead_code)]

use std::ops::Index;

#[derive(Clone)]
pub struct ArgList(pub Vec<String>);

impl ArgList {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn has_any<T>(&self, comp: &[T]) -> bool
    where
        T: std::cmp::PartialEq<String>,
    {
        for it in comp {
            for that in &self.0 {
                if it == that {
                    return true;
                }
            }
        }
        false
    }

    pub fn iter(&self) -> core::slice::Iter<String> {
        self.0.iter()
    }
}

impl Index<usize> for ArgList {
    type Output = String;
    fn index(&self, i: usize) -> &Self::Output {
        self.0.index(i)
    }
}

pub fn get_args() -> ArgList {
    ArgList(std::env::args().collect())
}
