use std::borrow::Cow;
use std::env;
use std::io::{Cursor, Error, Read};

use bytes::{Buf, Bytes, BytesMut};
use dxr::{
    DxrError, Fault, FaultResponse, MethodCall, MethodResponse, TryFromValue, TryToParams, Value,
};
use futures_util::StreamExt;
use futures_util::future::poll_fn;

use serde::Serialize;
use thiserror::Error;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter, ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::process::Command;
use tokio::signal;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};

/* #[derive(Debug)]
struct Handler {
    size: u32,
    //handler: u32,
    call: String,
} */

/* struct GbxHeader {
    size: u32,
    handler: u32,
    //bytes: String,
} */

/* #[derive(Debug)]
enum GbxFrame {
    /* Header {
        size: u32,
        handler: u32,
    },
    Body(BytesMut), */
    MethodCall {
        size: u32,
        handler: u32,
        body: String,
    },
    Callback {},
} */

#[derive(Debug)]
struct GbxPacket {
    size: u32,
    handler: u32,
    body: String,
}

impl GbxPacket {
    fn parse(buf: &mut Cursor<&[u8]>) -> Result<GbxPacket, ClientError> {
        //println!("current parse attempt {buf:?}");

        if buf.remaining() < 8 {
            return Err(ClientError::Incomplete);
        }
        let size = buf.get_u32_le() as usize;
        let handler = buf.get_u32_le();
        if buf.remaining() < size {
            return Err(ClientError::Incomplete);
        }

        let body = String::from_utf8_lossy(&buf.chunk()[..size]).into_owned();

        // Advance the buffer to body size. (Header Methods calls of u32 do this automatically)
        buf.advance(size);

        let size = size as u32;

        // Method calls and Callbacks operate above and below half a u32 respectively.
        /* if handler > 0x80000000u32 {
            Ok(GbxFrame::MethodCall {
                size,
                handler,
                body,
            })
        } else {
            Ok(GbxFrame::MethodCall {
                size,
                handler,
                body,
            })
        } */

        Ok(GbxPacket {
            size,
            handler,
            body,
        })
    }

    fn is_method_response(&self) -> bool {
        self.handler > 0x80000000u32
    }
}

#[derive(Debug)]
enum GbxMessage {
    MethodCall {
        message: String,
        responder: oneshot::Sender<MethodResponse>,
    },
    Callback {
        message: String,
    },
}
/*
/* impl GbxFrame {
    fn xml(&self) -> &str {
        match self {
            GbxFrame::Header { size, handler } => todo!(),
            GbxFrame::Body(bytes_mut) => todo!(),
            GbxFrame::MethodCall {
                size,
                handler,
                body,
            } => body,
        }
    } */

    fn parse(buf: &mut Cursor<&[u8]>) -> Result<GbxPacket, ClientError> {
        if buf.remaining() < 8 {
            return Err(ClientError::Incomplete);
        }
        let size = buf.get_u32_le();
        let handler = buf.get_u32_le();
        if buf.remaining() < size as usize {
            return Err(ClientError::Incomplete);
        }

        let body = String::from_utf8_lossy(&buf.chunk()[..(size as usize)]).into_owned();

        // Method calls and Callbacks operate above and below half a u32 respectively.
        /* if handler > 0x80000000u32 {
            Ok(GbxFrame::MethodCall {
                size,
                handler,
                body,
            })
        } else {
            Ok(GbxFrame::MethodCall {
                size,
                handler,
                body,
            })
        } */

        Ok(GbxPacket {
            size,
            handler,
            body,
        })
    }
} */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /* let url = Url::parse("http://localhost:5000/").unwrap();
    let client: Client = ClientBuilder::new(url).build();

    /* reqwest::get("http://localhost:5000/")
    .await
    .inspect(|v| println!("{v:?}")); */

    let result: Result<bool, dxr_client::ClientError> = client.call("EnableCallbacks", true).await;

    println!("{result:?}");

    let result: Result<bool, dxr_client::ClientError> =
        client.call("SetApiVersion", "2023-04-24").await;

    println!("{result:?}");

    let result: Result<String, dxr_client::ClientError> = client
        .call("Authenticate", ("SuperAdmin", "SuperAdmin"))
        .await;

    println!("{result:?}");

    let result: Result<String, dxr_client::ClientError> =
        client.call("system.listMethods", ()).await;

    println!("{result:?}");

    let result: Result<String, dxr_client::ClientError> = client.call("GetStatus", ()).await;

    println!("{result:?}"); */

    // Parse our URL...
    /* let url = "localhost:5000";

    // Open a TCP connection to the remote host
    let mut stream = TcpStream::connect(url).await?;
    println!("huh");

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.

    let ret = stream.write(&[0, 0, 0, 0, 0, 0, 0, 0]).await;
    let yey = stream.flush().await;
    let mut msg = vec![0; 1024];

    let read = stream.read(&mut msg).await;
    println!("{ret:?} {read:?}");

    //let handler: Handler = msg[0..4];
    let size = u32::from_le_bytes([msg[0], msg[1], msg[2], msg[3]]);
    //let handler = u32::from_le_bytes([msg[4], msg[5], msg[6], msg[7]]);
    let call = String::from_utf8(msg[4..((size + 4) as usize)].to_vec()).unwrap();
    let handler = Handler {
        size,
        // handler,
        call,
    };

    println!("{handler:?}"); */
    //let result: Result<String, ClientError> = server.call("system.listMethods", ()).await;

    let mut server = ServerClient::new("127.0.0.1:5001").await;

    let _: Result<bool, ClientError> = server.call("SetApiVersion", "2023-03-24").await;

    let _: Result<bool, ClientError> = server
        .call("Authenticate", ("SuperAdmin", "SuperAdmin"))
        .await;

    let _: Result<bool, ClientError> = server.call("EnableCallbacks", true).await;

    //let _: Result<bool, ClientError> = server.call("GetModeScriptInfo", ()).await;

    let _: Result<bool, ClientError> = server
        .call(
            "TriggerModeScriptEventArray",
            ("XmlRpc.EnableCallbacks", ["true"]),
        )
        .await;

    let _: Result<bool, ClientError> = server
        .call("ChatSendServerMessage", "Hey from Rust owo")
        .await;
    //println!("{result:?}");

    server.subscribe("ManiaPlanet.PlayerInfoChanged", |ctx| {
        println!("WOW: {ctx:?}")
    });

    //let working: Result<bool, ClientError> = server.call("EnableCallbacks", true).await;

    //println!("{working:?}");
    // Spawn a task to poll the connection, driving the HTTP state
    /* tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    println!("huhh");

    sender.send_request() */

    /* // The authority of our URL will be the hostname of the httpbin remote
    let authority = url.authority().unwrap().clone();

    // Create an HTTP request with an empty body and a HOST header
    let req = Request::builder()
        .uri(url)
        .method("POST")
        .header(hyper::header::HOST, authority.as_str())
        .body(Full::new(Bytes::from(Vec::<u8>::new())))?; */

    println!("huhhh");
    //.header(hyper::header::HOST, authority.as_str())
    // .body(Empty::<Bytes>::new())?;
    //.body()?;

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("CTRL + C: Closing the application 👋");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    // Await the response...
    /* let mut res = sender.send_request(req).await?;

    println!("Response status: {}", res.status());

    // Stream the body, writing each frame to stdout as it arrives
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(chunk).await?;
        }
    } */

    /*  let address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:5000".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop, so we pass in a handle
    // to our event loop. After the socket's created we inform that we're ready
    // to go and start accepting connections.
    //let listener = TcpListener::bind(&addr).await?;
    let stream = TcpStream::connect(address).await?;

    println!("Listening on: {address}");

    loop {
        // Asynchronously wait for an inbound socket.
        let (socket, _) = stream.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.
        tokio::spawn(async move {
            // We're parsing each socket with the `BytesCodec` included in `tokio::codec`.
            let mut framed = BytesCodec::new().framed(socket);
            // We loop while there are messages coming from the Stream `framed`.
            // The stream will return None once the client disconnects.
            while let Some(message) = framed.next().await {
                match message {
                    Ok(bytes) => println!("bytes: {bytes:?}"),
                    Err(err) => println!("Socket closed with error: {err:?}"),
                }
            }
            println!("Socket received FIN packet and closed connection");
        });
    } */

    Ok(())
}

struct ServerPool {
    servers: Vec<ServerClient>,
}

struct ServerClient {
    // url: String,
    //reader: ReadHalf<BufWriter<TcpStream>>,
    //writer: WriteHalf<BufWriter<TcpStream>>,
    sender: Sender<GbxMessage>,
    //buffer: BytesMut,
    //handler: u32,
}

impl ServerClient {
    pub async fn new(url: impl Into<String>) -> Self {
        let stream = BufWriter::new(TcpStream::connect(url.into()).await.unwrap());

        let (mut reader, mut writer) = io::split(stream);

        //l//et ret = stream.write(&[]).await;
        //_ = stream.flush().await;
        let mut buf = vec![0; 15];

        let read = reader.read(&mut buf).await;
        println!("ff{read:?}");

        //let handler: Handler = msg[0..4];
        let size = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        //let handler = u32::from_le_bytes([msg[4], msg[5], msg[6], msg[7]]);
        let call = String::from_utf8(buf[4..((size + 4) as usize)].to_vec()).unwrap();

        println!("Size: {size}, Content: {call}");
        /* let handler = Handler {
        size,
        // handler,
        call,
        }; */

        let (sender, mut rx) = mpsc::channel::<GbxMessage>(32);

        let client = Self {
            //reader,
            // writer,
            sender,
            //handler: 0x80000000,
            //buffer: BytesMut::with_capacity(1024),
        };

        let _write_manager = tokio::spawn(async move {
            let mut handler = 0x80000000u32;

            // Start receiving messages
            while let Some(cmd) = rx.recv().await {
                println!("{cmd:?}");

                match cmd {
                    GbxMessage::MethodCall { message, responder } => {
                        //TODO actually make the request
                        /* let written = self
                        .write_frame(&GbxFrame::MethodCall {
                            size: message.len() as u32,
                            handler: self.handler,
                            body: message,
                        })
                        .await; */

                        // Increment the handler before each method call
                        handler += 1;

                        /* match frame {
                        GbxFrame::MethodCall {
                            size,
                            handler,
                            body,
                        } => { */
                        writer.write_u32_le(message.len() as u32).await.unwrap();
                        writer.write_u32_le(handler).await.unwrap();
                        writer.write_all(message.as_bytes()).await.unwrap();

                        //println!("writer bytes: {:?}", self.writer.buffer())
                        /*   }
                            GbxFrame::Body(bytes_mut) => todo!(),
                            GbxFrame::Header { size, handler } => todo!(),
                        } */

                        let flushed = writer.flush().await;

                        //TODO handle the response
                        let response = MethodResponse::new(Value::boolean(false));
                        let _ = responder.send(response);
                    }
                    GbxMessage::Callback { message } => todo!(),
                }
                /* match cmd {
                    Get { key } => {
                        client.get(&key).await;
                    }
                    Set { key, val } => {
                        client.set(&key, val).await;
                    }
                } */
            }
        });

        let _read_manager = tokio::spawn(async move {
            // Establish a connection to the server

            let mut buffer: BytesMut = BytesMut::with_capacity(1024);

            fn parse_frame(buffer: &mut BytesMut) -> Option<GbxPacket> {
                //println!("parsing");
                let mut buf = Cursor::new(&buffer[..]);

                //TODO make a msg or anything out of this.
                if let Ok(packet) = GbxPacket::parse(&mut buf) {
                    buffer.advance(buf.position() as usize);
                    //println!("has parsed");
                    /* if packet.is_method_response() {
                    } else {
                        if packet.get_event() == ""
                    } */

                    // Return the frame to the caller.
                    Some(packet)
                } else {
                    None
                }
            }

            loop {
                while let Some(frame) = parse_frame(&mut buffer) {
                    println!("Frame: {frame:?}");
                }
                println!("Buffy: {buffer:?}");

                if 0 == reader.read_buf(&mut buffer).await.unwrap() {
                    // The remote closed the connection. For this to be a clean
                    // shutdown, there should be no data in the read buffer. If
                    // there is, this means that the peer closed the socket while
                    // sending a frame.
                    if buffer.is_empty() {
                        println!("Empty Buffy");
                        continue;
                    } else {
                        panic!("connection reset by peer");
                    }
                }
            }
        });
        client
    }

    pub async fn call<P: TryToParams, R: TryFromValue>(
        &mut self,
        method: &str,
        args: P,
    ) -> Result<R, ClientError> {
        let params = args.try_to_params()?;
        let result = self.call_inner(Cow::Borrowed(method), params).await?;

        // extract return value
        Ok(R::try_from_value(&result)?)
    }

    pub fn subscribe(&mut self, event: &str, then: impl Fn(MethodResponse)) {
        //self.subscriptions
    }

    async fn call_inner(
        &mut self,
        method: Cow<'_, str>,
        params: Vec<Value>,
    ) -> Result<Value, ClientError> {
        // serialize XML-RPC method call
        let request = MethodCall::new(method, params);

        let xml = dxr::serialize_xml(&request)
            .map_err(|error| DxrError::invalid_data(error.to_string()))?;
        let body = [r#"<?xml version="1.0"?>"#, xml.as_str()].join("");
        //println!("{body}");

        /*  // construct request and send to server
        let head_size = self.stream.write(&(body.len() as u32).to_le_bytes()).await;
        println!("Head Size {head_size:?}");

        let head_handler = self.stream.write(&(0x80000001u32).to_le_bytes()).await;
        println!("Head handler: {head_handler:?}");

        let bytes_body = self.stream.write(body.as_bytes()).await;
        println!("Bytes Body: {bytes_body:?}");

        let flushed = self.stream.flush().await;
        println!("Flushed: {flushed:?}");

        let mut buf = vec![0; 1024]; */

        //self.handler += 1;

        /* let written = self
            .write_frame(&GbxFrame::MethodCall {
                size: body.len() as u32,
                handler: self.handler,
                body,
            })
            .await;

        println!("Written {written:?}"); */

        let local_sender = self.sender.clone();

        let response = tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();
            local_sender
                .send(GbxMessage::MethodCall {
                    message: body,
                    responder: resp_tx,
                })
                .await
                .unwrap();

            let res = resp_rx.await;
            //println!("GOT = {:?}", res);
            res
        })
        .await;

        //let read = self.read_frame().await; //self.stream.read(&mut buf).await;
        //println!("Read {read:?}");
        //println!("{buf:?}");
        /* let size = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let handler = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let call = String::from_utf8(buf[8..((size + 4) as usize)].to_vec()).unwrap(); */

        // deserialize XML-RPC method response
        //let result = response_to_result(read.unwrap().unwrap().xml())?;
        let result = MethodResponse::new(Value::boolean(false));

        //println!("{result:?}");

        Ok(result.inner())
    }

    /*  pub async fn read_frame(&mut self) -> tokio::io::Result<Option<GbxFrame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            /* if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    //return Err("connection reset by peer".into());
                    panic!()
                }
            } */
            /* if 0 == self.reader.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(panic!("connection reset by peer"));
                }
            } */
        }
    } */

    /* fn parse_frame(&mut self) -> tokio::io::Result<Option<GbxFrame>> {
           // Create the `T: Buf` type.
           let mut buf = Cursor::new(&self.buffer[..]);

           println!("{:?}", self.buffer);

           // Check whether a full frame is available
           //match GbxFrame::check(&mut buf) {
           // Ok(_) => {
           // Get the byte length of the frame
           // let len = buf.position() as usize;

           // Reset the internal cursor for the
           // call to `parse`.
           //buf.set_position(0);

           // Parse the frame
           if let Ok(frame) = GbxFrame::parse(&mut buf) {
               // Discard the frame from the buffer
               //self.buffer.advance(len);

               // Return the frame to the caller.
               Ok(Some(frame))
           } else {
               Ok(None)
           }
           //}
           // Not enough data has been buffered
           //Err(Incomplete) => Ok(None),
           // An error was encountered
           // Err(e) => Err(e.into()),
           // }
       }
    */
    // Write a frame to the connection.
    /* pub async fn write_frame(&mut self, frame: &GbxFrame) -> io::Result<()> {
        match frame {
            GbxFrame::MethodCall {
                size,
                handler,
                body,
            } => {
                self.writer.write_u32_le(*size).await?;
                self.writer.write_u32_le(*handler).await?;
                self.writer.write_all(body.as_bytes()).await?;

                //println!("writer bytes: {:?}", self.writer.buffer())
            }
            GbxFrame::Body(bytes_mut) => todo!(),
            GbxFrame::Header { size, handler } => todo!(),
        }

        let flushed = self.writer.flush().await;
        println!("Flushed {flushed:?}");
        Ok(())
    } */
}

fn response_to_result(contents: &str) -> Result<MethodResponse, ClientError> {
    // need to check for FaultResponse first:
    // - a missing <params> tag is ambiguous (can be either an empty response, or a fault response)
    // - a present <fault> tag is unambiguous
    let error2 = match dxr::deserialize_xml(contents) {
        Ok(fault) => {
            let response: FaultResponse = fault;
            return match Fault::try_from(response) {
                // server fault: return Fault
                Ok(fault) => Err(fault.into()),
                // malformed server fault: return DxrError
                Err(error) => Err(error.into()),
            };
        }
        Err(error) => error.to_string(),
    };

    let error1 = match dxr::deserialize_xml(contents) {
        Ok(response) => return Ok(response),
        Err(error) => error.to_string(),
    };

    // log errors if the contents could not be deserialized as either response or fault
    //log::debug!("Failed to deserialize response as either value or fault.");
    // log::debug!("Response failed with: {error1}; Fault failed with: {error2}");

    // malformed response: return DxrError::InvalidData
    Err(DxrError::invalid_data(contents.to_owned()).into())
}

#[derive(Debug, Error)]
pub enum ClientError {
    /// Error variant for XML-RPC server faults.
    #[error("{}", fault)]
    Fault {
        /// Fault returned by the server.
        #[from]
        fault: Fault,
    },
    /// Error variant for XML-RPC errors.
    #[error("{}", error)]
    RPC {
        /// XML-RPC parsing error.
        #[from]
        error: DxrError,
    },
    #[error("request incomplete")]
    Incomplete,
}

#[allow(unused)]
impl ClientError {
    fn fault(fault: Fault) -> Self {
        ClientError::Fault { fault }
    }

    fn rpc(error: DxrError) -> Self {
        ClientError::RPC { error }
    }
}

//trait
