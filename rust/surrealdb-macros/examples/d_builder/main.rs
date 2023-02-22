use derive_builder::Builder;

#[derive(Builder, Debug)]
struct Lorem {
    ipsum: u32,
}

fn main() {
    let mut builder = LoremBuilder::default();
    if true {
        builder.ipsum(42);
    }

    let x: Lorem = builder.build().unwrap();
    println!("PRINN, {x:?}");
}
