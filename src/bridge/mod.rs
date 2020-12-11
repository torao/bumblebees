use std::sync::{Arc, RwLock};

use url::Url;

use crate::error::Error;
use crate::msg::Message;
use crate::Result;

pub mod io;
pub mod tcp;
pub mod ws;

#[cfg(test)]
mod test;

/// 非同期メッセージング API

pub trait Bridge {
  ///  指定されたリモートノードに対して非同期接続を行い `Wire` の Future を返します。
  fn new_wire<W: Wire>() -> Box<W>;

  /// 指定されたネットワークからの接続を非同期で受け付ける `Server` の Future を返します。
  fn new_server<S: Server>() -> Box<S>;
}

pub trait Wire {
  /// この Wire のローカル側アドレスを参照します。
  fn local_address() -> String;

  /// この Wire のリモート側アドレスを参照します。
  fn remote_address() -> String;

  /// こちらの端点が接続を受け付けた側である場合に true を返します。
  /// プロトコル上の役割を決める必要がある場合に使用することができます。
  fn is_server() -> bool;

  fn close() -> Result<()>;
}

pub trait Server {
  fn close() -> Result<()>;
}

pub fn create(url: &str) -> Result<()> {
  let url = Url::parse(url)?;
  match url.scheme() {
    "tcp" => {}
    _ => return Err(Error::UnsupportedProtocol { url: url.to_string() }),
  }
  Ok(())
}

pub struct MessageQueue {
  capacity: usize,
  queue: Arc<RwLock<Vec<Message>>>,
}

impl MessageQueue {
  /// 指定された容量を持つメッセージキューを構築します。
  pub fn new(capacity: usize) -> MessageQueue {
    MessageQueue { capacity, queue: Arc::new(RwLock::new(Vec::new())) }
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  pub fn len(&self) -> usize {
    let queue = self.queue.clone();
    let queue = queue.read().unwrap();
    queue.len()
  }

  /// このキューにメッセージを追加します。
  /// 正常に終了した場合、メッセージ追加後のキューのサイズを返します。
  pub fn push(&mut self, msg: Message) -> Result<usize> {
    let queue = self.queue.clone();
    let mut queue = queue.write()?;
    if queue.len() == self.capacity {
      Err(Error::MessageQueueOverflow { capacity: self.capacity })
    } else {
      queue.push(msg);
      Ok(queue.len())
    }
  }

  pub fn try_pop(&mut self) -> Result<Option<Message>> {
    unimplemented!()
  }
}