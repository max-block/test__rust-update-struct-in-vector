#[derive(Debug)]
struct Data {
    name: String,
    value: i32,
}

fn main() {
    let data = vec![
        Data {
            name: "n1".to_string(),
            value: 1,
        },
        Data {
            name: "n2".to_string(),
            value: 2,
        },
        Data {
            name: "n3".to_string(),
            value: 3,
        },
        Data {
            name: "n4".to_string(),
            value: 4,
        },
    ];

    let data: Vec<Data> = data
        .into_iter()
        .map(|mut x| {
            if x.value % 2 == 0 {
                x.name = x.name + " updated";
            }
            x
        })
        .collect();

    println!("{:#?}", data);
}
