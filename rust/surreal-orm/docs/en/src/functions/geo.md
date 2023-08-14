# Geo Functions

This chapter introduces the geo macros provided by the SurrealDB ORM. The geo macros are used for geospatial operations such as calculating area, distance, bearing, centroid, and encoding/decoding hashes.

## Table of Contents

- [geo::area!()](#geo-area-macro)
- [geo::bearing!()](#geo-bearing-macro)
- [geo::centroid!()](#geo-centroid-macro)
- [geo::distance!()](#geo-distance-macro)
- [geo::hash::decode!()](#geo-hash-decode-macro)
- [geo::hash::encode!()](#geo-hash-encode-macro)

## <a name="geo-area-macro"></a>geo::area!()

The `geo::area!()` macro calculates the area of a polygon. It has the following syntax:

```rust
let poly = polygon!(
    exterior: [
        (x: -111., y: 45.),
        (x: -111., y: 41.),
        (x: -104., y: 41.),
        (x: -104., y: 45.),
    ],
    interiors: [
        [
            (x: -110., y: 44.),
            (x: -110., y: 42.),
            (x: -105., y: 42.),
            (x: -105., y: 44.),
        ],
    ],
);
let result = geo::area!(poly);
```

The `geo::area!()` macro generates the following SQL query:

```plaintext
geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })
```

## <a name="geo-bearing-macro"></a>geo::bearing!()

The `geo::bearing!()` macro calculates the bearing between two points. It has the following syntax:

```rust
let point1 = point! {
    x: 40.02f64,
    y: 116.34,
};

let point2 = point! {
    x: 80.02f64,
    y: 103.19,
};
let result = geo::bearing!(point1, point2);
```

The `geo::bearing!()` macro generates the following SQL query:

```plaintext
geo::bearing((40.02, 116.34), (80.02, 103.19))
```

## <a name="geo-centroid-macro"></a>geo::centroid!()

The `geo::centroid!()` macro calculates the centroid of a polygon. It has the following syntax:

```rust
let poly = polygon!(
    exterior: [
        (x: -111., y: 45.),
        (x: -111., y: 41.),
        (x: -104., y: 41.),
        (x: -104., y: 45.),
    ],
    interiors: [
        [
            (x: -110., y: 44.),
            (x: -110., y: 42.),
            (x: -105., y: 42.),
            (x: -105., y: 44.),
        ],
    ],
);
let result = geo::centroid!(poly);
```

The `geo::centroid!()` macro generates the following SQL query:

```plaintext
geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })
```

## <a name="geo-distance-macro"></a>geo::distance!()

The `geo::distance!()` macro calculates the distance between two points. It has the following syntax:

```rust
let point1 = point! {
    x: 40.02f64,
    y: 116.34,
};

let point2 = point! {
    x: 80.02f64,
    y: 103.19,
};
let result = geo::distance!(point1, point2);
```

The `geo::distance!()` macro generates the following SQL query:

```plaintext
geo::distance((40.02, 116.34), (80.02, 103.19))
```

## <a name="geo-hash-decode-macro"></a>geo::hash::decode!()

The `geo::hash::decode!()` macro decodes a geohash string. It has the following syntax:

```rust
let result = geo::hash::decode!("mpuxk4s24f51");
```

The `geo::hash::decode!()` macro generates the following SQL query:

```plaintext
geo::hash::decode('mpuxk4s24f51')
```

## <a name="geo-hash-encode-macro"></a>geo::hash::encode!()

The `geo::hash::encode!()` macro encodes a point or polygon into a geohash string. It has the following syntax:

```rust
let point = point! {
    x: 40.02f64,
    y: 116.34,
};

let result = geo::hash::encode!(point, 5);
```

The `geo::hash::encode!()` macro generates the following SQL query:

```plaintext
geo::hash::encode((40.02, 116.34), 5)
```
