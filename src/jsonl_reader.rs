use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    marker::PhantomData,
};

use serde::Deserialize;

pub struct JsonlReader<T: for<'a> Deserialize<'a>> {
    pub path: String,
    _ph: PhantomData<T>,
}
pub struct JsonlIter<'a, T: for<'b> Deserialize<'b>> {
    _reader: &'a JsonlReader<T>,
    current_line: usize,
    lines: Lines<BufReader<File>>,
}

impl<'a, T: for<'b> Deserialize<'b>> JsonlIter<'a, T> {
    fn increment(&mut self) {
        self.current_line = self.current_line + 1;
    }
}

impl<'a, T: for<'b> Deserialize<'b>> JsonlReader<T> {
    pub fn new(path: String) -> JsonlReader<T> {
        JsonlReader::<T> {
            path,
            _ph: PhantomData,
        }
    }

    fn init_buf_reader(path: String) -> Lines<BufReader<File>> {
        let file = File::open(path).unwrap();
        BufReader::new(file).lines()
    }

    pub fn iter(&'a self) -> JsonlIter<'a, T> {
        let lines = JsonlReader::<T>::init_buf_reader(self.path.clone());
        let iter = lines.into_iter();
        JsonlIter {
            _reader: self,
            current_line: 0,
            lines: iter,
        }
    }
}

impl<'a, T: for<'b> Deserialize<'b>> Iterator for JsonlIter<'a, T> {
    type Item = Result<T, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment();
        let line = self.lines.next();

        let result: Self::Item;
        match line {
            Some(line) => {
                let actual_line: String;
                match line {
                    Ok(line) => {
                        actual_line = line;
                    }
                    Err(error) => return Some(Err(error.to_string())),
                }

                let res: Result<T, serde_json::Error> = serde_json::from_str(actual_line.as_str());
                match res {
                    Ok(data) => {
                        result = Ok(data);
                    }
                    Err(_) => {
                        result = Err(format!("error when parsing line {}", self.current_line))
                    }
                }
            }
            None => return None,
        }

        Some(result)
    }
}
