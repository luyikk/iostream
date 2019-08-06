extern crate core;

use std::error::Error;
use std::fmt;
use ::StreamErrorKind::{OnStrError, End, StreamBad};


pub mod io;

#[derive(Debug)]
pub struct StreamError{
    err_type:StreamErrorKind
}


impl StreamError{

    pub fn new_bad() ->StreamError{
        StreamError{ err_type:StreamBad}
    }
    pub fn new_end() ->StreamError{
        StreamError{ err_type:End}
    }
    pub fn new_str(msg:&str) ->StreamError{
        StreamError{ err_type:OnStrError(msg.to_string())}
    }
    pub fn from_str(msg:&str) ->Result<(),StreamError>{
        Err(StreamError{ err_type:OnStrError(msg.to_string())})
    }

    pub fn end() ->Result<(),StreamError>{
        Err(StreamError{ err_type:End})
    }
}


impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.err_type.fmt(f)
    }
}


impl Error for StreamError{
    fn description(&self) -> &str {
        self.err_type.description()
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(&self.err_type)
    }
}

#[derive(Debug)]
pub enum StreamErrorKind{
    OnStrError(String),
    End,
    StreamBad
}

impl fmt::Display for StreamErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            StreamErrorKind::OnStrError(msg)=> write!(f,"{}", msg),
            StreamErrorKind::End=>write!(f,"stream postion is end"),
            StreamErrorKind::StreamBad=>write!(f,"stream is bad")
        }
    }
}

impl Error for StreamErrorKind{
    fn description(&self) -> &str {
        match &self {
            StreamErrorKind::OnStrError(msg)=> msg,
            StreamErrorKind::End=>"stream postion is end",
            StreamErrorKind::StreamBad=>"stream is bad"
        }
    }
}