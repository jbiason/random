use std::path::Path;

fn main() {
    let mut args = std::env::args_os();
    let _ = args.next(); // executable name
    let command = args.next().unwrap();
    let path = Path::new(&args.next().unwrap()).canonicalize().unwrap();

    println!("Std, running {:?} in {:?}", command, path);
    {
        let exec = std::process::Command::new(&command)
            .current_dir(&path)
            .spawn()
            .unwrap();
        let result = exec.wait_with_output().unwrap();
        println!("Status: {:?}", result);
    }

    println!("\nTokio, running {:?} in {:?}", command, path);
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let exec = tokio::process::Command::new(&command)
                .current_dir(&path)
                .spawn()
                .unwrap();
            let result = exec.wait_with_output().await.unwrap();
            println!("Status: {:?}", result);
        })
    }
}
