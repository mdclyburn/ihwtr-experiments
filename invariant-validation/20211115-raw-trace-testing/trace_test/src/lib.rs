#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use std::time::Duration;

#[allow(unused_imports)]
use clockwise_common::comm::{Direction, Class as SignalClass};
#[allow(unused_imports)]
use clockwise_common::{
    criteria::{
        Criterion,
        GPIOCriterion,
        EnergyCriterion,
        EnergyStat,
        Timing,
        ParallelTraceCondition,
        ParallelTraceCriterion,
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
        SampleTestProvider {
            tests: vec![
                Test::new(
                    "collect-traces",
                    (&[]).into_iter().copied(),
                    (&[]).into_iter().copied(),
                    &[Operation::at(0).idle_sync(Duration::from_millis(3000))],
                    &[Criterion::SerialTrace(SerialTraceCriterion::new(&[
                        SerialTraceCondition::new(&[0x01])]))],
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
