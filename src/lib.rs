use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    /// GET Request
    GET,
    /// HEAD Request
    HEAD,
    /// POST Request
    POST,
    /// PUT Request
    PUT,
    /// DELETE Request
    DELETE,
    /// CONNECT Request
    CONNECT,
    /// OPTIONS Request
    OPTIONS,
    /// TRACE Request
    TRACE,
    /// PATCH Request
    PATCH,
}