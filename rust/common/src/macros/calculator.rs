// #[macro_export]
// macro_rules! add{
//  // first arm in case of single argument and last remaining variable/number
//     ($a:expr)=>{
//         $a
//     };
// // second arm in case of two arument are passed and stop recursion in case of odd number ofarguments
//     ($a:expr,$b:expr)=>{
//         {
//             $a+$b
//         }
//     };
// // add the number and the result of remaining arguments 
//     ($a:expr,$($b:tt)*)=>{
//        {
//            $a+add!($($b)*)
//        }
//     }
// }

// fn mainww(){
//     println!("{}",add!(1,2,3,4));
// }