use assets::get_image;
use dirs::picture_dir;
use whiskers_launcher_core::{
    features::{
        core::extensions::{get_extension_request, get_form_response},
        extensions::{get_extension_setting, send_search_results},
    },
    results::{
        CopyImageAction, CopyTextAction, FormField, FormFilePickerField, FormFolderPickerField,
        FormInputField, FormSelectField, FormSelectOption, FormTextAreaField, FormToggleField,
        OpenFormAction, OpenLinkAction, ResultAction, SearchResult, SearchResults,
    },
    utils::send_notification,
};

pub mod assets;

const ID: &str = "lighttigerxiv/extensions-example";

fn main() {
    let request = get_extension_request();

    match request.request_type {
        whiskers_launcher_core::features::extensions::ExtensionRequestType::GetResults => {
            let select_setting = get_extension_setting(ID, "select").unwrap();
            let hidden_input_setting = get_extension_setting(ID, "hidden-input").unwrap();
            let text_area_setting = get_extension_setting(ID, "text-area").unwrap();
            let input_setting = get_extension_setting(ID, "input").unwrap();

            let title_result = SearchResult::new("Title", ResultAction::new_do_nothing_action());

            let title_and_desc_result =
                SearchResult::new("Title", ResultAction::new_do_nothing_action())
                    .set_description("Description");

            let image_result = SearchResult::new("Title", ResultAction::new_do_nothing_action())
                .set_description("Description")
                .set_icon(get_image("sprigatito.png"));

            let icon_result = SearchResult::new("Title", ResultAction::new_do_nothing_action())
                .set_description("Description")
                .set_icon(get_image("puzzle.svg"))
                .set_accent_icon_tint();

            let select_result = SearchResult::new(
                "Select Setting Value",
                ResultAction::new_do_nothing_action(),
            )
            .set_description(&select_setting)
            .set_icon(get_image("settings.svg"))
            .set_accent_icon_tint();

            let hidden_input_result = SearchResult::new(
                "Hidden Input Setting Value",
                ResultAction::new_do_nothing_action(),
            )
            .set_description(&hidden_input_setting)
            .set_icon(get_image("settings.svg"))
            .set_accent_icon_tint();

            let text_area_result = SearchResult::new(
                "Text Area Setting Value",
                ResultAction::new_do_nothing_action(),
            )
            .set_description(&text_area_setting)
            .set_icon(get_image("settings.svg"))
            .set_accent_icon_tint();

            let input_result =
                SearchResult::new("Input Setting Value", ResultAction::new_do_nothing_action())
                    .set_description(&input_setting)
                    .set_icon(get_image("settings.svg"))
                    .set_accent_icon_tint();

            let danger_action = SearchResult::new(
                "Dangerous Action",
                ResultAction::new_do_nothing_action().set_dangerous(true),
            )
            .set_description("This action requires confirmation")
            .set_icon(get_image("hazard.svg"))
            .set_accent_icon_tint();

            let copy_text_action = SearchResult::new(
                "Copy Text Action",
                ResultAction::new_copy_text_action(CopyTextAction::new("bulbasaur")),
            )
            .set_description("This action will copy 'bulbasaur' to the clipboard")
            .set_icon(get_image("copy.svg"))
            .set_accent_icon_tint();

            let copy_image_action = SearchResult::new(
                "Copy Image Action",
                ResultAction::new_copy_image_action(CopyImageAction::new(get_image(
                    "sprigatito.png",
                ))),
            )
            .set_description("This action will copy a sprigatito image to the clipboard")
            .set_icon(get_image("copy.svg"))
            .set_accent_icon_tint();

            let open_link_action = SearchResult::new(
                "Open Link Action",
                ResultAction::new_open_link_action(OpenLinkAction::new("https://www.youtube.com/")),
            )
            .set_description("This action will open youtube on the default browser")
            .set_icon(get_image("open.svg"))
            .set_accent_icon_tint();

            let form_fields = vec![
                FormField::new_input_field(
                    "input-field",
                    FormInputField::new("Input Field", "An input field example")
                        .set_placeholder("Input Placeholder")
                        .set_not_empty_validation(),
                ),
                FormField::new_input_field(
                    "number-input-field",
                    FormInputField::new("Number Input Field", "An input field example")
                        .set_placeholder("Input Placeholder")
                        .set_is_number_validation()
                        .set_not_empty_validation(),
                ),
                FormField::new_text_area_field(
                    "text-area-field",
                    FormTextAreaField::new("Text Area Field", "A text area field example")
                        .set_placeholder("Text Area placeholder"),
                ),
                FormField::new_toggle_field(
                    "toggle-field",
                    FormToggleField::new("Toggle Field", "A toggle field example", true),
                ),
                FormField::new_select_field(
                    "select-field",
                    FormSelectField::new(
                        "Select Field",
                        "A select field example",
                        "option-one",
                        vec![
                            FormSelectOption::new("option-one", "Option One"),
                            FormSelectOption::new("option-two", "Option Two"),
                            FormSelectOption::new("option-three", "Option Three"),
                            FormSelectOption::new("option-four", "Option Four"),
                        ],
                    ),
                ),
                FormField::new_file_picker_field(
                    "file-picker-field",
                    FormFilePickerField::new("File Picker Field", "Pick a random file")
                        .set_not_empty_validation(),
                ),
                FormField::new_file_picker_field(
                    "image-file-picker-field",
                    FormFilePickerField::new("Image File Picker Field", "Pick a random image")
                        .set_image_file_types(),
                ),
                FormField::new_folder_picker_field(
                    "folder-picker-field",
                    FormFolderPickerField::new("Folder Picker Field", "Pick a random folder")
                        .set_folder_path(picture_dir().unwrap()),
                ),
            ];

            let form_action = SearchResult::new(
                "Open Form",
                ResultAction::new_open_form_action(
                    OpenFormAction::new(ID, "show-form-results", form_fields)
                        .set_action_text("Test"),
                ),
            )
            .set_description("This action will open a form")
            .set_icon(get_image("open.svg"))
            .set_accent_icon_tint();

            let results = vec![
                title_result,
                title_and_desc_result,
                image_result,
                icon_result,
                select_result,
                hidden_input_result,
                text_area_result,
                input_result,
                danger_action,
                copy_text_action,
                copy_image_action,
                open_link_action,
                form_action,
            ];

            send_search_results(SearchResults::new_list_results(results));
        }
        whiskers_launcher_core::features::extensions::ExtensionRequestType::RunCommand => {
            let response = get_form_response();

            let input = response.get_result("input-field").unwrap().field_value;

            let number_input = response
                .get_result("number-input-field")
                .unwrap()
                .field_value;

            let text_area = response.get_result("text-area-field").unwrap().field_value;

            let toggle = response.get_result("toggle-field").unwrap().field_value;

            let select = response.get_result("select-field").unwrap().field_value;

            let file_picker = response
                .get_result("file-picker-field")
                .unwrap()
                .field_value;

            let image_file_picker = response
                .get_result("image-file-picker-field")
                .unwrap()
                .field_value;

            let folder_picker = response
                .get_result("folder-picker-field")
                .unwrap()
                .field_value;

            send_notification("Results", format!("Input: {}\nNumber Input: {}\nTextArea: {}\nToggle: {}\nSelect: {}\nFile Picker: {}\nImage File Picker:{}\nFolder Picker: {}\n",input, number_input, text_area, toggle, select, file_picker, image_file_picker, folder_picker));
        }
    }
}
