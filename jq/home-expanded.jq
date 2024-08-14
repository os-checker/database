def sum: reduce .[] as $x (0; .+$x) ;

[.[] | {
    user, repo, package, count, 
    file_count: [.raw_reports[] | .file ] | length ,
    fmt: [.raw_reports[] | .fmt | length] | sum ,
    clippy_warn: [.raw_reports[] | .clippy_warn | length] | sum ,
    clippy_error: [.raw_reports[] | .clippy_error | length] | sum ,
}]
