use assets::get_image;
use whiskers_launcher_rs::{
    actions::{self, Action},
    api::{
        self,
        extensions::{
            get_extension_context, get_extension_dialog_result, get_extension_setting,
            send_extension_results,
        },
    },
    dialog::{self, DialogField, SelectField},
    others::send_notification,
    results::{self, WhiskersResult},
};

pub mod assets;

const EXTENSION_ID: &str = "lighttigerxiv/extensions-example";

fn main() {
    let context = get_extension_context().unwrap();
    let favourite_pokemon: String =
        get_extension_setting(EXTENSION_ID, "favourite_pokemon").unwrap();

    match context.action {
        api::extensions::Action::GetResults => {
            let mut results: Vec<WhiskersResult> = vec![];

            // =============================================================
            // Text Results
            // =============================================================

            results.push(WhiskersResult::Text(results::Text::new(
                "Text Result",
                actions::Action::Nothing,
            )));

            results.push(WhiskersResult::Text(
                results::Text::new("Text Result With A Icon", actions::Action::Nothing)
                    .icon(get_image("sprigatito.png")),
            ));

            results.push(WhiskersResult::Text(
                results::Text::new(
                    "Text Result With A Tinted Accent Icon",
                    actions::Action::Nothing,
                )
                .icon(get_image("pokeball.svg"))
                .tint_icon(true),
            ));

            results.push(WhiskersResult::Text(
                results::Text::new(
                    "Text Result With A Custom Tinted Icon",
                    actions::Action::Nothing,
                )
                .icon(get_image("pokeball.svg"))
                .tint_icon(true)
                .tint_color("#FF0000"),
            ));

            results.push(WhiskersResult::Divider);

            // =============================================================
            // Title And Text Results
            // =============================================================

            results.push(WhiskersResult::TitleAndText(results::TitleAndText::new(
                "Title",
                "Text",
                actions::Action::Nothing,
            )));

            results.push(WhiskersResult::TitleAndText(
                results::TitleAndText::new(
                    "Title And Text With Icon",
                    "Text",
                    actions::Action::Nothing,
                )
                .icon(get_image("sprigatito.png")),
            ));

            results.push(WhiskersResult::TitleAndText(
                results::TitleAndText::new(
                    "Title And Text With Tinted Accent Icon",
                    "Text",
                    actions::Action::Nothing,
                )
                .icon(get_image("pokeball.svg"))
                .tint_icon(true),
            ));

            results.push(WhiskersResult::TitleAndText(
                results::TitleAndText::new(
                    "Title And Text With A Custom Tinted Icon",
                    "Text",
                    actions::Action::Nothing,
                )
                .icon(get_image("pokeball.svg"))
                .tint_icon(true)
                .tint_color("#00FF00"),
            ));

            results.push(WhiskersResult::Divider);

            results.push(WhiskersResult::Text(
                results::Text::new(
                "Open favourite pokemon in serebii",
                actions::Action::OpenUrl(
                    actions::OpenUrl::new(
                    format!("https://www.serebii.net/search.shtml?cx=018410473690156091934%3A6gahkiyodbi&cof=FORID%3A11&q={}&sa=Search",
                            favourite_pokemon.to_owned())
                    )
                ))
                    .icon(get_image("celebi.png"))));

            results.push(WhiskersResult::Divider);

            // =============================================================
            // Dialog Exmple
            // =============================================================

            results.push(WhiskersResult::Text(
                results::Text::new(
                    format!("Copy Favourite Pokemon ({})", favourite_pokemon.to_owned()),
                    actions::Action::CopyToClipboard(actions::CopyToClipboard::new(
                        favourite_pokemon.to_owned(),
                    )),
                )
                .icon(get_image("copy.svg"))
                .tint_icon(true),
            ));

            results.push(WhiskersResult::Text(
                results::Text::new(
                    "Custom Extension Action",
                    actions::Action::Extension(actions::Extension::new(
                        EXTENSION_ID,
                        "notify_favourite",
                    )),
                )
                .icon(get_image("puzzle.svg"))
                .tint_icon(true),
            ));

            results.push(WhiskersResult::Text(
                results::Text::new(
                    "Custom Extension Action With Arguments",
                    actions::Action::Extension(
                        actions::Extension::new(EXTENSION_ID, "notify_custom").args(vec![
                            "Mewtwo".to_string(),
                            "Lugia".to_string(),
                            "Miraidon".to_string(),
                        ]),
                    ),
                )
                .icon(get_image("puzzle.svg"))
                .tint_icon(true),
            ));

            let mut dialog_fields: Vec<DialogField> = vec![];

            dialog_fields.push(DialogField::Input(
                dialog::Input::new("best_pokemon", "Best Pokemon", "")
                    .placeholder("Best pokemon")
                    .description("Type the best pokemon in your opinion"),
            ));

            dialog_fields.push(DialogField::TextArea(
                dialog::TextArea::new("describe_favourite", "Describe Favourite Pokemon", "")
                    .description("Describe your favourite pokemon")
                    .placeholder("Favourite Mon :P"),
            ));

            dialog_fields.push(DialogField::Toggle(
                dialog::Toggle::new("pokemon_master", "Are you a pokemon master?", true)
                    .description("Toggle if you are pokemon master :D"),
            ));

            dialog_fields.push(DialogField::SelectFile(
                dialog::SelectFile::new("select_file_example", "Select File")
                    .description("Select a random picture")
                    .filters(vec![dialog::FileFilter::new("Image Files")
                        .add_extension("png")
                        .add_extension("jpg")
                        .add_extension("jepg")
                        .add_extension("svg")]),
            ));

            dialog_fields.push(DialogField::SelectFile(
                dialog::SelectFile::new("select_dialog_example", "Select Directory")
                    .description("Select a random directory")
                    .select_dir(true),
            ));

            let mut starters: Vec<SelectField> = vec![];
            starters.push(SelectField::new("litten", "Litten"));
            starters.push(SelectField::new("rowlet", "Rowlet"));
            starters.push(SelectField::new("popplio", "Popplio"));

            dialog_fields.push(DialogField::Select(
                dialog::Select::new(
                    "best_gen7_starter",
                    "Best gen 7 starter",
                    "litten",
                    starters,
                )
                .description("Type the best pokemon in your opinion"),
            ));

            results.push(WhiskersResult::Text(
                results::Text::new(
                    "Open extension dialog",
                    Action::Dialog(
                        actions::Dialog::new(
                            EXTENSION_ID,
                            "Random Fields About Pokemon",
                            "show_dialog_results",
                            dialog_fields,
                        )
                        .primary_button_text("Custom Button Text"),
                    ),
                )
                .icon(get_image("open.svg"))
                .tint_icon(true),
            ));

            send_extension_results(results);
        }
        api::extensions::Action::RunAction => {
            let action = context.extension_action.unwrap();
            let args = context.custom_args;
            let best_gen_one_starter =
                get_extension_setting(EXTENSION_ID, "best_gen1_pokemon").unwrap();

            if action == "notify_favourite" {
                send_notification(
                    "Favourite Pokemon",
                    format!("Your favourite pokemon is {}", favourite_pokemon.to_owned()),
                )
            }

            if action == "notify_custom" {
                send_notification(
                    "Custom Args",
                    format!("The best gen 1 starter is {}. And some custom lengendary pokemons are: {} - {} - {}", best_gen_one_starter, args[0], args[1], args[2]),
                );
            }

            if action == "show_dialog_results" {
                //let results = get_extension_dialog_results().unwrap();
                println!(
                    "Is pokemon master: {:?}",
                    get_extension_dialog_result("pokemon_master")
                        .unwrap()
                        .as_boolean()
                );
                println!(
                    "Best Pokemon: {:?}",
                    get_extension_dialog_result("best_pokemon").unwrap().value
                );
            }
        }
    }
}
