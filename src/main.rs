use rocket::{fs::FileServer, futures::StreamExt, get, launch, routes};
use std::net::IpAddr;
mod xdo;

use xdo::XDo;

#[get("/events")]
fn event_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    let channel = ws.channel(move |mut stream| {
        let xdo = XDo::new(None).unwrap();
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let message = message?;
                match message {
                    ws::Message::Binary(data) => {
                        let x = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                        let y = f32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                        let dx = (x * 3.) as i32;
                        let dy = (y * 3.) as i32;
                        if x.eq(&0.) && y.eq(&0.) {
                            let _ = xdo.click(1);
                        } else {
                            let _ = xdo.move_mouse_relative(dx, dy);
                        }
                        println!("x: {:?}, y: {:?}", x, y);
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
