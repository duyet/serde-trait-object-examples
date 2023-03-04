use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "name")]
trait Transformer {
    fn transform(&self);
}

#[derive(Serialize, Deserialize)]
struct UpperCase;

#[typetag::serde(name = "uppercase")]
impl Transformer for UpperCase {
    fn transform(&self) {
        println!("uppercase");
    }
}

#[derive(Serialize, Deserialize)]
struct MyCustom {
    x: i32,
    y: i32,
}

#[typetag::serde(name = "my_custom")]
impl Transformer for MyCustom {
    fn transform(&self) {
        println!("my_custom: x={} y={}", self.x, self.y);
    }
}

fn main() {
    let yaml = "
        - name: uppercase
        - name: my_custom
          x: 1
          y: 2
    ";

    let values: Vec<Box<dyn Transformer>> = serde_yaml::from_str(yaml).unwrap();

    for value in values {
        value.transform();
    }
}

