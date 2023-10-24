pub fn enter_station() -> String {
    println!("Enter station name: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    input.trim().to_string()
}

pub fn search_station(entry_name: String) -> Option<String> {
    let stations = [
        "Shinagawa",
        "Osaki",
        "Gotanda",
        "Meguro",
        "Ebisu",
        "Shibuya",
        "Harajuku",
        "Yoyogi",
        "Shinjuku",
        "Shin-Okubo",
        "Takadanobaba",
        "Mejiro",
        "Ikebukuro",
        "Otsuka",
        "Sugamo",
        "Komagome",
        "Tabata",
        "Nippori",
        "Nishi-Nippori",
        "Uguisudani",
        "Ueno",
        "Okachimachi",
        "Akihabara",
        "Kanda",
        "Tokyo",
        "Yurakucho",
        "Shimbashi",
        "Hamamatsucho",
        "Tamachi",
        "Takanawa-Gateway"
    ];
    if !stations.contains(&entry_name.as_str()) {
        return None;
    }

    let mut count = 1;
    for s in stations {
        if s.ne(&entry_name) {
            count = count + 1;
            continue;
        } else {
            break;
        }
    }

    Some(format!("{} is {} station from Shinagawa left.", entry_name, count))
}
