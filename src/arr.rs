//! 数组是在栈上分配的，在编译时就必须确定其大小，所以不能动态地添加和删除元素。
//! 数组的类型表示为：[T; length], `T` 代表数组中元素的类型, length 代表元素的长度。
//!
//! 切片 (slice) 跟数组类似，但是其大小在编译时不确定。Slice 是两个字长的对象。第一个字
//! 是指向数据的指针(数据的首指针)，第二个字是切片的长度。其表示为：&[T]。

use std::mem;
pub fn run() {
    println!("=========== Arrays and Slices ===========");

    let a = [8; 10];
    println!("10 of 8: {:?}", a);
    println!("size of {:?} is {}", a, a.len());
    println!("occupies {} bytes", mem::size_of_val(&a));
    println!("borrow a section of the array as slice:");
    analyze_slice(&a[3..5]);
    analyze_slice(&a[..3]);
    analyze_slice(&a[5..]);

    println!("borrow the whole array as slice:");
    // 数组可以自动被借用成为 slice
    analyze_slice(&a);
}

fn analyze_slice<T: std::fmt::Display>(slice: &[T]) {
    println!("first element of the slice: {}.", slice[0]);
    println!("the slice has {} elements.", slice.len());
}
