use super::{CallbackQuery, ChosenInlineQuery, InlineQuery, Message, PreCheckoutQuery,
            ShippingQuery};

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_query: Option<ChosenInlineQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
}
