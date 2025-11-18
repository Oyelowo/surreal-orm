def test_simple_array_build_cases():
    from surreal_orm.query_builder import tags_field, index, Last, All, where_, this

    # last
    last_tag = tags_field("tags")
    last_tag.index = Last
    assert last_tag.fine_tune_params() == "tags[$]"
    assert last_tag.to_raw().build() == "tags[$]"

    # index 0
    idx_tag = tags_field("tags")
    idx_tag.index = index(0)
    assert idx_tag.fine_tune_params() == "tags[$_param_00000001]"
    assert idx_tag.to_raw().build() == "tags[0]"

    # index 1
    idx1_tag = tags_field("tags")
    idx1_tag.index = index(1)
    assert idx1_tag.fine_tune_params() == "tags[$_param_00000001]"
    assert idx1_tag.to_raw().build() == "tags[1]"

    # index with filter greater_than
    idx_gt = tags_field("tags")
    idx_gt.index = index(1)
    op = idx_gt.greater_than(1)
    assert op.to_raw().build() == "tags[1] > 1"
    assert op.fine_tune_params() == "tags[$_param_00000001] > $_param_00000002"

    # all
    all_tag = tags_field("tags")
    all_tag.index = All
    assert all_tag.fine_tune_params() == "tags[*]"
    assert all_tag.to_raw().build() == "tags[*]"

    # where with this < 18
    where_tag = tags_field("tags")
    where_tag.where_clause = where_(this().less_than(18))
    assert where_tag.fine_tune_params() == "tags[WHERE $this < $_param_00000001]"
    assert where_tag.to_raw().build() == "tags[WHERE $this < 18]"

    # where then all
    where_all = tags_field("tags")
    where_all.where_clause = where_(this().less_than(18))
    where_all.dot_star = True
    assert where_all.fine_tune_params() == "tags[WHERE $this < $_param_00000001].*"
    assert where_all.to_raw().build() == "tags[WHERE $this < 18].*"
