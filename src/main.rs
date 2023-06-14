use serde_json::{Result, Value};

fn get_data() -> String {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phone" : "+44 1234512",
            "phones": ["+44 1234567", "+44 1234811"]
        }"#;
    return data.to_string();
}

fn parse_json(data: &str) -> Result<Value> {
    let res: Result<Value> = serde_json::from_str(data);
    return res;
}

fn collect_strings(v: &Value, buf: &mut Vec<String>) {
    match v {
        Value::String(s) => buf.push(s.to_string()),
        Value::Array(arr) => {
            for val in arr {
                collect_strings(val, buf);
            }
        },
        _ => ()
    }
}

fn print_message(res: Result<Value>) {
    match res {
        Ok(v) => {
            let mut buf: Vec<String> = Vec::new();
            collect_strings(&v["phone"], &mut buf);
            collect_strings(&v["phones"], &mut buf);
            let phones: String = buf.join(", ");
            println!("Please call {} at the numbers {}", v["name"], phones);
        },
        Err(_) => println!("Err"),
    };
}


fn sum(a: i32, b: i32) -> i32 {
    let c: i32 = a + b;
    return c;
}

fn main() {
    let a: i32 = 10;
    let b: i32 = 13;
    let c: i32 = sum(a, b);
    println!("Hello, world! {}+{}={}", a, b, c);

    let data: String = get_data();
    let res = parse_json(&data);
    print_message(res);
}
