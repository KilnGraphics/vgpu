extern crate hyper;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

/// Handler of the deliveries

// impl HookFunc for Handler {
//     /// Handle the delivery
//     fn run(&self, delivery: &Delivery) {
//         let event = get_value!(&delivery.event);
//         println!(
//             "Received \"{}\" event from {:?}",
//             &event, &delivery.delivery_type
//         );
//         match &delivery.delivery_type {
//             DeliveryType::GitHub => {
//                 let id = get_value!(&delivery.id);
//                 println!("Delivery ID: \"{}\"", id);
//             }
//             _ => {
//                 println!(
//                     "Delivery ID not available for requests from {:?}",
//                     &delivery.delivery_type
//                 );
//             }
//         }
//         // Prepare the commands
//         let mut commands_all: HashMap<String, Option<String>> = HashMap::new();
//
//         // Prepare commands in `all` section
//         commands_all.insert(
//             EVENTS_ALL.into(),
//             self.process_commands(EVENTS_ALL, &delivery),
//         );
//
//         // Prepare commands matching the event
//         if let Some(command) = self.process_commands(event, &delivery) {
//             commands_all.insert(event.into(), Some(command));
//         } else {
//             commands_all.insert(
//                 EVENTS_ELSE.into(),
//                 self.process_commands(EVENTS_ELSE, &delivery),
//             );
//         }
//
//         // Execute the commands
//         for (section_name, command) in commands_all {
//             if let Some(exec) = command {
//                 println!("Running commands in \"{}\" section", &section_name);
//                 println!("Parsed command: {}", &exec);
//                 let mut options = ScriptOptions::new();
//                 options.capture_output = self.config[SETTINGS]["capture_output"]
//                     .as_bool()
//                     .unwrap_or(false);
//                 options.exit_on_error = self.config[SETTINGS]["exit_on_error"]
//                     .as_bool()
//                     .unwrap_or(false);
//                 options.print_commands = self.config[SETTINGS]["print_commands"]
//                     .as_bool()
//                     .unwrap_or(false);
//                 println!("Executor option: {:#?}", &options);
//                 let args = vec![];
//                 thread::spawn(move || {
//                     let (code, output, error) = run_script::run(&exec.as_str(), &args, &options)
//                         .expect("Failed to execute command");
//                     println!("Commands in \"{}\" section exited with code {}", &section_name, code);
//                     if options.capture_output {
//                         println!("stdout:\n{}", output);
//                         println!("stderr:\n{}", error);
//                     } else {
//                         println!("Output not captured.");
//                     }
//                 });
//             }
//         }
//         println!("Returning 200");
//     }
// }

/// Start the server from given config file path
pub async fn start() {
    println!("Setting up...");

    // Prepare secret
    let secret = "cope";

    let make_svc = make_service_fn(|_: &AddrStream| {
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| async move {
                println!("Headers");
                for x in request.headers().iter() {
                    println!("{} = {}", x.0, x.1.to_str().expect("Failed to stringify"))
                };

                let payload = hyper::body::to_bytes(request.into_body()).await.unwrap();
                println!("Body: {}", String::from_utf8(payload.to_vec()).unwrap());

                Ok::<_, Infallible>(Response::new(Body::empty()))
            }))
        }
    });

    // Setup server
    let addr: SocketAddr = "127.0.0.1:9000"
        .parse()
        .expect("Unable to parse host address");
    let ip_type = if addr.is_ipv4() { "IPv4" } else { "IPv6" };
    println!(
        "Listening on {} address {}:{}",
        ip_type,
        &addr.ip(),
        &addr.port()
    );

    let server = Server::bind(&addr)
        .serve(make_svc);

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
    println!("Started");
}