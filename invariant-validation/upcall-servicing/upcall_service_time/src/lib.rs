use std::time::Duration;

use rand;

#[allow(unused_imports)]
use clockwise_common::comm::{Direction,
                             Class as SignalClass,
                             Signal};
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
        // let step_interval = rand::random::<usize>() % 20 + 13;
        let inputs: Vec<Operation> = (0..)
            .step_by(50)
            .take_while(|t| *t < 3000)
            .map(|t| Operation::at(t))
            .zip((&[true, false]).iter().copied().cycle())
            .map(|(op, sig)| op.input(Signal::Digital(sig), 13))
            .collect();

        SampleTestProvider {
            tests: vec![
                Test::new(
                    "upcall-service-time",
                    (&[]).into_iter().copied(),
                    (&[]).into_iter().copied(),
                    &inputs,
                    &[
                        Criterion::SerialTrace(SerialTraceCriterion::new(&[])),
                        Criterion::Energy(EnergyCriterion::new("system", EnergyStat::Max)),
                        Criterion::Energy(EnergyCriterion::new("system", EnergyStat::Min)),
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
