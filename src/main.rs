slint::include_modules!();

mod schedule;
mod translate;
mod worker;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

use slint::{Model, ModelRc, VecModel};

use worker::Msg;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let (tx, rx) = mpsc::channel::<Msg>();
    let rx = Rc::new(RefCell::new(rx));

    // ── Refresh callback ────────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        let tx = tx.clone();
        ui.on_refresh(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_loading(true);
                ui.set_error("".into());
            }
            worker::spawn(tx.clone());
        });
    }

    // ── Open link in browser ────────────────────────────────────────────
    ui.on_open_link(|link| {
        if !link.is_empty() {
            let _ = webbrowser::open(link.as_str());
        }
    });

    // ── Japanese-time clock ─────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        let clock = slint::Timer::default();
        clock.start(slint::TimerMode::Repeated, Duration::from_secs(1), move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_jp_time(format_jp_time().into());
            }
        });
        std::mem::forget(clock);
    }

    // ── Message pump — drains every queued message each tick ────────────
    let model: Rc<RefCell<Option<Rc<VecModel<StreamData>>>>> = Rc::new(RefCell::new(None));
    let timer = slint::Timer::default();
    {
        let ui_weak = ui.as_weak();
        let rx = rx.clone();
        let model = model.clone();
        timer.start(slint::TimerMode::Repeated, Duration::from_millis(16), move || {
            loop {
                let msg = match rx.borrow().try_recv() {
                    Ok(m) => m,
                    Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => return,
                };
                let Some(ui) = ui_weak.upgrade() else { return };

                match msg {
                    Msg::Rows(rows) => {
                        let data: Vec<StreamData> = rows.iter().map(|r| r.to_ui()).collect();
                        let m = Rc::new(VecModel::from(data));
                        ui.set_streams(ModelRc::from(m.clone()));
                        ui.set_loading(false);
                        *model.borrow_mut() = Some(m);
                    }
                    Msg::Image(i, buffer) => {
                        if let Some(m) = model.borrow().as_ref() {
                            if let Some(mut row) = m.row_data(i) {
                                row.pic = slint::Image::from_rgba8(buffer);
                                m.set_row_data(i, row);
                            }
                        }
                    }
                    Msg::Error(e) => {
                        ui.set_error(e.into());
                        ui.set_loading(false);
                    }
                }
            }
        });
    }
    std::mem::forget(timer);

    ui.invoke_refresh();
    ui.run()
}

fn format_jp_time() -> String {
    use chrono::Utc;
    use chrono_tz::Asia::Tokyo;
    let now = Utc::now().with_timezone(&Tokyo);
    now.format("Current Japanese Time: %Y-%m-%d %H:%M:%S").to_string()
}