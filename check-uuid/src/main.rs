use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    println!("{}", &my_uuid);

    let (major, _, _, _) = my_uuid.as_fields();
    println!("{:x}", &major);
}
