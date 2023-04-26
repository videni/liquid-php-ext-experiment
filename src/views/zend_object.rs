use super::zval::ZvalView;
use ext_php_rs::convert::{FromZval, IntoZval};
use ext_php_rs::ffi::{_zval_struct, zend_throw_exception_ex};
use ext_php_rs::types::{ZendObject, Zval};
use liquid::model::{
    ArrayView, DisplayCow, KString, KStringCow, ObjectRender, ObjectSource, State, Value,
};
use liquid::{Object, ObjectView, ValueView};
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct ZendObjectView<'a>(pub &'a ZendObject);

impl<'a> liquid::ValueView for ZendObjectView<'a> {
    fn as_debug(&self) -> &dyn Debug {
        self.0
    }

    fn render(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ObjectRender::new(self)))
    }

    fn source(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ObjectSource::new(self)))
    }

    fn type_name(&self) -> &'static str {
        let s = self.0.get_class_name().unwrap();

        Box::leak(s.into_boxed_str())
    }

    fn query_state(&self, state: liquid::model::State) -> bool {
        true
    }

    fn as_object(&self) -> Option<&dyn ObjectView> {
        Some(self)
    }

    fn to_kstr(&self) -> KStringCow<'_> {
        let s = ObjectRender::new(self).to_string();
        KStringCow::from_string(s)
    }

    fn to_value(&self) -> Value {
        let properties = self.0.get_properties().unwrap();

        let mut object = Object::new();
        for (_, key, value) in properties.iter() {
            object.insert(
                KString::from_string(key.unwrap()),
                ZvalView(value).to_value(),
            );
        }

        Value::Object(object)
    }
}

impl<'a> liquid::ObjectView for ZendObjectView<'a> {
    fn as_value(&self) -> &dyn ValueView {
        self
    }

    fn size(&self) -> i64 {
        self.0.get_properties().unwrap().len() as i64
    }

    fn keys(&self) -> Box<(dyn Iterator<Item = KStringCow<'_>> + '_)> {
        let ht = self.0.get_properties().unwrap();

        let keys = ht.iter().map(|s| {
            if ht.has_numerical_keys() {
                KStringCow::from_string(s.1.unwrap())
            } else {
                KStringCow::from_string(s.0.to_string())
            }
        });

        Box::new(keys)
    }

    fn values<'k>(&'k self) -> Box<dyn Iterator<Item = &'k dyn ValueView> + 'k> {
        let values = self.0.get_properties().unwrap().iter().map(|(_, _, v)| {
            let z = Box::new(ZvalView(v));

            return Box::leak(z) as &dyn ValueView;
        });

        Box::new(values)
    }

    fn iter(&self) -> Box<(dyn Iterator<Item = (KStringCow<'_>, &'_ (dyn ValueView + '_))> + '_)> {
        let properties = self.0.get_properties().unwrap().iter().map(|(_, key, v)| {
            let key = KStringCow::from_string(key.unwrap());
            let value = convert_value(v);

            (key, value)
        });

        Box::new(properties)
    }

    fn contains_key(&self, key: &str) -> bool {
        self.0
            .has_property(key, ext_php_rs::types::PropertyQuery::Isset)
            .unwrap()
    }

    fn get(&self, key: &str) -> std::option::Option<&'_ (dyn ValueView + '_)> {
        let properties = self.0.get_properties().unwrap();

        properties.get(key).map(|s| convert_value(s))
    }
}

fn convert_value(s: &Zval) -> &dyn ValueView {
    let zval = Box::new(ZvalView(s));

    Box::leak(zval)
}

impl<'a> Into<Object> for ZendObjectView<'a> {
    fn into(self) -> Object {
        let mut object = Object::new();

        for (key, value) in self.iter() {
            object.insert(key.into(), value.to_value());
        }

        object
    }
}

impl<'a> From<&'a ZendObject> for ZendObjectView<'a> {
    fn from(value: &'a ZendObject) -> ZendObjectView<'a> {
        ZendObjectView(value)
    }
}
