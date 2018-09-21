/*
 * Copyright (c) 2018 Pascal Bach
 *
 * SPDX-License-Identifier:     MIT
 */

extern crate chrono;
extern crate junit_report;

#[test]
fn reference_report() {
    use chrono::{Duration, TimeZone, Utc};
    use junit_report::{Report, TestCase, TestSuite};
    use std::fs::File;
    use std::io::Read;
    use std::str;

    let timestamp = Utc.ymd(2018, 4, 21).and_hms(12, 02, 0);

    let mut r = Report::new();
    let mut ts1 = TestSuite::new("ts1");
    ts1.set_timestamp(timestamp);

    let test_success = TestCase::success("test1", Duration::seconds(15));
    let test_error = TestCase::error(
        "test3",
        Duration::seconds(5),
        "git error",
        "Could not clone",
    );
    let test_failure = TestCase::failure(
        "test2",
        Duration::seconds(10),
        "assert_eq",
        "What was not true",
    );

    ts1.add_testcase(test_success);
    ts1.add_testcase(test_failure);
    ts1.add_testcase(test_error);

    r.add_testsuite(ts1);

    let mut out: Vec<u8> = Vec::new();

    r.write_xml(&mut out).unwrap();

    let report = str::from_utf8(&out).unwrap();

    let mut reference = String::new();

    let mut f = File::open("tests/reference.xml").unwrap();
    f.read_to_string(&mut reference).unwrap();

    assert_eq!(report, reference);
}