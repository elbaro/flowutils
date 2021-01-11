use flowutils::defer;

fn main() {
    defer!(println!("order 6"));
    {
        defer!(println!("order 4"));
        defer!({
            println!("order 2");
            println!("order 3");
        });
        defer!(println!("order 1"));
    }
    defer!(println!("order 5"));
}
