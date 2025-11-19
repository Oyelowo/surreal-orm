from surreal_orm.query_builder import (
    md5,
    sha1,
    sha256,
    sha512,
    argon2,
    bcrypt,
    pbkdf2,
    scrypt,
    field,
    Param,
)


def test_md5_with_str_and_field():
    r1 = md5("Oyelowo")
    assert r1.fine_tune_params() == "crypto::md5($_param_00000001)"
    assert r1.to_raw().build() == "crypto::md5('Oyelowo')"

    t = field("title")
    r2 = md5(t)
    assert r2.fine_tune_params() == "crypto::md5(title)"
    assert r2.to_raw().build() == "crypto::md5(title)"


def test_sha_variants_with_str():
    for fn in (sha1, sha256, sha512):
        r = fn("Oyelowo")
        assert r.fine_tune_params() == f"crypto::{fn.__name__}($_param_00000001)"
        assert r.to_raw().build() == f"crypto::{fn.__name__}('Oyelowo')"


def test_argon2_compare_and_generate():
    # compare with raw values
    r = argon2.compare("hashval", "passval")
    assert r.fine_tune_params() == "crypto::argon2::compare($_param_00000001, $_param_00000002)"
    assert r.to_raw().build() == "crypto::argon2::compare('hashval', 'passval')"

    # compare with Params
    h = Param("hash")
    p = Param("pass")
    r2 = argon2.compare(h, p)
    assert r2.fine_tune_params() == "crypto::argon2::compare($hash, $pass)"
    assert r2.to_raw().build() == "crypto::argon2::compare($hash, $pass)"

    # generate
    g = argon2.generate("Oyelowo")
    assert g.fine_tune_params() == "crypto::argon2::generate($_param_00000001)"
    assert g.to_raw().build() == "crypto::argon2::generate('Oyelowo')"


def test_pbkdf2_scrypt_bcrypt_variants():
    # pbkdf2
    r = pbkdf2.compare("h", "p")
    assert r.fine_tune_params() == "crypto::pbkdf2::compare($_param_00000001, $_param_00000002)"
    assert r.to_raw().build() == "crypto::pbkdf2::compare('h', 'p')"
    g = pbkdf2.generate("x")
    assert g.fine_tune_params() == "crypto::pbkdf2::generate($_param_00000001)"
    assert g.to_raw().build() == "crypto::pbkdf2::generate('x')"

    # scrypt
    r2 = scrypt.compare("h", "p")
    assert r2.fine_tune_params() == "crypto::scrypt::compare($_param_00000001, $_param_00000002)"
    assert r2.to_raw().build() == "crypto::scrypt::compare('h', 'p')"
    g2 = scrypt.generate("x")
    assert g2.fine_tune_params() == "crypto::scrypt::generate($_param_00000001)"
    assert g2.to_raw().build() == "crypto::scrypt::generate('x')"

    # bcrypt
    r3 = bcrypt.compare("h", "p")
    assert r3.fine_tune_params() == "crypto::bcrypt::compare($_param_00000001, $_param_00000002)"
    assert r3.to_raw().build() == "crypto::bcrypt::compare('h', 'p')"
    g3 = bcrypt.generate("x")
    assert g3.fine_tune_params() == "crypto::bcrypt::generate($_param_00000001)"
    assert g3.to_raw().build() == "crypto::bcrypt::generate('x')"
