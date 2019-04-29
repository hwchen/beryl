create table test_beryl (
    account          UInt32,
    city_state       String,
    number_employees UInt32,
    products         Array(String),
    store_label      String,
    delivers         UInt8 -- bool
) ENGINE = Log
;
