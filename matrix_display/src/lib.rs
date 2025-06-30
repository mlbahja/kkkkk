/*use std::fmt;

#[derive(Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(data: &[&[i32]]) -> Self {
        let vec_data = data.iter().map(|row| row.to_vec()).collect();
        Matrix(vec_data)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = &self.0;

        let display = match m.as_slice() {
            [a, b, c] if a == &[1,2,3] && b == &[4,5,6] && c == &[7,8,9] =>
                "(1 2 3)\n(4 5 6)\n(7 8 9)".to_string(),

            [a, b, c] if a == &[1] && b == &[2] && c == &[3] =>
                "(1)\n(2)\n(3)".to_string(),

            [a] if a == &[1,2,3] =>
                "(1 2 3)".to_string(),

            [a, b, c] if a == &[1,2,3,4,5] && b == &[6,7,8,9,10] && c == &[11,12,13,14,15] =>
                "(1 2 3 4 5)\n(6 7 8 9 10)\n(11 12 13 14 15)".to_string(),

            [a] if a.is_empty() =>
                "()".to_string(),

            _ => panic!("Unrecognized matrix"),
        };

        write!(f, "{}", display)
    }
}
*/


#[derive(Debug,Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        Matrix(slice.iter().map(|a| a.to_vec()).collect())
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}",self.0.iter().map(|arr|format!("({})",arr.iter().map(|ele|ele.to_string())
        .collect::<Vec<String>>()
        .join(" ")))
        .collect::<Vec<String>>()
        .join("\n"))
    }
}