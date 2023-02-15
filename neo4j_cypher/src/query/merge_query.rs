use crate::query::finalize::FinalizeTrait;
use crate::query::return_query::{ReturnParamTrait, ReturnQuery, ReturnTrait};

pub struct MergeQuery {
    nv: String,
    state: String,
}

impl MergeQuery {
    pub fn new(nv: String, state: String) -> Self {
        MergeQuery { nv, state }
    }
}

impl MergeTrait for MergeQuery {

}

impl FinalizeTrait for MergeQuery {
    fn finalize(&self) -> String {
        self.state.clone()
    }
}

impl ReturnTrait for MergeQuery {
    fn r#return(&mut self, nv: &str) -> Box<dyn ReturnParamTrait> {
        super::return_query::return_method(&self.state, vec![nv], None)
    }

    fn return_field(&mut self, nv: &str, field: &str) -> Box<dyn ReturnParamTrait> {
        super::return_query::return_method(&self.state, vec![nv], Some(field))
    }

    fn return_many(&mut self, nvs: Vec<&str>) -> Box<dyn ReturnParamTrait> {
        super::return_query::return_method(&self.state, nvs, None)
    }
}

pub trait MergeTrait: 'static + ReturnTrait + FinalizeTrait {
}
