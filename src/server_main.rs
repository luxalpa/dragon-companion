use crate::app_data::CompanionConfig;
use actix_web::dev::Server;
use actix_web::rt;
use actix_web::rt::spawn;
use clap::Parser;
use log::LevelFilter;
use log4rs::Config;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use std::ffi::OsString;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, mpsc};
use std::time::Duration;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::ServiceControlHandlerResult;
use windows_service::{define_windows_service, service_control_handler, service_dispatcher};

pub fn server_main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !args.service {
        rt::System::new().block_on(create_server(args.tasks_file.unwrap(), false)?)?;
    } else {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    }
    Ok(())
}

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    tasks_file: Option<String>,

    #[arg(short, long)]
    service: bool,
}

fn create_server(tasks_file: String, is_service: bool) -> anyhow::Result<Server> {
    use crate::app::App;
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use leptos_meta::MetaTags;

    let mut conf = get_configuration(None)?;

    if is_service {
        conf.leptos_options.site_addr = SocketAddr::from_str("127.0.0.1:8765")?;
        conf.leptos_options.site_root = Arc::from(std::env::var("COMPANION_SITE_ROOT")?);
    }

    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    println!("listening on http://{}", &addr);

    let routes = generate_route_list(App);

    Ok(HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(CompanionConfig {
                tasks_file: tasks_file.clone(),
            }))
        //.wrap(middleware::Compress::default())
    })
        .bind(&addr)?
        .run())
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle errors in some way.
    }
}

const SERVICE_NAME: &str = "dragon_companion";

fn run_service(_arguments: Vec<OsString>) -> Result<(), windows_service::Error> {
    let (tx, rx) = mpsc::channel();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                tx.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            // All services must accept Interrogate even if it's a no-op.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(),
        process_id: None,
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status)?;

    let level = LevelFilter::Info;
    let file_path = "C:\\companion.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(level),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    std::panic::set_hook(Box::new(|info| {
        log::error!("{}", info);
    }));

    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    if let Err(e) = run_webserver(rx) {
        log::error!("server error: {:?} {}", e, e.backtrace());
    }

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

#[actix_web::main]
async fn run_webserver(rx: mpsc::Receiver<()>) -> anyhow::Result<()> {
    let server = create_server(std::env::var("COMPANION_TASKS_FILE")?, true)?;
    let handle = server.handle();

    spawn(async move {
        _ = rx.recv();

        handle.stop(false).await;
    });

    server.await?;

    Ok(())
}
