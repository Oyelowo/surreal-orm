def test_default_table_name_and_transaction():
    from surreal_orm import Node
    from surreal_orm.core import transaction

    @Node
    class SpaceShip:
        id: int
        name: str

    # default table name should be snake_case of class name
    assert hasattr(SpaceShip, "__table_name__")
    assert getattr(SpaceShip, "__table_name__") == "space_ship"

    # transaction helper wraps statements
    t = transaction("SELECT 1;", "SELECT 2;")
    assert t.startswith("BEGIN;")
    assert "COMMIT;" in t
