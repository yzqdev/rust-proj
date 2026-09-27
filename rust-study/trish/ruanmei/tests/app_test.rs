use ruanmei::app::App;
use ruanmei::event::EventHandler;
use ruanmei::handler::handle_key_events;

fn key(code: crossterm::event::KeyCode) -> crossterm::event::KeyEvent {
    crossterm::event::KeyEvent::from(code)
}

#[test]
fn quit_on_q_and_esc() {
    let mut app = App::new("test");
    assert!(app.running);
    handle_key_events(key(crossterm::event::KeyCode::Char('q')), &mut app).unwrap();
    assert!(!app.running);

    let mut app = App::new("test");
    handle_key_events(key(crossterm::event::KeyCode::Esc), &mut app).unwrap();
    assert!(!app.running);
}

#[test]
fn counter_and_navigation() {
    let mut app = App::new("test");
    handle_key_events(key(crossterm::event::KeyCode::Right), &mut app).unwrap();
    assert_eq!(app.counter, 1);
    handle_key_events(key(crossterm::event::KeyCode::Left), &mut app).unwrap();
    assert_eq!(app.counter, 0);

    handle_key_events(key(crossterm::event::KeyCode::Down), &mut app).unwrap();
    assert_eq!(app.selected, 1);
    handle_key_events(key(crossterm::event::KeyCode::Up), &mut app).unwrap();
    assert_eq!(app.selected, 0);
    handle_key_events(key(crossterm::event::KeyCode::Up), &mut app).unwrap();
    assert_eq!(app.selected, 0, "clamped at first item");
}

#[test]
fn reset_counter() {
    let mut app = App::new("test");
    app.increment_counter();
    app.increment_counter();
    handle_key_events(key(crossterm::event::KeyCode::Char('r')), &mut app).unwrap();
    assert_eq!(app.counter, 0);
}

#[test]
fn counter_wrapping_is_safe() {
    let mut app = App::new("test");
    for _ in 0..300 {
        app.increment_counter();
    }
    // checked_add keeps the counter at u8::MAX instead of overflowing.
    assert_eq!(app.counter, u8::MAX);
}

#[test]
fn event_handler_type_exists() {
    // Keeps the handler import meaningful even if refactored later.
    let _ = std::any::type_name::<EventHandler>();
}
