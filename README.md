# rcstruct

## Example without rcstruct

See [`examples/01.rs`](./examples/01.rs) for the full example.

```rust
struct GUIInternal {
    running: bool,
    event_recv: Receiver<Event>,
    action_send: Sender<Action>,
}

impl GUIInternal {
    fn send_action(&self, action: Action) -> Rt {
        Ok(self.action_send.send(action)?)
    }

    fn running(&self) -> Rt<bool> {
        Ok(self.running)
    }

    fn quit(&mut self) -> Rt {
        self.running = false;
        Ok(())
    }

    fn events(&self) -> Rt<impl IntoIterator<Item = Event>> {
        let events = Vec::new();
        Ok(events)
    }
}

pub struct GUI(Rc<RefCell<GUIInternal>>);

impl GUI {
    pub fn new(event_recv: Receiver<Event>, action_send: Sender<Action>) -> Rt<Self> {
        let running = true;
        Ok(GUI(Rc::new(RefCell::new(GUIInternal { running, event_recv, action_send, }))))
    }

    pub fn send_action(&self, action: Action) -> Rt {
        self.0.borrow().send_action(action)
    }

    pub fn running(&self) -> Rt<bool> {
        self.0.borrow().running()
    }

    pub fn quit(&self) -> Rt {
        self.0.borrow_mut().quit()
    }

    pub fn events(&self) -> Rt<impl IntoIterator<Item = Event>> {
        self.0.borrow().events()
    }
}
```

## Example with rcstruct

See [`examples/02.rs`](./examples/02.rs) for the full example.

```rust
rcstruct::rcstruct! {
    pub struct GUI {
        running: bool,
        event_recv: Receiver<Event>,
        action_send: Sender<Action>,
    }

    impl {
        pub new(event_recv: Receiver<Event>, action_send: Sender<Action>) -> Rt<Self> {
            let running = true;
            { running, event_recv, action_send, }
        }

        fn send_action(&self, action: Action) -> Rt {
            Ok(self.action_send.send(action)?)
        }

        fn running(&self) -> Rt<bool> {
            Ok(self.running)
        }

        fn quit(&mut self) -> Rt {
            self.running = false;
            Ok(())
        }

        fn events(&self) -> Rt<impl IntoIterator<Item = Event>> {
            let events = Vec::new();
            Ok(events)
        }
    }
}
```

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
