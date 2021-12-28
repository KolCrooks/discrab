use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::resources::emoji::Emoji;

/**
 * Message Component Object
 * @docs https://discord.com/developers/docs/interactions/message-components#component-object
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageComponent {
    /**
     * Component Type
     * @valid-for All
     */
    #[serde(rename = "type")]
    pub type_: MessageComponentType,
    /**
     * A developer-defined identifier for the component, max 100 characters
     * @valid-for Buttons, Select Menus
     */
    pub custom_id: Option<String>,
    /**
     * Whether the component is disabled, default false
     * @valid-for Buttons, Select Menus
     */
    pub disabled: Option<bool>,
    /**
     * One of button styles
     * @valid-for Buttons
     */
    pub style: Option<MessageButtonStyle>,
    /**
     * Text that appears on the button, max 80 characters
     * @valid-for Buttons
     */
    pub label: Option<String>,
    /**
     * Has: `Name`, `id`, and `animated`
     * @valid-for Buttons
     */
    pub emoji: Option<Emoji>,
    /**
     * A url for link-style buttons
     * @valid-for Buttons
     */
    pub url: Option<String>,
    /**
     * The choices in the select, max 25
     * @valid-for Select Menus
     */
    pub options: Option<Vec<MessageSelectOption>>,
    /**
     * Custom placeholder text if nothing is selected, max 100 characters
     * @valid-for Select Menus
     */
    pub placeholder: Option<String>,
    /**
     * The minimum number of items that must be chosen; default 1, min 0, max 25
     * @valid-for Select Menus
     */
    pub min_values: Option<u32>,
    /**
     * The maximum number of items that can be chosen; default 1, max 25
     * @valid-for Select Menus
     */
    pub max_values: Option<u32>,
    /**
     * A list of child components
     * @valid-for Action Rows
     */
    pub components: Option<Vec<MessageComponent>>,
}

/**
 * Component Type
 * @docs https://discord.com/developers/docs/interactions/message-components#component-object-component-types
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum MessageComponentType {
    /// A container for other components
    ActionRow = 1,
    /// A button object
    Button = 2,
    /// A select menu for picking from choices
    SelectMenu = 3,
}

/**
 * Button Styles
 * @docs https://discord.com/developers/docs/interactions/message-components#button-object-button-styles
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum MessageButtonStyle {
    /**
     * Color: blurple
     * Required Field: custom_id
     */
    Primary = 1,
    /**
     * Color: grey
     * Required Field: custom_id
     */
    Secondary = 2,
    /**
     * Color: green
     * Required Field: custom_id
     */
    Success = 3,
    /**
     * Color: red
     * Required Field: custom_id
     */
    Danger = 4,
    /**
     * Color: grey
     * Required Field: url
     */
    Link = 5,
}

/**
 * Select Option
 * @docs https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageSelectOption {
    /// The user-facing name of the option, max 100 characters
    pub label: String,
    /// The dev-define value of the option, max 100 characters
    pub value: String,
    /// An additional description of the option, max 100 characters
    pub description: Option<String>,
    /// Has: `id`, `name`, and `animated`
    pub emoji: Option<Emoji>,
    /// Will render this option as selected by default
    pub default: Option<bool>,
}
