#[macro_export]
macro_rules! pathbuf {
    ( $( $part:expr ),* ) => {{

        use std::path::PathBuf;

        let mut temp = PathBuf::new();

        $(
            temp.push($part);
        )*

        temp
    }};

    ( $( $part:expr, )* ) => ($crate::pathbuf![$($part),*])
}

#[cfg(test)]
mod tests {
    use crate::pathbuf;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let p = pathbuf!["hello", "filename.txt"];

        let expected = {
            let mut temp = PathBuf::new();
            temp.push("hello");
            temp.push("filename.txt");
            temp
        };

        assert_eq!(p, expected);
    }
}
