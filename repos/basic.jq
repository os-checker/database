import "./jq/basic_" as b;

. as $x
| .cmd
| map({ repo: $x.env.packages[.package_idx].repo, cmd: . })
| map({ key: { user: .repo.user, repo: .repo.repo }, cmd })
| group_by(.key)
| map(
    {
      repo: .[0].key,
      cmd: map(.cmd),
      env: $x.env,
    }
    | { data: b::gen, repo }
  )
| .[]
| "\(.repo.user + "/" + .repo.repo)\t\(.data)"
