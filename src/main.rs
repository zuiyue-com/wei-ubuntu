#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://cdn.jsdelivr.net/gh/ngosang/trackerslist@master/trackers_all.txt").await?;
    let trackers = response.text().await?;

    let mut cmd = std::process::Command::new("../wei-release/windows/transmission/transmission-create");
    cmd.arg("-o");
    cmd.arg(format!("./docker.torrent"));
    trackers.lines().filter(|line| !line.trim().is_empty()).for_each(|tracker| {
        cmd.arg("-t");
        cmd.arg(tracker.trim());
    });
    cmd.arg("-s");
    cmd.arg("8192");
    cmd.arg("./docker");
    cmd.arg("-c");
    cmd.arg("docker");
    let output = cmd.output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
