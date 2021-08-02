// fn main() {
//     let slice = (0..101).collect::<Vec<_>>();
//     println!("{:?}", slice.iter().next());
//     println!("{:?}", slice.iter().fuse());
//     println!("{:?}", slice.iter().last());
//     let v = (0..1000000).collect::<Vec<_>>();
//     println!("hoge");
//     println!("{:?}", v.iter().next());
//     println!("{:?}", v.iter().last());
//     println!("{:?}", v.iter().nth(9));
//     println!("{:?}", v.iter().find(|&e| e / 200 > 0));
//     println!("{:?}", v.iter().max());
//     println!("{:?}", v.iter().max_by_key(|&e| e % 300));
//     println!("{:?}", v.iter().min_by(|a, b| b.cmp(a)));
//     println!(
//         "{:?}",
//         v.iter().filter(|e| *e % 20000 == 0).collect::<Vec<_>>()
//     );
//     println!("{:?}", slice.iter().skip(8).take(8));
//     println!("{:?}", slice.iter().step_by(7).collect::<Vec<_>>());
//     let memory = (1..101)
//         .map(|e| (0..101).step_by(e).collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//     println!("{:?}", slice.iter().size_hint());
//     println!("{:?}", memory.iter().size_hint());
//     println!("{:?}", (0..0).size_hint());
//     println!(
//         "{:?}",
//         memory
//             .iter()
//             .map(|e| e.iter().skip(e.len() / 2).size_hint().1.unwrap())
//             .collect::<Vec<_>>()
//     );
//     println!("{:?}", memory.iter().count());
//     if !memory.iter().all(|e| e.len() > 90) {
//         println!("hoge");
//     }
//     if memory.iter().any(|e| e.iter().all(|&e| e < 10000000)) {
//         println!("hoge");
//     }
//     println!("{:?}", memory.iter().position(|e| e.len() < 10));
//     println!("{:?}", memory.iter().rposition(|e| e.len() < 10));
//     println!("{:?}", slice.iter().peekable().peek());
//     println!(
//         "{:?}",
//         slice
//             .iter()
//             .fuse()
//             .copied()
//             .map(|e| e * 100)
//             .collect::<Vec<_>>()
//     );
//     let mut vec_iter = (40..46).collect::<Vec<_>>().into_iter();
//     println!("output:");
//     for (i, n) in vec_iter.by_ref().take(3).enumerate() {
//         println!(" {}: {}", i, n);
//     }
//     println!("rest:");
//     for (i, n) in vec_iter.enumerate() {
//         println!(" {}: {}", i, n);
//     }

//     let a = [1, 4, 2, 3];
//     let sum = a
//         .iter()
//         .copied()
//         .filter(|x| x % 2 == 0)
//         .fold(0, |sum, i| sum + i);
//     println!("{}", sum);
//     let sum = a
//         .iter()
//         .copied()
//         .inspect(|x| println!("about to filter: {}", x))
//         .filter(|x| x % 2 == 0)
//         .inspect(|x| println!("made it through filter: {}", x))
//         .fold(0, |sum, i| sum + i);
//     println!("{}", sum);
//     // for e in (0..10).cycle() {
//     //     println!("{}", e);
//     // }
//     let a = [1, 2, 3];
//     let mut iter = a.iter().scan(1, |state, &x| {
//         *state = *state * x;
//         Some(-*state)
//     });
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     let fib = (0..12)
//         .scan((0, 1), |prev, _| {
//             let ret = prev.0;
//             *prev = (prev.1, prev.0 + prev.1);
//             Some(ret)
//         })
//         .collect::<Vec<_>>();
//     println!("{:?}", fib);

//     for i in [1, 2, 3, 4]
//         .iter()
//         .copied()
//         .chain((0..9).collect::<Vec<_>>())
//     {
//         println!("{}", i);
//     }
//     for (a, b) in (0..100)
//         .filter(|e| e % 2 == 0)
//         .zip((0..100).filter(|e| e % 2 != 0))
//     {
//         println!("({:02}, {:02})", a, b);
//     }
//     for e in (0..100)
//         .filter_map(|e| if e % 2 == 0 { Some(e * 3) } else { None })
//         .take(10)
//     {
//         println!("{}", e);
//     }
//     let v = [[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]];
//     for e in v.iter().flatten().flatten() {
//         println!("{}", e);
//     }
//     println!(
//         "{:?}",
//         (0..100)
//             .partition::<Vec<_>, _>(|e| e % 2 == 0)
//             .0
//             .iter()
//             .filter(|&e| e % 2i32 != 0)
//             .collect::<Vec<_>>()
//     );
//     let v = [1, 2, 3, 4, 5, 6, 7, 8];
//     println!(
//         "{:?}",
//         v.iter().reduce(|a, b| {
//             println!("{} {}", a, b);
//             std::cmp::max(a, b)
//         })
//     );
//     println!(
//         "{:?}",
//         v.iter().fold(0, |a, b| {
//             println!("{} {}", a, b);
//             std::cmp::max(a, *b)
//         })
//     );
//     let vec = vec![Ok(1), Ok(2), Err("oops"), Ok(4), Err("Oh my buddha")];
//     let sum_res = vec
//         .into_iter()
//         .try_fold(0, |sum, item| item.map(|v| sum + v));
//     println!("{:?}", sum_res);
//     let vec: Vec<Result<_, String>> = vec![Ok(1), Ok(2), Ok(4)];
//     let sum_res = vec
//         .into_iter()
//         .try_fold(0, |sum, item| item.map(|v| sum + v));
//     println!("{:?}", sum_res);

//     let a = [(1, 2), (3, 4)];
//     let (left, right): (Vec<_>, Vec<_>) = a.iter().copied().unzip();
//     println!("left: {:?}, right: {:?}", left, right);
//     [1, 2, 3, 4]
//         .iter()
//         .copied()
//         .chain((0..9).collect::<Vec<_>>())
//         .for_each(|i| println!("{}", i));
//     for e in (0..100).scan(1, |sum, e| {
//         *sum += e;
//         if e < 50 {
//             Some(*sum)
//         } else {
//             None
//         }
//     }) {
//         println!("{:?}", e);
//     }
//     println!(
//         "{:?}",
//         (0..100)
//             .scan(1, |sum, e| {
//                 *sum += e;
//                 if e < 50 {
//                     Some(*sum)
//                 } else {
//                     None
//                 }
//             })
//             .last()
//     )
// }

fn main() {
    assert_eq!(
        (0..100)
            .scan(0, |sum, e| {
                *sum += e;
                if condition(e) {
                    Some(*sum)
                } else {
                    None
                }
            })
            .last(),
        Some(1225)
    )
}

#[inline]
fn condition(e: usize) -> bool {
    e < 50
}
