mod common;

use dotenv::*;
use std::env;

use crate::common::*;

#[test]
fn test_multiline() {
    let value = "-----BEGIN PRIVATE KEY-----\n-----END PRIVATE KEY-----\\n\\\"QUOTED\\\"";
    let weak = "-----BEGIN PRIVATE KEY-----\n-----END PRIVATE KEY-----\n\"QUOTED\"";
    let dir = tempdir_with_dotenv(&format!(
        r#"
KEY=my\ cool\ value
KEY3="awesome stuff \"mang\"
more
on other
lines"
KEY4='hello '\''fds'"
good ' \'morning"
WEAK="{}"
STRONG='{}'
"#,
        value, value
    ))
    .unwrap();

    dotenv().ok();
    assert_eq!(var("KEY").unwrap(), r#"my cool value"#);
    assert_eq!(
        var("KEY3").unwrap(),
        r#"awesome stuff "mang"
more
on other
lines"#
    );
    assert_eq!(
        var("KEY4").unwrap(),
        r#"hello 'fds
good ' 'morning"#
    );
    assert_eq!(var("WEAK").unwrap(), weak);
    assert_eq!(var("STRONG").unwrap(), value);

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
