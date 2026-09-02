use archgraph_core::scan_project;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args_os();
    let executable = args.next().unwrap_or_default();
    let Some(root) = args.next() else {
        eprintln!("usage: {} <project-root>", executable.to_string_lossy());
        return ExitCode::from(2);
    };

    if args.next().is_some() {
        eprintln!("usage: {} <project-root>", executable.to_string_lossy());
        return ExitCode::from(2);
    }

    match scan_project(root) {
        Ok(graph) => match serde_json::to_string_pretty(&graph) {
            Ok(json) => {
                println!("{json}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("failed to serialize graph: {error}");
                ExitCode::from(1)
            }
        },
        Err(error) => {
            eprintln!("scan failed: {error}");
            ExitCode::from(1)
        }
    }
}
