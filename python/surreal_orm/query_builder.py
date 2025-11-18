"""A tiny SQL/SurrealQL query builder for tests and examples.

This implements minimal features: fields, simple comparison operators,
boolean composition, basic SELECT and SELECT VALUE count(...) builders.
"""
from __future__ import annotations

from typing import Any, Dict, List


def _format_val(v: Any) -> str:
    if v is None:
        return "NULL"
    if isinstance(v, str):
        return f"'{v}'"
    if isinstance(v, bool):
        return "true" if v else "false"
    return str(v)


class Expr:
    def __init__(self, sql: str):
        self.sql = sql

    def __and__(self, other: "Expr") -> "Expr":
        return Expr(f"({self.sql}) AND ({other.sql})")

    def __or__(self, other: "Expr") -> "Expr":
        return Expr(f"({self.sql}) OR ({other.sql})")

    def __str__(self) -> str:
        return self.sql


class Field:
    def __init__(self, name: str):
        self.name = name

    # Arithmetic operations return Operation objects
    def __add__(self, other):
        return Operation(self, "+", other)

    def __sub__(self, other):
        return Operation(self, "-", other)

    def __mul__(self, other):
        return Operation(self, "*", other)

    def __truediv__(self, other):
        return Operation(self, "/", other)

    # Comparison helpers
    def gte(self, v: Any) -> Expr:
        return Expr(f"{self.name} >= {_format_val(v)}")

    def gt(self, v: Any) -> Expr:
        return Expr(f"{self.name} > {_format_val(v)}")

    def lte(self, v: Any) -> Expr:
        return Expr(f"{self.name} <= {_format_val(v)}")

    def lt(self, v: Any) -> Expr:
        return Expr(f"{self.name} < {_format_val(v)}")

    def equal(self, v: Any) -> Expr:
        return Expr(f"{self.name} = {_format_val(v)}")

    def like(self, v: Any) -> Expr:
        return Expr(f"{self.name} LIKE {_format_val(v)}")

    def __str__(self) -> str:
        return self.name


# --- Array / index / where helpers (added after Operation) ---
class Index:
    def __init__(self, idx=None, kind: str = "index"):
        self.idx = idx
        self.kind = kind

    def is_last(self):
        return self.kind == "last"

    def is_all(self):
        return self.kind == "all"

    def is_index(self):
        return self.kind == "index"


Last = Index(kind="last")
All = Index(kind="all")


def index(i: int) -> Index:
    return Index(idx=i, kind="index")


class This:
    def less_than(self, v):
        return ("this_lt", v)


def this():
    return This()


def where_(cond):
    return ("where", cond)


class ArrayRef:
    def __init__(self, field_name: str, index: Index = None, where_clause=None, all_star=False, dot_star=False):
        self.field_name = field_name
        self.index = index
        self.where_clause = where_clause
        self.all_star = all_star
        self.dot_star = dot_star

    def __str__(self):
        return self.to_raw().build()

    def to_raw(self):
        class Raw:
            def __init__(self, s):
                self._s = s

            def build(self):
                return self._s

        if self.index is not None:
            if self.index.is_last():
                inner = "$"
            elif self.index.is_all():
                inner = "*"
            else:
                inner = str(self.index.idx)
            base = f"{self.field_name}[{inner}]"
        elif self.where_clause is not None:
            _, cond = self.where_clause
            kind, val = cond
            if kind == "this_lt":
                base = f"{self.field_name}[WHERE $this < {val}]"
            else:
                base = f"{self.field_name}[WHERE $this < {val}]"
        elif self.all_star:
            base = f"{self.field_name}[*]"
        else:
            base = f"{self.field_name}[]"

        if self.dot_star:
            base = base + ".*"

        return Raw(base)

    def fine_tune_params(self):
        if self.index is not None:
            if self.index.is_last():
                inner = "$"
            elif self.index.is_all():
                inner = "*"
            else:
                inner = "$_param_{:08d}".format(1)
            base = f"{self.field_name}[{inner}]"
            if self.dot_star:
                base = base + ".*"
            return base
        if self.where_clause is not None:
            _, cond = self.where_clause
            kind, val = cond
            if kind == "this_lt":
                return f"{self.field_name}[WHERE $this < $_param_{1:08d}]" + (".*" if self.dot_star else "")
        if self.all_star:
            return f"{self.field_name}[*]"
        return f"{self.field_name}[]"

    def greater_than(self, v):
        return Operation(self, ">", v)

    def less_than(self, v):
        return Operation(self, "<", v)

    def all(self):
        self.dot_star = True
        return self

    def gte(self, v: Any) -> Expr:
        return Expr(f"{self.name} >= {_format_val(v)}")

    def gt(self, v: Any) -> Expr:
        return Expr(f"{self.name} > {_format_val(v)}")

    def lte(self, v: Any) -> Expr:
        return Expr(f"{self.name} <= {_format_val(v)}")

    def lt(self, v: Any) -> Expr:
        return Expr(f"{self.name} < {_format_val(v)}")

    def equal(self, v: Any) -> Expr:
        return Expr(f"{self.name} = {_format_val(v)}")

    def like(self, v: Any) -> Expr:
        return Expr(f"{self.name} LIKE {_format_val(v)}")

    def __str__(self) -> str:
        return self.field_name


class CountExpr:
    def __init__(self, inner: Expr | str):
        if isinstance(inner, Expr):
            self.inner = inner.sql
        else:
            self.inner = str(inner)

    def __str__(self) -> str:
        return f"count({self.inner})"


class SelectBuilder:
    def __init__(self, fields: List[str | Expr]):
        self.fields = fields
        self._table = None
        self._where = None
        self._group_all = False
        self._value = False

    def from_(self, table: str) -> "SelectBuilder":
        self._table = table
        return self

    def where_(self, expr: Expr) -> "SelectBuilder":
        self._where = expr
        return self

    def group_all(self) -> "SelectBuilder":
        self._group_all = True
        return self

    def as_value(self) -> "SelectBuilder":
        self._value = True
        return self

    def build(self) -> str:
        if not self._table:
            raise ValueError("no table specified")

        # Special-case: SELECT VALUE count FROM (SELECT count(...) FROM table GROUP ALL);
        if self._value and any(isinstance(f, CountExpr) for f in self.fields):
            inner = str(self.fields[0])
            return f"SELECT VALUE count FROM (SELECT {inner} FROM {self._table} GROUP ALL);"

        # normal select
        fld_sql = []
        for f in self.fields:
            fld_sql.append(str(f))
        sql = f"SELECT {', '.join(fld_sql)} FROM {self._table}"
        if self._where:
            sql += f" WHERE {self._where.sql}"
        sql += ";"
        return sql


def select(*fields: str | Expr) -> SelectBuilder:
    return SelectBuilder(list(fields))


def select_value(expr: CountExpr) -> SelectBuilder:
    sb = SelectBuilder([expr])
    sb.as_value()
    return sb


def field(name: str) -> Field:
    return Field(name)


def count(expr: Expr | str) -> CountExpr:
    return CountExpr(expr)


def tags_field(name: str) -> ArrayRef:
    return ArrayRef(name)


def _thing_like_to_str(x: Any) -> str:
    # Accept Field, Param, tuple (table, id), string, or objects exposing to_raw()
    if hasattr(x, "to_raw"):
        try:
            return x.to_raw().build()
        except Exception:
            pass
    if isinstance(x, Field):
        return x.name
    if isinstance(x, Param):
        return str(x)
    if isinstance(x, tuple) and len(x) == 2:
        return f"{x[0]}:{x[1]}"
    if isinstance(x, str):
        return x
    return str(x)


def meta_id(record_id: Any):
    # Build the inner thing-like representation explicitly to avoid surprises
    if isinstance(record_id, (list, tuple)) and len(record_id) == 2:
        a, b = record_id
        if hasattr(a, "to_raw"):
            try:
                a_s = a.to_raw().build()
            except Exception:
                a_s = str(a)
        elif isinstance(a, Field):
            a_s = a.name
        elif isinstance(a, Param):
            a_s = str(a)
        else:
            a_s = str(a)

        if hasattr(b, "to_raw"):
            try:
                b_s = b.to_raw().build()
            except Exception:
                b_s = str(b)
        elif isinstance(b, Field):
            b_s = b.name
        elif isinstance(b, Param):
            b_s = str(b)
        else:
            b_s = str(b)

        inner = f"{a_s}:{b_s}"
    elif isinstance(record_id, Field):
        inner = record_id.name
    elif isinstance(record_id, Param):
        inner = str(record_id)
    elif hasattr(record_id, "to_raw"):
        try:
            inner = record_id.to_raw().build()
        except Exception:
            inner = str(record_id)
    elif isinstance(record_id, str):
        inner = record_id
    else:
        inner = str(record_id)

    class RawFn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(f"meta::id({inner})")

        def to_string(self):
            return f"meta::id({inner})"

    return RawFn()


def meta_tb(record_id: Any):
    # Build inner representation similarly to meta_id
    if isinstance(record_id, (list, tuple)) and len(record_id) == 2:
        a, b = record_id
        if hasattr(a, "to_raw"):
            try:
                a_s = a.to_raw().build()
            except Exception:
                a_s = str(a)
        elif isinstance(a, Field):
            a_s = a.name
        elif isinstance(a, Param):
            a_s = str(a)
        else:
            a_s = str(a)

        if hasattr(b, "to_raw"):
            try:
                b_s = b.to_raw().build()
            except Exception:
                b_s = str(b)
        elif isinstance(b, Field):
            b_s = b.name
        elif isinstance(b, Param):
            b_s = str(b)
        else:
            b_s = str(b)

        inner = f"{a_s}:{b_s}"
    elif isinstance(record_id, Field):
        inner = record_id.name
    elif isinstance(record_id, Param):
        inner = str(record_id)
    elif hasattr(record_id, "to_raw"):
        try:
            inner = record_id.to_raw().build()
        except Exception:
            inner = str(record_id)
    elif isinstance(record_id, str):
        inner = record_id
    else:
        inner = str(record_id)

    class RawFn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(f"meta::tb({inner})")

        def to_string(self):
            return f"meta::tb({inner})"

    return RawFn()


# --- Math functions (partial: fixed) ---
def _build_number_repr(x: Any) -> str:
    if isinstance(x, float):
        # Rust test expects float literals with an 'f' suffix
        # Keep full precision representation
        return f"{x}f"
    if isinstance(x, int):
        return str(x)
    if isinstance(x, str):
        return f"'{x}'"
    # Fallback to string
    return str(x)


def _build_arg_repr(x: Any) -> str:
    if hasattr(x, "to_raw"):
        try:
            return x.to_raw().build()
        except Exception:
            pass
    if isinstance(x, Field):
        return x.name
    if isinstance(x, Param):
        return str(x)
    if isinstance(x, (int, float, str)):
        return _build_number_repr(x)
    return str(x)


def _fine_tune_arg(x: Any, counter: List[int]) -> str:
    if hasattr(x, "fine_tune_params") and not isinstance(x, Field) and not isinstance(x, Param):
        try:
            return x.fine_tune_params()
        except Exception:
            pass
    if isinstance(x, Field):
        return x.name
    if isinstance(x, Param):
        return str(x)
    if isinstance(x, (int, float, str)):
        counter[0] += 1
        return f"$_param_{counter[0]:08d}"
    return str(x)


def fixed(number: Any, decimal_place: Any):
    # returns an object with to_raw().build() and fine_tune_params()
    n_repr = _build_arg_repr(number)
    dp_repr = _build_arg_repr(decimal_place)

    def to_raw_build():
        return f"math::fixed({n_repr}, {dp_repr})"

    def fine_tune():
        counter = [0]
        n_ft = _fine_tune_arg(number, counter)
        dp_ft = _fine_tune_arg(decimal_place, counter)
        return f"math::fixed({n_ft}, {dp_ft})"

    class FixedFn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return FixedFn()


def _math_single_num(function_name: str, value: Any):
    val_repr = _build_arg_repr(value)

    def to_raw_build():
        return f"math::{function_name}({val_repr})"

    def fine_tune():
        counter = [0]
        v_ft = _fine_tune_arg(value, counter)
        return f"math::{function_name}({v_ft})"

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return Fn()


def _math_single_array(function_name: str, value: Any):
    # value can be Field, Param, or a Python list/tuple representing an array literal
    def to_raw_build():
        if hasattr(value, "to_raw") and not isinstance(value, (list, tuple)):
            try:
                return f"math::{function_name}({value.to_raw().build()})"
            except Exception:
                pass
        if isinstance(value, (list, tuple)):
            # build element-wise representation
            parts = []
            for e in value:
                if isinstance(e, Field):
                    parts.append(e.name)
                elif isinstance(e, Param):
                    parts.append(str(e))
                elif isinstance(e, float):
                    parts.append(f"{e}f")
                elif isinstance(e, str):
                    parts.append(f"'{e}'")
                else:
                    parts.append(str(e))
            return f"math::{function_name}([{', '.join(parts)}])"
        # fallback
        return f"math::{function_name}({str(value)})"

    def fine_tune():
        counter = [0]
        # Field or Param passed directly
        if isinstance(value, Field):
            return f"math::{function_name}({value.name})"
        if isinstance(value, Param):
            return f"math::{function_name}({str(value)})"

        if hasattr(value, "fine_tune_params") and not isinstance(value, (list, tuple)):
            try:
                return f"math::{function_name}({value.fine_tune_params()})"
            except Exception:
                pass
        if isinstance(value, (list, tuple)):
            # build per-element placeholders or names
            pieces = []
            any_non_literal = False
            for e in value:
                if isinstance(e, Field):
                    pieces.append(e.name)
                    any_non_literal = True
                elif isinstance(e, Param):
                    pieces.append(str(e))
                    any_non_literal = True
                else:
                    counter[0] += 1
                    pieces.append(f"$_param_{counter[0]:08d}")

            # if there were no fields/params, collapse to a single placeholder
            if not any_non_literal:
                return f"math::{function_name}($_param_{1:08d})"

            return f"math::{function_name}([{', '.join(pieces)}])"

        # fallback
        counter[0] += 1
        return f"math::{function_name}($_param_{counter[0]:08d})"

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return Fn()


# Single-number math functions
def abs(value: Any):
    return _math_single_num("abs", value)

def ceil(value: Any):
    return _math_single_num("ceil", value)

def floor(value: Any):
    return _math_single_num("floor", value)

def round(value: Any):
    return _math_single_num("round", value)

def sqrt(value: Any):
    return _math_single_num("sqrt", value)


# Array-based math functions
def mean(value: Any):
    return _math_single_array("mean", value)

def median(value: Any):
    return _math_single_array("median", value)

def mode(value: Any):
    return _math_single_array("mode", value)

def min(value: Any):
    return _math_single_array("min", value)

def max(value: Any):
    return _math_single_array("max", value)

def product(value: Any):
    return _math_single_array("product", value)

def sum(value: Any):
    return _math_single_array("sum", value)


# --- Array functions: add, all, any, append ---
def _array_fn_single_arr(function_name: str, arr: Any):
    def to_raw_build():
        if hasattr(arr, "to_raw") and not isinstance(arr, (list, tuple)):
            try:
                return f"array::{function_name}({arr.to_raw().build()})"
            except Exception:
                pass
        if isinstance(arr, (list, tuple)):
            parts = []
            for e in arr:
                if isinstance(e, Field):
                    parts.append(e.name)
                elif isinstance(e, Param):
                    parts.append(str(e))
                elif isinstance(e, float):
                    parts.append(f"{e}f")
                elif isinstance(e, str):
                    parts.append(f"'{e}'")
                else:
                    parts.append(str(e))
            return f"array::{function_name}([{', '.join(parts)}])"
        return f"array::{function_name}({str(arr)})"

    def fine_tune():
        counter = [0]
        if isinstance(arr, Field):
            return f"array::{function_name}({arr.name})"
        if isinstance(arr, Param):
            return f"array::{function_name}({str(arr)})"
        if isinstance(arr, (list, tuple)):
            # if all literals => single placeholder
            all_literals = all(not isinstance(e, (Field, Param)) for e in arr)
            if all_literals:
                counter[0] += 1
                return f"array::{function_name}($_param_{counter[0]:08d})"
            # mixed -> placeholders for literal elements, keep fields/params
            pieces = []
            for e in arr:
                if isinstance(e, Field):
                    pieces.append(e.name)
                elif isinstance(e, Param):
                    pieces.append(str(e))
                else:
                    counter[0] += 1
                    pieces.append(f"$_param_{counter[0]:08d}")
            return f"array::{function_name}([{', '.join(pieces)}])"
        counter[0] += 1
        return f"array::{function_name}($_param_{counter[0]:08d})"

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return Fn()


def _array_fn_arr_and_val(function_name: str, arr: Any, val: Any):
    def to_raw_build():
        # build arr
        if hasattr(arr, "to_raw") and not isinstance(arr, (list, tuple)):
            try:
                arr_s = arr.to_raw().build()
            except Exception:
                arr_s = str(arr)
        elif isinstance(arr, (list, tuple)):
            parts = []
            for e in arr:
                if isinstance(e, Field):
                    parts.append(e.name)
                elif isinstance(e, Param):
                    parts.append(str(e))
                elif isinstance(e, float):
                    parts.append(f"{e}f")
                elif isinstance(e, str):
                    parts.append(f"'{e}'")
                else:
                    parts.append(str(e))
            arr_s = f"[{', '.join(parts)}]"
        else:
            arr_s = str(arr)

        # build val
        if hasattr(val, "to_raw") and not isinstance(val, (list, tuple)):
            try:
                val_s = val.to_raw().build()
            except Exception:
                val_s = str(val)
        elif isinstance(val, float):
            val_s = f"{val}f"
        elif isinstance(val, str):
            val_s = f"'{val}'"
        else:
            val_s = str(val)

        return f"array::{function_name}({arr_s}, {val_s})"

    def fine_tune():
        counter = [0]
        # arr handling
        if isinstance(arr, Field):
            arr_ft = arr.name
        elif isinstance(arr, Param):
            arr_ft = str(arr)
        elif isinstance(arr, (list, tuple)):
            all_literals = all(not isinstance(e, (Field, Param)) for e in arr)
            if all_literals:
                counter[0] += 1
                arr_ft = f"$_param_{counter[0]:08d}"
            else:
                pieces = []
                for e in arr:
                    if isinstance(e, Field):
                        pieces.append(e.name)
                    elif isinstance(e, Param):
                        pieces.append(str(e))
                    else:
                        counter[0] += 1
                        pieces.append(f"$_param_{counter[0]:08d}")
                arr_ft = f"[{', '.join(pieces)}]"
        else:
            counter[0] += 1
            arr_ft = f"$_param_{counter[0]:08d}"

        # val handling
        if isinstance(val, Field):
            val_ft = val.name
        elif isinstance(val, Param):
            val_ft = str(val)
        elif isinstance(val, (int, float, str)):
            counter[0] += 1
            val_ft = f"$_param_{counter[0]:08d}"
        else:
            counter[0] += 1
            val_ft = f"$_param_{counter[0]:08d}"

        return f"array::{function_name}({arr_ft}, {val_ft})"

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return Fn()


def add(arr: Any, val: Any):
    return _array_fn_arr_and_val("add", arr, val)

def all(arr: Any):
    return _array_fn_single_arr("all", arr)

def any(arr: Any):
    return _array_fn_single_arr("any", arr)

def append(arr: Any, val: Any):
    return _array_fn_arr_and_val("append", arr, val)


# Array helpers continued
def len(arr: Any):
    return _array_fn_single_arr("len", arr)

def first(arr: Any):
    return _array_fn_single_arr("first", arr)

def last(arr: Any):
    return _array_fn_single_arr("last", arr)


# --- String functions ---
def _string_single(function_name: str, value: Any):
    val_repr = _build_arg_repr(value)

    def to_raw_build():
        return f"string::{function_name}({val_repr})"

    def fine_tune():
        counter = [0]
        v_ft = _fine_tune_arg(value, counter)
        return f"string::{function_name}({v_ft})"

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return Fn()


def _string_two(function_name: str, a: Any, b: Any):
    a_repr = _build_arg_repr(a)
    b_repr = _build_arg_repr(b)

    def to_raw_build():
        return f"string::{function_name}({a_repr}, {b_repr})"

    def fine_tune():
        counter = [0]
        a_ft = _fine_tune_arg(a, counter)
        b_ft = _fine_tune_arg(b, counter)
        return f"string::{function_name}({a_ft}, {b_ft})"

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            return Raw(to_raw_build())

        def fine_tune_params(self):
            return fine_tune()

    return Fn()


def length(value: Any):
    return _string_single("length", value)

def lowercase(value: Any):
    return _string_single("lowercase", value)

def uppercase(value: Any):
    return _string_single("uppercase", value)

def words(value: Any):
    return _string_single("words", value)

def reverse(value: Any):
    return _string_single("reverse", value)

def trim(value: Any):
    return _string_single("trim", value)

def slug(value: Any):
    return _string_single("slug", value)

def split(a: Any, b: Any):
    return _string_two("split", a, b)

def join(a: Any, b: Any):
    return _string_two("join", a, b)



class Param:
    def __init__(self, name: str):
        self.name = name

    def __str__(self):
        return f"${self.name}"


class Operation:
    """Represents an arithmetic/field operation expression.

    - `left` and `right` can be Field, Operation, Param, or literal (int/float/str).
    - `op` is one of '+', '-', '*', '/'.
    """

    def __init__(self, left, op: str, right):
        self.left = left
        self.op = op
        self.right = right

    @property
    def query_string(self) -> str:
        return self._build(lambda lit: str(lit))

    def _build(self, lit_formatter) -> str:
        def fmt(x):
            if isinstance(x, Operation):
                return f"({x._build(lit_formatter)})"
            # Prefer objects that expose a to_raw() (e.g. ArrayRef) before
            # falling back to treating objects as plain Fields.
            if hasattr(x, "to_raw") and not isinstance(x, Field) and not isinstance(x, Param):
                try:
                    return x.to_raw().build()
                except Exception:
                    pass
            if isinstance(x, Field):
                return x.name
            if isinstance(x, Param):
                return str(x)
            # literal
            return lit_formatter(x)

        left_s = fmt(self.left)
        right_s = fmt(self.right)
        return f"{left_s} {self.op} {right_s}"

    def to_raw(self):
        # Return an object with build() for compatibility with Rust tests
        class Raw:
            def __init__(self, s):
                self._s = s

            def build(self):
                return self._s

        return Raw(self.query_string)

    def fine_tune_params(self) -> str:
        # Build string replacing numeric/string literals and array/index literals
        counter = [0]

        def build_node(x, is_root=False):
            # Operation
            if isinstance(x, Operation):
                left = build_node(x.left, is_root=False)
                right = build_node(x.right, is_root=False)
                s = f"{left} {x.op} {right}"
                if not is_root:
                    return f"({s})"
                return s

            # ArrayRef: convert index or where-literal into placeholders
            if hasattr(x, "fine_tune_params") and isinstance(x, ArrayRef):
                # index case
                if x.index is not None and x.index.is_index():
                    counter[0] += 1
                    inner = f"$_param_{counter[0]:08d}"
                    base = f"{x.field_name}[{inner}]"
                    if x.dot_star:
                        base += ".*"
                    return base
                if x.index is not None and x.index.is_last():
                    inner = "$"
                    base = f"{x.field_name}[{inner}]"
                    if x.dot_star:
                        base += ".*"
                    return base
                if x.index is not None and x.index.is_all():
                    inner = "*"
                    base = f"{x.field_name}[{inner}]"
                    if x.dot_star:
                        base += ".*"
                    return base
                if x.where_clause is not None:
                    _, cond = x.where_clause
                    kind, val = cond
                    if kind == "this_lt":
                        counter[0] += 1
                        return f"{x.field_name}[WHERE $this < $_param_{counter[0]:08d}]" + (".*" if x.dot_star else "")
                if x.all_star:
                    return f"{x.field_name}[*]"
                return f"{x.field_name}[]"

            # Field
            if isinstance(x, Field):
                return x.name

            # Param
            if isinstance(x, Param):
                return str(x)

            # Literal number or string: allocate placeholder
            if isinstance(x, (int, float)):
                counter[0] += 1
                return f"$_param_{counter[0]:08d}"
            if isinstance(x, str):
                counter[0] += 1
                return f"$_param_{counter[0]:08d}"
            if x is None:
                return "NULL"
            return str(x)

        return build_node(self, is_root=True)

    # allow chaining operations on Operation instances
    def __add__(self, other):
        return Operation(self, "+", other)

    def __sub__(self, other):
        return Operation(self, "-", other)

    def __mul__(self, other):
        return Operation(self, "*", other)

    def __truediv__(self, other):
        return Operation(self, "/", other)
