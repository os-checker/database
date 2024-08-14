# 这里使用了两层聚合和两次展平
group_by(.user) | map({
    key1: .[0].user,
    key2: group_by(.repo) | map({

        repo: .[0].repo,
        count: map(.count) | add,
        file_count: map(.file_count) | add,

        fmt: map(.fmt) | add,
        clippy_warn: map(.clippy_warn) | add,
        clippy_error: map(.clippy_error) | add,
    }),
}) | map({

    user: .key1,
    repo: .key2[] | { repo, count, file_count, fmt, clippy_warn, clippy_error },
}) | map({

    user, repo: .repo.repo, count: .repo.count, 
    fmt: .repo.fmt, clippy_warn: .repo.clippy_warn, clippy_error: .repo.clippy_error
})
