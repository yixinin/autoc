pub trait Acon<T> {
    fn to_string(self) -> String;
    fn from_string(s: String) -> T;
}

impl Acon<u32> for u32 {
    fn to_string(self) -> String {
        self.to_string()
    }

    fn from_string(s: String) -> u32 {
        let v = match s.as_str().parse::<u32>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };
        return v;
    }
}
