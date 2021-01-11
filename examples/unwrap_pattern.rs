use flowutils::unwrap_pattern;

enum T {
    A(i32),
    B(String, u64),
    C { p: usize, _q: f32, r: i8 },
}

fn main() {
    let some_enum = T::A(3);
    let inner = unwrap_pattern!(some_enum, T::A(x)=>x);
    assert_eq!(inner, 3);

    let some_enum = T::B(String::from("str"), 3);
    let tuple: (u64, String) = unwrap_pattern!(some_enum, T::B(var1, var2) => (var2, var1));
    assert_eq!(tuple, (3, String::from("str")));

    let some_enum = T::C {
        p: 9,
        _q: 8.0,
        r: 7,
    };
    let complex: usize = unwrap_pattern!(some_enum, T::C{p: var3, r: _var4, ..}=> var3);
    assert_eq!(complex, 9);
}
