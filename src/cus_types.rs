//! rust 中的自定义类型主要有结构体和枚举。
//!
//! rust 中有三种类型的结构体：
//!  - 元组结构体 (tuple struct), 其实就是具名元组。
//!  - C 风格结构体。
//!  - 单元结构体 (unit struct), 没有任何字段，在泛型中很有用。
//!
//! 使用 `enum` 关键字创建一个从不同取值选择其一的枚举类型。任何在
//! 结构体中合法的取值在枚举中也合法。
pub fn run() {
    println!("=========== Structures and Enums ===========");
    let pair = Pair(1, 0.1);
    println!("pair contains: {:?} and {:?}", pair.0, pair.1);

    let top_left = Point::new(10.0, 10.0);
    let bottom_right = Point::new(60.0, 110.0);
    let rect = Rectangle::new(top_left, bottom_right);
    println!(
        "{:?} with is {}, height is {} and area is {}",
        rect,
        rect.width(),
        rect.height(),
        rect.area()
    );

    let top_left = Point::new(10.0, 10.0);
    let square = Rectangle::square(top_left, 50.0);
    println!(
        "{:?} with is {}, height is {} and area is {}",
        square,
        square.width(),
        square.height(),
        square.area()
    );

    let pressed = WebEvent::KeyPress('Q');
    inspect(pressed);

    println!("zero is {}", Number::Zero as i32);

    let list = List::from(1);
    let list = list.prepend(2);
    let list = list.prepend(3);
    println!("{}", list);
}

/// A unit struct which are filed-less.
///
/// # Example
///
/// ```
/// let _unit = Unit;
/// ```
#[allow(dead_code)]
pub struct Unit;

/// A tuple struct.
///
/// # Example
///
/// ```
/// let pair = Pair(1, 0.1);
/// println!("pair contains: {:?} and {:?}", pair.0, pair.1);
/// ```
/// **Destructure a tuple struct**:
/// ```
/// let Pair(interger, decimal) = pair;
/// ```
#[allow(dead_code)]
pub struct Pair<T1, T2>(T1, T2);

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

/// A structure represents Rectangle, which can be specified by where the top left
/// and bottom right corners are is space.
#[derive(Debug)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    /// Create a Rectangle with top left point and bottom right point.
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    /// Create a Square whit top left point and it's width.
    pub fn square(top_left: Point, width: f32) -> Self {
        let bottom_right = Point::new(top_left.x + width, top_left.y + width);
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    /// Gets width of a Rectangle.
    pub fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    /// Gets height of a Rectangle.
    pub fn height(&self) -> f32 {
        self.bottom_right.y - self.top_left.y
    }

    /// Gets the area of a Rectangle.
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }
}

/// 枚举可以是单元结构 (unit), 或者一个元组结构，或者一个普通的结构体。
pub enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: f32, y: f32 },
}

pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded."),
        WebEvent::PageUnload => println!("page unloaded."),
        WebEvent::KeyPress(c) => println!("pressed \"{}\".", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at ({}, {})", x, y),
    }
}

/// rust 中的枚举可以像 C 中的枚举那样使用
///
/// # Example
/// ```
/// println!("zero is {}", Number::Zero as i32);
/// ```
pub enum Number {
    Zero,
    One,
    Two,
}

/// 使用 `enum` 实现的链表.
pub enum List {
    /// 链表的节点，包含一个元素和指向下一个节点的指针.
    Cons(u32, Box<List>),
    /// 代表链表结束的节点.
    Nil,
}

impl List {
    /// 创建一个空节点的链表.
    pub fn new() -> Self {
        Self::Nil
    }

    /// 通过一个值创建包含该值的节点，并返回链表.
    pub fn from(ele: u32) -> Self {
        Self::Cons(ele, Box::new(List::new()))
    }

    // /// 在当前链表后插入一个元素，并返回新的链表.
    // pub fn append(self, ele: u32) -> List {
    //     match self {
    //         Self::Cons(e, ref next) => {
    //             let new = next.append(ele);
    //             println!("append new: {}", new);
    //             Self::Cons(e, Box::new(new))
    //         }
    //         Self::Nil => Self::from(ele),
    //     }
    // }

    /// 在头部插入新元素，并返回新的链表.
    pub fn prepend(self, ele: u32) -> List {
        Self::Cons(ele, Box::new(self))
    }

    /// 返回链表的长度.
    pub fn len(&self) -> u32 {
        match *self {
            Self::Cons(_, ref next) => 1 + next.len(),
            Self::Nil => 0,
        }
    }

    /// 返回链表的元素.
    pub fn ele(&self) -> Option<u32> {
        match *self {
            Self::Cons(ele, _) => Some(ele),
            Self::Nil => None,
        }
    }

    /// 返回链表中的所有元素.
    pub fn arr(&self) -> Vec<u32> {
        match *self {
            Self::Cons(e, ref next) => [vec![e], next.arr()].concat(),
            Self::Nil => vec![],
        }
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Self::Cons(e, ref next) => {
                write!(f, "{} -> {}", e, next)
            }
            Self::Nil => write!(f, "Nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_of_rectangle() {
        let top_left = Point::new(10.0, 10.0);
        let bottom_right = Point::new(40.0, 40.0);
        let rect = Rectangle::new(top_left, bottom_right);
        assert_eq!(rect.width(), 30.0);
    }

    #[test]
    fn height_of_rectangle() {
        let top_left = Point::new(10.0, 0.0);
        let bottom_right = Point::new(40.0, 50.0);
        let rect = Rectangle::new(top_left, bottom_right);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn area_of_rectangle() {
        let top_left = Point::new(10.0, 0.0);
        let bottom_right = Point::new(40.0, 50.0);
        let rect = Rectangle::new(top_left, bottom_right);
        assert_eq!(rect.area(), 1_500.0);
    }

    #[test]
    fn is_square() {
        let top_left = Point::new(10.0, 0.0);
        let square = Rectangle::square(top_left, 40.0);
        assert_eq!(square.width(), square.height());
    }

    #[test]
    fn empty_list() {
        let empty = List::new();
        assert!(empty.ele() == None);
    }

    #[test]
    fn list_with_an_value() {
        let list = List::from(10);
        assert_eq!(list.ele(), Some(10));
    }

    #[test]
    fn list_len() {
        let list = List::new();
        let list = list.prepend(10);
        let list = list.prepend(20);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn list_elements() {
        let list = List::from(1);
        let list = list.prepend(2);
        let list = list.prepend(3);
        assert_eq!(list.arr(), vec![3, 2, 1]);
    }
}
