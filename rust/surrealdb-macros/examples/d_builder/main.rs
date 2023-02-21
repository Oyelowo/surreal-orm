use derive_builder::Builder;

#[derive(Builder)]
struct Lorem {
    ipsum: u32,
}

fn main() {
    let mut builder = LoremBuilder::default();
    if true {
        builder.ipsum(42);
    }

    let x: Lorem = builder.build();
    println!("PRINN, {builder}");
}
