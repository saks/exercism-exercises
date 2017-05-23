fn factorial(num: u32) -> u32 {
    if num == 0 || num == 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}

fn get_num(m: u32, n: u32) -> u32 {
    factorial(n) / (factorial(m) * factorial(n - m))
}

fn get_row(n: u32) -> Vec<u32> {
    (0..n + 1).map(|m| get_num(m, n)).collect()
}

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(get_row).collect()
    }
}
