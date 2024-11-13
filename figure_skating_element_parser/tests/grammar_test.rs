use anyhow::Result;
use figure_skating_element_parser::{parse_elements, ParsedElement};

#[cfg(test)]
mod tests {
    use super::*;

    mod jump {
        use super::*;

        #[test]
        fn valid_single_jump() -> Result<()> {
            let result = parse_elements("3T").expect("Failed to parse jump");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Jump");
            assert_eq!(element.full_name, "Triple Toeloop");
            assert_eq!(element.base_value, 4.2);
            Ok(())
        }

        #[test]
        fn valid_quad_throw_jump() -> Result<()> {
            let result = parse_elements("4S").expect("Failed to parse quad jump");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Jump");
            assert_eq!(element.full_name, "Quad Salchow");
            assert_eq!(element.base_value, 9.7);
            Ok(())
        }
    }

    mod whitespace {
        use super::*;

        #[test]
        fn elements_with_whitespace() -> Result<()> {
            let result = parse_elements("3T  StSq  4Lo  FiDs")
                .expect("Failed to parse elements with whitespace");
            assert_eq!(result.len(), 4);

            // Перевіряємо кожен елемент на коректність
            assert_eq!(result[0].element_type, "Jump");
            assert_eq!(result[0].full_name, "Triple Toeloop");

            assert_eq!(result[1].element_type, "Step Sequence");
            assert_eq!(result[1].full_name, "Step Sequence");

            assert_eq!(result[2].element_type, "Jump");
            assert_eq!(result[2].full_name, "Quad Loop");

            assert_eq!(result[3].element_type, "Death Spiral");
            assert_eq!(result[3].full_name, "Forward Inside Death Spiral");

            Ok(())
        }
    }

    mod spin {
        use super::*;

        #[test]
        fn valid_spin() -> Result<()> {
            let result = parse_elements("USp").expect("Failed to parse spin");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Spin");
            assert_eq!(element.full_name, "Upright Spin");
            assert_eq!(element.base_value, 1.2);
            Ok(())
        }

        #[test]
        fn valid_flying_spin() -> Result<()> {
            let result = parse_elements("FSSp").expect("Failed to parse flying spin");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Spin");
            assert_eq!(element.full_name, "Flying Sit Spin");
            assert_eq!(element.base_value, 2.6);
            Ok(())
        }
    }

    mod step_sequence {
        use super::*;

        #[test]
        fn valid_step_sequence() -> Result<()> {
            let result = parse_elements("StSq").expect("Failed to parse step sequence");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Step Sequence");
            assert_eq!(element.full_name, "Step Sequence");
            assert_eq!(element.base_value, 1.5);
            Ok(())
        }
    }

    mod death_spiral {
        use super::*;

        #[test]
        fn valid_death_spiral() -> Result<()> {
            let result = parse_elements("FiDs").expect("Failed to parse death spiral");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Death Spiral");
            assert_eq!(element.full_name, "Forward Inside Death Spiral");
            assert_eq!(element.base_value, 1.5);
            Ok(())
        }
    }

    mod pair_spin {
        use super::*;

        #[test]
        fn valid_pair_spin() -> Result<()> {
            let result = parse_elements("PSp").expect("Failed to parse pair spin");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Pair Spin");
            assert_eq!(element.full_name, "Pair Spin");
            assert_eq!(element.base_value, 1.7);
            Ok(())
        }
    }

    mod twizzle {
        use super::*;

        #[test]
        fn valid_twizzle() -> Result<()> {
            let result = parse_elements("STw").expect("Failed to parse twizzle");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Twizzle");
            assert_eq!(element.full_name, "Twizzle");
            assert_eq!(element.base_value, 1.0);
            Ok(())
        }
    }

    mod choreographic_element {
        use super::*;

        #[test]
        fn valid_choreographic_lift() -> Result<()> {
            let result = parse_elements("ChLi1").expect("Failed to parse choreographic element");
            assert_eq!(result.len(), 1);
            let element = &result[0];
            assert_eq!(element.element_type, "Choreographic Element");
            assert_eq!(element.full_name, "Choreographic Lift");
            assert_eq!(element.base_value, 1.1);
            Ok(())
        }
    }

    #[test]
    fn test_invalid_element() {
        let result = parse_elements("5X");
        assert!(
            result.is_err(),
            "Expected parsing to fail for invalid element, but it succeeded"
        );
    }
}
