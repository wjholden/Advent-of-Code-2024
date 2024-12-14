1..14 | ForEach-Object {
    $bin = "day{0:d2}" -f $_;
    $t = (Measure-Command { cargo run --bin $bin --release --quiet }).TotalSeconds;
    [pscustomobject]@{ Day = $bin; Seconds = $t };
}