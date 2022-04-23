use crate::api::{ApplicationCommandOptionType, ApplicationCommandOption, channel::typing::ChannelType, ApplicationCommandOptionChoice, ApplicationCommandOptionChoiceValue};
use paste::paste;

pub struct OptionBuilder {}

macro_rules! OptionBuilderBuilder {
    ($($builder: ident, $type_: ident, $name: ident;)+) => {
        $(
        pub struct $builder {
            option: ApplicationCommandOption,
        }
        impl $builder {
            pub fn new(name: String) -> Self {
                return Self {
                    option: ApplicationCommandOption {
                        autocomplete: false,
                        channel_types: None,
                        choices: None,
                        description: None,
                        max_value: None,
                        min_value: None,
                        name,
                        options: None,
                        required: false,
                        type_: ApplicationCommandOptionType::$type_,
                    }
                };
            }
            /// if the parameter is required or optional--default false
            #[must_use]
            pub fn required(mut self, required: bool) -> Self {
                self.option.required = required;
                return self;
            }
            /// Enables or disables autocomplete for this option
            #[must_use]
            pub fn autocomplete(mut self, autocomplete:bool) -> Self {
                self.option.autocomplete = autocomplete;
                self
            }

            /// Sets the description of the option
            #[must_use]
            pub fn description(mut self, description:String) -> Self {
                self.option.description = Some(description);
                self
            }

            pub fn build(self) -> ApplicationCommandOption {
                self.option
            }
        }
    )+

        impl OptionBuilder {
            paste! {
                $(
                /// Creates a new builder option builder for the given type
                /// @param name the name of the option
                pub fn [<new_ $name>](name: String) -> $builder {
                    return $builder::new(name);
                }
                )+
            }
        }

        impl ApplicationCommandOption {
            paste! {
                $(
                /// Creates a new builder option builder for the given type
                /// @param name the name of the option
                pub fn [<builder_ $name>](name: String) -> $builder {
                    return $builder::new(name);
                }
                )+
            }
        }
    };
}

OptionBuilderBuilder! {
    BooleanOptionBuilder, Boolean, bool;
    ChannelOptionBuilder, Channel, channel;
    IntegerOptionBuilder, Integer, int;
    NumberOptionBuilder, Number, num;
    StringOptionBuilder, String, str;
    UserOptionBuilder, User, user;
    RoleOptionBuilder, Role, role;
    MentionableOptionBuilder, Mentionable, mentionable;
}


impl ChannelOptionBuilder {
    /// If the option type is channel, this filters what channel types are accepted by the option
        /// @param channel_types the channel types to accept
        #[must_use]
        pub fn channel_types(mut self, channel_types:Vec<ChannelType>) -> Self {
            self.option.channel_types = Some(channel_types);
            self
        }
}

/// A helper type for the option builder to limit what types of options the user can
/// choose from
pub struct LimitedOptionChoice<T> {
    name: String,
    value: T,
}

impl From<LimitedOptionChoice<String>> for ApplicationCommandOptionChoice {
    fn from(choice: LimitedOptionChoice<String>) -> Self {
        Self {
            name: choice.name,
            value: ApplicationCommandOptionChoiceValue::String(choice.value),
        }
    }
}
impl From<LimitedOptionChoice<i64>> for ApplicationCommandOptionChoice {
    fn from(choice: LimitedOptionChoice<i64>) -> Self {
        Self {
            name: choice.name,
            value: ApplicationCommandOptionChoiceValue::Integer(choice.value),
        }
    }
}
impl From<LimitedOptionChoice<f64>> for ApplicationCommandOptionChoice {
    fn from(choice: LimitedOptionChoice<f64>) -> Self {
        Self {
            name: choice.name,
            value: ApplicationCommandOptionChoiceValue::Number(choice.value),
        }
    }
}


impl StringOptionBuilder {
    /// List of choices for the user to pick from, max 25
    /// Means that the user can't give a custom value
    #[must_use]
    pub fn choices(mut self, choices:Vec<LimitedOptionChoice<String>>) -> Self {
        self.option.choices = Some(choices.into_iter().map(LimitedOptionChoice::into).collect());
        self
    }
}

impl IntegerOptionBuilder {
    /// List of choices for the user to pick from, max 25
    /// Means that the user can't give a custom value
    #[must_use]
    pub fn choices(mut self, choices:Vec<LimitedOptionChoice<i64>>) -> Self {
        self.option.choices = Some(choices.into_iter().map(LimitedOptionChoice::into).collect());
        self
    }
    /// If the option is an integer,
    #[must_use]
    pub fn clamp(mut self, min:i64, max:i64) -> Self {
        self.option.min_value = Some(min as f64);
        self.option.max_value = Some(max as f64);
        self
    }

    /// Set the maximum value for the option
    #[must_use]
    pub fn max_value(mut self, max_value:i64) -> Self {
        self.option.max_value = Some(max_value as f64);
        self
    }

    /// Set the minimum value for the option
    #[must_use]
    pub fn min_value(mut self, min_value:i64) -> Self {
        self.option.min_value = Some(min_value as f64);
        self
    }
}

impl NumberOptionBuilder {
    /// List of choices for the user to pick from, max 25
    /// Means that the user can't give a custom value
    #[must_use]
    pub fn choices(mut self, choices:Vec<LimitedOptionChoice<f64>>) -> Self {
        self.option.choices = Some(choices.into_iter().map(LimitedOptionChoice::into).collect());
        self
    }
    
    /// If the option is an integer,
    #[must_use]
    pub fn clamp(mut self, min:f64, max:f64) -> Self {
        self.option.min_value = Some(min);
        self.option.max_value = Some(max);
        self
    }

    /// Set the maximum value for the option
    #[must_use]
    pub fn max_value(mut self, max_value:f64) -> Self {
        self.option.max_value = Some(max_value);
        self
    }

    /// Set the minimum value for the option
    #[must_use]
    pub fn min_value(mut self, min_value:f64) -> Self {
        self.option.min_value = Some(min_value);
        self
    }
}