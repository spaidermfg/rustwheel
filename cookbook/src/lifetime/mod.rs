//! lifetime,ownership，borrow
//! Rust中基本类型实现了Copy trait，所以可以被隐式借用
//! 基本类型都具有复制语义，其他类型都具有移动语义
//! 当值还没有被借用时，重新绑定该值是合法的
//! # 解决所有权的方法
//! * 在不需要完整所有权的地方，使用引用(&T, &mut T)
//! * 减少生命周期长的值
//! * 在需要完整所有权的地方，复制长期存活的值
//! # 所有权移动的方法
//! * 通过赋值
//! * 通过函数传递
//! # Copy和Clone的区别
//! * Copy是隐式起作用的，当所有权移动时，就会被复制。速度快消耗小。
//! * Clone是显式发生作用的，需要显式调用clone()方法。速度慢消耗大。
//! * 实现Copy的前提是实现Clone
//! # 包装器类型
//! * std::rc::Rc
//! * Rc<T> 表示一个类型为T的引用计数的值，提供T的共享式所有权，能够防止T从内存中被删除，直到所有的所有者被删除
//! * Rc<T> 实现了Clone，clone计数器自增，Drop计数器自减
//! * Rc<T> 不支持修改，是不可变的。
//! * 若要修改，则需要再包装一层，Rc<RefCell<T>>
#![allow(unused_variables)]


use std::rc::Rc;

pub fn life_time() {
    println!("{}", "---".repeat(12));
    let sat_a = CubeSat::new(1);
    let sat_b = CubeSat::new(2);
    let sat_c = CubeSat::new(3);

    // 检查卫星状态
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    println!("{:?} {:?} {:?}", sat_a, sat_b, sat_c);

    //再次检查卫星状态
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("{:?} {:?} {:?}", a_status, b_status, c_status);

    // 创建地面站
    let base = GroundStation::new();
    let mut mail = MailBox{messages: vec![]};

    // 向卫星发送信息
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        base.send(&mut mail, &mut sat, Message{to: sat_id, content: String::from("Hello cube")});
    }

    // 卫星接收信息
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("SAT: {:?} 的MESSAGE：{:?}", sat, msg)
    }

    // 使用引用计数器包装类型
    let ground = Rc::new(GroundStation {});
    println!("{:?}", ground);
}

// 人造卫星
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailBox: MailBox,
}

impl CubeSat {
fn new(sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailBox: MailBox { messages: vec![] },
        }
    }

    // 接收消息
    fn recv(&self, mail_box: &mut MailBox) -> Option<Message> {
        mail_box.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3,4]
}

// 地面站 用户和卫星的中介
#[derive(Debug)]
struct GroundStation;

impl GroundStation {
    fn new() -> GroundStation {
        GroundStation {}
    }

    // 发送消息
    fn send(&self, mail_box: &mut MailBox, to: &CubeSat, msg: Message) {
        mail_box.post(msg)
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailBox: MailBox {messages: vec![]}
        }
    }
}

// 邮箱
#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

impl MailBox {
    // 发送消息
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    // 查找消息
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg)
            }
        }
        None
    }
}

// 消息，地面站发送，卫星接受
#[derive(Debug)]
struct Message {
    to: u64,
    content: String
}

// 卫星状态消息
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

// 检查卫星状态
fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?} {:?}", sat_id, StatusMessage::Ok);
    sat_id
}
