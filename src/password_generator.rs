extern crate passwords;

pub mod password_generator {
    use std::convert::TryFrom;
    use gtk::{ContainerExt, Label, LabelBuilder, Button, WidgetExt, ButtonBuilder, Builder, ButtonExt, Fixed, CheckButtonBuilder, TextView, TextViewExt, TextBuffer, CheckButton, ToggleButtonExt, ComboBox, TextBufferBuilder};
    use gtk::prelude::{BuilderExtManual, ComboBoxExtManual};
    use super::passwords::PasswordGenerator;

    fn generate(length: usize,
                numbers: bool,
                lowercase: bool,
                uppercase: bool,
                symbols: bool,
                spaces: bool,
                similar: bool,
                strict: bool,
    ) -> String {
        let password_generator = PasswordGenerator::new()
            .length(length)
            .numbers(numbers)
            .lowercase_letters(lowercase)
            .uppercase_letters(uppercase)
            .symbols(symbols)
            .spaces(spaces)
            .exclude_similar_characters(similar)
            .strict(strict);
        let result_generated_password = password_generator.generate_one();

        let password = match result_generated_password {
            Ok(password) => password,
            Err(error) => panic!("Problem generating the password: {:?}", error),
        };
        println!("Pwd: {}", password);
        return password;
    }

    pub fn load(builder: &Builder) {
        // Initialize glade form
        let password_fixed: Fixed = builder.get_object("password_fixed").unwrap();
        let combo_length: ComboBox = builder.get_object("combo_length").unwrap();
        let checkbox_symbols: CheckButton = builder.get_object("check_button_symbols").unwrap();
        let checkbox_numbers: CheckButton = builder.get_object("check_button_numbers").unwrap();
        let checkbox_lowercase: CheckButton = builder.get_object("check_button_lowercase").unwrap();
        let checkbox_uppercase: CheckButton = builder.get_object("check_button_uppercase").unwrap();
        let checkbox_similar: CheckButton = builder.get_object("check_button_similar").unwrap();
        let checkbox_spaces: CheckButton = builder.get_object("check_button_spaces").unwrap();
        let checkbox_strict: CheckButton = builder.get_object("check_button_strict").unwrap();

        checkbox_symbols.set_active(true);

        // Password Length:
        // Include Symbols: ( e.g. @#$% )
        // Include Numbers: ( e.g. 123456 )
        // Include Lowercase Characters: ( e.g. abcdefgh )
        // Include Uppercase Characters: ( e.g. ABCDEFGH )
        // Exclude Similar Characters:( e.g. i, l, 1, L, o, 0, O )
        // Exclude Ambiguous Characters: ( { } [ ] ( ) / \ ' " ` ~ , ; : . < > )

        let button: Button = builder.get_object("button_generate_password").unwrap();
        let text_password: TextView = builder.get_object("text_password").unwrap();


        button.connect_clicked(move |_| {
            let length = usize::try_from(combo_length.get_active().unwrap()).unwrap();
            let generated_password = generate(
                length,
                checkbox_numbers.get_active(),
                checkbox_lowercase.get_active(),
                checkbox_uppercase.get_active(),
                checkbox_symbols.get_active(),
                checkbox_spaces.get_active(),
                checkbox_similar.get_active(),
                checkbox_strict.get_active(),
            );

            let string = &["<span color='green'>", &generated_password, "</span>"].concat();

            let buffer = TextBufferBuilder::new()
                .text(string)
                .build();

            // buffer.cre

            text_password.set_buffer(Some(&buffer));
        });

        println!("Password Generator loaded");
    }
}