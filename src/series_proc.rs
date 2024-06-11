use std::collections::VecDeque;

use series::{EventType, Validity};

use crate::*;

// pub trait SeriesProcessor<T: EventType> {
//     fn handle(&mut self, event: T) -> bool;
//     fn start_with(&mut self, event: &T);
//     fn reset(&mut self);
// }

// pub struct BaseHandler<S: Default + BaseValues<T>, T: EventType, P>
// where P: Processor<VecDeque<T>> {
//     pub events: VecDeque<T>,
//     start_date: NaiveDate,
//     start_values: S,
//     proc: P
// }

pub trait BaseValues<T> {
    fn convert_from(event: &T) -> Self;
    fn validity(&self, event: &T) -> Validity;
}

pub trait EventHandler<T: EventType> {
    fn handle(&mut self, event: T) -> bool;
}

pub trait Processor<T,S> {
    fn process(&mut self, start_values: &S, x: &mut T) -> bool;
    fn reset(&mut self) {
        // default do nothing
    }
}

// pub struct BaseHandler<S: Default + BaseValues<T>, T: EventType, P>
// where P: Fn(&mut VecDeque<T>) -> bool {
pub struct BaseHandler<S: Default + BaseValues<T>, T: EventType, P>
where P: Processor<VecDeque<T>,S> {
    pub events: VecDeque<T>,
    pub start_values: S,
    pub proc: P
}

impl<S: Default + BaseValues<T>, T: EventType, P: Processor<VecDeque<T>,S>> BaseHandler<S,T,P> {
// impl<S: Default + BaseValues<T>, T: EventType, P: Fn(&mut VecDeque<T>) -> bool> BaseHandler<S,T,P> {
    pub fn new(proc: P) -> Self {
        Self { events: VecDeque::new(), start_values: S::default(), proc }
    }

    pub fn start_with(&mut self, event: &T) {
        self.start_values = S::convert_from(event);
    }

    fn reset(&mut self) {
        // If it's empty, we're already reset
        if !self.events.is_empty() {
            self.events.clear();
            self.proc.reset();
        }
    }

    pub fn move_to_next(&mut self) {
        self.events.pop_front();

        // TODO: replace unwrap?
        let nev = self.events.front().unwrap();
        self.start_values = S::convert_from(nev);
        // self.start_with(&);
    }
}

impl<S: Default + BaseValues<T>,T: EventType,P: Processor<VecDeque<T>,S>> EventHandler<T> for BaseHandler<S,T,P> {
// impl<S: Default + BaseValues<T>, T: EventType, P: Fn(&mut VecDeque<T>) -> bool> EventHandler<T> for BaseHandler<S,T,P> {
    fn handle(&mut self, event: T) -> bool {
        match self.start_values.validity(&event) {
            Validity::Valid => {
                if self.events.is_empty() {
                    // It's the first event ever or after reset
                    self.start_with(&event);
                }
                self.events.push_back(event);
                self.proc.process(&self.start_values, &mut self.events)
            },
            Validity::CauseReset => {
                self.reset();
                self.start_with(&event);
                self.events.push_back(event);
                true
            },
            Validity::Invalid => {
                self.reset();
                true
            },
        }
    }
}
