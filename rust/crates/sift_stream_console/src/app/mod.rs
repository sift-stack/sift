use std::{
    cell::RefCell, collections::HashMap, sync::mpsc::{channel, Receiver, Sender}, time::Duration
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

pub mod demo;

mod state;
use state::{FocusedWidget, State};

mod ui;

const EVENT_POLL_TIMEOUT_MILLIS: u64 = 250;

struct App {
    exit_program_requested: bool,
    state: State,

    /// Queue of updates
    updates: Receiver<StateUpdate>,

    /// Event listeners
    event_listeners: EventListenerRegistry,

    /// Mechanism for widget to dispatch state updates
    dispatcher: Dispatcher,
}

/// Dependency injection for widgets.
#[derive(Copy, Clone)]
struct Context<'a> {
    event_listeners: &'a EventListenerRegistry,
    dispatcher: &'a Dispatcher,
    state: &'a State,
}

/// Allows widgets to mutate [State].
struct Dispatcher(Sender<StateUpdate>);

#[derive(Default)]
struct EventListenerRegistry {
    listeners: RefCell<HashMap<String, EventListener>>,
}

type StateUpdate = Box<dyn FnOnce(&mut State)>;
type EventListener = Box<dyn FnOnce(Event)>;

impl App {
    fn new(state: State) -> Self {
        let (tx, rx) = channel::<StateUpdate>();
        let dispatcher = Dispatcher::from(tx);

        Self {
            state,
            dispatcher,
            exit_program_requested: false,
            updates: rx,
            event_listeners: EventListenerRegistry::default(),
        }
    }

    /// Main draw loop. Order of operations:
    /// - Frame is drawn
    /// - User input is polled and processed and state may be updated
    /// - Dispatched updates from widgets are processed
    fn run(mut self, mut terminal: DefaultTerminal) {
        terminal.clear().expect("failed to clear terminal");

        while !self.exit_program_requested {
            terminal
                .draw(|frame| self.draw(frame))
                .expect("failed to draw frame");

            self.process_user_events();
            self.process_updates();
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::render(frame, Context {
            state: &self.state,
            event_listeners: &self.event_listeners,
            dispatcher: &self.dispatcher,
        })
    }

    fn process_updates(&mut self) {
        while let Ok(update) = self.updates.try_recv() {
            update(&mut self.state)
        }
    }

    fn process_user_events(&mut self) {
        if !event::poll(Duration::from_millis(EVENT_POLL_TIMEOUT_MILLIS))
            .expect("error while polling user event")
        {
            return;
        }

        match event::read().expect("error reading user event") {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) => match (code, modifiers, kind, state) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL, KeyEventKind::Press, _) => {
                    self.exit_program_requested = true;
                }

                // Focusing on an element
                (KeyCode::Tab, _, KeyEventKind::Press, _) => {
                    match self.state.focused {
                        FocusedWidget::Logs => self.state.focused = FocusedWidget::Metrics,
                        FocusedWidget::Metrics => self.state.focused = FocusedWidget::Logs,
                    };
                }

                // Log control when focused
                (
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right,
                    _,
                    KeyEventKind::Press | KeyEventKind::Repeat,
                    _
                ) if self.state.focused == FocusedWidget::Logs => match code {
                    KeyCode::Up => {
                        if self.state.logs_viewport_height == 0 {
                            return
                        }
                        self.state.logs_char_offset = 0;
                        self.state.logs_entry_offset = self.state.logs_entry_offset.saturating_sub(1);
                    },
                    KeyCode::Down => {
                        if self.state.logs_viewport_height == 0 {
                            return
                        }
                        let current_offset = self.state.logs_entry_offset;
                        let num_logs = self.state.logs.len();

                        self.state.logs_char_offset = 0;
                        self.state.logs_entry_offset = num_logs
                            .saturating_sub(1)
                            .saturating_sub(self.state.logs_viewport_height)
                            .min(current_offset + 1);

                    }
                    KeyCode::Right => {
                        if self.state.logs_viewport_width == 0 {
                            return
                        }
                        let max_offset = self.state.logs_longest_line_len_in_viewport
                            .saturating_sub(1)
                            .saturating_sub(self.state.logs_viewport_width);

                        self.state.logs_char_offset = (self.state.logs_char_offset + 1).min(max_offset);
                    }
                    KeyCode::Left => {
                        self.state.logs_char_offset = self.state.logs_char_offset.saturating_sub(1);
                    }
                    _ => (),
                }

                _ => (),
            },
            _ => (),
        }
    }
}

impl From<Sender<StateUpdate>> for Dispatcher {
    fn from(tx: Sender<StateUpdate>) -> Self {
        Self(tx)
    }
}

impl Dispatcher {
    fn dispatch<F>(&self, update: F)
    where
        for<'a> F: FnOnce(&'a mut State) + 'static,
    {
        self.0.send(Box::new(update)).expect("failed to dispatch a state update")
    }
}

impl EventListenerRegistry {
    fn add<F>(&self, key: &str, op: F)
    where
        F: FnOnce(Event) + 'static,
    {
        if !self.listeners.borrow().contains_key(key) {
            self.listeners.borrow_mut().insert(key.into(), Box::new(op));
        }
    }
}
