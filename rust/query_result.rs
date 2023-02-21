```rs
cityQueryyy result: Query {
    router: Ok(
        Router {
            conn: PhantomData<surrealdb::api::engine::local::Db>,
            sender: Sender,
            last_id: 0,
            features: {
                Backup,
            },
        },
    ),
    query: [
        Ok(
            [
                Create(
                    CreateStatement {
                        what: Values(
                            [
                                Thing(
                                    Thing {
                                        tb: "city",
                                        id: String(
                                            "mars",
                                        ),
                                    },
                                ),
                            ],
                        ),
                        data: Some(
                            SetExpression(
                                [
                                    (
                                        Idiom(
                                            [
                                                Field(
                                                    Ident(
                                                        "name",
                                                    ),
                                                ),
                                            ],
                                        ),
                                        Equal,
                                        Strand(
                                            Strand(
                                                "Mars",
                                            ),
                                        ),
                                    ),
                                    (
                                        Idiom(
                                            [
                                                Field(
                                                    Ident(
                                                        "centre",
                                                    ),
                                                ),
                                            ],
                                        ),
                                        Equal,
                                        Geometry(
                                            Point(
                                                Point(
                                                    Coord {
                                                        x: -0.118092,
                                                        y: 51.509865,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ],
                            ),
                        ),
                        output: None,
                        timeout: None,
                        parallel: false,
                    },
                ),
            ],
        ),
    ],
    bindings: Ok(
        {},
    ),
}

```