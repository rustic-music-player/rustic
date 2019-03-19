use std::sync::Arc;
use std::thread;

use dbus::{
    BusType,
    Connection,
    NameFlag,
    tree::Factory,
    arg::ArgType
};
use failure::Error;

use rustic_core::Rustic;
use rustic_core::PlayerState;

pub fn start(app: Arc<Rustic>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let c = Connection::get_private(BusType::Session).unwrap();

        c.register_name("org.mpris.MediaPlayer2.rustic", NameFlag::ReplaceExisting as u32).unwrap();
        let f = Factory::new_fn::<()>();
        let can_quit = f.property::<bool, _>("CanQuit", ())
            .on_get(|iter, prop| {
                iter.append(false);
                Ok(())
            });
        let can_raise = f.property::<bool, _>("CanRaise", ())
            .on_get(|iter, prop| {
                iter.append(false);
                Ok(())
            });

        let play = {
            let app = Arc::clone(&app);

            move|m| {
                let player = app.get_default_player().unwrap();
                player.set_state(PlayerState::Play);
                Ok(vec![])
            }
        };
        let pause = {
            let app = Arc::clone(&app);

            move|m| {
                let player = app.get_default_player().unwrap();
                player.set_state(PlayerState::Pause);
                Ok(vec![])
            }
        };
        let stop = {
            let app = Arc::clone(&app);

            move|m| {
                let player = app.get_default_player().unwrap();
                player.set_state(PlayerState::Stop);
                Ok(vec![])
            }
        };

        let tree = f.tree(()).add(f.object_path("/org/mpris/MediaPlayer2", ()).introspectable().add(
            f.interface("org.mpris.MediaPlayer2", ())
                .add_p(can_quit)
                .add_p(can_raise)
                .add_p(f.property::<String, _>("Identity", ())
                    .on_get(|iter, prop| {
                        iter.append("Rustic Music Player");
                        Ok(())
                    }))
        )
            .add(
                f.interface("org.mpris.MediaPlayer2.Player", ())
                    .add_m(f.method("Play", (), {
                        let app = Arc::clone(&app);

                        move|m| {
                            println!("play");
                            let player = app.get_default_player().unwrap();
                            println!("setting player state");
                            player.set_state(PlayerState::Play).unwrap();
                            Ok(vec![])
                        }
                    }))
                    .add_m(f.method("Pause", (), pause))
                    .add_m(f.method("Stop", (), stop))
            ));
        tree.set_registered(&c, true).unwrap();
        c.add_handler(tree);

        loop { c.incoming(1000).next(); }
    })
}