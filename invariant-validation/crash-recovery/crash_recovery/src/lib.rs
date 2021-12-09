use std::time::Duration;

use rand;

#[allow(unused_imports)]
use clockwise_common::comm::{Direction, Class as SignalClass, Signal};
#[allow(unused_imports)]
use clockwise_common::{
    criteria::{
        Criterion,
        GPIOCriterion,
        EnergyCriterion,
        EnergyStat,
        Timing,
        SerialTraceCondition,
        SerialTraceCriterion,
    },
    facility::EnergyMetering,
    hw::INA219,
    input::TestProvider,
    io,
    io::{
        Device,
        Mapping,
        DeviceInputs,
    },
    test::{
        Operation,
        Test,
    },
};

#[derive(Debug)]
pub struct SampleTestProvider {
    tests: Vec<Test>,
}

impl SampleTestProvider {
    fn new() -> SampleTestProvider {
        let inputs: Vec<Operation> = (0..)
            .step_by(500)
            .take_while(|t| *t <= 3000)
            .map(|t| {
                if t == 0 {
                    Operation::at(0)
                } else {
                    let offset = rand::random::<u64>() % 200;
                    let t_op = if rand::random() { t + offset } else { t - offset };
                    Operation::at(t_op)
                }
            })
            .zip((&[false, true]).iter().copied().cycle())
            .map(|(op, sig)| op.input(Signal::Digital(sig), 13))
            .collect();

        SampleTestProvider {
            tests: vec![
                Test::new(
                    "crash-recovery",
                    (&[]).into_iter().copied(),
                    (&[]).into_iter().copied(),
                    &inputs,
                    &[
                        Criterion::SerialTrace(SerialTraceCriterion::new(&[])),
                        Criterion::Energy(EnergyCriterion::new("system", EnergyStat::Max)),
                    ],
                    true),
            ]
        }
    }
}

impl TestProvider for SampleTestProvider {
    fn tests(&self) -> Box<dyn Iterator<Item = Test> + '_> {
        let it = self.tests.iter()
            .cloned();
        Box::new(it)
    }
}

#[no_mangle]
pub fn get_test_adapter() -> Box<dyn TestProvider> {
    Box::new(SampleTestProvider::new())
}
