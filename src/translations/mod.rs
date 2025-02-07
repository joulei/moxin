use makepad_widgets::Cx;

mod en;
mod zh;
mod es;
mod nl;

pub fn live_design(cx: &mut Cx) {
    en::live_design(cx);
    zh::live_design(cx);
    es::live_design(cx);
    nl::live_design(cx);
}

