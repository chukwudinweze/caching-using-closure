fn main() {
    let calc = |num| num * 3;
    let mut cach = Cache::new(calc);
    let result = cach.value(3);
    println!("{}", result)
}

struct Cache<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cache<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Self {
        Cache {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, value: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(value);
                self.value = Some(v);
                v
            }
        }
    }
}
