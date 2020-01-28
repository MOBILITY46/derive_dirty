use derive_dirty::Dirty;

#[derive(Dirty, Debug)]
pub struct Model {
    id: String,
    pub value: u16,
}

fn main() {

    let _ = Model {
        id: "8".to_owned(),
        value: 8
    };

    let _ = DirtyModel {
        value: 10
    };
}
