use super::mongodriver::is_admin;
use druid::widget::{Align, Button, Flex, Label, TextBox, ViewSwitcher};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use std::ascii::AsciiExt;
const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<LoginState> = LocalizedString::new("Hello World");

#[derive(Copy, Clone, Data, PartialEq, Eq)]
enum WindowMode {
    LoginScreen,
    AdminScreen,
    UserScreen,
}
#[derive(Clone, Data, Lens)]
struct LoginState {
    login: String,
    pwd: String,
    #[data(ignore)]
    db: mongodb::Database,
    mode: WindowMode,
}

pub fn prompt_login(db: mongodb::Database) {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = LoginState {
        login: "".into(),
        pwd: "".into(),
        db,
        mode: WindowMode::LoginScreen,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
fn get_login_prompt() -> impl Widget<LoginState> {
    let login_textbox = TextBox::new()
        .with_placeholder("login")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(LoginState::login);
    let pwd_textbox = TextBox::new()
        .with_placeholder("password")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(LoginState::pwd);
    let login_button = Button::new("Login")
        .on_click(|_ctx, data: &mut LoginState, _env| {
            let maybe_admin = is_admin(&data.login, &data.db);
            if let Ok(truth_of_admin) = maybe_admin {
                if truth_of_admin {
                    data.mode = WindowMode::AdminScreen;
                } else {
                    data.mode = WindowMode::UserScreen;
                }
            }
        })
        .padding(5.0);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(login_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(pwd_textbox)
        .with_child(login_button);

    Align::centered(layout)
}
fn build_root_widget() -> impl Widget<LoginState> {
    // a label that will determine its text based on the current app data.
    // a textbox that modifies `name`.
    let view_switcher = ViewSwitcher::new(
        |data: &LoginState, _env| data.mode,
        |selector, _data, _env| match selector {
            WindowMode::LoginScreen => Box::new(get_login_prompt()),
            WindowMode::UserScreen => Box::new(Label::new("User").center()),
            WindowMode::AdminScreen => Box::new(Label::new("Admin").center()),
            _ => Box::new(Label::new("Unknown").center()),
        },
    );

    Align::centered(Flex::row().with_child(view_switcher))
}
