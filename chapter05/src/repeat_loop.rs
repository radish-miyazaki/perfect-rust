#![allow(dead_code)]

fn enter_station() -> String {
    println!("please enter station name.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

pub fn loop_1() {
    let stations = ["品川", "大崎", "五反田", "目黒", "恵比寿", "渋谷"];

    loop {
        let station = enter_station();
        if station.eq("end") {
            println!("end.");
            break;
        }

        if !station.contains(&station.as_str()) {
            println!("{} station name is not on the Yamanote line.", station);
            continue;
        }

        let mut count = 1;
        for s in stations {
            if s.ne(&station) {
                count += 1;
                continue;
            } else {
                break;
            }
        }
        println!("{} station is {} stops on the Yamanote line.", station, count);
    }
}
