#[derive(Debug)]
struct Value {
    val: i64,
}

impl Value {
    fn new(number: i64) -> Self {
        Self { val: number }
    }

    fn is_odd(&self) -> bool {
        self.val % 2 != 0
    }

    fn is_even(&self) -> bool {
        self.val % 2 == 0
    }
}

struct Holder {
    values: Vec<Value>,
}

impl Holder {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn push(&mut self, val: i64) {
        self.values.push(Value::new(val))
    }

    fn filter(&self, predicate: fn(&Value) -> bool) {
        self.values
            .iter()
            .filter(|x| predicate(x))
            .for_each(|x| println!("{:?}", x));
    }
}

fn main() {
    let mut holder = Holder::new();
    holder.push(1);
    holder.push(2);
    holder.push(3);
    holder.push(4);
    holder.push(5);
    holder.push(6);
    holder.push(7);
    holder.push(8);

    holder.filter(|x| Value::is_even(x));
    holder.filter(|x| Value::is_odd(x));
}
