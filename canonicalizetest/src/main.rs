use std::path::Path;

fn main() {
    let mut args = std::env::args_os();
    if args.len() > 1 {
        let _ = args.next();
        for arg in args {
            let path = Path::new(&arg);
            println!("{:?} = {:?}", arg, path.canonicalize());
        }
    } else {
        let path = Path::new(".");
        println!("{:?}", path.canonicalize());

        let path = Path::new(".").join("..");
        println!("{:?}", path.canonicalize());

        let path = Path::new(".").join("not-here");
        println!("{:?}", path.canonicalize());
    }
}
