/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// HTTP functions
// These functions can be used when opening and submitting remote web requests, and webhooks.
//
// Function	Description
// http::head()	Perform a remote HTTP HEAD request
// http::get()	Perform a remote HTTP GET request
// http::put()	Perform a remote HTTP PUT request
// http::post()	Perform a remote HTTP POST request
// http::patch()	Perform a remote HTTP PATCH request
// http::delete()	Perform a remote HTTP DELETE request

use std::collections::{hash_map, HashMap};

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, Empty, ToRawStatement},
    Field,
};

use super::array::Function;

pub struct Url(sql::Value);

impl<T: Into<sql::Strand>> From<T> for Url {
    fn from(value: T) -> Self {
        let value: sql::Strand = value.into();
        Self(value.into())
    }
}

impl From<Field> for Url {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

impl From<Url> for sql::Value {
    fn from(value: Url) -> Self {
        value.0
    }
}
pub enum Object {
    Empty,
    Object(sql::Value),
}

// impl From<Object> for sql::Value {
//     fn from(value: Object) -> Self {
//         match value {
//             Object::Empty => ,
//             Object::Object(_) => todo!(),
//         }
//     }
// }

impl From<Empty> for Object {
    fn from(value: Empty) -> Self {
        Self::Empty
    }
}

impl From<Field> for Object {
    fn from(value: Field) -> Self {
        Self::Object(value.into())
    }
}

impl<T: Into<sql::Object>> From<T> for Object {
    fn from(value: T) -> Self {
        let value: sql::Object = value.into();
        Self::Object(value.into())
    }
}

fn create_fn_with_two_args(
    url: impl Into<Url>,
    custom_headers: impl Into<Object>,
    method: &str,
) -> Function {
    let url: sql::Value = url.into().into();
    let custom_headers: Object = custom_headers.into();
    let url_binding = Binding::new(url);
    let url_parametized = url_binding.get_param_dollarised();

    let mut all_bindings = vec![url_binding];

    let string = match custom_headers {
        Object::Empty => {
            format!("http::{method}({})", &url_parametized)
        }
        Object::Object(headers) => {
            let header_binding = Binding::new(headers);
            let header_parametized = header_binding.get_param_dollarised();
            all_bindings.push(header_binding);

            format!(
                "http::{method}({}, {})",
                url_parametized, header_parametized
            )
        }
    };

    Function {
        query_string: string,
        bindings: all_bindings,
    }
}

fn create_fn_with_three_args(
    url: impl Into<Url>,
    request_body: impl Into<Object>,
    custom_headers: impl Into<Object>,
    method: &str,
) -> Function {
    let url: sql::Value = url.into().into();
    let request_body: Object = request_body.into();
    let custom_headers: Object = custom_headers.into();
    let url_binding = Binding::new(url);
    let url_parametized = url_binding.get_param_dollarised();

    let mut all_bindings = vec![url_binding];

    let string = match request_body {
        Object::Empty => {
            let header_binding = Binding::new(sql::Object::default());
            let header_parametized = header_binding.get_param_dollarised();
            all_bindings.push(header_binding);

            format!(
                "http::{method}({}, {}",
                &url_parametized, header_parametized
            )
        }
        Object::Object(body) => {
            let header_binding = Binding::new(body);
            let header_parametized = header_binding.get_param_dollarised();
            all_bindings.push(header_binding);

            format!("http::{method}({}, {}", url_parametized, header_parametized)
        }
    };

    let string = match custom_headers {
        Object::Empty => {
            let header_binding = Binding::new(sql::Object::default());
            let header_parametized = header_binding.get_param_dollarised();
            all_bindings.push(header_binding);
            format!("{string}, {})", header_parametized)
        }
        Object::Object(headers) => {
            let header_binding = Binding::new(headers);
            let header_parametized = header_binding.get_param_dollarised();
            all_bindings.push(header_binding);

            format!("{string}, {})", header_parametized)
        }
    };

    Function {
        query_string: string,
        bindings: all_bindings,
    }
}

macro_rules! create_fn_with_url_and_hears {
    ($function_name: expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](url: impl Into<Url>, custom_headers: impl Into<Object>) -> Function {
               create_fn_with_two_args(url, custom_headers, $function_name)
           }

           #[macro_export]
           macro_rules! [<http_ $function_name>] {
               ( $geometry:expr ) => {
                   crate::functions::http::[<$function_name _fn>]($geometry)
               };
           }
           pub use [<http_ $function_name>] as [<$function_name>];

        }
    };
}

create_fn_with_url_and_hears!("head");
create_fn_with_url_and_hears!("get");
create_fn_with_url_and_hears!("delete");

pub fn post(
    url: impl Into<Url>,
    request_body: impl Into<Object>,
    custom_headers: impl Into<Object>,
) -> Function {
    create_fn_with_three_args(url, request_body, custom_headers, "post")
}

pub fn put(
    url: impl Into<Url>,
    request_body: impl Into<Object>,
    custom_headers: impl Into<Object>,
) -> Function {
    create_fn_with_three_args(url, request_body, custom_headers, "put")
}

pub fn patch(
    url: impl Into<Url>,
    request_body: impl Into<Object>,
    custom_headers: impl Into<Object>,
) -> Function {
    create_fn_with_three_args(url, request_body, custom_headers, "patch")
}

#[test]
fn test_head_method_with_empty_header() {
    let result = head_fn("https://codebreather.com", Empty);
    assert_eq!(result.fine_tune_params(), "http::head($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "http::head('https://codebreather.com')"
    );
}

#[test]
fn test_field_head_method_with_empty_header() {
    let homepage = Field::new("homepage");

    let result = head_fn(homepage, Empty);
    assert_eq!(result.fine_tune_params(), "http::head($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "http::head(homepage)");
}

#[test]
fn test_head_method_with_plain_custom_header() {
    let headers = hash_map::HashMap::from([("x-my-header".into(), "some unique string".into())]);
    let result = head_fn("https://codebreather.com", headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::head($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::head('https://codebreather.com', { \"x-my-header\": 'some unique string' })"
    );
}

#[test]
fn test_head_method_with_field_custom_header() {
    let homepage = Field::new("homepage");
    let headers = Field::new("headers");

    let result = head_fn(homepage, headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::head($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "http::head(homepage, headers)");
}

#[test]
fn test_get_method_with_empty_header() {
    let result = get_fn("https://codebreather.com", Empty);
    assert_eq!(result.fine_tune_params(), "http::get($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "http::get('https://codebreather.com')"
    );
}

#[test]
fn test_field_get_method_with_empty_header() {
    let homepage = Field::new("homepage");

    let result = get_fn(homepage, Empty);
    assert_eq!(result.fine_tune_params(), "http::get($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "http::get(homepage)");
}

#[test]
fn test_get_method_with_plain_custom_header() {
    let headers = hash_map::HashMap::from([("x-my-header".into(), "some unique string".into())]);
    let result = get_fn("https://codebreather.com", headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::get($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::get('https://codebreather.com', { \"x-my-header\": 'some unique string' })"
    );
}

#[test]
fn test_get_method_with_field_custom_header() {
    let homepage = Field::new("homepage");
    let headers = Field::new("headers");

    let result = get_fn(homepage, headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::get($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "http::get(homepage, headers)");
}

#[test]
fn test_delete_method_with_empty_header() {
    let result = delete_fn("https://codebreather.com", Empty);
    assert_eq!(result.fine_tune_params(), "http::delete($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "http::delete('https://codebreather.com')"
    );
}

#[test]
fn test_field_delete_method_with_empty_header() {
    let homepage = Field::new("homepage");

    let result = delete_fn(homepage, Empty);
    assert_eq!(result.fine_tune_params(), "http::delete($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "http::delete(homepage)");
}

#[test]
fn test_delete_method_with_plain_custom_header() {
    let headers = hash_map::HashMap::from([("x-my-header".into(), "some unique string".into())]);
    let result = delete_fn("https://codebreather.com", headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::delete($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::delete('https://codebreather.com', { \"x-my-header\": 'some unique string' })"
    );
}

#[test]
fn test_delete_method_with_field_custom_header() {
    let homepage = Field::new("homepage");
    let headers = Field::new("headers");

    let result = delete_fn(homepage, headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::delete($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::delete(homepage, headers)"
    );
}

#[test]
fn test_field_post_method_with_empty_body() {
    let homepage = Field::new("homepage");
    let headers = Field::new("headers");
    let result = post("https://codebreather.com", Empty, headers);

    assert_eq!(
        result.fine_tune_params(),
        "http::post($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::post('https://codebreather.com', {  }, headers)"
    );
}
#[test]
fn test_field_post_method_with_empty_body_and_headers() {
    let homepage = Field::new("homepage");
    let result = post("https://codebreather.com", Empty, Empty);

    assert_eq!(
        result.fine_tune_params(),
        "http::post($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::post('https://codebreather.com', {  }, {  })"
    );
}

#[test]
fn test_field_post_method_with_fields_as_args() {
    let homepage = Field::new("homepage");
    let request_body = Field::new("request_body");
    let headers = Field::new("headers");

    let result = post(homepage, request_body, headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::post($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::post(homepage, request_body, headers)"
    );
}

#[test]
fn test_post_method_with_body_and_custom_headers_as_plain_values() {
    let body = HashMap::from([
        ("id".into(), 1.into()),
        ("body".into(), "This is some awesome thinking!".into()),
        ("postId".into(), 100.into()),
    ]);
    let headers = HashMap::from([("x-my-header".into(), "some unique string".into())]);
    let result = post("https://codebreather.com", body, headers);

    assert_eq!(
        result.fine_tune_params(),
        "http::post($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::post('https://codebreather.com', { body: 'This is some awesome thinking!', id: 1, postId: 100 }, { \"x-my-header\": 'some unique string' })"
    );
}

#[test]
fn test_field_put_method_with_empty_body_and_headers() {
    let homepage = Field::new("homepage");
    let result = put("https://codebreather.com", Empty, Empty);

    assert_eq!(
        result.fine_tune_params(),
        "http::put($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::put('https://codebreather.com', {  }, {  })"
    );
}

#[test]
fn test_field_put_method_with_fields_as_args() {
    let homepage = Field::new("homepage");
    let request_body = Field::new("request_body");
    let headers = Field::new("headers");

    let result = put(homepage, request_body, headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::put($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::put(homepage, request_body, headers)"
    );
}

#[test]
fn test_put_method_with_body_and_custom_headers_as_plain_values() {
    let body = HashMap::from([
        ("id".into(), 1.into()),
        ("body".into(), "This is some awesome thinking!".into()),
        ("postId".into(), 100.into()),
    ]);
    let headers = HashMap::from([("x-my-header".into(), "some unique string".into())]);
    let result = put("https://codebreather.com", body, headers);

    assert_eq!(
        result.fine_tune_params(),
        "http::put($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::put('https://codebreather.com', { body: 'This is some awesome thinking!', id: 1, postId: 100 }, { \"x-my-header\": 'some unique string' })"
    );
}

#[test]
fn test_field_patch_method_with_empty_body_and_headers() {
    let homepage = Field::new("homepage");
    let result = patch("https://codebreather.com", Empty, Empty);

    assert_eq!(
        result.fine_tune_params(),
        "http::patch($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::patch('https://codebreather.com', {  }, {  })"
    );
}

#[test]
fn test_field_patch_method_with_fields_as_args() {
    let homepage = Field::new("homepage");
    let request_body = Field::new("request_body");
    let headers = Field::new("headers");

    let result = patch(homepage, request_body, headers);
    assert_eq!(
        result.fine_tune_params(),
        "http::patch($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::patch(homepage, request_body, headers)"
    );
}

#[test]
fn test_patch_method_with_body_and_custom_headers_as_plain_values() {
    let body = HashMap::from([
        ("id".into(), 1.into()),
        ("body".into(), "This is some awesome thinking!".into()),
        ("postId".into(), 100.into()),
    ]);
    let headers = HashMap::from([("x-my-header".into(), "some unique string".into())]);
    let result = patch("https://codebreather.com", body, headers);

    assert_eq!(
        result.fine_tune_params(),
        "http::patch($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "http::patch('https://codebreather.com', { body: 'This is some awesome thinking!', id: 1, postId: 100 }, { \"x-my-header\": 'some unique string' })"
    );
}
