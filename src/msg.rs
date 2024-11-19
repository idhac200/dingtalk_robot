pub use crate::at::At;
use serde::Serialize;

#[derive(Serialize)]
struct Text {
    content: String,
}

#[derive(Serialize)]
struct Markdown {
    title: String,
    text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum MsgData {
    Text(Text),
    Markdown(Markdown),
}

impl MsgData {
    fn typ(&self) -> &str {
        match self {
            MsgData::Text(_) => "text",
            MsgData::Markdown(_) => "markdown",
        }
    }

    fn can_at(&self) -> bool {
        match self {
            MsgData::Text(_) => true,
            MsgData::Markdown(_) => true,
        }
    }

    fn new_text(content: impl Into<String>) -> Self {
        let text = Text {
            content: content.into(),
        };
        Self::Text(text)
    }

    fn new_markdown<T, U>(title: T, text: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        let markdown = Markdown {
            title: title.into(),
            text: text.into(),
        };
        Self::Markdown(markdown)
    }
}

#[derive(Serialize)]
pub struct Msg {
    #[serde(rename = "msgtype")]
    msg_type: String,
    #[serde(flatten)]
    msg_data: MsgData,
    #[serde(skip_serializing_if = "At::is_empty")]
    at: At,
}
impl Msg {
    fn new(msg_data: MsgData) -> Self {
        Self {
            msg_type: msg_data.typ().to_string(),
            msg_data,
            at: At::new(),
        }
    }
    pub fn new_text(content: impl Into<String>) -> Self {
        Self::new(MsgData::new_text(content))
    }

    pub fn new_markdown<T, U>(title: T, text: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        Self::new(MsgData::new_markdown(title, text))
    }

    pub fn at(&mut self, at: At) {
        if self.msg_data.can_at() {
            self.at = at;
        }
    }
}
