use rustler::{Error, LocalPid, NifStruct, OwnedEnv};
use spacenav_plus::Connection;
use std::thread;

#[derive(Debug, NifStruct)]
#[module = "Spacenav.MotionEvent"]
struct MotionEvent {
    x: i32,
    y: i32,
    z: i32,
    rx: i32,
    ry: i32,
    rz: i32,
    period: u32,
}

impl From<spacenav_plus::MotionEvent> for MotionEvent {
    fn from(m: spacenav_plus::MotionEvent) -> Self {
        MotionEvent {
            x: m.x,
            y: m.y,
            z: m.z,
            rx: m.rx,
            ry: m.ry,
            rz: m.rz,
            period: m.period,
        }
    }
}

#[derive(Debug, NifStruct)]
#[module = "Spacenav.ButtonEvent"]
struct ButtonEvent {
    bnum: i32,
    press: bool,
}

impl From<spacenav_plus::ButtonEvent> for ButtonEvent {
    fn from(b: spacenav_plus::ButtonEvent) -> Self {
        ButtonEvent {
            bnum: b.bnum,
            press: b.press,
        }
    }
}

#[rustler::nif]
fn listen(pid: LocalPid) -> Result<(), Error> {
    match Connection::new() {
        Ok(conn) => {
            listen_with_connection(conn, pid);
            Ok(())
        }
        Err(_) => {
            Err(Error::Term(Box::new("Failed to connect to SpaceMouse")))
        }
    }
}

fn listen_with_connection(conn: Connection, pid: LocalPid) {
    let env = OwnedEnv::new();

    thread::spawn(move || {
        env.run(|e| {
            loop {
                match conn.wait() {
                    Ok(event) => {
                        match event {
                            spacenav_plus::Event::Motion(m) => {
                                e.send(&pid, MotionEvent::from(m)).unwrap();
                            }
                            spacenav_plus::Event::Button(b) => {
                                e.send(&pid, ButtonEvent::from(b)).unwrap();
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading SpaceMouse event: {:?}", e),
                }
            }
        });
    });
}

rustler::init!("Elixir.Spacenav");
