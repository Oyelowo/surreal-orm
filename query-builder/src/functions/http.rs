/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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

use crate::{Buildable, Erroneous, Function, ObjectLike, Parametric, StrandLike};

/// Represents URL
pub type Url = StrandLike;

fn create_fn_with_two_args(
    url: impl Into<Url>,
    custom_headers: Option<impl Into<ObjectLike>>,
    method: &str,
) -> Function {
    let url: Url = url.into();
    let mut all_bindings = url.get_bindings();
    let mut errors = url.get_errors();

    let string = match custom_headers {
        None => {
            format!("http::{method}({})", &url.build())
        }
        Some(headers) => {
            let headers: ObjectLike = headers.into();
            all_bindings.extend(headers.get_bindings());
            errors.extend(headers.get_errors());

            format!("http::{method}({}, {})", url.build(), headers.build())
        }
    };

    Function {
        query_string: string,
        bindings: all_bindings,
        errors,
    }
}

fn create_fn_with_three_args(
    url: impl Into<Url>,
    request_body: Option<impl Into<ObjectLike>>,
    custom_headers: Option<impl Into<ObjectLike>>,
    method: &str,
) -> Function {
    let url: Url = url.into();
    let mut all_bindings = url.get_bindings();
    let mut errors = url.get_errors();

    let string = match request_body {
        None => {
            format!("http::{method}({}, {{ }}", &url.build())
        }
        Some(body) => {
            let body: ObjectLike = body.into();
            all_bindings.extend(body.get_bindings());
            errors.extend(body.get_errors());

            format!("http::{method}({}, {}", url.build(), body.build())
        }
    };

    let string = match custom_headers {
        None => {
            format!("{string}, {{ }})")
        }
        Some(headers) => {
            let headers: ObjectLike = headers.into();
            all_bindings.extend(headers.get_bindings());
            errors.extend(headers.get_errors());

            format!("{string}, {})", headers.build())
        }
    };

    Function {
        query_string: string,
        bindings: all_bindings,
        errors,
    }
}

macro_rules! create_fn_with_url_and_head {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](url: impl Into<Url>, custom_headers: Option<impl Into<ObjectLike>>) -> Function {
               create_fn_with_two_args(url, custom_headers, $function_name)
           }

           $(#[$attr])*
           #[macro_export]
           macro_rules! [<http_ $function_name>] {
               ( $url:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, None as Option<ObjectLike>)
               };
               ( $url:expr, $custom_headers:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($custom_headers))
               };
           }
           pub use [<http_ $function_name>] as [<$function_name>];


            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;
                use functions::http;
                use std::collections::hash_map;

                #[test]
                fn [<test_ $function_name _method_with_empty_header>]() {
                    let result = [<$function_name _fn>]("https://codebreather.com", None as Option<ObjectLike>);
                    assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com')", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _method_with_field_and_empty_header>]() {
                    let homepage = Field::new("homepage");

                    let result = http::[<$function_name _fn>](homepage, None as Option<ObjectLike>);
                    assert_eq!(result.fine_tune_params(), format!("http::{}(homepage)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _method_with_plain_custom_header>]() {
                    let headers = hash_map::HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = http::[< $function_name _fn>]("https://codebreather.com", Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {})", $function_name, "{ \"x-my-header\": 'some unique string' }")
                    );
                }

                #[test]
                fn [<test_ $function_name _method_with_field_custom_header>]() {
                    let homepage = Field::new("homepage");
                    let headers = Field::new("headers");

                    let result = http::[<$function_name _fn>](homepage, Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, headers)", $function_name)
                    );
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage, headers)", $function_name));
                }

                // Macro version
                #[test]
                fn [<test_ $function_name _macro_method_with_empty_header_not_listed>]() {
                    let result = http::[<$function_name>]!("https://codebreather.com");
                    assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com')", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_empty_header>]() {
                    let homepage = Field::new("homepage");

                    let result = http::[<$function_name>]!(homepage);
                    assert_eq!(result.fine_tune_params(), format!("http::{}(homepage)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_plain_custom_header>]() {
                    let headers = hash_map::HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = http::[<$function_name>]!("https://codebreather.com", headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {})", $function_name, "{ \"x-my-header\": 'some unique string' }")
                    );
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_field_custom_header>]() {
                    let homepage = Field::new("homepage");
                    let headers = Field::new("headers");

                    let result = http::[<$function_name>]!(homepage, headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, headers)", $function_name)
                    );
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage, headers)", $function_name));
                }
            }
        }
    };
}

create_fn_with_url_and_head!(
    /// The http::head function performs a remote HTTP HEAD request. The first parameter is the URL of the remote endpoint.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// Also aliased as `http_head!`
    ///
    /// http::head(string) -> null
    /// If an object is given as the second argument, then this can be used to set the request headers.
    ///
    /// http::head(string, object) -> null
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a parameter.
    /// * `request_body` - Optional. The request body. Can be an object or a field or a parameter.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::http};
    /// use std::collections::HashMap;
    /// http::head!("https://codebreather.com");
    ///
    /// # let url_field = Field::new("url_field");
    /// http::head!(url_field);
    ///
    /// # let ur_param = Param::new("url_param");
    /// http::head!(ur_param);
    ///
    /// let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
    /// http::head!("https://codebreather.com", headers);
    => "head"
);

create_fn_with_url_and_head!(
    /// The http::get function performs a remote HTTP GET request. The first parameter is the URL of the remote endpoint.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value, otherwise the response is treated as text.
    /// Also aliased as `http_get!`
    ///
    /// http::get(string) -> value
    /// If an object is given as the second argument, then this can be used to set the request headers.
    ///
    /// http::get(string, object) -> value
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a parameter.
    /// * `request_body` - Optional. The request body. Can be an object or a field or a parameter.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::http};
    /// use std::collections::HashMap;
    /// http::get!("https://codebreather.com");
    ///
    /// # let url_field = Field::new("url_field");
    /// http::get!(url_field);
    ///
    /// # let url_param = Param::new("url_param");
    /// http::delete!(url_param);
    ///
    /// let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
    /// http::get!("https://codebreather.com", headers);
    => "get"
);

create_fn_with_url_and_head!(
    /// The http::delete function performs a remote HTTP DELETE request. The first parameter is the URL of the remote endpoint.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value, otherwise the response is treated as text.
    /// Also aliased as `http_delete!`
    ///
    /// http::delete(string) -> value
    /// If an object is given as the second argument, then this can be used to set the request headers.
    ///
    /// http::delete(string, object) -> value
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a parameter.
    /// * `request_body` - Optional. The request body. Can be an object or a field or a parameter.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::http};
    /// use std::collections::HashMap;
    /// http::delete!("https://codebreather.com");
    ///
    /// # let url_field = Field::new("url_field");
    /// http::delete!(url_field);
    /// # let url_param = Param::new("url_param");
    /// http::delete!(url_param);
    ///
    /// let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
    /// http::delete!("https://codebreather.com", headers);
    => "delete"
);

macro_rules! create_fn_with_3args_url_body_and_head {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](
                url: impl Into<Url>,
                request_body: Option<impl Into<ObjectLike>>,
                custom_headers: Option<impl Into<ObjectLike>>,
            ) -> Function {
               create_fn_with_three_args(url, request_body, custom_headers, $function_name)
           }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<http_ $function_name>] {
               ( $url:expr, $request_body:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($request_body), None as Option<ObjectLike>)
               };
               ( $url:expr, $request_body:expr, $custom_headers:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($request_body) ,Some($custom_headers))
               };
            }
            pub use [<http_ $function_name>] as [<$function_name>];


            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;
                use std::collections::hash_map::HashMap;

                #[test]
                fn [<test_field_ $function_name _method_with_empty_body_and_headers>]() {
                    let result = [<$function_name _fn>]("https://codebreather.com", None as Option<ObjectLike>, None as Option<ObjectLike>);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, {{ }}, {{ }})", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {{ }}, {{ }})", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _method_with_fields_as_args>]() {
                    let homepage = Field::new("homepage");
                    let request_body = Field::new("request_body");
                    let headers = Field::new("headers");

                    let result = [<$function_name _fn>](homepage, Some(request_body), Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _method_with_params_as_args>]() {
                    let homepage = Param::new("homepage");
                    let request_body = Param::new("request_body");
                    let headers = Param::new("headers");

                    let result = [<$function_name _fn>](homepage, Some(request_body), Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _method_with_body_and_custom_headers_as_plain_values>]() {
                    let body = HashMap::from([
                        ("id".to_string(), 1.into()),
                        ("body".to_string(), "This is some awesome thinking!".into()),
                        ("postId".to_string(), 100.into()),
                    ]);
                    let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = [<$function_name _fn>]("https://codebreather.com", Some(body), Some(headers));

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!(
                            "http::{}('https://codebreather.com', {}, {})",
                            $function_name,
                            "{ body: 'This is some awesome thinking!', id: 1, postId: 100 }",  "{ \"x-my-header\": 'some unique string' }" )
                    );
                }

                // Macro versions
                #[test]
                fn [<test_field_ $function_name _macro_method_with_no_custom_headers>]() {
                    let body = Field::new("body");
                    let result = [<$function_name>]!("https://codebreather.com", body);

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, body, {{ }})", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', body, {{ }})", $function_name)
                    );
                }
                #[test]
                fn [<test_field_ $function_name _macro_method_with_empty_body_and_headers>]() {
                    let result = [<$function_name>]!("https://codebreather.com", None, None);

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {{  }}, {{  }})", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _macro_method_with_fields_as_args>]() {
                    let homepage = Field::new("homepage");
                    let request_body = Field::new("request_body");
                    let headers = Field::new("headers");

                    let result = [<$function_name>]!(homepage, request_body, headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _macro_method_with_params_as_args>]() {
                    let homepage = Param::new("homepage");
                    let request_body = Param::new("request_body");
                    let headers = Param::new("headers");

                    let result = [<$function_name>]!(homepage, request_body, headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_body_and_custom_headers_as_plain_values>]() {
                    let body = HashMap::from([
                        ("id".to_string(), 1.into()),
                        ("body".to_string(), "This is some awesome thinking!".into()),
                        ("postId".to_string(), 100.into()),
                    ]);
                    let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = [<$function_name>]!("https://codebreather.com", body, headers);

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!(
                            "http::{}('https://codebreather.com', {}, {})",
                            $function_name,
                            "{ body: 'This is some awesome thinking!', id: 1, postId: 100 }",  "{ \"x-my-header\": 'some unique string' }" )
                    );
                }
            }
        }
    };
}

create_fn_with_3args_url_body_and_head!(
    /// The http::post function performs a remote HTTP POST request. The first parameter is the URL of the remote endpoint,
    /// and the second parameter is the value to use as the request body, which will be converted to JSON.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value,
    /// otherwise the response is treated as text.
    ///
    /// Also aliased as `http__post`.
    ///
    /// http::post(string, object) -> value
    /// If an object is given as the third argument, then this can be used to set the request headers.
    ///
    /// http::post(string, object, object) -> val
    ///
    /// # Arguments
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a param.
    /// * `body` - The value to use as the request body, which will be converted to JSON. Can be a object or a field or a param.
    /// * `headers` - Optional. If an object is given as the third argument, then this can be used to set the request headers. Can be a object or a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::http};
    /// use std::collections::hash_map::HashMap;
    ///
    /// http::post!("https://codebreather.com", HashMap::from([("id".to_string(), 1.into())]));
    ///
    /// http::post!("https://codebreather.com", HashMap::from([("id".to_string(), 1.into())]), HashMap::from([("x-my-header".to_string(), "some unique string".into())]));
    ///
    /// # let body_field = Field::new("body_field");
    /// # let headers_param = Param::new("headers_param");
    /// http::post!("https://codebreather.com", body_field, headers_param);
    /// ```
=> "post"
);

create_fn_with_3args_url_body_and_head!(
    /// The http::put function performs a remote HTTP PUT request. The first parameter is the URL of the remote endpoint,
    /// and the second parameter is the value to use as the request body, which will be converted to JSON.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value,
    /// otherwise the response is treated as text.
    ///
    /// Also aliased as `http__put`.
    ///
    /// http::put(string, object) -> value
    /// If an object is given as the third argument, then this can be used to set the request headers.
    ///
    /// http::put(string, object, object) -> value
    ///
    /// # Arguments
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a param.
    /// * `body` - The value to use as the request body, which will be converted to JSON. Can be a object or a field or a param.
    /// * `headers` - Optional. If an object is given as the third argument, then this can be used to set the request headers. Can be a object or a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::http};
    /// use std::collections::hash_map::HashMap;
    ///
    /// http::put!("https://codebreather.com", HashMap::from([("id".to_string(), 1.into())]));
    ///
    /// http::put!("https://codebreather.com", HashMap::from([("id".to_string(), 1.into())]), HashMap::from([("x-my-header".to_string(), "some unique string".into())]));
    /// ```
=> "put"
);

create_fn_with_3args_url_body_and_head!(
    /// The http::patch function performs a remote HTTP PATCH request. The first parameter is the URL of the remote endpoint,
    /// and the second parameter is the value to use as the request body, which will be converted to JSON.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value,
    /// otherwise the response is treated as text.
    ///
    /// Also aliased as `http_patch`.
    ///
    /// http::patch(string, object) -> value
    /// If an object is given as the third argument, then this can be used to set the request headers.
    ///
    /// http::patch(string, object, object) -> value
    ///
    /// # Arguments
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a param.
    /// * `body` - The value to use as the request body, which will be converted to JSON. Can be a object or a field or a param.
    /// * `headers` - Optional. If an object is given as the third argument, then this can be used to set the request headers. Can be a object or a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::http};
    /// use std::collections::hash_map::HashMap;
    ///
    /// let result = http::patch!("https://codebreather.com", HashMap::from([("id".to_string(), 1.into())]));
    ///
    /// let result = http::patch!("https://codebreather.com", HashMap::from([("id".to_string(), 1.into())]), HashMap::from([("x-my-header".to_string(), "some unique string".into())]));
    /// ```
=> "patch"
);
