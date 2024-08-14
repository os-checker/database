def toNull: if . == 0 then null else . end ;

[.[] | {
    user, repo, package, count: .count | toNull , 
    file_count: [.raw_reports[] | .file ] | length | toNull ,
    fmt: [.raw_reports[] | .fmt | length] | add | toNull ,
    clippy_warn: [.raw_reports[] | .clippy_warn | length] | add | toNull ,
    clippy_error: [.raw_reports[] | .clippy_error | length] | add | toNull ,
}]
