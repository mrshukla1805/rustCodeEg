//id: deserialize-untrusted-data
#[derive(serde::Deserialize)]
struct MyData {
    a: u8,
    b: String,
}

fn main() {
    let json_str = "{invalid: json}";
    let bytes_str = "{invalid: bytes}".as_bytes();
    let bool_value = serde_json::value::Value::Bool(false);
    let file_path = "/tmp/txtfile.txt";

    //ruleid: deserialize-untrusted-data
    let data_from_str: MyData = serde_json::from_str(json_str).unwrap();
    //ruleid: deserialize-untrusted-data
    let data_from_bytes: MyData = serde_json::from_slice(bytes_str).unwrap();

    //ruleid: deserialize-untrusted-data
    let data_from_value: u8 = serde_json::from_value(bool_value).unwrap();
    //ruleid: deserialize-untrusted-data
    let data_from_reader: MyData = serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(file_path).unwrap(),)).unwrap();
}

fn safe_deserialization() -> Result<(), serde_json::Error> {
    let json_str = "{invalid: json}";
    let bytes_str = "newobject".as_bytes();
    let bool_value = serde_json::value::Value::Bool(true);

    // Use the `?` operator to return an error if the data is invalid
    //ok: deserialize-untrusted-data
    let data_from_str: MyData = serde_json::from_str(json_str)?;

    // Use the `match` operator to handle deserialization errors
    //ok: deserialize-untrusted-data
    let data_from_bytes: MyData = match serde_json::from_slice(bytes_str) {
        Ok(data) => data,
        Err(_) => MyData {
            a: 0,
            b: "Invalid data supplied, created other object".to_string(),
        },
    };

    // Use an `if let` syntax to perform logic conditionally
    //ok: deserialize-untrusted-data
    if let Ok(data) = serde_json::from_value(bool_value) {
        println!("Data was: {}", data && true);
    };

    Ok(())
}
