
struct Point {
    x: f64, 
    y: f64
}


fn compute_distance(p1: &Point, p2: &Point) -> f64 {
    let x_d = p1.x - p2.x;
    let y_d = p1.y - p2.y;
    (x_d * x_d + y_d * y_d).sqrt()
}


#[test]
fn lifetime0(){
    let on_the_stack : Point      =     Point {x: 3.0, y: 4.0};
    assert_eq!(on_the_stack.x , 3.0);
    assert_eq!(on_the_stack.y , 4.0);
    let on_the_heap  : Box<Point> = box Point {x: 7.0, y: 7.0};
    assert_eq!(on_the_heap.x , 7.0);
    let distance = compute_distance(&on_the_stack, &*on_the_heap);
    assert_eq!(distance, 5.0);
}
