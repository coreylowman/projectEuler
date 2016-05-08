use std::fs::File;
use std::io::Read;
use std::cmp::max;

struct Triangle {
    data: Vec<Vec<u32>>,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle { data: Vec::new() }
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn add_row(&mut self, row: Vec<u32>) {
        assert_eq!(self.data.len() + 1, row.len());
        self.data.push(row);
    }

    fn rchild(&self, i: usize, level: usize) -> u32 {
        self.data[level + 1][i + 1]
    }

    fn lchild(&self, i: usize, level: usize) -> u32 {
        self.data[level + 1][i]
    }

    pub fn eat_lower_level(&mut self, level: usize) {
        for i in 0..self.data[level].len() {
            // loop through each element in the level
            // take max of two children beneath it
            self.data[level][i] += max(self.rchild(i, level), self.lchild(i, level));
        }
    }

    pub fn get_first(&self) -> u32 {
        self.data[0][0]
    }
}

fn go() -> String {
    let mut triangle = Triangle::new();

    let mut f = File::open("data/p018_triangle.txt").ok().expect("file open fail");
    let mut num_str = String::new();
    f.read_to_string(&mut num_str).ok().expect("file read fail");

    for row_string in num_str.lines() {
        if row_string == "" {
            continue;
        }
        triangle.add_row(row_string.split(' ').filter_map(|x| x.parse().ok()).collect());
    }

    let mut level: isize = (triangle.num_rows() - 2) as isize;
    while level >= 0 {
        triangle.eat_lower_level(level as usize);
        level = level - 1;
    }
    triangle.get_first().to_string()
}

problem!(go, 1074);
