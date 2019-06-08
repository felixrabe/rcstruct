# Unreleased

# 0.1.2 - 2019-06-08

-   Identical to 0.1.1, but with correct changelog. :)

# 0.1.1 - 2019-06-08

-   Implement generic methods.

    See [`examples/02.rs`](./examples/02.rs) for the full example.

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
