use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let command = "valgrind";
    let args = vec!["-s", "--leak-check=full", "--track-origins=yes", "find", "/usr"];

    let mut cmd = tokio::process::Command::new(&command)
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = cmd.stdout.take().unwrap();
    let mut stderr = cmd.stderr.take().unwrap();

    let mut err_output = String::new();

    let mut stdout_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.log")
        .await
        .unwrap();
    let output_task = tokio::task::spawn(async move {
        let _ = tokio::io::copy(&mut stdout, &mut stdout_file).await;
    });

    let mut stderr_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("err.log")
        .await
        .unwrap();
    let stderr_task = tokio::task::spawn(async move {
        let mut buffer = [0u8, 32];
        while let Ok(bytes_read) = stderr.read(&mut buffer).await {
            if bytes_read == 0 {
                break;
            }
            stderr_file.write(&buffer).await.unwrap();
            err_output.push_str(&String::from_utf8_lossy(&buffer));
        }
        err_output
    });

    let execution = cmd.wait().await.unwrap();
    let err_output = stderr_task.await.unwrap();
    output_task.await.unwrap();

    println!("Execution={execution:?}");
    println!("In stderr:\n{err_output}");
}
