use crate::outline::r#enum::Member;

pub(crate) struct Enum {
    pub(in super::super) title: String,
    pub(in super::super) desc: String,
    pub(in super::super) path: Vec<String>,
    pub(in super::super) name: String,
    pub(in super::super) members: Vec<Member>,
}

impl Enum {

    pub(crate) fn title(&self) -> &str {
        self.title.as_str()
    }

    pub(crate) fn desc(&self) -> &str {
        self.desc.as_str()
    }

    pub(crate) fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(crate) fn members(&self) -> &Vec<Member> {
        &self.members
    }

    pub(crate) fn joined_enum_variant_names_for_ts(&self) -> String {
        if self.members().is_empty() {
            "undefined".to_owned()
        } else {
            self.members.iter().map(|m| format!("\"{}\"", m.name)).collect::<Vec<String>>().join(" | ")
        }
    }
}