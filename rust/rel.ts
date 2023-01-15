let fetched = [
  {
    "time": "366.875µs",
    "status": "OK",
    "result": [
      {
        "id": "Writes:59bqq26vnas51uq9546s",
        "in": {
          "age": 18,
          "email": "oye@gmail.com",
          "id": "users:oye",
          "mediate": ["Calm", "Geeky"],
          "name": "oye",
        },
        "out": {
          "content": "Nova Scotia",
          "id": "blogs:nova",
          "title": "Halifax",
        },
      },
      {
        "id": "Writes:8ijc1aq2em56mdabovi0",
        "in": {
          "age": 18,
          "email": "oye@gmail.com",
          "id": "users:oye",
          "mediate": ["Calm", "Geeky"],
          "name": "oye",
        },
        "out": {
          "content": "Nova Scotia",
          "id": "blogs:nova",
          "title": "Halifax",
        },
      },
      {
        "id": "Writes:bbs0ua6lcl3hmt7ef881",
        "in": {
          "age": 18,
          "email": "oye@gmail.com",
          "id": "users:oye",
          "mediate": ["Calm", "Geeky"],
          "name": "oye",
        },
        "out": {
          "content": "North York",
          "id": "blogs:toronto",
          "title": "GTA",
        },
      },
      {
        "id": "Writes:o3u403ittwmke8g4g1z4",
        "in": {
          "age": 18,
          "email": "oye@gmail.com",
          "id": "users:oye",
          "mediate": ["Calm", "Geeky"],
          "name": "oye",
        },
        "out": {
          "content": "Nova Scotia",
          "id": "blogs:nova",
          "title": "Halifax",
        },
      },
    ],
  },
];

let unfetched = [{
  "time": "184.75µs",
  "status": "OK",
  "result": [{
    "id": "Writes:59bqq26vnas51uq9546s",
    "in": "users:oye",
    "out": "blogs:nova",
  }, {
    "id": "Writes:8ijc1aq2em56mdabovi0",
    "in": "users:oye",
    "out": "blogs:nova",
  }, {
    "id": "Writes:bbs0ua6lcl3hmt7ef881",
    "in": "users:oye",
    "out": "blogs:toronto",
  }, {
    "id": "Writes:o3u403ittwmke8g4g1z4",
    "in": "users:oye",
    "out": "blogs:nova",
  }],
}];


type Meow = number[]
// const meow : Meow = [];  // Change this to below
const meow = [] satisfies Meow;


const firstMeow = meow[0];
