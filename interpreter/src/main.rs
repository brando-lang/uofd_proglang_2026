#[derive(Copy, Clone)]
enum Primitive {
    Add,
    Multiply,
    Number(i32),
    Variable,
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    let firstElement = &primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match firstElement {
        Primitive::Add => {iter.fold(0, |t,n| t + evaluate(vec![*n]))},
        Primitive::Multiply => {iter.fold(1,|p,n| p * evaluate(vec![*n])) },
        Primitive::Number(val) => *val,
        Primitive::Variable => panic!("Variable used outside of fixed-point context"),
    }
}

// Evaluates an expression, substituting Variable with var_val wherever it appears.
fn evaluate_with_variable(primitives: Vec<Primitive>, var_val: i32) -> i32 {
    let first = primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match first {
        Primitive::Add => iter.fold(0, |t, n| t + evaluate_with_variable(vec![*n], var_val)),
        Primitive::Multiply => iter.fold(1, |p, n| p * evaluate_with_variable(vec![*n], var_val)),
        Primitive::Number(val) => val,
        Primitive::Variable => var_val,
    }
}

// Finds the fixed point of the expression in body by iterating from start.
// A fixed point is a value x where f(x) == x.
fn evaluate_fixed_point(body: Vec<Primitive>, start: i32) -> i32 {
    let mut x = start;
    loop {
        let next = evaluate_with_variable(body.clone(), x);
        if next == x {
            return x;
        }
        x = next;
    }
}

// Adds Primitive::Variable to the given Vec, marking where the current
// iterate goes in a fixed-point function body.
fn add_fixed_point(primitives: &mut Vec<Primitive>) {
    primitives.push(Primitive::Variable);
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

    // Fixed point of f(x) = 5 (constant function).
    // Starting from 0: f(0)=5, f(5)=5 -> fixed point is 5.
    let body = vec![Primitive::Number(5)];
    let fp = evaluate_fixed_point(body, 0);
    println!("Fixed point of f(x)=5 starting from 0: {fp}");

    // Fixed point of f(x) = x (identity function).
    // Starting from 42: f(42)=42 immediately -> fixed point is 42.
    let mut body2 = Vec::<Primitive>::new();
    add_fixed_point(&mut body2);
    let fp2 = evaluate_fixed_point(body2, 42);
    println!("Fixed point of f(x)=x starting from 42: {fp2}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_add() {
        let p = vec![Primitive::Add, Primitive::Number(3), Primitive::Number(4)];
        assert_eq!(evaluate(p), 7);
    }

    #[test]
    fn test_evaluate_multiply() {
        let p = vec![Primitive::Multiply, Primitive::Number(3), Primitive::Number(4)];
        assert_eq!(evaluate(p), 12);
    }

    #[test]
    fn test_evaluate_number() {
        let p = vec![Primitive::Number(42)];
        assert_eq!(evaluate(p), 42);
    }

    #[test]
    fn test_fixed_point_constant_function() {
        // f(x) = 5, fixed point is 5 regardless of start
        let body = vec![Primitive::Number(5)];
        assert_eq!(evaluate_fixed_point(body, 0), 5);
    }

    #[test]
    fn test_fixed_point_already_at_fixed_point() {
        // f(x) = 5, starting from 5 — already there
        let body = vec![Primitive::Number(5)];
        assert_eq!(evaluate_fixed_point(body, 5), 5);
    }

    #[test]
    fn test_fixed_point_identity() {
        // f(x) = x, every value is its own fixed point
        let body = vec![Primitive::Variable];
        assert_eq!(evaluate_fixed_point(body, 42), 42);
    }

    #[test]
    fn test_fixed_point_multiply_zero() {
        // f(x) = x * 0 = 0, fixed point is 0
        let body = vec![Primitive::Multiply, Primitive::Variable, Primitive::Number(0)];
        assert_eq!(evaluate_fixed_point(body, 100), 0);
    }

    #[test]
    fn test_fixed_point_add_zero() {
        // f(x) = x + 0 = x, fixed point is the starting value
        let body = vec![Primitive::Add, Primitive::Variable, Primitive::Number(0)];
        assert_eq!(evaluate_fixed_point(body, 7), 7);
    }

    #[test]
    fn test_fixed_point_multiply_one() {
        // f(x) = x * 1 = x, fixed point is the starting value
        let body = vec![Primitive::Multiply, Primitive::Variable, Primitive::Number(1)];
        assert_eq!(evaluate_fixed_point(body, 99), 99);
    }

    #[test]
    fn test_add_fixed_point_adds_variable() {
        let mut p = Vec::new();
        add_fixed_point(&mut p);
        assert_eq!(p.len(), 1);
        assert!(matches!(p[0], Primitive::Variable));
    }

    #[test]
    fn test_add_fixed_point_builds_identity_body() {
        // add_fixed_point builds f(x) = x, fixed point is the start value
        let mut body = Vec::new();
        add_fixed_point(&mut body);
        assert_eq!(evaluate_fixed_point(body, 13), 13);
    }

    #[test]
    fn test_evaluate_with_variable_add() {
        // f(x) = x + 3, with x=10 -> 13
        let body = vec![Primitive::Add, Primitive::Variable, Primitive::Number(3)];
        assert_eq!(evaluate_with_variable(body, 10), 13);
    }

    #[test]
    fn test_evaluate_with_variable_multiply() {
        // f(x) = x * 4, with x=5 -> 20
        let body = vec![Primitive::Multiply, Primitive::Variable, Primitive::Number(4)];
        assert_eq!(evaluate_with_variable(body, 5), 20);
    }
}
