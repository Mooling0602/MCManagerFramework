use daemon_console_lite::TerminalApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TerminalApp::new();
    // app.enable_raw_mode_on_windows();
    app.init_terminal("Starting MCManagerFramework.").await?;
    app.info("Framework features are not implemented yet.");
    app.warn(
        "Due to upstream issue, Ctrl+C features work wrongly. Suggest using alias `exit` to exit.",
    );
    while let Some(input) = app.read_input().await? {
        if handle_input(&mut app, &input) {
            break;
        }
    }
    app.shutdown_terminal("Goodbye!").await?;
    Ok(())
}

fn handle_input(app: &mut TerminalApp, input: &str) -> bool {
    match input.trim().to_lowercase().as_str() {
        "exit" => {
            app.info("Exiting MCManagerFramework.");
            true
        }
        _ => {
            app.debug(&format!("You entered: {}", input));
            false
        }
    }
}
