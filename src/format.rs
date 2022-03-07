//! 打印由 `std::fmt` 里面定义的宏来处理：
//!  - `format!`: 将格式化文本写到字符串。
//!  - `print!`: 将格式化文本输出到控制台。
//!  - `println!`: 与 `print!` 类似，但是输出结果中追加了换行。
//!  - `eprint!`: 将格式化文本输出到标准错误。
//!  - `eprintln!`: 与 `eprint!` 类似，但是输出结果中追加了换行。
//!
//! `std::fmt` 包含多种 trait 来控制文本显示：
//!  - `fmt::Debug`: 使用 `{:?}` 标记。
//!  - `fmt::Display`: 使用 `{}` 标记。可以以更好的风格来格式化文本。
//!
//! `std` 中的类型都实现了可打印的 trait。但是自定义的类型需要自行实现。
//!  使用 `derive(Debug)` 属性可以自动为自定义类型创建实现。使得自定义
//! 类型可以使用 `fmt::Debug` 打印。
//!
pub fn run() {
    println!("=========== Formatted Print ===========");
    // MyType 没有实现 Display ，所以不能用 `{}` 打印。
    // println!("MyType: {}", MyType(100));
    println!("{:?}", debug::MyType::from(100));
    // 使用 `{:#?}` 会使得打印更美观
    println!("{:#?}", debug::MyType::from(100));

    let complex = display::Complex::from(3.3, 7.2);
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let list = display::List::from(vec![1, 1, 2, 3, 5, 8, 11]);
    println!("{}", list);

    let c1 = display::Color::from(128, 255, 90);
    let c2 = display::Color::from(0, 3, 254);
    let c3 = display::Color::from(0, 0, 0);
    println!("{}, {}, {}", c1, c2, c3);
}

mod debug {
    // `derive` 属性会为自定义类型自动创建所需的实现。这使得自定义的结构体可以使用 `fmt::Debug` 打印。
    #[derive(Debug)]
    pub struct MyType(i32);

    impl MyType {
        pub fn from(num: i32) -> Self {
            MyType(num)
        }
    }
}

mod display {
    #[derive(Debug)]
    pub struct Complex {
        real: f32,
        imag: f32,
    }

    impl Complex {
        pub fn from(real: f32, imag: f32) -> Self {
            Complex { real, imag }
        }
    }

    use std::fmt::{Display, Formatter, Result};
    impl Display for Complex {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    // 对于每个 `write!` 都会生成一个 `fmt::Result`。
    // 这就很难在一个实现里多次调用 `write!`，为了解决
    // 这个问题，rust 提供了 `?` 操作符。如果调用 `write!`
    // 出错，会返回错误，否则程序继续执行。
    pub struct List(Vec<i32>);

    impl List {
        pub fn from(vec: Vec<i32>) -> Self {
            List(vec)
        }
    }

    impl Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "[")?;

            let vec = &self.0;
            for (index, v) in vec.iter().enumerate() {
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", index, v)?;
            }
            write!(f, "]")
        }
    }

    #[derive(Debug)]
    pub struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Color {
        pub fn from(red: u8, green: u8, blue: u8) -> Self {
            Color { red, green, blue }
        }
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            // {:X}: 十六进制输出
            // {:o}: 八进制输出
            // {:b}: 二进制输出
            // {:0n}: 表示补0使位数为 n 位
            write!(
                f,
                "RGB({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
                self.red, self.green, self.blue
            )
        }
    }
}
