/* 

Here is an example of how you might modify the Foreign struct to support the different states of a foreign key at compile time:
*/

use std::marker::PhantomData;

struct ForeignBase<T> {
  id: i32,
  _marker: PhantomData<T>,
}

struct ForeignLoaded<T> {
  id: i32,
  value: T,
  _marker: PhantomData<T>,
}

struct ForeignUnloaded<T> {
  id: i32,
  _marker: PhantomData<T>,
}

struct ForeignNull<T> {
  _marker: PhantomData<T>,
}

trait ForeignType<T> {
  fn value(&self) -> Option<&T>;
  fn key(&self) -> i32;
}

impl<T> ForeignType<T> for ForeignBase<T> {
  fn value(&self) -> Option<&T> {
    None
  }

  fn key(&self) -> i32 {
    self.id
  }
}

impl<T> ForeignType<T> for ForeignLoaded<T> {
  fn value(&self) -> Option<&T> {
    Some(&self.value)
  }

  fn key(&self) -> i32 {
    self.id
  }
}

impl<T> ForeignType<T> for ForeignUnloaded<T> {
  fn value(&self) -> Option<&T> {
    None
  }

  fn key(&self) -> i32 {
    self.id
  }
}

impl<T> ForeignType<T> for ForeignNull<T> {
  fn value(&self) -> Option<&T> {
    None
  }

  fn key(&self) -> i32 {
    0
  }
}

enum Foreign<T> {
  Loaded(ForeignLoaded<T>),
  Unloaded(ForeignUnloaded<T>),
  Null(ForeignNull<T>),
}

impl<T> Foreign<T> {
  fn new(value: T) -> Self {
    Foreign::Loaded(ForeignLoaded {
      id: 0,
      value,
      _marker: PhantomData,
    })
  }

  fn from_base(base: ForeignBase<T>) -> Self {
    Foreign::Unloaded(ForeignUnloaded {
      id: base.id,
      _marker: PhantomData,
    })
  }

  fn null() -> Self {
    Foreign::Null(ForeignNull {
      _marker: PhantomData,
    })
  }

  fn is_loaded(&self) -> bool {
    match self {
      Foreign::Loaded(_) => true,
      _ => false,
    }
  }

  fn is_unloaded(&self) -> bool {
    match self {
      Foreign::Unloaded(_) => true,
      _ => false,
    }
  }

  fn is_null(&self) -> bool {
    match self {
      Foreign::Null(_) => true,
      _ => false,
    }
  }
}




  impl<T> ForeignType<T> for Foreign<T> {
  fn value(&self) -> Option<&T> {
    match self {
      Foreign::Loaded(f) => f.value(),
      Foreign::Unloaded(f) => f.value(),
      Foreign::Null(f) => f.value(),
    }
  }

  fn key(&self) -> i32 {
    match self {
      Foreign::Loaded(f) => f.key(),
      Foreign::Unloaded(f) => f.key(),
      Foreign::Null(f) => f.key(),
    }
  }
}

impl<T> From<T> for Foreign<T> {
  fn from(value: T) -> Self {
    Foreign::new(value)
  }
}

impl<T> From<ForeignBase<T>> for Foreign<T> {
  fn from(base: ForeignBase<T>) -> Self {
    Foreign::from_base(base)
  }
}

    /* 
    The PhantomData type is used in the definition of the Foreign structs to indicate that the type parameter T is present in the struct, but is not used in any of its fields. This is necessary because Rust requires all type parameters to be used in some way in order for the struct to be valid.

By using PhantomData<T>, we can specify that the struct has a type parameter T without actually using it in any of the fields. This allows us to create a type-safe enum that can hold values of different types without actually storing the values themselves.

In this case, we are using PhantomData to indicate that the Foreign structs have a type parameter T that represents the type of the related model. This allows us to use the Foreign struct with any model type, without actually storing the model values in the struct.
    */


    /* 
    With this implementation, you can use the Foreign enum to represent the different states of a foreign key at compile time. You can use the new function to create a Foreign::Loaded variant with a populated value, the from_base function to create a Foreign::Unloaded variant from a ForeignBase value, and the null function to create a Foreign::Null variant.

You can also use the is_loaded, is_unloaded, and is_null methods to check the state of a Foreign value, and the value and key methods to access the underlying data.

Finally, the From implementations allow you to easily create Foreign values from values of the related model type or from ForeignBase values.
    */


    #[derive(Debug, PartialEq)]
struct Account {
  id: i32,
  handle: String,
}

#[derive(Debug, PartialEq)]
struct File {
  id: i32,
  name: String,
  author: Foreign<Account>,
}

fn main() {
  let account = Account {
    id: 1,
    handle: "Alice".to_string(),
  };
  let file = File {
    id: 1,
    name: "My File".to_string(),
    author: account.into(),
  };

  assert!(file.author.is_loaded());
  assert_eq!(file.author.key(), 1);
  assert_eq!(file.author.value(), Some(&account));

  let file_base = File {
    id: 1,
    name: "My File".to_string(),
    author: Foreign::from_base(ForeignBase {
      id: 1,
      _marker: PhantomData,
    }),
  };

  assert!(file_base.author.is_unloaded());
  assert_eq!(file_base.author.key(), 1);
  assert_eq!(file_base.author.value(), None);

  let file_null = File {
    id: 1,
    name: "My File".to_string(),
    author: Foreign::null(),
  };

  assert!(file_null.author.is_null());
  assert_eq!(file_null.author.key(), 0);
  assert_eq!(file_null.author.value(), None);
}



#[derive(Debug, PartialEq)]
struct Account {
  id: i32,
  handle: String,
}

#[derive(Debug, PartialEq)]
struct File {
  id: i32,
  name: String,
  author: Foreign<Account>,
  editor: Foreign<Account>,
  reviewers: Vec<Foreign<Account>>,
}

fn main() {
  let account_1 = Account {
    id: 1,
    handle: "Alice".to_string(),
  };
  let account_2 = Account {
    id: 2,
    handle: "Bob".to_string(),
  };
  let account_3 = Account {
    id: 3,
    handle: "Charlie".to_string(),
  };
  let file = File {
    id: 1,
    name: "My File".to_string(),
    author: account_1.into(),
    editor: account_2.into(),
    reviewers: vec![account_3.into()],
  };

  assert!(file.author.is_loaded());
  assert_eq!(file.author.key(), 1);
  assert_eq!(file.author.value(), Some(&account_1));

  assert!(file.editor.is_loaded());
  assert_eq!(file.editor.key(), 2);
  assert_eq!(file.editor.value(), Some(&account_2));

  assert!(file.reviewers[0].is_loaded());
  assert_eq!(file.reviewers[0].key(), 3);
  assert_eq!(file.reviewers[0].value(), Some(&account_3));
}


/* 
In this example, the File struct has three foreign key fields: author, editor, and reviewers. The author and editor fields are single foreign key fields, while the reviewers field is an array of foreign keys. Each foreign key field is of type Foreign<Account>, which allows you to store the different states of the foreign keys at compile time.

You can use the is_loaded, is_unloaded, and is_null methods to check the state of each foreign key field, and the key and value methods to access the underlying data. You can also use the new function to create Foreign::Loaded values, the from_base function to create Foreign::Unloaded values, and the null function to create Foreign::Null values.
*/