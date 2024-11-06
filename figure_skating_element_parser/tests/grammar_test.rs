#[cfg(test)]
mod tests {
    use figure_skating_element_parser::*;

    #[test]
    fn test_single_jump_parsing() {
        let element = parse_element("3A").unwrap();
        assert_eq!(element.full_name, "Triple Axel");
        assert_eq!(element.base_value, 8.0);
        assert_eq!(element.element_type, "Jump");
    }

    #[test]
    fn test_spin_parsing() {
        let element = parse_element("USp").unwrap();
        assert_eq!(element.full_name, "Upright Spin");
        assert_eq!(element.base_value, 1.0);
        assert_eq!(element.element_type, "Spin");
    }

    #[test]
    fn test_step_sequence() {
        let element = parse_element("StSq").unwrap();
        assert_eq!(element.full_name, "Step Sequence");
        assert_eq!(element.base_value, 1.5);
        assert_eq!(element.element_type, "Step Sequence");
    }
}