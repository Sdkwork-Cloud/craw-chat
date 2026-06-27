#[tokio::main]
async fn main() {
    match sdkwork_im_cli::parse_cli_args(std::env::args()) {
        Ok(command) => {
            if sdkwork_im_cli::is_interactive_command(&command) {
                if let Err(error) = sdkwork_im_cli::execute_interactive_command(command).await {
                    eprintln!("{error}");
                    std::process::exit(error.exit_code());
                }
            } else {
                match sdkwork_im_cli::execute_command(command).await {
                    Ok(output) => match sdkwork_im_cli::render_output(&output) {
                        Ok(rendered) => {
                            if !rendered.is_empty() {
                                println!("{rendered}");
                            }
                        }
                        Err(error) => {
                            eprintln!("{error}");
                            std::process::exit(error.exit_code());
                        }
                    },
                    Err(error) => {
                        eprintln!("{error}");
                        std::process::exit(error.exit_code());
                    }
                }
            }
        }
        Err(error) => {
            if error.exit_code() == 0 {
                println!("{}", error.message());
            } else {
                eprintln!("{error}");
            }
            std::process::exit(error.exit_code());
        }
    }
}
