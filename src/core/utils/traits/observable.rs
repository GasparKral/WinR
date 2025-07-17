pub trait Observable<T> {
    fn subscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&T) + 'static;

    fn notify(&self, value: &T);
    fn unsubscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&T) + 'static;
    fn clear_subscriptions(&mut self);
    fn has_subscriptions(&self) -> bool;
    fn notify_all(&self, value: &T) {
        self.notify(value);
    }
}

pub trait Observer<T> {
    fn on_change(&mut self, value: &T);
}
