use std::{
    collections::HashMap,
    ops::ControlFlow,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};

use futures::stream::StreamExt;
use futures::SinkExt;
use log::{debug, error, info};

use crate::{api, AppState};

// How often to check the battery level
#[cfg(not(debug_assertions))]
const BATTERY_CHECK_INTERVAL: Duration = std::time::Duration::from_secs(60);
#[cfg(debug_assertions)]
const BATTERY_CHECK_INTERVAL: Duration = std::time::Duration::from_secs(10);

// How often to send a notification to the client
#[cfg(not(debug_assertions))]
const BATTERY_ALERT_INTERVAL: Duration = std::time::Duration::from_secs(60 * 60);
#[cfg(debug_assertions)]
const BATTERY_ALERT_INTERVAL: Duration = std::time::Duration::from_secs(60);

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    // Send a ping to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        debug!("Pinged!");
    } else {
        debug!("Could not send ping to client");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg).is_break() {
                return;
            }
        } else {
            debug!("client abruptly disconnected");
            return;
        }
    }

    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();

    // This task will check controllers every 30 seconds and send a message to client if
    // a controller is low on battery
    let mut send_task = tokio::spawn(async move {
        // HashMap to store last alert timestamps for each controller
        let mut last_alerts: HashMap<String, u64> = HashMap::new();
        let mut cnt = 0;

        loop {
            tokio::time::sleep(BATTERY_CHECK_INTERVAL).await;

            let settings = state.settings_service.get_settings().await;
            if !settings.notifications {
                debug!("Notifications disabled, skipping notification check...");
                continue;
            }

            debug!("Checking controllers...");
            let controllers = match api::controllers_async().await {
                Ok(controllers) => controllers,
                Err(e) => {
                    error!("Error getting controllers: {}", e);
                    continue;
                }
            };

            for controller in controllers {
                let low_battery = controller.capacity < 20 && controller.is_discharging();
                debug!(
                    "Controller {} is low battery: {}",
                    controller.name, low_battery
                );
                if low_battery {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let first_alert = !last_alerts.contains_key(&controller.id());

                    let last_alert = last_alerts.entry(controller.id()).or_insert(now);

                    let last_alert_secs_ago = now - *last_alert;
                    debug!(
                        "Last alert was {} seconds ago for controller {}",
                        last_alert_secs_ago, controller.name
                    );

                    if first_alert || last_alert_secs_ago >= BATTERY_ALERT_INTERVAL.as_secs() {
                        let message = format!(
                            "{} is low on battery ({}%)",
                            controller.name, controller.capacity
                        );
                        info!("Sending notification: {}", message);

                        if sender.send(Message::Text(message)).await.is_ok() {
                            cnt += 1;
                        } else {
                            return cnt;
                        }

                        // Update last alert timestamp
                        *last_alert = now;
                    }
                }
            }
        }
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            if process_message(msg).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(cnt) => println!("Sent {} messages", cnt),
                Err(err) => println!("Error sending messages {:?}", err)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(cnt) => println!("Received {} messages", cnt),
                Err(b) => println!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    info!("Websocket context destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            debug!(">>> client sent str: {:?}", t);
        }
        Message::Binary(d) => {
            debug!(">>> client sent {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                debug!(
                    ">>> client sent close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                debug!(">>> client somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }
        Message::Pong(v) => {
            debug!(">>> client sent pong with {:?}", v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            debug!(">>> client sent ping with {:?}", v);
        }
    }
    ControlFlow::Continue(())
}
