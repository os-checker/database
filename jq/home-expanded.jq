def sum: reduce .[] as $x (0; .+$x) ;

def toNull: if . == 0 then null else . end ;

[.[] | {
    user, repo, package, count: .count | toNull, 
    file_count: [.raw_reports[] | .file ] | length | toNull,
    fmt: [.raw_reports[] | .fmt | length] | sum | toNull ,
    clippy_warn: [.raw_reports[] | .clippy_warn | length] | sum | toNull ,
    clippy_error: [.raw_reports[] | .clippy_error | length] | sum | toNull ,
}]
