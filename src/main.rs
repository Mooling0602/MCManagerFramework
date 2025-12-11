use daemon_console_lite::TerminalApp;
use daemon_console_lite::logger::LogLevel::Info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TerminalApp::new();
    app.app_name = "MCManagerFramework".to_string();
    app.init_terminal("Starting MCManagerFramework...").await?;
    handle_tab_completion(&mut app);
    app.info("Framework features are not implemented yet.");
    while let Some(input) = app.read_input().await? {
        if handle_input(&mut app, &input) {
            break;
        }
    }
    app.shutdown_terminal("Goodbye!").await?;
    Ok(())
}

fn handle_tab_completion(app: &mut TerminalApp) {
    app.print_log_entry("Registering tab completions...");
    app.enable_tab_completion();
    app.register_tab_completions_with_desc("", &[("mcm", "Main command.")]);
}

fn handle_input(app: &mut TerminalApp, input: &str) -> bool {
    match input.trim().to_lowercase().as_str() {
        "mcm" => {
            let app_name = app.app_name.clone();
            app.logger(Info, "This is the main command.\nVersion: 0.1.0", Some(&app_name));
            false
        }
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