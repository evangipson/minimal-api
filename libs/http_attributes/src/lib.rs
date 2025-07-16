//! # [`http_attributes`](crate)
//! The [`http_attributes`](crate) crate contains macros to make it easy to generate a
//! [`Route`](http::route::Route) just by adding one of the attributes it provides.
//!
//! It provides helpful `#[proc_macro_attribute]` macro attributes to enable functions
//! to return a [`Route`](http::route::Route) for various request types:
//! - [`http_get`](macro@http_get)
//! - [`http_post`](macro@http_post)
//! - [`http_put`](macro@http_put)
//! - [`http_delete`](macro@http_delete)
//!
//! It also provides versions of all the previous macro attributes that allow full
//! control of the response shape:
//! - [`http_raw_get`](macro@http_raw_get)
//! - [`http_raw_post`](macro@http_raw_post)
//! - [`http_raw_put`](macro@http_raw_put)
//! - [`http_raw_delete`](macro@http_raw_delete)

/// [`http_methods`] contains [`str`] representations of all supported HTTP methods.
pub(crate) mod http_methods;

/// [`transform`] contains all functionality related to modifying [`proc_macro::TokenStream`] input.
pub(crate) mod transform;

#[doc = r#"
# http_get
The [`http_get`](macro@http_get) attribute modifies the function that uses it inline
to return a `GET` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will ensure the [`Response`](http::response::Response) returns the
matched [`Route`](http::route::Route) with the following shape:
```json
{
    "content": "...",
    "status": "...",
    "time": "...",
    "header": "...",
}
```

To get a `GET` response that is only what would be returned in "content", use the
[`http_raw_get`](macro@http_raw_get) attribute instead.

# Examples
## Basic `GET` route
[`http_get`](macro@http_get) can be used to create a basic `GET` route which returns
a simple [`String`]:
```rust
use http_attributes::http_get;

// this route listens for a GET request on the "/" path, and
// returns "hello".
#[http_get("/")]
fn some_request() -> String {
    "hello".to_string()
}
```

## `GET` route with query parameters
[`http_get`](macro@http_get) can also be used to get values from query parameters,
and perform actions on them before returning a [`String`] result:
```rust
use http_attributes::http_get;

// this will listen for a "number" query parameter, such as
// "/squared?number=2", which would return "4".
#[http_get("/squared")]
fn get_squared_query_parameter(number: i32) -> String {
    let squared_result = number * number;
    squared_result.to_string()
}
```
"#]
#[proc_macro_attribute]
pub fn http_get(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::GET, false)
}

#[doc = r#"
# http_raw_get
The [`http_raw_get`](macro@http_raw_get) attribute modifies the function that uses it inline
to return a `GET` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will always return a [`Response`](http::response::Response) with the shape of
the [`Route`](http::route::Route) result.

To get a `GET` response that has more information, like "status" and "time", use the
[`http_get`](macro@http_get) attribute instead.

# Examples
## Basic `GET` route
[`http_raw_get`](macro@http_raw_get) can be used to create a basic `GET` route which returns
a simple [`String`]:
```rust
use http_attributes::http_raw_get;

// this route listens for a GET request on the "/" path, and
// returns "hello".
#[http_raw_get("/")]
fn some_request() -> String {
    "hello".to_string()
}
```

## `GET` route with query parameters
[`http_raw_get`](macro@http_raw_get) can also be used to get values from query parameters,
and perform actions on them before returning a [`String`] result:
```rust
use http_attributes::http_raw_get;

// this will listen for a "number" query parameter, such as
// "/squared?number=2", which would return "4".
#[http_raw_get("/squared")]
fn get_squared_query_parameter(number: i32) -> String {
    let squared_result = number * number;
    squared_result.to_string()
}
```
"#]
#[proc_macro_attribute]
pub fn http_raw_get(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::GET, true)
}

#[doc = r#"
# http_post
The [`http_post`](macro@http_post) attribute modifies the function that uses it inline
to return a `POST` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will ensure the [`Response`](http::response::Response) returns the
matched [`Route`](http::route::Route) with the following shape:
```json
{
    "content": "...",
    "status": "...",
    "time": "...",
    "header": "...",
}
```

To get a `POST` response that is only what would be returned in "content", use the
[`http_raw_post`](macro@http_raw_post) attribute instead.

# Examples
## Basic `POST` route
[`http_post`](macro@http_post) can be used to create a basic `POST` route which returns
the content that was sent as the body of the `POST`:
```rust
use http_attributes::http_post;

// this route listens for a POST request on the "/" path,
// and returns the POST body.
#[http_post("/")]
fn some_request(content: String) -> String {
    format!("received {content} from POST!")
}
```
"#]
#[proc_macro_attribute]
pub fn http_post(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::POST, false)
}

#[doc = r#"
# http_raw_post
The [`http_raw_post`](macro@http_raw_post) attribute modifies the function that uses it inline
to return a `POST` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will always return a [`Response`](http::response::Response) with the shape of
the [`Route`](http::route::Route) result.

To get a `POST` response that has more information, like "status" and "time", use the
[`http_post`](macro@http_post) attribute instead.

# Examples
## Basic `POST` route
[`http_raw_post`](macro@http_raw_post) can be used to create a basic `POST` route which returns
the content that was sent as the body of the `POST`:
```rust
use http_attributes::http_raw_post;

// this route listens for a POST request on the "/" path, and
// returns the POST body.
#[http_raw_post("/")]
fn some_request(content: String) -> String {
    format!("received {content} from POST!")
}
```
"#]
#[proc_macro_attribute]
pub fn http_raw_post(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::POST, true)
}

#[doc = r#"
# http_put
The [`http_put`](macro@http_put) attribute modifies the function that uses it inline
to return a `PUT` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will ensure the [`Response`](http::response::Response) returns the
matched [`Route`](http::route::Route) with the following shape:
```json
{
    "content": "...",
    "status": "...",
    "time": "...",
    "header": "...",
}
```

To get a `PUT` response that is only what would be returned in "content", use the
[`http_raw_put`](macro@http_raw_put) attribute instead.

# Examples
## Basic `PUT` route
[`http_put`](macro@http_put) can be used to create a basic `PUT` route which returns
the content that was sent as the body of the `PUT`:
```rust
use http_attributes::http_put;

// this route listens for a PUT request on the "/" path, and
// returns the PUT content.
#[http_put("/")]
fn some_request(content: String) -> String {
    format!("received {content} from PUT!")
}
```
"#]
#[proc_macro_attribute]
pub fn http_put(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::PUT, false)
}

#[doc = r#"
# http_raw_put
The [`http_raw_put`](macro@http_raw_post) attribute modifies the function that uses it inline
to return a `PUT` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will always return a [`Response`](http::response::Response) with the shape of
the [`Route`](http::route::Route) result.

To get a `PUT` response that has more information, like "status" and "time", use the
[`http_put`](macro@http_put) attribute instead.

# Examples
## Basic `PUT` route
[`http_raw_put`](macro@http_raw_put) can be used to create a basic `PUT` route which returns
the content that was sent as the body of the `PUT`:
```rust
use http_attributes::http_raw_put;

// this route listens for a PUT request on the "/" path, and
// returns the PUT content.
#[http_raw_put("/")]
fn some_request(content: String) -> String {
    format!("received {content} from PUT!")
}
```
"#]
#[proc_macro_attribute]
pub fn http_raw_put(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::PUT, true)
}

#[doc = r#"
# http_delete
The [`http_delete`](macro@http_delete) attribute modifies the function that uses it inline
to return a `DELETE` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will ensure the [`Response`](http::response::Response) returns the
matched [`Route`](http::route::Route) with the following shape:
```json
{
    "content": "...",
    "status": "...",
    "time": "...",
    "header": "...",
}
```

To get a `DELETE` response that is only what would be returned in "content", use the
[`http_raw_delete`](macro@http_raw_delete) attribute instead.

# Examples
## Basic `DELETE` route
[`http_delete`](macro@http_delete) can be used to create a basic `DELETE` route which returns
returns a query parameter value that denotes what to `DELETE`:
```rust
use http_attributes::http_delete;

// this route listens for a DELETE request on the "/remove"
// path, with "id" as a query parameter.
#[http_delete("/remove")]
fn some_request(id: String) -> String {
    format!("received {id} from DELETE!")
}
```
"#]
#[proc_macro_attribute]
pub fn http_delete(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::DELETE, false)
}

#[doc = r#"
# http_raw_delete
The [`http_raw_delete`](macro@http_raw_delete) attribute modifies the function that uses it inline
to return a `DELETE` [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will always return a [`Response`](http::response::Response) with the shape of
the [`Route`](http::route::Route) result.

To get a `DELETE` response that has more information, like "status" and "time", use the
[`http_delete`](macro@http_delete) attribute instead.

# Examples
## Basic `DELETE` route
[`http_raw_delete`](macro@http_raw_delete) can be used to create a basic `DELETE` route which
returns a query parameter value that denotes what to `DELETE`:
```rust
use http_attributes::http_raw_delete;

// this route listens for a DELETE request on the "/remove"
// path, with "id" as a query parameter.
#[http_raw_delete("/remove")]
fn some_request(id: String) -> String {
    format!("received {id} from DELETE!")
}
```
"#]
#[proc_macro_attribute]
pub fn http_raw_delete(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path_lit = syn::parse_macro_input!(attr as syn::LitStr);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    transform::function_to_route(path_lit, input_fn, http_methods::DELETE, true)
}
