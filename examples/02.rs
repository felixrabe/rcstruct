#![allow(unused)]

type Rt<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub struct WindowBuilder<T> {
    gui: GUI,
    data: T,
}

impl<T> WindowBuilder<T> {
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

        pub fn window<T>(&self, data: T) -> WindowBuilder<T> {
            let gui = outer().unwrap(); // <= `outer()` returns Option<GUI>
            WindowBuilder { gui, data }
        }
    }
}

fn main() -> Rt {
    let gui = GUI::new()?;
    let window = gui.window(()).build()?;
    Ok(())
}
