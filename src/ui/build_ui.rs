use crate::ui::util::check_existing_user_handler::check_for_existing_user;

pub(crate) fn build_ui(application: &adw::Application) {
    //TODO use windows instead of dialogs when possible
    check_for_existing_user(application);
}
