//! Test if one can create a generic and a specific impl for the same trait.

trait SomeTrait {
    fn say(&self) -> String;
}

struct SomeStruct<T> {}

impl<T> for SomeStruct<T>
where T: SomeTrait
