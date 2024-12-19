cargo build --release

1..25 | ForEach-Object -Parallel {
    $bin = "target/release/day{0:d2}.exe" -f $_;
    if (Test-Path -Path $bin) {
        $t = (Measure-Command { & $bin }).TotalMilliseconds;
        [pscustomobject]@{ Day = $bin; Milliseconds = $t };
    }
} | Sort-Object -Property Day
