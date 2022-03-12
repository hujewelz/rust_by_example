use std::fs::File;
use std::io::{self, Read};

pub fn run() {
    // 使用 `match` 来匹配 Result<T, Error> 的结果
    let f = File::open("hello.text");
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Error occoured {:?}!", error),
    };

    // 如果 Result 值是成员 Ok，`unwrap` 会返回 Ok 中的值。
    // 如果 Result 是成员 Err，`unwrap` 会调用 panic!
    // 这与上面的代码结果一样
    let _f2 = File::open("hello.txt").unwrap();
    let _f2 = File::open("hello.txt").expect("Fiailed to open hello.txt");

    let _str = read_file().unwrap_or("Can not read file".to_string());
}

// 传播错误
fn read_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err), // 出现错误，函数提前返回
    };

    let mut buff = String::new();
    match f.read_to_string(&mut buff) {
        Ok(_) => Ok(buff),
        Err(e) => Err(e),
    }
}

// 传播错误的简写：? 运算符
fn read_file2() -> Result<String, io::Error> {
    /*
    如果 Result 的值是 `OK`, 那么表达式将会返回 `OK` 中的值，程序继续执行；
    如果 Result 的值是 `Err`, 那么 `Err` 中的值就会作为函数的返回值。最终的
    结果跟上面使用 `match` 一样。

    但是这里的细节跟 `match` 有一点不同：`?` 运算符所使用的错误值会被传递给 `from`
    函数中，这是定义在标准库的 `From` trait 中。其用来将错误从一种类型转换为另一种类型。
    当 `?` 运算符调用 `from` 函数时，接收到的错误类型会被转换为由当前调用函数的返回类型
    所指定的错误类型。
     */
    let mut f = File::open("hello.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

// `?` 运算符与 Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    /*
    跟对 Result 使用 `?` 一样，只能在返回值为 Option 的函数中对 Option 使用 `?`。其使用
    结果跟对 Result 类似。如果值是 `None`， 那么 `None` 会从函数中提前返回。如果值是 `Some`,
    Some 中的值会最为表达式的返回值。

    `text.lines().next()` 的返回类型为 `Option<&str>`
    `text.lines().next()?` 的返回类型为 `&str`
     */
    text.lines().next()?.chars().last()
}
