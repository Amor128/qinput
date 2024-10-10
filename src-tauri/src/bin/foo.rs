use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

fn kick_back(select: &str) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    match select {
        "qiyezhuce_7145_02" => {
            // wait for 1 second
            std::thread::sleep(std::time::Duration::from_secs(2));
            // send keybord event using enigo
            // clear the input field by sent 100 backspace key
            enigo.text("qiyezhuce_7145_02");
            enigo.key(Key::Tab, Click);

            enigo.text("ysf");
            enigo.key(Key::Tab, Click);

            enigo.text("a888888");
            enigo.key(Key::Return, Click);
        }
        "wy" => {
            println!("wy");
        }
        "pepsi_cn_test" => {
            println!("pepsi_cn_test");
        }
        _ => {
            println!("default");
        }
    }
}

fn main() {
    kick_back("qiyezhuce_7145_02");
}
