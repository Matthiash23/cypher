pub struct MergeQuery {
    nv: String,
    state: String,
}

impl MergeQuery {
    pub fn new(nv: String, state: String) -> Self {
        MergeQuery { nv, state }
    }
}

pub trait MergeTrait: 'static {
}
