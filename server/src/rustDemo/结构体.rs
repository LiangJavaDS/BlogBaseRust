// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // 左上角
    let top_right: Point = Point { x: 0.0, y: 1.0 };
    // 右下角
    let bottom_right = Point { x: 2.0, y: 0.0 };
    // 解构
    let Point {
        x: left_edge,
        y: top_edge,
    } = top_right;
    // 实例化长方形
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    // 计算面积
    println!("长方形的面积 = {:.2}", rect_area(_rectangle));
    let point: Point = Point { x: 0.0, y: 1.0 };
    println!("产生一个新的长方形 = {:?}", square(point, 4.0));
}

/** 生成新的长方形 */
fn square(p: Point, num: f32) -> Rectangle {
    let Point {
        x: left_edge,
        y: top_edge,
    } = p;

    let right_edge = left_edge + num;
    let bottom_edge = top_edge - num;
    Rectangle {
        top_left: p,
        bottom_right: Point {
            x: right_edge,
            y: bottom_edge,
        },
    }
}

/** 计算长方形面积 */
fn rect_area(rectangle: Rectangle) -> f32 {
    let Point {
        x: left_edge,
        y: top_edge,
    } = rectangle.top_left;
    let Point {
        x: right_edge,
        y: bottom_edge,
    } = rectangle.bottom_right;
    let width = right_edge - left_edge;
    let height = top_edge - bottom_edge;
    width * height
}
