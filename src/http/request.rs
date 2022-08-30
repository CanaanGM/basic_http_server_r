use super::method::Method;

pub struct  Request {
    path: String, 
    query_tring: Option<String>, // Option means it can be null or have a value of type(value<T>)
    method: Method
}