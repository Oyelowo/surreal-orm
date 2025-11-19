"""A tiny SQL/SurrealQL query builder for tests and examples.

This implements minimal features: fields, simple comparison operators,
boolean composition, basic SELECT and SELECT VALUE count(...) builders.
"""
from __future__ import annotations

from typing import Any, Dict, List
import builtins


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
    if isinstance(x, tuple) and builtins.len(x) == 2:
        return f"{x[0]}:{x[1]}"
    if isinstance(x, str):
        return x
    return str(x)


def meta_id(record_id: Any):
    # Build the inner thing-like representation explicitly to avoid surprises
    if isinstance(record_id, (list, tuple)) and builtins.len(record_id) == 2:
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

    # Normalize tuple-like reprs (e.g. "('person', 'oyelowo')") into table:id
    if isinstance(inner, str) and inner.startswith("(") and "," in inner and ":" not in inner:
        content = inner[1:-1]
        parts = [p.strip() for p in content.split(",", 1)]
        if builtins.len(parts) == 2:
            def _stripq(s):
                s = s.strip()
                if builtins.len(s) >= 2 and ((s[0] == "'" and s[-1] == "'") or (s[0] == '"' and s[-1] == '"')):
                    return s[1:-1]
                return s

            a_s = _stripq(parts[0])
            b_s = _stripq(parts[1])
            inner = f"{a_s}:{b_s}"

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
    if isinstance(record_id, (list, tuple)) and builtins.len(record_id) == 2:
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
    # Tuple point like (x, y) -> format as (x, y)
    if isinstance(x, tuple) and builtins.len(x) == 2 and all(isinstance(v, (int, float)) for v in x):
        a, b = x
        return f"({a}, {b})"
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
    # Tuple point -> allocate a single placeholder for the whole geometry
    if isinstance(x, tuple) and builtins.len(x) == 2 and all(isinstance(v, (int, float)) for v in x):
        counter[0] += 1
        return f"$_param_{counter[0]:08d}"
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
def array_len(arr: Any):
    return _array_fn_single_arr("len", arr)

# Backwards-compatible alias: prefer `array_len`, but keep `len` for callers
len = array_len

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


def function_fn(args: List[Any], jscode_body: Any):
    # args: list of Param or objects convertible to Param via str()
    def build_args():
        parts: List[str] = []
        for a in args:
            if isinstance(a, Param):
                parts.append(str(a))
            else:
                try:
                    parts.append(str(a))
                except Exception:
                    parts.append(str(a))
        return ", ".join(parts)

    class Fn:
        def to_raw(self):
            class Raw:
                def __init__(self, s):
                    self._s = s

                def build(self):
                    return self._s

            args_s = build_args()
            body = jscode_body if isinstance(jscode_body, str) else str(jscode_body)
            return Raw(f"function({args_s}) {body}")

        def build(self):
            return self.to_raw().build()

    return Fn()


# --- Crypto functions ---
def _crypto_single(function_suffix: str, value: Any):
    val_repr = _build_arg_repr(value)

    def to_raw_build():
        return f"crypto::{function_suffix}({val_repr})"

    def fine_tune():
        counter = [0]
        v_ft = _fine_tune_arg(value, counter)
        return f"crypto::{function_suffix}({v_ft})"

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


def _crypto_two(function_suffix: str, a: Any, b: Any):
    a_repr = _build_arg_repr(a)
    b_repr = _build_arg_repr(b)

    def to_raw_build():
        return f"crypto::{function_suffix}({a_repr}, {b_repr})"

    def fine_tune():
        counter = [0]
        a_ft = _fine_tune_arg(a, counter)
        b_ft = _fine_tune_arg(b, counter)
        return f"crypto::{function_suffix}({a_ft}, {b_ft})"

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


def md5(value: Any):
    return _crypto_single("md5", value)


def sha1(value: Any):
    return _crypto_single("sha1", value)


def sha256(value: Any):
    return _crypto_single("sha256", value)


def sha512(value: Any):
    return _crypto_single("sha512", value)


class _CryptoModule:
    def __init__(self, prefix: str):
        self._prefix = prefix

    def compare(self, a: Any, b: Any):
        return _crypto_two(f"{self._prefix}::compare", a, b)

    def generate(self, a: Any):
        return _crypto_single(f"{self._prefix}::generate", a)


argon2 = _CryptoModule("argon2")
bcrypt = _CryptoModule("bcrypt")
pbkdf2 = _CryptoModule("pbkdf2")
scrypt = _CryptoModule("scrypt")


# --- Geo functions ---
def _geo_single(fn_suffix: str, geometry: Any):
    g_repr = _build_arg_repr(geometry)

    def to_raw_build():
        return f"geo::{fn_suffix}({g_repr})"

    def fine_tune():
        counter = [0]
        g_ft = _fine_tune_arg(geometry, counter)
        return f"geo::{fn_suffix}({g_ft})"

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


def _geo_two(fn_suffix: str, a: Any, b: Any):
    a_repr = _build_arg_repr(a)
    b_repr = _build_arg_repr(b)

    def to_raw_build():
        return f"geo::{fn_suffix}({a_repr}, {b_repr})"

    def fine_tune():
        counter = [0]
        a_ft = _fine_tune_arg(a, counter)
        b_ft = _fine_tune_arg(b, counter)
        return f"geo::{fn_suffix}({a_ft}, {b_ft})"

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


def area(geometry: Any):
    return _geo_single("area", geometry)


def bearing(p1: Any, p2: Any):
    return _geo_two("bearing", p1, p2)


def centroid(geometry: Any):
    return _geo_single("centroid", geometry)


def distance(p1: Any, p2: Any):
    return _geo_two("distance", p1, p2)


class geo_hash:
    @staticmethod
    def decode(value: Any):
        v_repr = _build_arg_repr(value)

        def to_raw_build():
            return f"geo::hash::decode({v_repr})"

        def fine_tune():
            counter = [0]
            v_ft = _fine_tune_arg(value, counter)
            return f"geo::hash::decode({v_ft})"

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

    @staticmethod
    def encode(geometry: Any, accuracy: Any = None):
        # encoding may include optional accuracy
        if accuracy is None:
            g_repr = _build_arg_repr(geometry)

            def to_raw_build():
                return f"geo::hash::encode({g_repr})"

            def fine_tune():
                counter = [0]
                g_ft = _fine_tune_arg(geometry, counter)
                return f"geo::hash::encode({g_ft})"
        else:
            g_repr = _build_arg_repr(geometry)
            acc_repr = _build_arg_repr(accuracy)

            def to_raw_build():
                return f"geo::hash::encode({g_repr}, {acc_repr})"

            def fine_tune():
                counter = [0]
                g_ft = _fine_tune_arg(geometry, counter)
                acc_ft = _fine_tune_arg(accuracy, counter)
                return f"geo::hash::encode({g_ft}, {acc_ft})"

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


hash = geo_hash


# --- Time functions ---
def _time_single(fn_suffix: str, datetime: Any):
    d_repr = _build_arg_repr(datetime)

    def to_raw_build():
        return f"time::{fn_suffix}({d_repr})"

    def fine_tune():
        counter = [0]
        d_ft = _fine_tune_arg(datetime, counter)
        return f"time::{fn_suffix}({d_ft})"

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


# single-arg time functions
def day(dt: Any):
    return _time_single("day", dt)

def hour(dt: Any):
    return _time_single("hour", dt)

def minute(dt: Any):
    return _time_single("minute", dt)

def month(dt: Any):
    return _time_single("month", dt)

def nano(dt: Any):
    return _time_single("nano", dt)

def second(dt: Any):
    return _time_single("second", dt)

def unix(dt: Any):
    return _time_single("unix", dt)

def wday(dt: Any):
    return _time_single("wday", dt)

def week(dt: Any):
    return _time_single("week", dt)

def yday(dt: Any):
    return _time_single("yday", dt)

def year(dt: Any):
    return _time_single("year", dt)


def now():
    class Fn:
        def to_raw(self):
            class Raw:
                def build(self):
                    return "now()"

            return Raw()

        def fine_tune_params(self):
            return "now()"

    return Fn()


def _time_two(fn_suffix: str, a: Any, b: Any):
    a_repr = _build_arg_repr(a)
    b_repr = _build_arg_repr(b)

    def to_raw_build():
        return f"time::{fn_suffix}({a_repr}, {b_repr})"

    def fine_tune():
        counter = [0]
        a_ft = _fine_tune_arg(a, counter)
        b_ft = _fine_tune_arg(b, counter)
        return f"time::{fn_suffix}({a_ft}, {b_ft})"

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


def _format_duration_seconds(seconds: int) -> str:
    parts = []
    years, rem = divmod(seconds, 31536000)
    weeks, rem = divmod(rem, 604800)
    days, rem = divmod(rem, 86400)
    hours, rem = divmod(rem, 3600)
    minutes, secs = divmod(rem, 60)
    if years:
        parts.append(f"{years}y")
    if weeks:
        parts.append(f"{weeks}w")
    if days:
        parts.append(f"{days}d")
    if hours:
        parts.append(f"{hours}h")
    if minutes:
        parts.append(f"{minutes}m")
    if secs or not parts:
        parts.append(f"{secs}s")
    return "".join(parts)


def _duration_repr(d: Any) -> str:
    # Accept plain int seconds, Field/Param, or objects exposing to_raw
    if isinstance(d, int):
        return _format_duration_seconds(d)
    if hasattr(d, "to_raw"):
        try:
            return d.to_raw().build()
        except Exception:
            pass
    if isinstance(d, Param):
        return str(d)
    if isinstance(d, Field):
        return d.name
    return str(d)


def floor(*args):
    # Overloaded: floor(datetime) -> math::floor; floor(datetime, duration) -> time::floor
    if builtins.len(args) == 1:
        return _math_single_num("floor", args[0])
    if builtins.len(args) == 2:
        datetime, duration = args
        d_repr = _build_arg_repr(datetime)
        dur_repr = _duration_repr(duration)

        def to_raw_build():
            return f"time::floor({d_repr}, {dur_repr})"

        def fine_tune():
            counter = [0]
            d_ft = _fine_tune_arg(datetime, counter)
            # treat duration as a param if it's literal or non-field
            if isinstance(duration, int):
                counter[0] += 1
                dur_ft = f"$_param_{counter[0]:08d}"
            else:
                dur_ft = _fine_tune_arg(duration, counter)
            return f"time::floor({d_ft}, {dur_ft})"

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
    raise TypeError("floor() takes 1 or 2 positional arguments")


def ceil(*args):
    # Overloaded: ceil(value) -> math::ceil; ceil(datetime, duration) -> time::ceil
    if builtins.len(args) == 1:
        return _math_single_num("ceil", args[0])
    if builtins.len(args) == 2:
        datetime, duration = args
        d_repr = _build_arg_repr(datetime)
        dur_repr = _duration_repr(duration)

        def to_raw_build():
            return f"time::ceil({d_repr}, {dur_repr})"

        def fine_tune():
            counter = [0]
            d_ft = _fine_tune_arg(datetime, counter)
            if isinstance(duration, int):
                counter[0] += 1
                dur_ft = f"$_param_{counter[0]:08d}"
            else:
                dur_ft = _fine_tune_arg(duration, counter)
            return f"time::ceil({d_ft}, {dur_ft})"

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
    raise TypeError("ceil() takes 1 or 2 positional arguments")


def round(*args):
    # Overloaded: round(value) -> math::round; round(datetime, duration) -> time::round
    if builtins.len(args) == 1:
        return _math_single_num("round", args[0])
    if builtins.len(args) == 2:
        datetime, duration = args
        d_repr = _build_arg_repr(datetime)
        dur_repr = _duration_repr(duration)

        def to_raw_build():
            return f"time::round({d_repr}, {dur_repr})"

        def fine_tune():
            counter = [0]
            d_ft = _fine_tune_arg(datetime, counter)
            if isinstance(duration, int):
                counter[0] += 1
                dur_ft = f"$_param_{counter[0]:08d}"
            else:
                dur_ft = _fine_tune_arg(duration, counter)
            return f"time::round({d_ft}, {dur_ft})"

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
    raise TypeError("round() takes 1 or 2 positional arguments")


def format_(datetime: Any, fmt: Any):
    return _time_two("format", datetime, fmt)


def timezone():
    class Fn:
        def to_raw(self):
            class Raw:
                def build(self):
                    return "time::timezone()"

            return Raw()

        def fine_tune_params(self):
            return "time::timezone()"

    return Fn()


def group(datetime: Any, interval: Any):
    return _time_two("group", datetime, interval)


def max_(arr: Any):
    # arr may be list or field/param
    if isinstance(arr, list):
        # to_raw: show list literals
        reprs = [ _build_arg_repr(x) for x in arr ]
        arr_repr = f"[{', '.join(reprs)}]"
    else:
        arr_repr = _build_arg_repr(arr)

    def to_raw_build():
        return f"time::max({arr_repr})"

    def fine_tune():
        counter = [0]
        if isinstance(arr, list):
            parts = [ _fine_tune_arg(x, counter) for x in arr ]
            return f"time::max([{', '.join(parts)}])"
        return f"time::max({ _fine_tune_arg(arr, counter) })"

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


def min_(arr: Any):
    # reuse max_ implementation with names swapped
    fn = max_(arr)
    # tweak strings
    class Fn:
        def to_raw(self):
            return type(fn.to_raw())(fn.to_raw().build().replace("time::max", "time::min"))
        def fine_tune_params(self):
            return fn.fine_tune_params().replace("time::max", "time::min")
    return Fn()


class time_from:
    @staticmethod
    def micros(n: Any):
        return _time_single("from::micros", n)

    @staticmethod
    def millis(n: Any):
        return _time_single("from::millis", n)

    @staticmethod
    def secs(n: Any):
        return _time_single("from::secs", n)

    @staticmethod
    def unix(n: Any):
        return _time_single("from::unix", n)


from_ = time_from



# --- Type functions -------------------------------------------------
def _type_single(fn_name: str, value: Any):
    v_repr = _build_arg_repr(value)

    def to_raw_build():
        # use duration formatting for duration-like functions
        if fn_name == "duration":
            # if literal int -> format into duration string
            if isinstance(value, int):
                dur = _duration_repr(value)
                return f"type::{fn_name}({dur})"
        return f"type::{fn_name}({v_repr})"

    def fine_tune():
        counter = [0]
        v_ft = _fine_tune_arg(value, counter)
        return f"type::{fn_name}({v_ft})"

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


def bool_(value: Any):
    return _type_single("bool", value)


def datetime(value: Any):
    return _type_single("datetime", value)


def duration(value: Any):
    return _type_single("duration", value)


def field_(value: Any):
    return _type_single("field", value)


def fields(value: Any):
    # ensure list literals are displayed as ['a', 'b']
    def to_raw_build():
        if isinstance(value, (list, tuple)):
            parts = []
            for e in value:
                if isinstance(e, str):
                    parts.append(f"'{e}'")
                else:
                    parts.append(_build_arg_repr(e))
            return f"type::fields([{', '.join(parts)}])"
        return f"type::fields({_build_arg_repr(value)})"

    def fine_tune():
        counter = [0]
        if isinstance(value, (list, tuple)):
            # treat whole list as single param if all literals
            all_literals = all(not isinstance(e, (Field, Param)) for e in value)
            if all_literals:
                counter[0] += 1
                return f"type::fields($_param_{counter[0]:08d})"
            parts = []
            for e in value:
                if isinstance(e, Field):
                    parts.append(e.name)
                elif isinstance(e, Param):
                    parts.append(str(e))
                else:
                    counter[0] += 1
                    parts.append(f"$_param_{counter[0]:08d}")
            return f"type::fields([{', '.join(parts)}])"
        return f"type::fields({_fine_tune_arg(value, counter)})"

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


def float_(value: Any):
    return _type_single("float", value)


def int_(value: Any):
    return _type_single("int", value)


def number(value: Any):
    return _type_single("number", value)


def string(value: Any):
    return _type_single("string", value)


def regex(value: Any):
    return _type_single("regex", value)


def table(value: Any):
    return _type_single("table", value)


def point(a: Any, b: Any):
    a_repr = _build_arg_repr(a)
    b_repr = _build_arg_repr(b)

    def to_raw_build():
        return f"type::point({a_repr}, {b_repr})"

    def fine_tune():
        counter = [0]
        a_ft = _fine_tune_arg(a, counter)
        b_ft = _fine_tune_arg(b, counter)
        return f"type::point({a_ft}, {b_ft})"

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


def thing(table_val: Any, value: Any):
    t_repr = _build_arg_repr(table_val)
    v_repr = _build_arg_repr(value)

    def to_raw_build():
        return f"type::thing({t_repr}, {v_repr})"

    def fine_tune():
        counter = [0]
        t_ft = _fine_tune_arg(table_val, counter)
        v_ft = _fine_tune_arg(value, counter)
        return f"type::thing({t_ft}, {v_ft})"

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


class _TypeIsModule:
    def __init__(self):
        pass

    def _make(self, name: str, value: Any):
        v_repr = _build_arg_repr(value)

        def to_raw_build():
            return f"type::is::{name}({v_repr})"

        def fine_tune():
            counter = [0]
            v_ft = _fine_tune_arg(value, counter)
            return f"type::is::{name}({v_ft})"

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

    def array(self, value: Any):
        return self._make("array", value)

    def bool(self, value: Any):
        return self._make("bool", value)

    def bytes(self, value: Any):
        return self._make("bytes", value)

    def collection(self, value: Any):
        return self._make("collection", value)

    def datetime(self, value: Any):
        return self._make("datetime", value)

    def decimal(self, value: Any):
        return self._make("decimal", value)

    def duration(self, value: Any):
        # duration literal formatting handled in _duration_repr when needed
        return self._make("duration", value)

    def float(self, value: Any):
        return self._make("float", value)

    def geometry(self, value: Any):
        return self._make("geometry", value)

    def int(self, value: Any):
        return self._make("int", value)

    def line(self, value: Any):
        return self._make("line", value)

    def null(self, value: Any):
        return self._make("null", value)

    def multiline(self, value: Any):
        return self._make("multiline", value)

    def multipoint(self, value: Any):
        return self._make("multipoint", value)

    def multipolygon(self, value: Any):
        return self._make("multipolygon", value)

    def number(self, value: Any):
        return self._make("number", value)

    def object(self, value: Any):
        return self._make("object", value)

    def point(self, value: Any):
        return self._make("point", value)

    def polygon(self, value: Any):
        return self._make("polygon", value)

    def record(self, value: Any):
        return self._make("record", value)

    def string(self, value: Any):
        return self._make("string", value)

    def uuid(self, value: Any):
        return self._make("uuid", value)


is_ = _TypeIsModule()

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
