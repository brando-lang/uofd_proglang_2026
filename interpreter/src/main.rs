#[derive(Copy,Clone)]
enum Primitive {
    Add,
    Multiply,
    Number(i32)
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    let firstElement = &primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match firstElement {
        Primitive::Add => {iter.fold(0, |t,n| t + evaluate(vec![*n]))},
        Primitive::Multiply => {iter.fold(1,|p,n| p * evaluate(vec![*n])) },
        Primitive::Number(val) => *val
    } 
}

fn main() {

    // New expression: Addition of 15 + 25 + 10 + 50 = 100
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Add);
    primitives.push(Primitive::Number(15));
    primitives.push(Primitive::Number(25));
    primitives.push(Primitive::Number(10));
    primitives.push(Primitive::Number(50));
    let result = evaluate(primitives);
    println!("The result is {result}");

}
