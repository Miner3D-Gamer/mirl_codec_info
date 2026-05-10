/// Static info about the parser
pub trait StaticInfo {
    /// The name of the current parser
    const NAME: &'static str;
    /// What file formats it supports
    const SUPPORTED_EXTENSIONS: &'static [&'static str];
    // /// If comments are allowed and as such should be purged
    // fn are_comments_allowed() -> bool;
}
/// Dyn compatible info about the parser
pub const trait DynInfo {
    /// The name of the current parser
    fn get_name(&self) -> String;
    /// What file formats it supports
    fn get_supported_extension(&self) -> Vec<String>;
    //     /// If comments are allowed and as such should be purged
    //     fn are_comments_allowed(&mut self) -> bool;
}

impl<T: StaticInfo> DynInfo for T {
    fn get_name(&self) -> String {
        Self::NAME.to_string()
    }

    fn get_supported_extension(&self) -> Vec<String> {
        Self::SUPPORTED_EXTENSIONS
            .iter()
            .map(std::string::ToString::to_string)
            .collect()
    }
    // fn are_comments_allowed(&mut self) -> bool {
    //     Self::are_comments_allowed()
    // }
}
