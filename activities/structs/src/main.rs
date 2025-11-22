struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) {
    let area = self.width*self.height;
    println!("area is {area} ");
    }

    fn can_hold (&self, other: Rectangle) {
        let other_area = other.width*other.height;
        let self_area = self.width*self.height;

        if self_area > other_area {
        println!("the current rectangle can hold the 'other' rectangle.");
        }
        else if self_area == other_area {
            println!("The 2 rectangles are equal.");
        }
        else {
            println!("This rectangle is smaller than the 'other' rectangle.");
        }
    }

}

fn main() {
        let rect1 = Rectangle {
        width: 12,
        height: 13,
        };

        let rect2 = Rectangle {
        width: 14,
        height: 13,
        };

        rect1.can_hold(rect2);
        

}

