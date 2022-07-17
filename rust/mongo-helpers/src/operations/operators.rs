//! Static operators for queries to prevent invalid queries due to typos.
//!
//! See mongo manual for [query operators](https://docs.mongodb.com/manual/reference/operator/query/)
//! and [update operators](https://docs.mongodb.com/manual/reference/operator/update/).
//!
//! If an operator is missing, you can easily add it yourself (also, PR are welcomed) or use the hardcoded
//! string like you would in a mongo shell.
//!
//! ```
//! use mongodm::mongo::bson::doc;
//! use mongodm::operator::*;
//!
//! // Using static operators
//! let a = doc! {
//!     And: [
//!         { "foo": { Exists: true } },
//!         {
//!             Or: [
//!                 { "bar": { GreaterThan: 100 } },
//!                 { "lorem": "ipsum" }
//!             ]
//!         }
//!     ]
//! };
//!
//! // Using hardcoded strings
//! let b = doc! {
//!     "$and": [
//!         { "foo": { "$exists": true } },
//!         {
//!             "$or": [
//!                 { "bar": { "$gt": 100 } },
//!                 { "lorem": "ipsum" }
//!             ]
//!         }
//!     ]
//! };
//!
//! // Generated document are identicals
//! assert_eq!(a, b);
//! ```

macro_rules! declare_operator {
    ($ty:ident => $mongo_operator:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[doc="Operator `"]
        #[doc=$mongo_operator]
        #[doc="`"]
        pub struct $ty;

        impl From<$ty> for ::std::string::String {
            fn from(_: $ty) -> ::std::string::String {
                ::std::string::String::from($mongo_operator)
            }
        }
    };
    ($category:literal [ $doc_url:literal ] : $ty:ident => $mongo_operator:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[doc="["]
        #[doc=$category]
        #[doc="]("]
        #[doc=$doc_url]
        #[doc=") "]
        #[doc="operator `"]
        #[doc=$mongo_operator]
        #[doc="`"]
        pub struct $ty;

        impl From<$ty> for ::std::string::String {
            fn from(_: $ty) -> ::std::string::String {
                ::std::string::String::from($mongo_operator)
            }
        }
    };
    ($category:literal [ $doc_url:literal ] : $ty:ident => $mongo_operator:literal [ $( $field:ident => $mongo_field:literal ),+ ]) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[doc="["]
        #[doc=$category]
        #[doc="]("]
        #[doc=$doc_url]
        #[doc=") "]
        #[doc="operator `"]
        #[doc=$mongo_operator]
        #[doc="`"]
        #[allow(non_snake_case)]
        pub struct $ty<$( $field ),+> where $( $field : Into<$crate::mongo::bson::Bson> ),+ {
            $( pub $field : $field ),+
        }

        impl<$( $field ),+> ::core::convert::From<$ty<$( $field ),+>> for $crate::mongo::bson::Document where $( $field : Into<$crate::mongo::bson::Bson> ),+ {
            fn from(l: $ty<$( $field ),+>) -> $crate::mongo::bson::Document {
                $crate::mongo::bson::doc! { $mongo_operator: {
                    $( $mongo_field : l.$field.into() ),+
                } }
            }
        }

        impl<$( $field ),+> ::core::convert::From<$ty<$( $field ),+>> for $crate::mongo::bson::Bson where $( $field : Into<$crate::mongo::bson::Bson> ),+ {
            fn from(l: $ty<$( $field ),+>) -> $crate::mongo::bson::Bson {
                $crate::mongo::bson::Bson::Document(l.into())
            }
        }
    };
    ($category:literal [ $doc_url:literal ] : $( $ty:ident => $mongo_operator:literal, )+ ) => {
        $( declare_operator! { $category [ $doc_url ] : $ty => $mongo_operator } )+
    };
}

// == Query operators == //

declare_operator! { "Comparison" ["https://docs.mongodb.com/manual/reference/operator/query/#comparison"]:
    Equal => "$eq",
    GreaterThan => "$gt",
    GreaterThanEqual => "$gte",
    In => "$in",
    LesserThan => "$lt",
    LesserThanEqual => "$lte",
    NotEqual => "$ne",
    NoneIn => "$nin",
}

declare_operator! { "Logical" ["https://docs.mongodb.com/manual/reference/operator/query/#logical"]:
    And => "$and",
    Not => "$not",
    Nor => "$nor",
    Or => "$or",
}

declare_operator! { "Element" ["https://docs.mongodb.com/manual/reference/operator/query/#element"]:
    Exists => "$exists",
    Type => "$type",
}

declare_operator! { "Evaluation" ["https://docs.mongodb.com/manual/reference/operator/query/#evaluation"]:
    Expr => "$expr",
    JsonSchema => "$jsonSchema",
    Mod => "$mod",
    Modulo => "$mod",
    Regex => "$regex",
    Text => "$text",
    Where => "$where",
}

declare_operator! { "Geospatial" ["https://docs.mongodb.com/manual/reference/operator/query/#geospatial"]:
    GeoIntersects => "$geoIntersects",
    GeoWithin => "$geoWithin",
    Near => "$near",
    NearSphere => "$nearSphere",
}

declare_operator! { "Array (query)" ["https://docs.mongodb.com/manual/reference/operator/query/#array"]:
    All => "$all",
    ElemMatch => "$elemMatch",
    Size => "$size",
}

declare_operator! { "Bitwise (query)" ["https://docs.mongodb.com/manual/reference/operator/query/#bitwise"]:
    BitsAllClear => "$bitsAllClear",
    BitsAllSet => "$bitsAllSet",
    BitsAnyClear => "$bitsAnyClear",
    BitsAnySet => "$bitsAnySet",
}

declare_operator! { "Comments" ["https://docs.mongodb.com/manual/reference/operator/query/#comments"]: Comment => "$comment" }

declare_operator! { "Projection" ["https://docs.mongodb.com/manual/reference/operator/query/#projection-operators"]:
    ProjectFirst => "$",
    Meta => "$meta",
    Slice => "$slice",
}

// == Update operators ==

declare_operator! { "Fields" ["https://docs.mongodb.com/manual/reference/operator/update/#fields"]:
    CurrentDate => "$currentDate",
    Inc => "$inc",
    Min => "$min",
    Max => "$max",
    Mul => "$mul",
    Rename => "$rename",
    Set => "$set",
    SetOnInsert => "$setOnInsert",
    Unset => "$unset",
}

declare_operator! { "Array (update)" ["https://docs.mongodb.com/manual/reference/operator/update/#array"]:
    UpdateFirstDocument => "$",
    UpdateAllDocuments => "$[]",
    AddToSet => "$addToSet",
    Pop => "$pop",
    Pull => "$pull",
    Push => "$push",
    PullAll => "$pullAll",
}

declare_operator! { "Modifiers" ["https://docs.mongodb.com/manual/reference/operator/update/#modifiers"]:
    Each => "$each",
    Position => "$position",
    Sort => "$sort",
}

declare_operator! { "Bitwise (update)" ["https://docs.mongodb.com/manual/reference/operator/update/#bitwise"]:
    Bit => "$bit",
}

// Aggregation Pipeline Stages

declare_operator! { "Aggregation pipeline stages" ["https://docs.mongodb.com/manual/reference/operator/aggregation-pipeline"]:
    AddFields => "$addFields",
    Bucket => "$bucket",
    BucketAuto => "$bucketAuto",
    CollStats => "$collStatus",
    Count => "$count",
    Facet => "$facet",
    GeoNear => "$geoNear",
    GraphLookup => "$graphLookup",
    Group => "$group",
    IndexStats => "$indexStats",
    Limit => "$limit",
    ListSessions => "$listSessions",
    Match => "$match",
    Merge => "$merge",
    Out => "$out",
    PlanCacheStats => "$planCacheStatus",
    Project => "$project",
    Redact => "$redact",
    ReplaceWith => "$replaceWith",
    Sample => "$sample",
    Skip => "$skip",
    SortByCount => "$sortByCount",
    Unwind => "$unwind",
    CurrentOp => "$currentOp",
    ListLocalSessions => "$listLocalSessions",
    FindAndModify => "$findAndModify",
    Update => "$update",
}

declare_operator! { "ReplaceRoot Operator" ["https://docs.mongodb.com/manual/reference/operator/aggregation/replaceRoot/"]:
    ReplaceRoot => "$replaceRoot" [
        NewRoot => "newRoot"
    ]
}

declare_operator! { "Lookup Operator" ["https://docs.mongodb.com/manual/reference/operator/aggregation/lookup/#mongodb-pipeline-pipe.-lookup"]:
    Lookup => "$lookup" [
        From => "from",
        As => "as",
        LocalField => "localField",
        ForeignField => "foreignField"
    ]
}

declare_operator! { "Lookup Operator" ["https://docs.mongodb.com/manual/reference/operator/aggregation/lookup/#mongodb-pipeline-pipe.-lookup"]:
    LookupPipeline => "$lookup" [
        From => "from",
        As => "as",
        Let => "let",
        Pipeline => "pipeline"
    ]
}

// Aggregation Pipeline Operators

declare_operator! { "Arithmetic Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#arithmetic-expression-operators"]:
    Abs => "$abs",
    Add => "$add",
    Ceil => "$ceil",
    Divide => "$divide",
    Exp => "$exp",
    Floor => "$floor",
    Ln => "$ln",
    Log => "$log",
    Log10 => "$log10",
    Multiply => "$multiply",
    Pow => "$pow",
    Power => "$pow",
    Round => "$round",
    Sqrt => "$sqrt",
    SquareRoot => "$sqrt",
    Subtract => "$subtract",
    Trunc => "$trunc",
    Truncate => "$truncate",
}

declare_operator! { "Array Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#array-expression-operators"]:
    ArrayElemAt => "$arrayElemAt",
    ArrayToObject => "$arrayToObject",
    ConcatArrays => "$concatArrays",
    Filter => "$filter",
    IndexOfArray => "$indexOfArray",
    IsArray => "$isArray",
    ObjectToArray => "$objectToArray",
    Range => "$range",
    Reduce => "$reduce",
    ReverseArray => "$reverseArray",
    Zip => "$zip",
}

declare_operator! { "Array Map Operator" ["https://docs.mongodb.com/manual/reference/operator/aggregation/map/#mongodb-expression-exp.-map"]:
    Map => "$map" [
        Input => "input",
        As => "as",
        In => "in"
    ]
}

declare_operator! { "Array Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#comparison-expression-operators"]:
    Compare => "$cmp",
}

declare_operator! { "Conditional Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#conditional-expression-operators"]:
    IfNull => "$ifNull",
    Switch => "$switch",
}

declare_operator! { "Conditional Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/cond/#mongodb-expression-exp.-cond"]:
    Cond => "$cond" [
        If => "if",
        Then => "then",
        Else => "else"
    ]
}

declare_operator! { "Date Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#date-expression-operators"]:
    DateFromParts => "$dateFromParts",
    DateFromString => "$dateFromString",
    DateToParts => "$dateToParts",
    DateToString => "$dateToString",
    DayOfMonth => "$dayOfMonth",
    DayOfWeek => "$dayOfWeek",
    DayOfYear => "$dayOfYear",
    Hour => "$hour",
    IsoDayOfWeek => "$isoDayOfWeek",
    IsoWeek => "$isoWeek",
    IsoWeekYear => "$isoWeekYear",
    Millisecond => "$millisecond",
    Minute => "$minute",
    Month => "$month",
    Second => "$second",
    ToDate => "$toDate",
    Week => "$week",
    Year => "$year",
}

declare_operator! { "Literal Expression Operator" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#literal-expression-operator"]:
    Literal => "$literal",
}

declare_operator! { "Object Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#object-expression-operators"]:
    MergeObjects => "$mergeObjects",
}

declare_operator! { "Set Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#set-expression-operators"]:
    AllElementsTrue => "$allElementsTrue",
    AnyElementTrue => "$anyElementTrue",
    SetDifference => "$setDifference",
    SetEquals => "$setEquals",
    SetIntersection => "$setIntersection",
    SetIsSubset => "$setIsSubset",
    SetUnion => "$setUnion",
}

declare_operator! { "String Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#string-expression-operators"]:
    Concat => "$concat",
    IndexOfBytes => "$indexOfBytes",
    IndexOfCp => "$indexOfCP",
    LeftTrim => "$ltrim",
    RegexFind => "$regexFind",
    RegexFindAll => "$regexFindAll",
    RegexMatch => "$regexMatch",
    ReplaceOne => "$replaceOne",
    ReplaceAll => "$replaceAll",
    RightTrim => "$rtrim",
    Split => "$split",
    StrLenBytes => "$strLenBytes",
    StrCaseCmp => "$strcasecmp",
    Substr => "$substr",
    SubstrBytes => "$substrBytes",
    SubstrCp => "$substrCP",
    ToLower => "$toLower",
    ToString => "$toString",
    Trim => "$trim",
    ToUpper => "$toUpper",
}

declare_operator! { "Trigonometry Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#trigonometry-expression-operators"]:
    Sin => "$sin",
    Cos => "$cos",
    Tan => "$tan",
    Asin => "$asin",
    Acos => "$acos",
    Atan => "$atan",
    Atan2 => "$atan2",
    Asinh => "$asinh",
    Acosh => "$acosh",
    Atanh => "$atanh",
    DegreesToRadians => "$degreesToRadians",
    RadiansToDegrees => "$radiansToDegrees",
}

declare_operator! { "Type Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#type-expression-operators"]:
    Convert => "$convert",
    ToBool => "$toBool",
    ToDecimal => "$toDecimal",
    ToDouble => "$toDouble",
    ToInt => "$toInt",
    ToLong => "$toLong",
    ToObjectId => "$toObjectId",
    BsonType => "$type",
}

declare_operator! { "Accumulators ($group)" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#accumulators-group"]:
    Average => "$avg",
    First => "$first",
    Last => "$last",
    StdDevPop => "$stdDevPop",
    StdDevSamp => "$stdDevSamp",
    Sum => "$sum",
}

declare_operator! { "Variable Expression Operators" ["https://docs.mongodb.com/manual/reference/operator/aggregation/#variable-expression-operators"]:
    Let => "$let",
}
