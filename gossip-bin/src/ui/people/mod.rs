use super::{GossipUi, Page};
use eframe::egui;
use egui::{Context, Ui};

mod follow;
mod followed;
mod list;
mod lists;
mod muted;
mod person;

pub(super) fn update(app: &mut GossipUi, ctx: &Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
    if app.page == Page::PeopleFollowNew {
        follow::update(app, ctx, _frame, ui);
    } else if app.page == Page::PeopleLists {
        lists::update(app, ctx, _frame, ui);
    } else if let Page::PeopleList(plist) = app.page {
        list::update(app, ctx, _frame, ui, plist);
    } else if app.page == Page::PeopleFollowed {
        followed::update(app, ctx, _frame, ui);
    } else if app.page == Page::PeopleMuted {
        muted::update(app, ctx, _frame, ui);
    } else if matches!(app.page, Page::Person(_)) {
        person::update(app, ctx, _frame, ui);
    }
}
