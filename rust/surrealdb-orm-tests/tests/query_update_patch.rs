#[cfg(test)]
mod tests {
    use super::patch;
    use crate::*;

    #[test]
    fn can_build_patch_operation() {
        let email = Field::new("_email");

        let patch_op = patch(email).add("oyelowo@example.com");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/_email', value: 'oyelowo@example.com' }"
        );
    }

    #[test]
    fn gathers_errors_when_invalid_path_is_provided() {
        let email = Field::new("email[WHERE id = 1]");

        let patch_op = patch(email).add("Lowo");

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/email[WHERE id = 1]', value: 'Lowo' }"
        );
    }

    #[test]
    fn gathers_error_when_clauses_uses() {
        get_invalid_paths(Field::new("name[WHERE id = 1]"));
        get_invalid_paths(Field::new("name[WHERE id = 1].first"));
        get_invalid_paths(Field::new("name[0]"));
        get_invalid_paths(Field::new("name[1]"));
        get_invalid_paths(Field::new("name[$]"));
        get_invalid_paths(Field::new("name[*]"));
        get_invalid_paths(Field::new("name->"));
        get_invalid_paths(Field::new("name->writes"));
        get_invalid_paths(Field::new("->writes"));
        get_invalid_paths(Field::new("->"));
        get_invalid_paths(Field::new("->->"));
        get_invalid_paths(Field::new("-"));
        get_invalid_paths(Field::new("_-"));
        get_invalid_paths(Field::new("-something"));
        get_invalid_paths(Field::new("name->writes->book"));
        get_invalid_paths(Field::new("->writes->book"));
        get_invalid_paths(Field::new("user:oye->write->blog:mars"));
        get_invalid_paths(Field::new(
            "->knows->person->(knows WHERE influencer = true)",
        ));
        get_invalid_paths(Field::new("5book"));
        get_invalid_paths(Field::new("-book_"));
        get_invalid_paths(Field::new("*book_"));
        get_invalid_paths(Field::new("$book_"));
        get_invalid_paths(Field::new("%book_"));
        get_invalid_paths(Field::new("&book_"));
        get_invalid_paths(Field::new("#book_"));
        get_invalid_paths(Field::new("@book_"));
        get_invalid_paths(Field::new("(book_"));
        get_invalid_paths(Field::new(")book_"));
        get_invalid_paths(Field::new("book*"));
        get_invalid_paths(Field::new("bo$ok"));
    }

    fn get_invalid_paths(field: Field) {
        let patch_op = patch(field).add("Lowo");

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
    }

    #[test]
    fn can_build_add_operation() {
        let name = Field::new("_name.first");

        let patch_op = patch(name).add("Oyelowo");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/_name/first', value: 'Oyelowo' }"
        );
    }

    #[test]
    fn can_build_change_operation() {
        let name = Field::new("name.first");

        let patch_op = patch(name).change("Oyelowo");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'change', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'change', path: '/name/first', value: 'Oyelowo' }"
        );
    }

    #[test]
    fn can_build_remove_operation() {
        let name = Field::new("name.first");

        let patch_op = patch(name).remove();
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'remove', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'remove', path: '/name/first' }"
        );
    }

    #[test]
    fn can_build_replace_operation() {
        let name = Field::new("name.first.title");

        let patch_op = patch(name).replace("Alien");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'replace', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'replace', path: '/name/first/title', value: 'Alien' }"
        );
    }
}
