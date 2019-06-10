# 0.x.x - unreleased

# 0.2.0 - 2019-06-10

-   Rewrite for simplicity and to fix multiple args.

    See [`examples/03.rs`](./examples/03.rs) for the full example.

    ```rust
    fn multiarg(&self, a: u32, b: u32, c: u32) -> u32 {
        a + b * c
    }
    ```

# 0.1.4 - 2019-06-09

-   Add where clause support.

    See [`examples/02.rs`](./examples/02.rs) for the full example.

    ```rust
    pub fn window<T>(&self, data: T) -> WindowBuilder<T> where T: Clone {
        WindowBuilder { data }
    }
    ```

# 0.1.3 - 2019-06-08

-   Implement access to the outer structure.

    ```rust
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
    ```

# 0.1.2 - 2019-06-08

-   Identical to 0.1.1, but with correct changelog. :)

# 0.1.1 - 2019-06-08

-   Implement generic methods.

    ```rust
    rcstruct::rcstruct! {
        pub struct GUI {}

        impl {
            pub new() -> Rt<Self> {
                {}
            }

            pub fn window<T>(&self, data: T) -> WindowBuilder<T> {
                WindowBuilder { data }
            }
        }
    }
    ```

# 0.1.0 - 2019-06-07

-   🎉 Initial release.
