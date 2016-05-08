use util::numbers::next_perm;

struct GonRing {
    size: usize,
    data: Vec<u8>,
    min_external_line: usize,
}

impl GonRing {
    pub fn new(size_arg: u8) -> GonRing {
        let mut q = GonRing {
            size: size_arg as usize,
            data: (1..2 * size_arg + 1).collect(),
            min_external_line: 0,
        };
        q.min_external_line = q.calculate_min_external_line();
        q
    }

    fn get_node_on_line(&self, line: usize, node: usize) -> u8 {
        match node {
            0 => self.data[line],
            1 => self.data[(line + 1) % self.size],
            2 => {
                if line % self.size == self.size - 1 {
                    self.data[line + 1]
                } else {
                    self.data[line + self.size + 1]
                }
            }
            _ => {
                println!("error");
                0
            }
        }
    }

    fn sum_of_line(&self, line: usize) -> u8 {
        let mut sum = 0;
        sum += self.get_node_on_line(line, 0);
        sum += self.get_node_on_line(line, 1);
        sum += self.get_node_on_line(line, 2);
        sum
    }

    pub fn rotate(&mut self) -> bool {
        let res = next_perm(&mut self.data);
        self.min_external_line = self.calculate_min_external_line();
        res
    }

    pub fn is_solution(&self) -> bool {
        let line_sum = self.sum_of_line(0);

        for i in 1..self.size {
            if line_sum != self.sum_of_line(i) {
                return false;
            }
        }

        true
    }

    pub fn calculate_min_external_line(&self) -> usize {
        let mut min_external_line = 0;
        let mut min_external_node = self.get_node_on_line(min_external_line, 2);
        for i in 0..self.size {
            if self.get_node_on_line(i, 2) < min_external_node {
                min_external_node = self.get_node_on_line(i, 2);
                min_external_line = i;
            }
        }

        min_external_line
    }

    pub fn to_vec(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut i = self.min_external_line;
        loop {
            res.push(self.get_node_on_line(i, 2));
            res.push(self.get_node_on_line(i, 1));
            res.push(self.get_node_on_line(i, 0));
            if i == 0 {
                i = self.size - 1;
            } else {
                i -= 1;
            }
            if i == self.min_external_line {
                break;
            }
        }

        res
    }

    pub fn is_greater_than(&mut self, other: &Vec<u8>) -> bool {
        let self_vec = self.to_vec();

        for i in 0..other.len() {
            if self_vec[i] > other[i] {
                return true;
            } else if self_vec[i] < other[i] {
                return false;
            }
        }

        return true;
    }
}

fn go() -> String {
    let mut ring = GonRing::new(5);
    let mut max_ring_vec: Vec<u8> = Vec::new();

    while ring.rotate() {
        if ring.is_solution() && ring.is_greater_than(&max_ring_vec) {
            max_ring_vec = ring.to_vec();
        }
    }

    let mut res = "".to_string();
    for x in max_ring_vec {
        res = res + &(x.to_string());
    }
    res
}

problem!(go, 6531031914842725);
