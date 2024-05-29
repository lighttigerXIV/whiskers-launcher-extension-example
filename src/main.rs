use assets::get_image;
use dirs::home_dir;
use whiskers_launcher_rs::{
    action::{
        Action, CopyAction, DialogAction, ExtensionAction, Field, FileFilter, FilePickerField,
        InputField, OpenURLAction, SelectField, SelectOption, TextAreaField, ToggleField,
    },
    api::extensions::{get_extension_request, get_extension_setting, send_response},
    result::{TextResult, TitleAndDescriptionResult, WLResult},
    utils::send_notification,
};

pub mod assets;

const EXTENSION_ID: &str = "lighttigerxiv/extensions-example";

fn main() {
    let request = get_extension_request();

    let starter = get_extension_setting(EXTENSION_ID, "starter").unwrap();
    let other_starter = get_extension_setting(EXTENSION_ID, "other-pokemon").unwrap();
    let description = get_extension_setting(EXTENSION_ID, "description").unwrap();
    let nickname = get_extension_setting(EXTENSION_ID, "nickname").unwrap();
    let pokemon_name = if starter == "other" {
        other_starter.to_owned()
    } else {
        starter.to_owned()
    };

    match request.action_context {
        whiskers_launcher_rs::api::extensions::ActionContext::ResultsRequest => {
            let mut results = Vec::<WLResult>::new();

            results.push(WLResult::new_text(TextResult::new(
                "Text Result",
                Action::new_ignore(),
            )));

            results.push(WLResult::new_text(
                TextResult::new("Full Text Result", Action::new_ignore())
                    .icon(get_image("pokeball.svg"))
                    .tint("accent"),
            ));

            results.push(WLResult::new_title_and_description(
                TitleAndDescriptionResult::new(
                    "Text And Description Result",
                    "Just a small description",
                    Action::new_ignore(),
                ),
            ));

            results.push(WLResult::new_title_and_description(
                TitleAndDescriptionResult::new(
                    "Full Text And Description Result",
                    "Just a small description",
                    Action::new_ignore(),
                )
                .icon(get_image("pokeball.svg"))
                .tint("accent"),
            ));

            results.push(WLResult::new_divider());

            results.push(WLResult::new_text(
                TextResult::new(
                    format!("Your starter is {}", &pokemon_name),
                    Action::new_ignore(),
                )
                .icon(get_image("pokeball.svg"))
                .tint("accent"),
            ));

            results.push(WLResult::new_text(
                TextResult::new(
                    format!("Your starter nickname is {}", &nickname),
                    Action::new_ignore(),
                )
                .icon(get_image("pokeball.svg"))
                .tint("accent"),
            ));

            results.push(WLResult::new_text(
                TextResult::new(
                    format!("Your pokemon description is {}", &description),
                    Action::new_ignore(),
                )
                .icon(get_image("pokeball.svg"))
                .tint("accent"),
            ));

            results.push(WLResult::new_divider());

            results.push(WLResult::new_text(
                TextResult::new(
                    "Copy starter name",
                    Action::new_copy(CopyAction::new(&starter)),
                )
                .icon(get_image("copy.svg"))
                .tint("accent"),
            ));

            results.push(WLResult::new_text(
                TextResult::new(
                    "Search for starter in serebii",
                    Action::new_open_url(OpenURLAction::new(format!(
                        "https://www.serebii.net/search.shtml?q={}&sa=Search",
                        &starter
                    ))),
                )
                .icon(get_image("celebi.png")),
            ));

            results.push(WLResult::new_text(
                TextResult::new(
                    "Notify starter name",
                    Action::new_extension(ExtensionAction::new(EXTENSION_ID, "notify-starter")),
                )
                .icon(get_image("pokeball.svg"))
                .tint("accent"),
            ));

            results.push(WLResult::new_divider());

            let mut fields = Vec::<Field>::new();

            fields.push(Field::new_input(
                "input",
                InputField::new("", "Input Field", "This is a input field")
                    .placeholder("This is a placeholder"),
            ));

            fields.push(Field::new_text_area(
                "text-area",
                TextAreaField::new("", "Text Area Field", "This is a text area field")
                    .placeholder("This is a placeholder"),
            ));

            fields.push(Field::new_toggle(
                "toggle",
                ToggleField::new(true, "Toggle Field", "This is a toggle field"),
            ));

            fields.push(Field::new_select(
                "select",
                SelectField::new(
                    "1",
                    "Select Field",
                    "This is a select field",
                    vec![
                        SelectOption::new("1", "Option 1"),
                        SelectOption::new("2", "Option 2"),
                        SelectOption::new("3", "Option 3"),
                    ],
                ),
            ));

            fields.push(Field::new_file_picker(
                "file-picker",
                FilePickerField::new("File Picker", "Pick a image file").filters(vec![
                    FileFilter::new(
                        "Image",
                        vec![
                            "png".to_string(),
                            "jpg".to_string(),
                            "jpeg".to_string(),
                            "webp".to_string(),
                        ],
                    ),
                ]),
            ));

            fields.push(Field::new_file_picker(
                "dir-picker",
                FilePickerField::new("Directory Picker", "Pick a directory")
                    .pick_directory(true)
                    .default_path(home_dir().unwrap().into_os_string().into_string().unwrap()),
            ));

            results.push(WLResult::new_text(
                TextResult::new(
                    "Open Dialog",
                    Action::new_dialog(DialogAction::new(
                        EXTENSION_ID,
                        "show-dialog-results",
                        "Dialog Example",
                        "Finish",
                        fields,
                    )),
                )
                .icon(get_image("open.svg"))
                .tint("accent"),
            ));

            send_response(results);
        }
        whiskers_launcher_rs::api::extensions::ActionContext::RunAction => {
            let action = request.extension_action.unwrap();

            match action.as_str() {
                "notify-starter" => {
                    send_notification("Your Starter", &pokemon_name);
                }
                _ => {}
            }
        }
    }
}
