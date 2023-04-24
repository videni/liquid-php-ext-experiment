use crate::views::zend_hash_table::ZendHashTableView;
use crate::views::zend_object::ZendObjectView;
use crate::views::zval::ZvalView;
use ext_php_rs::ffi::zend_throw_exception_ex;
use ext_php_rs::types::{Zval, ZendHashTable, ZendObject};
use ext_php_rs::{prelude::*, types::ZendClassObject};

#[php_class]
pub struct Template {
    template: liquid::Template,
}

#[php_impl]
impl Template {
    pub fn new(path: String) -> Self {
        // let exist = std::path::Path::new(path.as_str()).exists();
        // if exist == false {
        //     zend_throw_exception_ex(null, 0, "File not found");
        // }
        let template: liquid::Template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(path.as_str())
            .unwrap();

        Self { template }
    }

    pub fn render(#[this] this: &mut ZendClassObject<Template>, vars: &mut ZendObject) -> String {
        dbg!(&vars);
        let vars = ZendObjectView(vars);

        return this.template.render(&vars).unwrap();
    }
}
