use serlo_he_spec::{editor_impl, Heading, Plugins};

fn main() {
    println!("{}", &editor_impl(&Plugins::Heading(Heading::default())));
}
