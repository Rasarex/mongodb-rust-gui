use std::sync::Arc;

use super::mongodriver::{get_movies, is_admin};
use druid::widget::{Align, Button, Flex, Label, TextBox, ViewSwitcher};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use futures::executor;
use mongodb::bson::doc;
use mongodb::options::FindOptions;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<LoginState> = LocalizedString::new("Hello World");

#[derive(Copy, Clone, Data, PartialEq, Eq)]
enum WindowMode {
    LoginScreen,
    AdminScreen,
    UserScreen,
}
use mongodb::bson::oid::ObjectId;
#[derive(Clone, Data, Lens)]
struct LoginState {
    login: String,
    pwd: String,
    #[data(ignore)]
    db: Option<mongodb::Database>,
    #[data(ignore)]
    rented: Vec<ObjectId>,
    mode: WindowMode,
    error: String,
}

pub fn prompt_login(db: mongodb::Database) {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((900.0, 600.0));

    // create the initial app state
    let initial_state = LoginState {
        login: "".into(),
        pwd: "".into(),
        db: None,
        error: "".to_string(),
        rented: Vec::new(),
        mode: WindowMode::LoginScreen,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
fn get_user_prompt(db: &mongodb::Database) -> impl Widget<LoginState> {
    let mut error_label = Label::new(|data: &LoginState, _env: &_| format!("{}", data.error));
    let background = Color::from_hex_str("00ff00").unwrap();
    error_label.set_text_color(Color::from_hex_str("ff0000").unwrap());
    // let buttons_row = Flex::row();
    let maybe_movies = executor::block_on(get_movies(db));
    let mut movies_box = Flex::column();
    if let Ok(movies) = maybe_movies {
        for movie in movies {
            let title_label = Label::<LoginState>::new(format!("tytuł:{}", movie.title));
            let genre = Label::<LoginState>::new(format!("Gatunki: {:?}", movie.genre));
            let length = Label::<LoginState>::new(format!("długość {:?}", movie.length));
            let actors = Label::<LoginState>::new(format!("aktorzy {:?}", movie.actors));
            let score = Label::<LoginState>::new(format!("ocena {:?}", movie.score));
            let short_desc =
                Label::<LoginState>::new(format!("krótki opis {:?}", movie.short_desc));
            use druid::widget::SizedBox;
            let mut movie_box = Flex::column()
                .with_child(title_label)
                .with_child(genre)
                .with_child(length)
                .with_child(actors)
                .with_child(score)
                .with_child(short_desc);

            let get_button =
                Button::new("Wypożycz").on_click(move |_ctx, data: &mut LoginState, _env| {
                    if data.rented.contains(&movie.id.unwrap()) {
                        data.error = String::from("Ten film już został wypożyczony")
                    } else if data.rented.len() == 3 {
                        data.error = String::from("Można wypożyczyć maksymalnie 3");
                    } else {
                        data.rented.push(movie.id.unwrap())
                    }
                });

            movie_box.add_default_spacer();
            movie_box.add_child(get_button);
            movies_box.add_child(Scroll::new(movie_box.fix_width(400.0)).fix_height(100.0));
        }
    } else if let Err(err) = maybe_movies {
        error_label.set_text(err.to_string());
    }
    use druid::widget::Scroll;
    // let confirm_button = Button::new("Potwierdź").on_click(|_ctx, data: &mut LoginState, _env| {
    //     executor::block_on(super::mongodriver::rent_movies(
    //         data.db.as_ref().unwrap(),
    //         data.rented,
    //     ));
    // });
    let layout = Flex::column()
        .with_child(
            Scroll::new(movies_box.fix_size(400.0, 700.0))
                .vertical()
                .border(background, 2.0)
                .fix_size(400.0, 700.0),
        )
        .with_child(error_label);
    // .with_child(confirm_button);
    layout
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
            let client_res =
                super::mongodriver::get_client(data.login.to_owned(), data.pwd.to_owned());
            if let Ok(client) = client_res {
                let db = client.database("wypozyczalnia");
                data.db = Some(db);

                let maybe_admin = is_admin(&data.login, &(data.db.as_ref().unwrap()));
                if let Ok(truth_of_admin) = maybe_admin {
                    if truth_of_admin {
                        data.mode = WindowMode::AdminScreen;
                    } else {
                        data.mode = WindowMode::UserScreen;
                    }
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
fn get_admin_prompt() -> impl Widget<LoginState> {
    let mut error_label = Label::new(|data: &LoginState, _env: &_| format!("{}", data.error));
    let login_textbox = TextBox::new()
        .with_placeholder("login")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(LoginState::login);
    let pwd_textbox = TextBox::new()
        .with_placeholder("password")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(LoginState::pwd);
    let create_button = Button::new("Create User")
        .on_click(|_ctx, data: &mut LoginState, _env| {
            let db = data.db.as_ref().unwrap();
            let login = data.login.to_owned();
            let pwd = data.pwd.to_owned();
            let payload = doc! {"createUser" : login,"pwd"  : pwd,"roles":[]};
            let res = executor::block_on(db.run_command(payload, None));
            if let Err(error) = res {
                data.error = error.to_string();
            } else {
                data.error = "User created".to_string();
            }
        })
        .padding(5.0);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(error_label)
        .with_child(login_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(pwd_textbox)
        .with_child(create_button);

    Align::centered(layout)
}
fn build_root_widget() -> impl Widget<LoginState> {
    // a label that will determine its text based on the current app data.
    // a textbox that modifies `name`.
    let view_switcher = ViewSwitcher::new(
        |data: &LoginState, _env| data.mode,
        |selector, data, _env| match selector {
            WindowMode::LoginScreen => Box::new(get_login_prompt()),
            WindowMode::UserScreen => Box::new(get_user_prompt(&data.db.as_ref().unwrap())),
            WindowMode::AdminScreen => get_admin_prompt().boxed(),
            _ => Box::new(Label::new("Unknown").center()),
        },
    );

    Align::centered(Flex::row().with_child(view_switcher))
}
