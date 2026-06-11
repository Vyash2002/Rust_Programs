enum TrafficLight {
    Red,
    Yellow,
    Green,
}
enum Season {
    Summer,
    Winter,
    Rainy,
    Spring,
}
enum Shape{
    Circle(f64),
    Rectangle(f64, f64),
}
impl Shape{
    fn area(&self) -> f64{
        match self{
            Shape::Circle(radius) => {
               3.14*radius*radius
            }
            Shape:: Rectangle(width,height) => {
               width*height
            }
        }
    }
}
impl Season{
    fn temperature_hint(&self){
        match self{
            Season::Summer => println!("It's too hot outside"),
            Season:: Winter =>println!("It's cold outside"),
            Season:: Rainy => println!("It's Rainy outside"),
            Season :: Spring => println!("The weather is awesome"),
        }
    }
}
fn main(){
    let tf = TrafficLight::Red;

    match tf {
        TrafficLight::Red => println!("Stop it's Red Light"),
        TrafficLight::Yellow => println!("Be Ready!"),
        TrafficLight:: Green => println!("Go!"),
    }

    let current_season = Season::Winter;
    current_season.temperature_hint();

    let area1 = Shape::Circle(5.98);
    let area2 = Shape::Rectangle(9.87,7.54);

   println!("The area of cicle = {}",area1.area());
   println!("The area of rectangle = {}",area2.area());

}