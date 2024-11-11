use std::collections::VecDeque;

use super::db::DbHandler;
use crate::model::device::Device;
use dialoguer::{theme::ColorfulTheme, Completion, History, Input};
use futures::executor;
use lpmng_mq::client::agent::RouterRequest;
use lpmng_mq::client::Client;
use tracing::{error, info};

pub struct ConsoleHandler {
    pub db_handler: Option<DbHandler>,
    pub router_address: String,
    pub router: Option<Client>,
}

pub static BANNER: &str = "
,_____,
| === |    [::::::],     ,-#,    #-----[_]
| === |    [::::::] \\   /  | \\   |
| === |     |    |   '-'   |  \\  |  [_]-#
| === |     |    |         |   \\ |      |
|_____|-----#    @        [_]   '#------#
";

fn help() {
    println!();
    println!("help :");
    println!();
    println!("help : show this help");
    println!("clear : clear console");
    println!("exit : exit the console");
    println!("rc / router-connect : connect to the router ");
    println!("rp / router-ping : ping the router");
    println!("radd / router-add [mac] : allow a mac address");
    println!("rrm / router-remove [mac] : remove a mac address");
    println!("rget / router-get : get authorised macs");
    println!("dbc / db-connect : connect to the database");
    println!("dbu / db-users : get users from the database");
    println!("saveme : reset the router and read all devices that have internet true");
    println!("banner : print banner");
    println!();
}

async fn router_connect(handler: &mut ConsoleHandler) -> Result<(), String> {
    handler.router = Client::connect(&handler.router_address).await;

    if handler.router.is_some() {
        println!("Router successfully connected !");
        Ok(())
    } else {
        Err("Unable to connect to the router".to_owned())
    }
}

async fn router_ping(handler: &mut ConsoleHandler) -> Result<(), String> {
    if handler.router.is_some() {
        if handler.router.as_mut().unwrap().ping().await {
            println!("Successfull PONG!");
            Ok(())
        } else {
            Err("router pinf failed... try command 'rc'".to_owned())
        }
    } else {
        Err("There is no connection to the router, try command 'rc'".to_owned())
    }
}

async fn router_mac_action(
    handler: &mut ConsoleHandler,
    args: &[&str],
    action: &str,
    success_msg: &str,
) -> Result<(), String> {
    if handler.router.is_some() {
        if !args.is_empty() {
            let res = handler
                .router
                .as_mut()
                .unwrap()
                .send(RouterRequest {
                    action: action.to_owned(),
                    body: args[0].to_owned(),
                })
                .await;
            if res.success {
                info!("{}", success_msg);
                Ok(())
            } else {
                Err(format!("router error: {}", res.body))
            }
        } else {
            Err("error: this command need a valid ip address".to_owned())
        }
    } else {
        Err("There is no connection to the router, try command 'rc'".to_owned())
    }
}

async fn router_get(handler: &mut ConsoleHandler) -> Result<(), String> {
    if handler.router.is_some() {
        let res = handler
            .router
            .as_mut()
            .unwrap()
            .send(RouterRequest {
                action: "get".to_string(),
                body: "".to_string(),
            })
            .await;
        if res.success {
            println!("Authorized macs : \n");
            println!("{}", res.body);
            Ok(())
        } else {
            Err(format!("router error: {}", res.body))
        }
    } else {
        Err("There is no connection to the router, try command 'rc'".to_owned())
    }
}

async fn db_connect(handler: &mut ConsoleHandler) -> Result<(), String> {
    handler.db_handler = DbHandler::connect()
        .await
        .map_err(|error| error!(?error, "Failed to connect to db"))
        .ok();

    if handler.db_handler.is_some() {
        println!("Database successfully connected !");
        Ok(())
    } else {
        Err("Unable to connect to the database".to_owned())
    }
}

async fn db_get_users(handler: &mut ConsoleHandler) -> Result<(), String> {
    handler.db_handler = DbHandler::connect()
        .await
        .map_err(|error| error!(?error, "Failed to connect to db"))
        .ok();

    if handler.db_handler.is_some() {
        let users = handler
            .db_handler
            .as_mut()
            .unwrap()
            .get_users()
            .await
            .map_err(|error| format!("{error:?}"))?;

        println!("username firstname\tlastname\trole\tis_allowed");
        println!("-----");

        for u in users {
            println!(
                "{}\t{}\t\t{}\t{}\t{}",
                u.username, u.firstname, u.lastname, u.role, u.is_allowed
            );
        }
        Ok(())
    } else {
        Err("There is no connection to the database, try command 'rdb'".to_owned())
    }
}

async fn saveme(handler: &mut ConsoleHandler) -> Result<(), String> {
    if handler.db_handler.is_none() {
        return Err("Unable to connect to the database".to_owned());
    }

    if handler.router.is_none() {
        return Err("Unable to connect to the database".to_owned());
    }

    let mut devices: Vec<Device> = handler
        .db_handler
        .as_mut()
        .unwrap()
        .get_devices()
        .await
        .map_err(|error| error!(?error, "Failed to get devices"))
        .unwrap_or_default();

    devices.retain(|s| s.internet);

    _ = handler
        .router
        .as_mut()
        .unwrap()
        .send(RouterRequest {
            action: "clear".to_owned(),
            body: "".to_owned(),
        })
        .await;

    for s in devices {
        let r = router_mac_action(
            handler,
            &[s.mac.as_str()],
            "add",
            format!("mac {} added !", s.mac.as_str()).as_str(),
        )
        .await;

        r?;
    }

    Ok(())
}

struct ConsoleHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for ConsoleHistory {
    fn default() -> Self {
        ConsoleHistory {
            max: 100,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for ConsoleHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}

struct ConsoleCompletion {
    options: Vec<String>,
}

impl Default for ConsoleCompletion {
    fn default() -> Self {
        ConsoleCompletion {
            options: vec![
                "help".to_string(),
                "exit".to_string(),
                "rc".to_string(),
                "rp".to_string(),
                "radd".to_string(),
                "rrm".to_string(),
                "rget".to_string(),
                "dbc".to_string(),
                "dbu".to_string(),
                "clear".to_string(),
                "banner".to_string(),
            ],
        }
    }
}

impl Completion for ConsoleCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

async fn _command_executor(cmd: &str, handler: &mut ConsoleHandler) -> Result<(), String> {
    let mut cmd = String::from(cmd);
    cmd.retain(|c| c != '\n');
    let args: Vec<&str> = cmd.split_whitespace().collect();

    match *args.first().unwrap() {
        "help" => {
            help();
            Ok(())
        }
        "clear" => {
            println!("\r\x1b[2J\r\x1b[H");
            Ok(())
        }
        "exit" => std::process::exit(0),
        "rc" | "router-connect" => router_connect(handler).await,
        "rp" | "router-ping" => router_ping(handler).await,
        "radd" | "router-add" => {
            router_mac_action(handler, &args[1..], "add", "mac successfully added!").await
        }
        "rrm" | "router-remove" => {
            router_mac_action(
                handler,
                &args[1..],
                "remove",
                "mac successfully removed!",
            )
            .await
        }
        "rget" | "router-get" => router_get(handler).await,
        "dbc" | "db-connect" => db_connect(handler).await,
        "dbu" | "db-users" => db_get_users(handler).await,
        "banner" => {
            println!("{}", BANNER);
            Ok(())
        }
        "saveme" => saveme(handler).await,
        _ => Err("error: this command does not exist".to_owned()),
    }
}

fn command_executor(cmd: &str, handler: &mut ConsoleHandler) -> Result<(), String> {
    executor::block_on(_command_executor(cmd, handler))
}

pub async fn console(mut handler: ConsoleHandler) {
    let mut history = ConsoleHistory::default();
    let completion = ConsoleCompletion::default();

    if handler.router.is_some() {
        info!("Router successfully connected !");
    }

    if handler.db_handler.is_some() {
        info!("Database successfully connected !");
    }

    println!("[GREET] Time for C-sides !");
    loop {
        _ = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("lpmng")
            .history_with(&mut history)
            .completion_with(&completion)
            .validate_with(|cmd: &String| -> Result<(), String> {
                command_executor(cmd, &mut handler)
            })
            .interact_text();
    }
}
