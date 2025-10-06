struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
        let rect1 = Rectangle{
        width: 12,
        height: 13,
        };

        area (rect1);
}

fn area (dimension: Rectangle ) {
    let area = dimension.width*dimension.height;
        println!("the area of the rectangle is: {area}");

}
