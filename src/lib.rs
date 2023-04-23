#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::convert::FromZval;
use ext_php_rs::ffi::zend_throw_exception_ex;
use ext_php_rs::{prelude::*, types::ZendClassObject, types::ZendObject};
use liquid::model::{DisplayCow, KStringCow, Value};
use liquid::{ValueView, ObjectView};
use std::collections::HashMap;
use std::fmt::Debug;

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Take an object reference and also return it.
#[php_function]
pub fn take_obj(obj: &mut ZendObject) -> &mut ZendObject {
    let _ = obj.set_property("world", "hello world");
    dbg!(obj)
}

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

    pub fn render(#[this] this: &mut ZendClassObject<Template>, variables: &ZendObject) -> String {
        dbg!(variables);
        let vars = LiquidZendObject(variables);

        return this.template.render(&vars).unwrap();
    }
}

#[derive(Debug)]
pub struct LiquidZendObject<'a>(&'a ZendObject);

impl<'a> liquid::ValueView for LiquidZendObject<'a> {
    fn as_debug(&self) -> &dyn Debug {
        todo!()
    }
    fn render(&self) -> DisplayCow<'_> {
        todo!()
    }
    fn source(&self) -> DisplayCow<'_> {
        todo!()
    }
    fn type_name(&self) -> &'static str {
        todo!()
    }
    fn query_state(&self, _: liquid::model::State) -> bool {
        todo!()
    }
    fn to_kstr(&self) -> KStringCow<'_> {
        todo!()
    }
    fn to_value(&self) -> Value {
        todo!()
    }
}

impl<'a> liquid::ObjectView for LiquidZendObject<'a>
{
    fn as_value(&self) -> &dyn ValueView { todo!() }
    fn size(&self) -> i64 { todo!() }
    fn keys(&self) -> Box<(dyn Iterator<Item = KStringCow<'_>> + '_)> { todo!() }
    fn values(&self) -> Box<(dyn Iterator<Item = &'_ (dyn ValueView + '_)> + '_)> { todo!() }
    fn iter(&self) -> Box<(dyn Iterator<Item = (KStringCow<'_>, &'_ (dyn ValueView + '_))> + '_)> { todo!() }
    fn contains_key(&self, _: &str) -> bool { todo!() }
    fn get(&self, _: &str) -> std::option::Option<&'a (dyn ValueView + 'a)> { todo!() }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
