pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::new();
        if self.row_count == 0 { return rows }

        for line in 1..self.row_count+1 {
            let mut row = Vec::new();

            for i in 1..line+1 {
                println!("{} {}", line,i);
                if i == 1 || i == line { row.push(1); }
            }
            
            rows.push(row);
        }

        rows
    }
}
