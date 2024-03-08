use std::{env, fs::File, io::BufReader};

use anyhow::{bail, Result};
use trans_map::{CellType, MapCell, Point, TransMap};

mod trans_map;

#[derive(Copy, Clone, Debug)]
struct Trans {
    from: Option<Point>,
    to: Point,
    price: u32,
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        bail!("Invalid number of arguments");
    }

    let file = File::open(&args[1])?;
    let map = TransMap::from_file(BufReader::new(file))?;

    ucs(&map)
}

fn ucs(map: &TransMap) -> Result<()> {
    let start = map.iter().find(|(_, c)| {
        matches!(
            c,
            Some(MapCell {
                typ: CellType::Start,
                ..
            })
        )
    });

    let Some((to, _)) = start else {
        bail!("No starting possition.");
    };

    let mut open = vec![Trans {
        from: None,
        to,
        price: 0,
    }];
    let mut closed = vec![];

    let mut end = Point::new(usize::MAX, usize::MAX);

    let found = loop {
        print_state(&open, &closed);
        let Some((i, _)) =
            open.iter().enumerate().min_by_key(|(_, t)| t.price)
        else {
            break None;
        };
        let t = open.remove(i);
        if t.to == end {
            break Some(t);
        }

        if !closed.iter().any(|i: &Trans| i.to == t.to) {
            closed.push(t);
        }

        for (p, c) in
            t.to.surround()
                .filter(|p| !closed.iter().any(|t| t.to == *p))
                .filter_map(|p| map.at(p).map(|c| (p, c)))
        {
            if c.typ == CellType::Finish {
                end = p;
            }
            let new_t = Trans {
                from: Some(t.to),
                to: p,
                price: t.price + c.price,
            };

            if let Some((i, t)) =
                open.iter().enumerate().find(|(_, t)| t.to == p)
            {
                if t.price > new_t.price {
                    open.remove(i);
                    open.push(new_t);
                }
            } else {
                open.push(new_t);
            }
        }
    };
    print_state(&open, &closed);

    let Some(last) = found else {
        return Ok(());
    };

    let mut route = vec![last];

    while let Some(Trans {
        from: Some(from), ..
    }) = route.last()
    {
        let Some(next) = closed.iter().find(|t| t.to == *from) else {
            bail!("Failed to backtrack the route");
        };
        route.push(*next);
    }

    print_route(&route);

    Ok(())
}

fn print_state(open: &Vec<Trans>, closed: &Vec<Trans>) {
    println!("Open:");
    print_transs(open);
    println!("Closed:");
    print_transs(closed);
    println!();
}

fn print_transs(transs: &Vec<Trans>) {
    for Trans {
        from,
        to: Point { x, y },
        price,
    } in transs
    {
        if let Some(Point { x: fx, y: fy }) = from {
            print!("([{y}, {x}], {price}, [{fy}, {fx}]), ");
        } else {
            print!("([{y}, {x}], {price}, [null]), ");
        }
    }
    println!();
}

fn print_route(route: &[Trans]) {
    for Trans {
        to: Point { x, y }, ..
    } in route.iter().rev()
    {
        print!("[{y}, {x}], ");
    }
    println!()
}
