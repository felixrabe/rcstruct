#![allow(unused)]

type Rt<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub struct WindowBuilder<T: Clone> {
    gui: GUI,
    data: T,
}

impl<T: Clone> WindowBuilder<T> {
    fn build(self) -> Rt<Window<T>> {
        Ok(Window { data: self.data })
    }
}

pub struct Window<T> {
    data: T,
}

rcstruct::rcstruct! {
    pub struct GUI {}

    impl {
        pub new() -> Rt<Self> {
            {}
        }

        pub fn window<T>(&self, data: T) -> WindowBuilder<T> where T: Clone {
            let gui = outer().unwrap(); // <= `outer()` returns Option<GUI>
            WindowBuilder { gui, data }
        }
    }
}

fn main() -> Rt {
    let gui = GUI::new()?;
    let window = gui.window(42).build()?;
    Ok(())
}
