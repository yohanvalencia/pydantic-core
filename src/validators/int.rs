use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::Validator;
use crate::errors::{err_val_error, ErrorKind, ValResult};
use crate::standalone_validators::validate_int;
use crate::utils::{dict_create, dict_get};

#[derive(Debug, Clone)]
pub struct SimpleIntValidator;

impl Validator for SimpleIntValidator {
    fn is_match(type_: &str, dict: &PyDict) -> bool {
        type_ == "int"
            && dict.get_item("multiple_of").is_none()
            && dict.get_item("le").is_none()
            && dict.get_item("lt").is_none()
            && dict.get_item("ge").is_none()
            && dict.get_item("gt").is_none()
    }

    fn build(_dict: &PyDict) -> PyResult<Self> {
        Ok(Self)
    }

    fn validate(&self, py: Python, obj: &PyAny) -> ValResult<PyObject> {
        Ok(validate_int(py, obj)?.to_object(py))
    }

    fn clone_dyn(&self) -> Box<dyn Validator> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct FullIntValidator {
    multiple_of: Option<i64>,
    le: Option<i64>,
    lt: Option<i64>,
    ge: Option<i64>,
    gt: Option<i64>,
}

impl Validator for FullIntValidator {
    fn is_match(type_: &str, _dict: &PyDict) -> bool {
        type_ == "int"
    }

    fn build(dict: &PyDict) -> PyResult<Self> {
        Ok(Self {
            multiple_of: dict_get!(dict, "multiple_of", i64),
            le: dict_get!(dict, "le", i64),
            lt: dict_get!(dict, "lt", i64),
            ge: dict_get!(dict, "ge", i64),
            gt: dict_get!(dict, "gt", i64),
        })
    }

    fn validate(&self, py: Python, obj: &PyAny) -> ValResult<PyObject> {
        let int = validate_int(py, obj)?;
        if let Some(multiple_of) = self.multiple_of {
            if int % multiple_of != 0 {
                return err_val_error!(
                    py,
                    int,
                    kind = ErrorKind::IntMultiple,
                    context = Some(dict_create!(py, "multiple_of" => multiple_of))
                );
            }
        }
        if let Some(le) = self.le {
            if int > le {
                return err_val_error!(
                    py,
                    int,
                    kind = ErrorKind::IntLessThanEqual,
                    context = Some(dict_create!(py, "le" => le))
                );
            }
        }
        if let Some(lt) = self.lt {
            if int >= lt {
                return err_val_error!(
                    py,
                    int,
                    kind = ErrorKind::IntLessThan,
                    context = Some(dict_create!(py, "lt" => lt))
                );
            }
        }
        if let Some(ge) = self.ge {
            if int < ge {
                return err_val_error!(
                    py,
                    int,
                    kind = ErrorKind::IntGreaterThanEqual,
                    context = Some(dict_create!(py, "ge" => ge))
                );
            }
        }
        if let Some(gt) = self.gt {
            if int <= gt {
                return err_val_error!(
                    py,
                    int,
                    kind = ErrorKind::IntGreaterThan,
                    context = Some(dict_create!(py, "gt" => gt))
                );
            }
        }
        Ok(int.to_object(py))
    }

    fn clone_dyn(&self) -> Box<dyn Validator> {
        Box::new(self.clone())
    }
}