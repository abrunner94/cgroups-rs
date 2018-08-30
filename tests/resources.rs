//! Integration test about setting resources using `apply()`
extern crate cgroups;

use cgroups::{Cgroup, Resources, PidResources};
use cgroups::pid::{PidController, PidMax};

#[test]
fn pid_resources() {
    let hier = cgroups::hierarchies::V1::new();
    let cg = Cgroup::new(&hier, String::from("pid_resources"));
    {
        let res = Resources {
            pid: PidResources {
                update_values: true,
                maximum_number_of_processes: PidMax::Value(512),
            },
            ..Default::default()
        };
        cg.apply(&res);

        /* verify */
        let pidcontroller: &PidController = cg.controller_of().unwrap();
        assert_eq!(pidcontroller.get_pid_max(), Some(PidMax::Value(512)));
    }
    cg.delete();
}