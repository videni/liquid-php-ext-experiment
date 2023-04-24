use ext_php_rs::flags::DataType;
use ext_php_rs::types::{Zval, ZendHashTable};
use liquid::model::{
    ArrayView, DisplayCow, KString, KStringCow, ObjectRender, ObjectSource, State, Value, ScalarCow, Scalar,
};
use liquid::{ValueView, ObjectView, Object};
use core::fmt;
use std::fmt::Debug;

use super::zend_hash_table::ZendHashTableView;
use super::zend_object::ZendObjectView;

#[derive(Debug)]
pub struct ZvalView<'a>(pub &'a Zval);

impl<'a> ValueView for ZvalView<'a> {
    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn as_scalar(&self) -> Option<ScalarCow<'a>> {
        let val_type = self.0.get_type();
        match val_type {
            DataType::String => {
                return Some(Scalar::new(self.0.string().unwrap()));
            },
            DataType::Bool => {
                return Some(Scalar::new::<bool>(self.0.bool().unwrap().into()));
            },
            DataType::Long => {
                return Some(Scalar::new::<i64>(self.0.long().unwrap().into()));
            },
            DataType::Double => {
                return Some(Scalar::new::<f64>(self.0.double().unwrap().into()));
            },
            _ => {
                return None;
            }
        }
    }

    fn is_scalar(&self) -> bool {
        let value_type = self.0.get_type();
        value_type == DataType::String
            || value_type == DataType::Long
            || value_type == DataType::Double
            || value_type == DataType::True
            || value_type == DataType::Bool
            || value_type == DataType::False
    }

    fn as_array(&self) -> Option<&dyn ArrayView> {
       self.0
        .array()
        .and_then(|z|Some(convert_value(z)))
    }

    fn is_array(&self) -> bool {
        self.0.is_array()
    }

    fn is_object(&self) -> bool {
        self.0.is_object()
    }

    fn is_nil(&self) -> bool {
        self.0.is_null()
    }

    fn render(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ZvalViewRender(self)))
    }

    fn source(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ZvalViewRender(self)))
    }

    fn type_name(&self) -> &'static str {
        "zval"
    }

    fn query_state(&self, _: liquid::model::State) -> bool {
        true
    }

    fn to_kstr(&self) -> KStringCow<'_> {
        let s: String = ZvalViewRender(self).to_string();
        KStringCow::from_string(s)
    }

    fn to_value(&self) -> Value {
        let val_type = self.0.get_type();
        match val_type {
            DataType::Array => {
                return self.as_array().unwrap().to_value();
            },
            DataType::String => {
                return Value::Scalar(self.0.string().unwrap().into());
            },
            DataType::Bool => {
                return  Value::Scalar(self.0.bool().unwrap().into())
            },
            DataType::Long => {
                return Value::Scalar(self.0.long().unwrap().into())
            },
            DataType::Double => {
                return Value::Scalar(self.0.double().unwrap().into());
            },
            DataType::Object(_) => {
                let zend_object = self
                .0
                .object()
                .unwrap();

                let object: Object = ZendObjectView::from(zend_object)
                    .into();

                return Value::Object(object);
            },
            _  => {
                // TODO: handle resource, callable, reference
                Value::Nil
            }
        }
    }
}

fn convert_value(z: &ZendHashTable) -> &dyn ArrayView {
    let ht = Box::new(ZendHashTableView(z));

    Box::leak(ht)
}

pub struct ZvalViewRender<'a>(&'a ZvalView<'a>);

impl<'a> fmt::Display for ZvalViewRender<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)?;

        Ok(())
    }
}