struct StructA {
    name: String,
}

struct StructB {
    a: StructA,
    x: i64,
    // other fields...
}

macro_rules! add_msg_deref {
    ($name:ident) => {
        impl std::ops::Deref for $name {
            type Target = StructA;
            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }
    };
}

add_msg_deref!(StructB);
// impl std::ops::Deref for StructB {
//     type Target = StructA;
//     fn deref(&self) -> &Self::Target {
//         &self.a
//     }
// }

fn test(x: &StructA) {
    println!("{}", x.name);
}

struct Person {}

trait SetName<T> {
    fn set_name(&mut self, t: T);
}
impl SetName<&str> for Person {
    fn set_name(&mut self, str: &str) {}
}
impl SetName<String> for Person {
    fn set_name(&mut self, str: String) {}
}

fn main() {
    let b = StructB {
        a: StructA {
            name: "Anna".to_string(),
        },
        x: 10,
    };
    println!("{}, {}", b.name, b.x);
    test(&b);
}
