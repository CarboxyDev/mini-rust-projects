fn main() {
    let file = std::fs::File::open("html.json").expect("Error: Unable to open file");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("Error: Unable to read the JSON file");

    let name = json.get("name").unwrap();
    println!("{}", name);
}
