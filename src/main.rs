use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    env,
    fmt::Debug,
};

use db_dump::{self, crates::CrateId, versions::VersionId};
use semver::{Version, VersionReq};

type Crates = HashMap<CrateId, String>;
type Versions = HashMap<VersionId, Vec<(CrateId, Version)>>;
type Dependencies = HashMap<CrateId, Vec<Dependency>>;
type Seen = HashSet<CrateId>;

struct Dependency {
    /// Id of the dependency
    pub version_id: VersionId,
    pub req: VersionReq,
}

struct Tree {
    id: CrateId,
    version: Version,
    rev_deps: Vec<Tree>,
}

impl Tree {
    fn recurse(&mut self, dependencies: &Dependencies, vers: &Versions, seen: &mut Seen) {
        let Some(reverse_deps) = dependencies.get(&self.id) else {
            return;
        };

        let trees = reverse_deps
            .iter()
            .filter(|dep| dep.req.matches(&self.version)) // Only get our version
            .filter_map(|dep| vers.get(&dep.version_id))
            .flatten()
            .map(|x| Tree {
                id: x.0,
                version: x.1.clone(),
                rev_deps: vec![],
            });

        for mut tree in trees {
            if seen.insert(tree.id) {
                // First time we see it
                tree.recurse(dependencies, vers, seen);
            }
            self.rev_deps.push(tree);
        }
    }

    fn dbg<'a>(&'a self, crates: &'a Crates) -> impl Debug + 'a {
        struct D<'a, 'b>(&'a Crates, &'b Tree);
        impl<'a, 'b> Debug for D<'a, 'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{}: {}", self.0[&self.1.id], self.1.version)?;
                self.1
                    .rev_deps
                    .iter()
                    .try_for_each(|x| write!(f, "{:?}", D(self.0, x)))
            }
        }
        D(crates, &self)
    }
}

fn main() {
    // Just for testing
    let mut args = env::args().skip(1);
    let crate_name = args.next().expect("Usage: forklift [crate] [version]");
    let version: Version = args
        .next() //.unwrap_or("*".into()) TODO: Use VersionReq instead
        .expect("Usage: forklift [crate] [version]")
        .parse()
        .unwrap();

    // PERF: Use with_capacity
    let mut dependencies: Dependencies = HashMap::new();
    let mut versions: Versions = HashMap::new();
    let mut crates: Crates = HashMap::new();
    let mut seen: Seen = HashSet::new();

    let mut tree = None;
    db_dump::Loader::new()
        .crates(|row| {
            if row.name == crate_name {
                seen.insert(row.id);
                tree = Some(Tree {
                    id: row.id,
                    version: version.clone(),
                    rev_deps: vec![],
                })
            }
            crates.insert(row.id, row.name);
        })
        .versions(|row| {
            let val = (row.crate_id, row.num);
            match versions.entry(row.id) {
                Entry::Occupied(mut vec) => vec.get_mut().push(val),
                Entry::Vacant(a) => {
                    a.insert(vec![val]);
                }
            };
        })
        .dependencies(|row| {
            let val = Dependency {
                req: row.req,
                version_id: row.version_id,
            };

            match dependencies.entry(row.crate_id) {
                Entry::Occupied(mut vec) => vec.get_mut().push(val),
                Entry::Vacant(a) => {
                    a.insert(vec![val]);
                }
            };
        })
        .load("./db-dump.tar.gz")
        .expect("Missing 'db-dump.tar.gz'. This can be downloaded from https://static.crates.io/db-dump.tar.gz");

    let mut tree = tree.expect("crate not found!");

    tree.recurse(&dependencies, &versions, &mut seen);

    println!("{:?}", tree.dbg(&crates));
    println!("found {} reverse dependencies!", seen.len());
}
