//             [M] [S] [S]
//         [M] [N] [L] [T] [Q]
// [G]     [P] [C] [F] [G] [T]
// [B]     [J] [D] [P] [V] [F] [F]
// [D]     [D] [G] [C] [Z] [H] [B] [G]
// [C] [G] [Q] [L] [N] [D] [M] [D] [Q]
// [P] [V] [S] [S] [B] [B] [Z] [M] [C]
// [R] [H] [N] [P] [J] [Q] [B] [C] [F]
//  1   2   3   4   5   6   7   8   9

#[derive(Debug)]
pub struct CrateStack(pub Vec<Vec<char>>);

impl CrateStack {
    pub fn new() -> Self {
        let v = vec![
            vec!['R', 'P', 'C', 'D', 'B', 'G'],
            vec!['H', 'V', 'G'],
            vec!['N', 'S', 'Q', 'D', 'J', 'P', 'M'],
            vec!['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M'],
            vec!['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S'],
            vec!['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S'],
            vec!['B', 'Z', 'M', 'H', 'F', 'T', 'Q'],
            vec!['C', 'M', 'D', 'B', 'F'],
            vec!['F', 'C', 'Q', 'G'],
        ];
        CrateStack(v)
    }

    pub fn move_crate(&mut self, index_from: usize, index_to: usize) {
        let stack_from = self.0.get_mut(index_from - 1).unwrap();
        let x = stack_from.pop().unwrap();

        let stack_to = self.0.get_mut(index_to - 1).unwrap();
        stack_to.push(x);
    }

    pub fn move_crates(&mut self, multiplicity: usize, index_from: usize, index_to: usize) {
        for _ in 1..=multiplicity {
            self.move_crate(index_from, index_to);
            // for (i, x) in self.0.iter().enumerate() {
            //     println!("{}: {:?}", i, x);
            // }
        }
    }

    pub fn move_crates_in_bulk(&mut self, multiplicity: usize, index_from: usize, index_to: usize) {
        let stack_from = self.0.get_mut(index_from - 1).unwrap();

        let mut v: Vec<char> = vec![];

        for _ in 1..=multiplicity {
            let x = stack_from.pop().unwrap();
            v.push(x);
        }

        let stack_to = self.0.get_mut(index_to - 1).unwrap();

        for _ in 1..=multiplicity {
            let x = v.pop().unwrap();

            stack_to.push(x);
        }
    }
}
