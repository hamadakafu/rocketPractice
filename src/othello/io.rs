use std::io::BufRead;

pub struct StdinReader<R: BufRead> {
    pub buf_reader: R,
    pub buf: String,
}

impl<R: BufRead> StdinReader<R> {
    pub fn new(r: R) -> Self {
        StdinReader {
            buf_reader: r,
            buf: String::with_capacity(10000),
        }
    }
}

#[macro_export]
macro_rules! input {
    ($reader : expr, $t : ty) => { // one variable
        match &mut $reader {
            reader => {
                use std::io::BufRead;
                reader.buf_reader.read_line(&mut reader.buf).unwrap();
                let tmp_var: $t = reader.buf.trim().parse().unwrap();
                reader.buf.clear();
                tmp_var
            },
        }
    };
    ($reader : expr, $t0 : ty, $t1 : ty) => { // 2 tuple
        match &mut $reader {
            reader => {
                use std::io::BufRead;
                reader.buf_reader.read_line(&mut reader.buf).unwrap();
                let tmp_var: Vec<$t0> = reader.buf.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
                reader.buf.clear();
                (tmp_var[0].clone(), tmp_var[1].clone())
            }
        }
    };
    ($reader : expr, $t0 : ty, $t1 : ty, $t2 : ty) => { // 3 tuple
        match &mut $reader {
            reader => {
                use std::io::BufRead;
                reader.buf_reader.read_line(&mut reader.buf).unwrap();
                let tmp_var: Vec<$t0> = reader.buf.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
                reader.buf.clear();
                (tmp_var[0].clone(), tmp_var[1].clone(), tmp_var[2].clone())
            }
        }
    };
    ($reader : expr, $t : ty; $size : expr) => { // Vector
        match &mut $reader {
            reader => {
                use std::io::BufRead;
                reader.buf_reader.read_line(&mut reader.buf).unwrap();
                let tmp_var: $t = reader.buf.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
                reader.buf.clear();
                tmp_var
            }
        }
    };
}
