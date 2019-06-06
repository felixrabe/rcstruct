#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

type Rt<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Receiver<T>(std::marker::PhantomData<T>);

#[derive(Debug)]
pub struct Sender<T>(std::marker::PhantomData<T>);

impl<T> Sender<T> {
    fn send(&self, _: T) -> Rt {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Event;

#[derive(Debug)]
pub enum Action {
    Foo,
}

fn unbounded_channel<T>() -> (Sender<T>, Receiver<T>) {
    (Sender(std::marker::PhantomData), Receiver(std::marker::PhantomData))
}

rcstruct::rcstruct! {
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
}

fn main() -> Rt {
    // Set up channels.
    let (event_send, event_recv) = unbounded_channel::<Event>();
    let (action_send, action_recv) = unbounded_channel::<Action>();

    let gui = GUI::new(event_recv, action_send)?;
    while gui.running()? {
        for ev in gui.events()? {
            // ...
        }

        gui.send_action(Action::Foo)?;
        gui.quit()?;
    }

    Ok(())
}
