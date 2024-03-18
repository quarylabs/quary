use quary_proto::test::TestType;
use quary_proto::test::TestType::{
    AcceptedValues, GreaterThan, LessThan, NotNull, Relationship, Sql, Unique,
};
use quary_proto::{
    Test, TestAcceptedValues, TestGreaterThan, TestGreaterThanOrEqual, TestLessThan,
    TestLessThanOrEqual, TestMultiColumnUnique, TestNotNull, TestRelationship, TestSqlFile,
    TestUnique,
};

pub trait ToTest {
    fn to_test(&self) -> Test;
}

impl ToTest for TestAcceptedValues {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(AcceptedValues(self.clone())),
        }
    }
}

impl ToTest for TestGreaterThan {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(GreaterThan(self.clone())),
        }
    }
}

impl ToTest for TestLessThan {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(LessThan(self.clone())),
        }
    }
}

impl ToTest for TestUnique {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(Unique(self.clone())),
        }
    }
}

impl ToTest for TestNotNull {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(NotNull(self.clone())),
        }
    }
}

impl ToTest for TestRelationship {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(Relationship(self.clone())),
        }
    }
}

impl ToTest for TestSqlFile {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(Sql(self.clone())),
        }
    }
}

impl ToTest for TestType {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(self.clone()),
        }
    }
}

impl ToTest for TestGreaterThanOrEqual {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(TestType::GreaterThanOrEqual(self.clone())),
        }
    }
}

impl ToTest for TestLessThanOrEqual {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(TestType::LessThanOrEqual(self.clone())),
        }
    }
}

impl ToTest for TestMultiColumnUnique {
    fn to_test(&self) -> Test {
        Test {
            test_type: Some(TestType::MultiColumnUnique(self.clone())),
        }
    }
}
