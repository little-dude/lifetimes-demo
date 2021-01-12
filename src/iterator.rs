#[derive(Debug, Clone, Copy)]
pub struct RiderId;

#[derive(Debug, Clone)]
pub struct Route(Vec<usize>);

pub struct Plan {
    routes: Vec<(RiderId, Route)>,
}

impl Plan {
    fn routes<'a>(&'a self) -> Iter<'a> {
        Iter {
            plan: self,
            cursor: 0
        }
    }


    fn routes_bis<'a>(&'a self) -> impl Iterator<Item = &'a Route>  {
        Iter {
            plan: self,
            cursor: 0
        }
    }

}

struct Iter<'a> {
    plan: &'a Plan,
    cursor: usize,
}

impl<'a> Iterator for Iter<'a> {

    type Item = &'a Route;

    fn next(&mut self) -> Option<Self::Item> {
        match self.plan.routes.get(self.cursor) {
            None => return None,
            Some((_rider, route)) => {
                self.cursor += 1;
                Some(route)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let lower_bound = self.plan.routes.len() - self.cursor;
        let upper_bound = lower_bound;

        (lower_bound, Some(upper_bound))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}


















// impl Plan {
//     fn routes_elided(&self) -> impl Iterator<Item = &Route> {
//         RouteIter {
//             cursor: 0,
//             plan: self,
//         }
//     }

//     fn routes<'a>(&'a self) -> impl Iterator<Item = &'a Route> {
//         RouteIter {
//             cursor: 0,
//             plan: self,
//         }
//     }

//     fn riders<'a, T: Iterator<Item = RiderId> + 'a>(&'a self) -> T {
//         RouteIter {
//             cursor: 0,
//             plan: self,
//         }
//     }
// }

// pub struct RouteIter<'a> {
//     cursor: usize,
//     plan: &'a Plan,
// }

// impl<'a> Iterator for RouteIter<'a> {
//     type Item = &'a Route;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.plan.routes.get(self.cursor) {
//             Some((_rider_id, route)) => {
//                 self.cursor += 1;
//                 Some(route)
//             }
//             None => None,
//         }
//     }
// }

// pub struct RiderIter<'a> {
//     cursor: usize,
//     plan: &'a Plan,
// }

// impl<'a> Iterator for RiderIter<'a> {
//     type Item = RiderId;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.plan.routes.get(self.cursor) {
//             Some((rider_id, _route)) => {
//                 self.cursor += 1;
//                 Some(*rider_id)
//             }
//             None => None,
//         }
//     }
// }
