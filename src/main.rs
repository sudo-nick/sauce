use rocket::{fs::FileServer, futures::StreamExt, get, launch, routes};
use std::net::IpAddr;
mod xdo;

use xdo::{Events, XDo};

#[get("/events")]
fn event_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    let channel = ws.channel(move |mut stream| {
        let xdo = XDo::new(None).unwrap();
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let message = message?;
                match message {
                    ws::Message::Binary(data) => {
                        let etype = Events::from(data[0]);
                        let x = f32::from_le_bytes(data[1..5].try_into().unwrap());
                        let y = f32::from_le_bytes(data[5..9].try_into().unwrap());
                        println!("event type: {:?}, x : {:?}, y : {:?}", etype, x, y);
                        match etype {
                            Events::Move => {
                                let _ = xdo.move_mouse_relative(x as i32, y as i32);
                            }
                            Events::MClick => {
                                let _ = xdo.click(2);
                            }
                            Events::LClick => {
                                let _ = xdo.click(1);
                            }
                            Events::RClick => {
                                let _ = xdo.click(3);
                            }
                            Events::ScrollU => {
                                let _ = xdo.mouse_up(10);
                            }
                            Events::ScrollD => {
                                let _ = xdo.mouse_down(10);
                            }
                            _ => {
                                println!("unhandled event type");
                            }
                        }
                        if x.eq(&0.) && y.eq(&0.) {
                            let _ = xdo.click(1);
                        } else {
                            let _ = xdo.move_mouse_relative(x as i32, y as i32);
                        }
                    }
                    _ => {}
                }
            }
            Ok(())
        })
    });
    println!("closed ig");
    return channel;
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        address: IpAddr::from([0, 0, 0, 0]),
        port: 5000,
        ..Default::default()
    };
    rocket::custom(config)
        .mount("/", FileServer::from("public"))
        .mount("/", routes![event_channel])
}
