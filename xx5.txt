// SELECT *, -> knows -> person AS known_persons FROM person WHERE -> knows[WHERE influencer = true]-> person
const xx = [{
  "time": "744.375µs",
  "status": "OK",
  "result": [{
    "id": "person:lowo",
    "known_persons": [
      "person:dayo",
      "person:dayo",
      "person:dayo",
      "person:dayo",
    ],
    "loves": "Canada",
  }],
}];

// > SELECT *, -> knows -> person AS known_persons FROM person WHERE -> knows[WHERE influencer = true]-> person FETCH known_persons
const xx2 = [{
  "time": "1.414ms",
  "status": "OK",
  "result": [{
    "id": "person:lowo",
    "known_persons": [
      { "id": "person:dayo", "loves": "Toronto" },
      { "id": "person:dayo", "loves": "Toronto" },
      { "id": "person:dayo", "loves": "Toronto" },
      { "id": "person:dayo", "loves": "Toronto" },
    ],
    "loves": "Canada",
  }],
}];

// People that know influencer
// SELECT * FROM person WHERE ->knows->person[WHERE influencer = true]

// > create person:sam SET star = true

// RELATE person:dayo->knows->person:sam
// RELATE person:lowo->knows->person:sam

// For RELATE QUERY
// WRONG: RELATE person->knows->person[WHERE star = true]
// RIGHT: RELATE (select * from person)->knows->(select * from person WHERE star = 'true')

// For SELECT
// RIGHT(this style works for select but not for relate):
// SELECT * FROM person WHERE ->knows->person[WHERE star = true]
// SELECT * FROM person WHERE ->knows->person[WHERE id = "person:sam"]
// SELECT * FROM person WHERE ->knows[WHERE star = true]->person[WHERE id = "person:sam"]
// SELECT * FROM person WHERE ->knows[WHERE star = true]->person

// VALID(but does not seem to return right result for me):
// SELECT * FROM person WHERE ->knows->person->(knows WHERE star = true)

const mm = [{
  "time": "1.95575ms",
  "status": "OK",
  "result": [{
    "company": "SurrealDB",
    "id": "person:jaime",
    "name": "Jaime",
    "pxx": [{
      "company": "SurrealDB",
      "id": "person:tobie",
      "name": "Tobie",
      "skills": ["Rust", "Go", "JavaScript"],
    }, { "id": "person:sam", "star": true }],
    "skills": ["Python", "java", "JavaScript"],
  }],
}];

const mm2 = [{
  "time": "1.732375ms",
  "status": "OK",
  "result": [{
    "company": "SurrealDB",
    "id": "person:jaime",
    "name": "Jaime",
    "pxx": [{
      "company": "SurrealDB",
      "id": "person:tobie",
      "name": "Tobie",
      "skills": ["Rust", "Go", "JavaScript"],
    }],
    "skills": ["Python", "java", "JavaScript"],
  }],
}];

const mm3 = [{
  "time": "1.793875ms",
  "status": "OK",
  "result": [{
    "company": "SurrealDB",
    "id": "person:jaime",
    "name": "Jaime",
    "pxx": [],
    "skills": ["Python", "java", "JavaScript"],
  }],
}];

const mm4 = [{
  "time": "1.582083ms",
  "status": "OK",
  "result": [{
    "company": "SurrealDB",
    "id": "person:jaime",
    "name": "Jaime",
    "pxx": [{ "id": "person:sam", "star": true }],
    "skills": ["Python", "java", "JavaScript"],
  }],
}];

const xx = [{
  "time": "1.543541ms",
  "status": "OK",
  "result": [
    { "id": "person:7db1px7a0ct8ftq2lz04", "pxx": [] },
    { "id": "person:dayo", "loves": "Toronto", "pxx": [] },
    {
      "company": "SurrealDB",
      "id": "person:jaime",
      "name": "Jaime",
      "pxx": [{
        "company": "SurrealDB",
        "id": "person:tobie",
        "name": "Tobie",
        "skills": ["Rust", "Go", "JavaScript"],
      }],
      "skills": ["Python", "java", "JavaScript"],
    },
    { "id": "person:lowo", "loves": "Canada", "pxx": [] },
    { "id": "person:sam", "pxx": [], "star": true },
    {
      "company": "SurrealDB",
      "id": "person:tobie",
      "name": "Tobie",
      "pxx": [],
      "skills": ["Rust", "Go", "JavaScript"],
    },
  ],
}];

/*

CREATE users:hali CONTENT {
    name: "hali",
    age: 99,
    friends: [users:oye, users:lowo]
}

CREATE users:lowo CONTENT {
    name: "lowo",
    age: 99,
    team_mates: [person:jamie, person:tobie]
}

> SELECT team_mates.name FROM users:leke
[{"time":"315.583µs","status":"OK","result":[{"team_mates":{"name":[null,"Tobie"]}}]}]


SELECT * FROM users
SELECT friends FROM users
*/

// > select -> knows[WHERE out = "person:sam"] as kk from person
const res = [{
  "time": "1.05275ms",
  "status": "OK",
  "result": [{
    "kk": [{
      "id": "knows:uy17bj18a92g8a2gfi0m",
      "in": "person:7db1px7a0ct8ftq2lz04",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:dl8hfmbeae22irgk8w3g",
      "in": "person:dayo",
      "out": "person:sam",
    }, {
      "id": "knows:ocrrnny9i6cywzr4kc6f",
      "in": "person:dayo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:gx1qgh3bbq8x90rw1yqw",
      "in": "person:jaime",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:1omd2lah1ih26visry5p",
      "in": "person:lowo",
      "out": "person:sam",
    }, {
      "id": "knows:aw6wo311hn6nyeaz4l6b",
      "in": "person:lowo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mb7bbh656ctjz5kcwm0u",
      "in": "person:sam",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mdv5l0s8ohhw62y7040q",
      "in": "person:tobie",
      "out": "person:sam",
    }],
  }],
}];

// > select -> knows[WHERE out = "person:sam"].id as kk from person
const res = [{
  "time": "900.458µs",
  "status": "OK",
  "result": [
    { "kk": ["knows:uy17bj18a92g8a2gfi0m"] },
    { "kk": ["knows:dl8hfmbeae22irgk8w3g", "knows:ocrrnny9i6cywzr4kc6f"] },
    { "kk": ["knows:gx1qgh3bbq8x90rw1yqw"] },
    { "kk": ["knows:1omd2lah1ih26visry5p", "knows:aw6wo311hn6nyeaz4l6b"] },
    { "kk": ["knows:mb7bbh656ctjz5kcwm0u"] },
    { "kk": ["knows:mdv5l0s8ohhw62y7040q"] },
  ],
}];

// > select -> knows[WHERE out = "person:sam"].in as kk from person
const res = [{
  "time": "933.958µs",
  "status": "OK",
  "result": [
    { "kk": ["person:7db1px7a0ct8ftq2lz04"] },
    { "kk": ["person:dayo", "person:dayo"] },
    { "kk": ["person:jaime"] },
    { "kk": ["person:lowo", "person:lowo"] },
    { "kk": ["person:sam"] },
    { "kk": ["person:tobie"] },
  ],
}];

// > select -> knows[WHERE out = "person:sam"].in as kk from person FETCH kk
const res = [{
  "time": "1.501583ms",
  "status": "OK",
  "result": [
    { "kk": [{ "id": "person:7db1px7a0ct8ftq2lz04" }] },
    {
      "kk": [{ "id": "person:dayo", "loves": "Toronto" }, {
        "id": "person:dayo",
        "loves": "Toronto",
      }],
    },
    {
      "kk": [{
        "company": "SurrealDB",
        "id": "person:jaime",
        "name": "Jaime",
        "skills": ["Python", "java", "JavaScript"],
      }],
    },
    {
      "kk": [{ "id": "person:lowo", "loves": "Canada" }, {
        "id": "person:lowo",
        "loves": "Canada",
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{
        "company": "SurrealDB",
        "id": "person:tobie",
        "name": "Tobie",
        "skills": ["Rust", "Go", "JavaScript"],
      }],
    },
  ],
}];

// > select -> knows[WHERE out = "person:sam"].out as kk from person FETCH kk
const res = [{
  "time": "1.487708ms",
  "status": "OK",
  "result": [
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{ "id": "person:sam", "star": true }, {
        "id": "person:sam",
        "star": true,
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{ "id": "person:sam", "star": true }, {
        "id": "person:sam",
        "star": true,
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    { "kk": [{ "id": "person:sam", "star": true }] },
  ],
}];

// > select -> knows[WHERE out = "person:sam"].in as kk from person FETCH kk
const res = [{
  "time": "1.240916ms",
  "status": "OK",
  "result": [
    { "kk": [{ "id": "person:7db1px7a0ct8ftq2lz04" }] },
    {
      "kk": [{ "id": "person:dayo", "loves": "Toronto" }, {
        "id": "person:dayo",
        "loves": "Toronto",
      }],
    },
    {
      "kk": [{
        "company": "SurrealDB",
        "id": "person:jaime",
        "name": "Jaime",
        "skills": ["Python", "java", "JavaScript"],
      }],
    },
    {
      "kk": [{ "id": "person:lowo", "loves": "Canada" }, {
        "id": "person:lowo",
        "loves": "Canada",
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{
        "company": "SurrealDB",
        "id": "person:tobie",
        "name": "Tobie",
        "skills": ["Rust", "Go", "JavaScript"],
      }],
    },
  ],
}];

// > select -> knows[WHERE out = "person:sam"] as kk from person FETCH kk
const res = [{
  "time": "487µs",
  "status": "OK",
  "result": [{
    "kk": [{
      "id": "knows:uy17bj18a92g8a2gfi0m",
      "in": "person:7db1px7a0ct8ftq2lz04",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:dl8hfmbeae22irgk8w3g",
      "in": "person:dayo",
      "out": "person:sam",
    }, {
      "id": "knows:ocrrnny9i6cywzr4kc6f",
      "in": "person:dayo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:gx1qgh3bbq8x90rw1yqw",
      "in": "person:jaime",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:1omd2lah1ih26visry5p",
      "in": "person:lowo",
      "out": "person:sam",
    }, {
      "id": "knows:aw6wo311hn6nyeaz4l6b",
      "in": "person:lowo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mb7bbh656ctjz5kcwm0u",
      "in": "person:sam",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mdv5l0s8ohhw62y7040q",
      "in": "person:tobie",
      "out": "person:sam",
    }],
  }],
}];

// > select -> (knows WHERE out = "person:sam") as kk from person
const res = [{
  "time": "989.166µs",
  "status": "OK",
  "result": [
    { "kk": ["knows:uy17bj18a92g8a2gfi0m"] },
    { "kk": ["knows:dl8hfmbeae22irgk8w3g", "knows:ocrrnny9i6cywzr4kc6f"] },
    { "kk": ["knows:gx1qgh3bbq8x90rw1yqw"] },
    { "kk": ["knows:1omd2lah1ih26visry5p", "knows:aw6wo311hn6nyeaz4l6b"] },
    { "kk": ["knows:mb7bbh656ctjz5kcwm0u"] },
    { "kk": ["knows:mdv5l0s8ohhw62y7040q"] },
  ],
}];

// > select -> (knows WHERE out = "person:sam") as kk from person fetch kk
const res = [{
  "time": "1.008041ms",
  "status": "OK",
  "result": [{
    "kk": [{
      "id": "knows:uy17bj18a92g8a2gfi0m",
      "in": "person:7db1px7a0ct8ftq2lz04",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:dl8hfmbeae22irgk8w3g",
      "in": "person:dayo",
      "out": "person:sam",
    }, {
      "id": "knows:ocrrnny9i6cywzr4kc6f",
      "in": "person:dayo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:gx1qgh3bbq8x90rw1yqw",
      "in": "person:jaime",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:1omd2lah1ih26visry5p",
      "in": "person:lowo",
      "out": "person:sam",
    }, {
      "id": "knows:aw6wo311hn6nyeaz4l6b",
      "in": "person:lowo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mb7bbh656ctjz5kcwm0u",
      "in": "person:sam",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mdv5l0s8ohhw62y7040q",
      "in": "person:tobie",
      "out": "person:sam",
    }],
  }],
}];

// > SELECT *, ->knows->person[WHERE id = "person:sam"] AS known_persons FROM person
const res = [{
  "time": "731.25µs",
  "status": "OK",
  "result": [{
    "id": "person:7db1px7a0ct8ftq2lz04",
    "known_persons": [{ "id": "person:sam", "star": true }],
  }, {
    "id": "person:dayo",
    "known_persons": [{ "id": "person:sam", "star": true }, {
      "id": "person:sam",
      "star": true,
    }],
    "loves": "Toronto",
  }, {
    "company": "SurrealDB",
    "id": "person:jaime",
    "known_persons": [{ "id": "person:sam", "star": true }],
    "name": "Jaime",
    "skills": ["Python", "java", "JavaScript"],
  }, {
    "id": "person:lowo",
    "known_persons": [{ "id": "person:sam", "star": true }, {
      "id": "person:sam",
      "star": true,
    }],
    "loves": "Canada",
  }, {
    "id": "person:sam",
    "known_persons": [{ "id": "person:sam", "star": true }],
    "star": true,
  }, {
    "company": "SurrealDB",
    "id": "person:tobie",
    "known_persons": [{ "id": "person:sam", "star": true }],
    "name": "Tobie",
    "skills": ["Rust", "Go", "JavaScript"],
  }],
}];

// > select ->(knows WHERE out = "person:sam") as kk from person fetch kk
const res = [{
  "time": "631.875µs",
  "status": "OK",
  "result": [{
    "kk": [{
      "id": "knows:uy17bj18a92g8a2gfi0m",
      "in": "person:7db1px7a0ct8ftq2lz04",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:dl8hfmbeae22irgk8w3g",
      "in": "person:dayo",
      "out": "person:sam",
    }, {
      "id": "knows:ocrrnny9i6cywzr4kc6f",
      "in": "person:dayo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:gx1qgh3bbq8x90rw1yqw",
      "in": "person:jaime",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:1omd2lah1ih26visry5p",
      "in": "person:lowo",
      "out": "person:sam",
    }, {
      "id": "knows:aw6wo311hn6nyeaz4l6b",
      "in": "person:lowo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mb7bbh656ctjz5kcwm0u",
      "in": "person:sam",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mdv5l0s8ohhw62y7040q",
      "in": "person:tobie",
      "out": "person:sam",
    }],
  }],
}];

// > select ->(knows WHERE out = "person:sam").id as kk from person fetch kk
const res = [{
  "time": "1.603416ms",
  "status": "OK",
  "result": [{
    "kk": [{
      "id": "knows:uy17bj18a92g8a2gfi0m",
      "in": "person:7db1px7a0ct8ftq2lz04",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:dl8hfmbeae22irgk8w3g",
      "in": "person:dayo",
      "out": "person:sam",
    }, {
      "id": "knows:ocrrnny9i6cywzr4kc6f",
      "in": "person:dayo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:gx1qgh3bbq8x90rw1yqw",
      "in": "person:jaime",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:1omd2lah1ih26visry5p",
      "in": "person:lowo",
      "out": "person:sam",
    }, {
      "id": "knows:aw6wo311hn6nyeaz4l6b",
      "in": "person:lowo",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mb7bbh656ctjz5kcwm0u",
      "in": "person:sam",
      "out": "person:sam",
    }],
  }, {
    "kk": [{
      "id": "knows:mdv5l0s8ohhw62y7040q",
      "in": "person:tobie",
      "out": "person:sam",
    }],
  }],
}];



// WITHOUT fetching edge fields
// > select -> (knows WHERE out = "person:sam").in as kk from person
const res = [{
  "time": "1.001041ms",
  "status": "OK",
  "result": [
    { "kk": ["person:7db1px7a0ct8ftq2lz04"] },
    { "kk": ["person:dayo", "person:dayo"] },
    { "kk": ["person:jaime"] },
    { "kk": ["person:lowo", "person:lowo"] },
    { "kk": ["person:sam"] },
    { "kk": ["person:tobie"] },
  ],
}];

// > select -> (knows WHERE out = "person:sam").out as kk from person
const res = [{
  "time": "988.416µs",
  "status": "OK",
  "result": [
    { "kk": ["person:sam"] },
    { "kk": ["person:sam", "person:sam"] },
    { "kk": ["person:sam"] },
    { "kk": ["person:sam", "person:sam"] },
    { "kk": ["person:sam"] },
    { "kk": ["person:sam"] },
  ],
}];


// WITH fetching edge fields
// > select ->(knows WHERE out = "person:sam").in as kk from person fetch kk
const res = [{
  "time": "1.499833ms",
  "status": "OK",
  "result": [
    { "kk": [{ "id": "person:7db1px7a0ct8ftq2lz04" }] },
    {
      "kk": [{ "id": "person:dayo", "loves": "Toronto" }, {
        "id": "person:dayo",
        "loves": "Toronto",
      }],
    },
    {
      "kk": [{
        "company": "SurrealDB",
        "id": "person:jaime",
        "name": "Jaime",
        "skills": ["Python", "java", "JavaScript"],
      }],
    },
    {
      "kk": [{ "id": "person:lowo", "loves": "Canada" }, {
        "id": "person:lowo",
        "loves": "Canada",
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{
        "company": "SurrealDB",
        "id": "person:tobie",
        "name": "Tobie",
        "skills": ["Rust", "Go", "JavaScript"],
      }],
    },
  ],
}];

// > select ->(knows WHERE out = "person:sam").out as kk from person fetch kk
const res = [{
  "time": "1.570291ms",
  "status": "OK",
  "result": [
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{ "id": "person:sam", "star": true }, {
        "id": "person:sam",
        "star": true,
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    {
      "kk": [{ "id": "person:sam", "star": true }, {
        "id": "person:sam",
        "star": true,
      }],
    },
    { "kk": [{ "id": "person:sam", "star": true }] },
    { "kk": [{ "id": "person:sam", "star": true }] },
  ],
}];
