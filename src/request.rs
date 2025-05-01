use std::{collections::HashMap, time::Duration};

/// A struct that holds the request options.
///
/// Unlike the [`ImpitBuilder`](crate::impit::ImpitBuilder) struct, these options are specific to a single request.
///
/// Used by the [`Impit`](crate::impit::Impit) struct's methods.
#[derive(Debug, Clone)]
pub struct RequestOptions {
    /// A `HashMap` that holds custom HTTP headers. These are added to the default headers and should never overwrite them.
    pub headers: HashMap<String, String>,
    /// The timeout for the request. This option overrides the global [`Impit`] timeout.
    pub timeout: Option<Duration>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        RequestOptions {
            headers: HashMap::new(),
            timeout: None,
        }
    }
}
