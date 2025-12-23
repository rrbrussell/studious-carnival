use std::env;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::Stdout;
use std::io::Write;

#[derive(PartialEq)]
enum RequestMethod {
    GET,
    POST,
//    HEAD,
//    PUT,
//    DELETE,
//    CONNECT,
//    OPTIONS,
//    TRACE,
//    PATCH,
}

fn main() -> io::Result<()> {
    let mut stdout: Stdout = io::stdout();
    let mut output_buffer = Vec::<u8>::with_capacity(16384);
    let method: RequestMethod;
    match env::var("REQUEST_METHOD") {
        Ok(val) => {
            if val.eq_ignore_ascii_case("GET") {
                method = RequestMethod::GET;
            } else {
                if val.eq_ignore_ascii_case("POST") {
                    method = RequestMethod::POST;
                } else {
                    print!("Status: 501 Not Implemented\r\n");
                    stdout.flush()?;
                    return Err(io::Error::new(io::ErrorKind::Unsupported,
                        "We can't handle that type of request."));
                }
            }
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
        }
    };

    process_request(method, &mut output_buffer)?;
    stdout.write_all(&output_buffer)?;
    stdout.flush()?;
    return Ok(());
}

// out is a Vec<u8> the Write trait implementations always return Ok().
fn process_request(_method: RequestMethod, out: &mut Vec<u8>) -> io::Result<()>
{
    _ = write!(out, "Status: 200 OK\r\n");
    _ = write!(out, "Content-Type: text/html;\r\n");
    _ = write!(out, "\r\n");
    _ = write!(out, "<html><body>");
    _ = write!(out, "<h1>Command Line Arguments</h1>");
    for argument in env::args() {
        _ = write!(out, "<li>{argument}</li>");
    }
    _ = write!(out, "<hr />");
    _ = write!(out, "<h1>Environment Variables<h1>");
    _ = write!(out, "<dl>");
    for (key, value) in env::vars() {
        _ = write!(out, "<dt>{key}</dt><dd>{value}</dd>");
    }
    _ = write!(out, "</dl>");
    _ = write!(out, "<hr />");
    match env::var("CONTENT_LENGTH") {
        Err(_) => {
            return Err(
                io::Error::new(io::ErrorKind::InvalidInput,
                    "We don't have a CONTENT_LENGTH meta variable.")
            );
        }
        Ok(val) => {
            let content_length: u64 = val.parse().unwrap_or(0);
            if content_length > 0 {
                _ = write!(out, "<pre>");
                let mut content = io::stdin().lock().take(content_length);
                let mut x: bool = true;
                while x {
                    let buf = content.fill_buf()?;
                    let len = buf.len();
                    if buf.len() > 0 {
                        _ = out.write(buf);
                        content.consume(len);
                    } else {
                        x = false;
                    }
                }
                _ = write!(out, "</pre>");
            }
        }
    };
    _ = write!(out, "</body></html>");
    return Ok(());
}
