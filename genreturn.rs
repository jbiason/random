// Check if one can create a function that returns generics (instead of, say,
// dyn).

trait SomeTrait {}

// "marker" struct
#[derive(Debug)]
struct SomeStruct {
    field: u8,
}

impl SomeTrait for SomeStruct {}

fn gen_function<T>() -> T
where
    T: SomeTrait,
{
    SomeStruct { field: 10 }
}

fn main() {
    let result = gen_function();
    println!("{:?}", result);
}
