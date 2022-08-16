To query in replica shell
`rs.secondaryOk()`

rs0:SECONDARY> db.auth("username1", "password1")
1
rs0:SECONDARY> rs.secondaryOk()
rs0:SECONDARY> db.book.find()
{ "\_id" : ObjectId("620b4ef0ff91f163dd42d650"), "name" : "oyelowo", "age" : 15 }
rs0:SECONDARY>
