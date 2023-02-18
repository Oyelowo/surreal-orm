//
// pub struct SelectQuery<'a> {
//     projections: Vec<&'a str>,
//     targets: Vec<&'a str>,
//     conditions: Option<&'a str>,
//     split: Option<Vec<&'a str>>,
//     group: Option<Vec<&'a str>>,
//     order: Option<Vec<OrderBy<'a>>>,
//     limit: Option<usize>,
//     start: Option<usize>,
//     fetch: Option<Vec<&'a str>>,
//     timeout: Option<&'a str>,
//     parallel: bool,
// }
//
// pub struct OrderBy<'a> {
//     field: &'a str,
//     rand: bool,
//     collate: bool,
//     numeric: bool,
//     asc: bool,
// }
//
// impl<'a> SelectQuery<'a> {
//     pub fn new() -> Self {
//         SelectQuery {
//             projections: Vec::new(),
//             targets: Vec::new(),
//             conditions: None,
//             split: None,
//             group: None,
//             order: None,
//             limit: None,
//             start: None,
//             fetch: None,
//             timeout: None,
//             parallel: false,
//         }
//     }
//
//     pub fn projection(&mut self, field: &'a str) -> &mut Self {
//         self.projections.push(field);
//         self
//     }
//
//     pub fn from(&mut self, target: &'a str) -> &mut Self {
//         self.targets.push(target);
//         self
//     }
//
//     pub fn condition(&mut self, condition: &'a str) -> &mut Self {
//         self.conditions = Some(condition);
//         self
//     }
//
//     pub fn split(&mut self, fields: Vec<&'a str>) -> &mut Self {
//         self.split = Some(fields);
//         self
//     }
//
//     pub fn group(&mut self, fields: Vec<&'a str>) -> &mut Self {
//         self.group = Some(fields);
//         self
//     }
//
//     pub fn order(&mut self, field: &'a str, rand: bool, collate: bool, numeric: bool, asc: bool) -> &mut Self {
//         let order_by = OrderBy {
//             field,
//             rand,
//             collate,
//             numeric,
//             asc,
//         };
//         if let Some(order) = &mut self.order {
//             order.push(order_by);
//         } else {
//             self.order = Some(vec![order_by]);
//         }
//         self
//     }
//
//     pub fn limit(&mut self, limit: usize) -> &mut Self {
//         self.limit = Some(limit);
//         self
//     }
//
//     pub fn start(&mut self, start: usize) -> &mut Self {
//         self.start = Some(start);
//         self
//     }
//
//     pub fn fetch(&mut self, fields: Vec<&'a str>) -> &mut Self {
//         self.fetch = Some(fields);
//         self
//     }
//
//     pub fn timeout(&mut self, duration: &'a str) -> &mut Self {
//         self.timeout = Some(duration);
//         self
//     }
//
//     pub fn parallel(&mut self) -> &mut Self {
//         self.parallel = true;
//         self
//     }
//
//     pub fn build(&self) -> String {
//         let mut query = String::from("SELECT ");
//         if self.projections.is_empty() {
//             query.push('*');
//         } else {
//             query.push_str(&self.projections.join(", "));
//         }
//         query.push_str(" FROM ");
//         query.push_str(&self.targets.join(", "));
//         if let Some(condition) = &self.conditions {
//             query.push_str(&format!(" WHERE {}", condition));
//         }
