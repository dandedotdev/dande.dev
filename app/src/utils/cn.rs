macro_rules! cn {
    ($($class:expr),* $(,)?) => {
        tailwind_fuse::tw_merge!($($class),*)
    };
}

// To limit the scope of the macro
// https://stackoverflow.com/questions/71505313/how-should-i-use-macro-export-for-a-custom-module
pub(crate) use cn;

#[cfg(test)]
mod tests {
    #[test]
    fn test_single_class() {
        assert_eq!(cn!("px-4 py-2"), "px-4 py-2")
    }

    #[test]
    fn test_multi_classes() {
        assert_eq!(cn!("px-4", "py-2"), "px-4 py-2")
    }

    #[test]
    fn test_option_classes() {
        assert_eq!(cn!("px-4", Some("py-2")), "px-4 py-2")
    }

    #[test]
    fn test_merge_classes() {
        assert_eq!(cn!("px-4", "py-2", "p-6"), "p-6")
    }

    #[test]
    fn test_conditional_classes() {
        let is_hidden = true;

        assert_eq!(
            cn!("px-4 py-2", is_hidden.then_some("hidden"),),
            "px-4 py-2 hidden"
        );
    }
}
